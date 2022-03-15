#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CommonFileQuery {
    type Abi = Self;
}
impl ::core::fmt::Debug for CommonFileQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommonFileQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CommonFileQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.CommonFileQuery;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for CommonFolderQuery {
    type Abi = Self;
}
impl ::core::fmt::Debug for CommonFolderQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CommonFolderQuery").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CommonFolderQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.CommonFolderQuery;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct ContentIndexer(::windows::core::IUnknown);
impl ContentIndexer {
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddAsync<'a, Param0: ::windows::core::IntoParam<'a, IIndexableContent>>(&self, indexablecontent: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddAsync)(::core::mem::transmute_copy(this), indexablecontent.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateAsync<'a, Param0: ::windows::core::IntoParam<'a, IIndexableContent>>(&self, indexablecontent: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateAsync)(::core::mem::transmute_copy(this), indexablecontent.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, contentid: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::core::mem::transmute_copy(this), contentid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn DeleteMultipleAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, contentids: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteMultipleAsync)(::core::mem::transmute_copy(this), contentids.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAllAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RetrievePropertiesAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, contentid: Param0, propertiestoretrieve: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RetrievePropertiesAsync)(::core::mem::transmute_copy(this), contentid.into_param().abi(), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Revision(&self) -> ::windows::core::Result<u64> {
        let this = self;
        unsafe {
            let mut result__: u64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Revision)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u64>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQueryWithSortOrderAndLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<SortEntry>>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, searchfilter: Param0, propertiestoretrieve: Param1, sortorder: Param2, searchfilterlanguage: Param3) -> ::windows::core::Result<ContentIndexerQuery> {
        let this = &::windows::core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateQueryWithSortOrderAndLanguage)(::core::mem::transmute_copy(this), searchfilter.into_param().abi(), propertiestoretrieve.into_param().abi(), sortorder.into_param().abi(), searchfilterlanguage.into_param().abi(), &mut result__).from_abi::<ContentIndexerQuery>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQueryWithSortOrder<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<SortEntry>>>(&self, searchfilter: Param0, propertiestoretrieve: Param1, sortorder: Param2) -> ::windows::core::Result<ContentIndexerQuery> {
        let this = &::windows::core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateQueryWithSortOrder)(::core::mem::transmute_copy(this), searchfilter.into_param().abi(), propertiestoretrieve.into_param().abi(), sortorder.into_param().abi(), &mut result__).from_abi::<ContentIndexerQuery>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateQuery<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, searchfilter: Param0, propertiestoretrieve: Param1) -> ::windows::core::Result<ContentIndexerQuery> {
        let this = &::windows::core::Interface::cast::<IContentIndexerQueryOperations>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateQuery)(::core::mem::transmute_copy(this), searchfilter.into_param().abi(), propertiestoretrieve.into_param().abi(), &mut result__).from_abi::<ContentIndexerQuery>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn GetIndexerWithName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(indexname: Param0) -> ::windows::core::Result<ContentIndexer> {
        Self::IContentIndexerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexerWithName)(::core::mem::transmute_copy(this), indexname.into_param().abi(), &mut result__).from_abi::<ContentIndexer>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn GetIndexer() -> ::windows::core::Result<ContentIndexer> {
        Self::IContentIndexerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexer)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ContentIndexer>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IContentIndexerStatics<R, F: FnOnce(&IContentIndexerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ContentIndexer, IContentIndexerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ContentIndexer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ContentIndexer {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ContentIndexer;{b1767f8d-f698-4982-b05f-3a6e8cab01a2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentIndexer {
    type Vtable = IContentIndexer_Vtbl;
    const IID: ::windows::core::GUID = <IContentIndexer as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentIndexer {
    const NAME: &'static str = "Windows.Storage.Search.ContentIndexer";
}
impl ::core::convert::From<ContentIndexer> for ::windows::core::IUnknown {
    fn from(value: ContentIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIndexer> for ::windows::core::IUnknown {
    fn from(value: &ContentIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentIndexer> for ::windows::core::IInspectable {
    fn from(value: ContentIndexer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIndexer> for ::windows::core::IInspectable {
    fn from(value: &ContentIndexer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentIndexer {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentIndexer {}
unsafe impl ::core::marker::Sync for ContentIndexer {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct ContentIndexerQuery(::windows::core::IUnknown);
impl ContentIndexerQuery {
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCountAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPropertiesAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetPropertiesRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPropertiesRangeAsync)(::core::mem::transmute_copy(this), startindex, maxitems, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetRangeAsync(&self, startindex: u32, maxitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetRangeAsync)(::core::mem::transmute_copy(this), startindex, maxitems, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IIndexableContent>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn QueryFolder(&self) -> ::windows::core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).QueryFolder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFolder>(result__)
        }
    }
}
impl ::core::clone::Clone for ContentIndexerQuery {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ContentIndexerQuery {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ContentIndexerQuery;{70e3b0f8-4bfc-428a-8889-cc51da9a7b9d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ContentIndexerQuery {
    type Vtable = IContentIndexerQuery_Vtbl;
    const IID: ::windows::core::GUID = <IContentIndexerQuery as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ContentIndexerQuery {
    const NAME: &'static str = "Windows.Storage.Search.ContentIndexerQuery";
}
impl ::core::convert::From<ContentIndexerQuery> for ::windows::core::IUnknown {
    fn from(value: ContentIndexerQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIndexerQuery> for ::windows::core::IUnknown {
    fn from(value: &ContentIndexerQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ContentIndexerQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ContentIndexerQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ContentIndexerQuery> for ::windows::core::IInspectable {
    fn from(value: ContentIndexerQuery) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ContentIndexerQuery> for ::windows::core::IInspectable {
    fn from(value: &ContentIndexerQuery) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ContentIndexerQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ContentIndexerQuery {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ContentIndexerQuery {}
unsafe impl ::core::marker::Sync for ContentIndexerQuery {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for DateStackOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for DateStackOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DateStackOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for DateStackOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.DateStackOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for FolderDepth {
    type Abi = Self;
}
impl ::core::fmt::Debug for FolderDepth {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderDepth").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FolderDepth {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.FolderDepth;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexer(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIndexer {
    type Vtable = IContentIndexer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb1767f8d_f698_4982_b05f_3a6e8cab01a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexer_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub AddAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexablecontent: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddAsync: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexablecontent: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub DeleteMultipleAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentids: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    DeleteMultipleAsync: usize,
    #[cfg(feature = "Foundation")]
    pub DeleteAllAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DeleteAllAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub RetrievePropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RetrievePropertiesAsync: usize,
    pub Revision: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerQuery(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIndexerQuery {
    type Vtable = IContentIndexerQuery_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70e3b0f8_4bfc_428a_8889_cc51da9a7b9d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerQuery_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetCountAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetPropertiesRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetPropertiesRangeAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetRangeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetRangeAsync: usize,
    pub QueryFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerQueryOperations(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIndexerQueryOperations {
    type Vtable = IContentIndexerQueryOperations_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x28823e10_4786_42f1_9730_792b3566b150);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerQueryOperations_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQueryWithSortOrderAndLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertiestoretrieve: ::windows::core::RawPtr, sortorder: ::windows::core::RawPtr, searchfilterlanguage: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQueryWithSortOrderAndLanguage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQueryWithSortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertiestoretrieve: ::windows::core::RawPtr, sortorder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQueryWithSortOrder: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, searchfilter: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, propertiestoretrieve: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateQuery: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IContentIndexerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IContentIndexerStatics {
    type Vtable = IContentIndexerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8c488375_b37e_4c60_9ba8_b760fda3e59d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContentIndexerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetIndexerWithName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetIndexer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IIndexableContent(::windows::core::IUnknown);
impl IIndexableContent {
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows::core::Result<super::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IRandomAccessStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStream)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn StreamContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetStreamContentType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStreamContentType)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IIndexableContent> for ::windows::core::IUnknown {
    fn from(value: IIndexableContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIndexableContent> for ::windows::core::IUnknown {
    fn from(value: &IIndexableContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IIndexableContent> for ::windows::core::IInspectable {
    fn from(value: IIndexableContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIndexableContent> for ::windows::core::IInspectable {
    fn from(value: &IIndexableContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IIndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IIndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IIndexableContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IIndexableContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ccf1a05f-d4b5-483a-b06e-e0db1ec420e4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IIndexableContent {
    type Vtable = IIndexableContent_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccf1a05f_d4b5_483a_b06e_e0db1ec420e4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIndexableContent_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Properties: usize,
    #[cfg(feature = "Storage_Streams")]
    pub Stream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Stream: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetStream: usize,
    pub StreamContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetStreamContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IQueryOptions {
    type Vtable = IQueryOptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1e5e46ee_0f45_4838_a8e9_d0479d446c30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptions_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FileTypeFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FileTypeFilter: usize,
    pub FolderDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FolderDepth) -> ::windows::core::HRESULT,
    pub SetFolderDepth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FolderDepth) -> ::windows::core::HRESULT,
    pub ApplicationSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetApplicationSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UserSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUserSearchFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IndexerOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut IndexerOption) -> ::windows::core::HRESULT,
    pub SetIndexerOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: IndexerOption) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub SortOrder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SortOrder: usize,
    pub GroupPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DateStackOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut DateStackOption) -> ::windows::core::HRESULT,
    pub SaveToString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub LoadFromString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_FileProperties")]
    pub SetThumbnailPrefetch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    SetThumbnailPrefetch: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
    pub SetPropertyPrefetch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage_FileProperties")))]
    SetPropertyPrefetch: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptionsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IQueryOptionsFactory {
    type Vtable = IQueryOptionsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x032e1f8c_a9c1_4e71_8011_0dee9d4811a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptionsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateCommonFileQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, filetypefilter: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateCommonFileQuery: usize,
    pub CreateCommonFolderQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IQueryOptionsWithProviderFilter(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IQueryOptionsWithProviderFilter {
    type Vtable = IQueryOptionsWithProviderFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5b9d1026_15c4_44dd_b89a_47a59b7d7c4f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IQueryOptionsWithProviderFilter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderIdFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderIdFilter: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFileQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageFileQueryResult {
    type Vtable = IStorageFileQueryResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52fda447_2baa_412c_b29f_d4b1778efa1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileQueryResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFileQueryResult2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageFileQueryResult2 {
    type Vtable = IStorageFileQueryResult2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e5db9dd_7141_46c4_8be3_e9dc9e27275c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFileQueryResult2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub GetMatchingPropertiesWithRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Data_Text", feature = "Foundation_Collections")))]
    GetMatchingPropertiesWithRanges: usize,
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IStorageFolderQueryOperations(::windows::core::IUnknown);
impl IStorageFolderQueryOperations {
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetIndexedStateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IndexedState>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexedStateAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IndexedState>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows::core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQueryOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateFileQuery(&self, query: CommonFileQuery) -> ::windows::core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQuery)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateFileQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<StorageFileQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows::core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateFolderQuery(&self, query: CommonFolderQuery) -> ::windows::core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQuery)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateFolderQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<StorageFolderQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateItemQuery(&self) -> ::windows::core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateItemQuery)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateItemQueryWithOptions<'a, Param0: ::windows::core::IntoParam<'a, QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateItemQueryWithOptions)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsync)(::core::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: CommonFileQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsync)(::core::mem::transmute_copy(this), query, startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: CommonFolderQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsync)(::core::mem::transmute_copy(this), startindex, maxitemstoretrieve, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn AreQueryOptionsSupported<'a, Param0: ::windows::core::IntoParam<'a, QueryOptions>>(&self, queryoptions: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AreQueryOptionsSupported)(::core::mem::transmute_copy(this), queryoptions.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn IsCommonFolderQuerySupported(&self, query: CommonFolderQuery) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCommonFolderQuerySupported)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn IsCommonFileQuerySupported(&self, query: CommonFileQuery) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCommonFileQuerySupported)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IStorageFolderQueryOperations> for ::windows::core::IUnknown {
    fn from(value: IStorageFolderQueryOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolderQueryOperations> for ::windows::core::IUnknown {
    fn from(value: &IStorageFolderQueryOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageFolderQueryOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageFolderQueryOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageFolderQueryOperations> for ::windows::core::IInspectable {
    fn from(value: IStorageFolderQueryOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageFolderQueryOperations> for ::windows::core::IInspectable {
    fn from(value: &IStorageFolderQueryOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageFolderQueryOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageFolderQueryOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageFolderQueryOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IStorageFolderQueryOperations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cb43ccc9-446b-4a4f-be97-757771be5203}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageFolderQueryOperations {
    type Vtable = IStorageFolderQueryOperations_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb43ccc9_446b_4a4f_be97_757771be5203);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryOperations_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetIndexedStateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetIndexedStateAsync: usize,
    pub CreateFileQueryOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFileQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFileQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFolderQueryOverloadDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFolderQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateFolderQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateItemQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateItemQueryWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncOverloadDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncOverloadDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncOverloadDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    pub AreQueryOptionsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryoptions: ::windows::core::RawPtr, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCommonFolderQuerySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFolderQuery, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCommonFileQuerySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, query: CommonFileQuery, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageFolderQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageFolderQueryResult {
    type Vtable = IStorageFolderQueryResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6654c911_7d66_46fa_aecf_e4a4baa93ab8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderQueryResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemQueryResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageItemQueryResult {
    type Vtable = IStorageItemQueryResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8948079_9d58_47b8_b2b2_41b07f4795f9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemQueryResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxnumberofitems: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncDefaultStartAndCount: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryChangeTrackerTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryChangeTrackerTriggerDetails {
    type Vtable = IStorageLibraryChangeTrackerTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1dc7a369_b7a3_4df2_9d61_eba85a0343d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryChangeTrackerTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ChangeTracker: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageLibraryContentChangedTriggerDetails(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageLibraryContentChangedTriggerDetails {
    type Vtable = IStorageLibraryContentChangedTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2a371977_abbf_4e1d_8aa5_6385d8884799);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageLibraryContentChangedTriggerDetails_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CreateModifiedSinceQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lastquerytime: super::super::Foundation::DateTime, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateModifiedSinceQuery: usize,
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IStorageQueryResultBase(::windows::core::IUnknown);
impl IStorageQueryResultBase {
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemCountAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Folder(&self) -> ::windows::core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContentsChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OptionsChanged)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOptionsChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindStartIndexAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindStartIndexAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<QueryOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentQueryOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<QueryOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn ApplyNewQueryOptions<'a, Param0: ::windows::core::IntoParam<'a, QueryOptions>>(&self, newqueryoptions: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ApplyNewQueryOptions)(::core::mem::transmute_copy(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IStorageQueryResultBase> for ::windows::core::IUnknown {
    fn from(value: IStorageQueryResultBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageQueryResultBase> for ::windows::core::IUnknown {
    fn from(value: &IStorageQueryResultBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageQueryResultBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageQueryResultBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageQueryResultBase> for ::windows::core::IInspectable {
    fn from(value: IStorageQueryResultBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageQueryResultBase> for ::windows::core::IInspectable {
    fn from(value: &IStorageQueryResultBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageQueryResultBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageQueryResultBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageQueryResultBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
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
unsafe impl ::windows::core::RuntimeType for IStorageQueryResultBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c297d70d-7353-47ab-ba58-8c61425dc54b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageQueryResultBase {
    type Vtable = IStorageQueryResultBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc297d70d_7353_47ab_ba58_8c61425dc54b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageQueryResultBase_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub GetItemCountAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemCountAsync: usize,
    pub Folder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ContentsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ContentsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveContentsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveContentsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub OptionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    OptionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveOptionsChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveOptionsChanged: usize,
    #[cfg(feature = "Foundation")]
    pub FindStartIndexAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FindStartIndexAsync: usize,
    pub GetCurrentQueryOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ApplyNewQueryOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newqueryoptions: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IValueAndLanguage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IValueAndLanguage {
    type Vtable = IValueAndLanguage_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb9914881_a1ee_4bc4_92a5_466968e30436);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueAndLanguage_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Language: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct IndexableContent(::windows::core::IUnknown);
impl IndexableContent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<IndexableContent, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Stream(&self) -> ::windows::core::Result<super::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Stream)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IRandomAccessStream>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetStream<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IRandomAccessStream>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStream)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn StreamContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StreamContentType)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetStreamContentType<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStreamContentType)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for IndexableContent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for IndexableContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.IndexableContent;{ccf1a05f-d4b5-483a-b06e-e0db1ec420e4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IndexableContent {
    type Vtable = IIndexableContent_Vtbl;
    const IID: ::windows::core::GUID = <IIndexableContent as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IndexableContent {
    const NAME: &'static str = "Windows.Storage.Search.IndexableContent";
}
impl ::core::convert::From<IndexableContent> for ::windows::core::IUnknown {
    fn from(value: IndexableContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IndexableContent> for ::windows::core::IUnknown {
    fn from(value: &IndexableContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IndexableContent> for ::windows::core::IInspectable {
    fn from(value: IndexableContent) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IndexableContent> for ::windows::core::IInspectable {
    fn from(value: &IndexableContent) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IndexableContent> for IIndexableContent {
    type Error = ::windows::core::Error;
    fn try_from(value: IndexableContent) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IndexableContent> for IIndexableContent {
    type Error = ::windows::core::Error;
    fn try_from(value: &IndexableContent) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IIndexableContent> for IndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, IIndexableContent> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IIndexableContent> for &IndexableContent {
    fn into_param(self) -> ::windows::core::Param<'a, IIndexableContent> {
        ::core::convert::TryInto::<IIndexableContent>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for IndexableContent {}
unsafe impl ::core::marker::Sync for IndexableContent {}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for IndexedState {
    type Abi = Self;
}
impl ::core::fmt::Debug for IndexedState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexedState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IndexedState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexedState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
unsafe impl ::windows::core::Abi for IndexerOption {
    type Abi = Self;
}
impl ::core::fmt::Debug for IndexerOption {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IndexerOption").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IndexerOption {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Search.IndexerOption;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct QueryOptions(::windows::core::IUnknown);
impl QueryOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QueryOptions, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FileTypeFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileTypeFilter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn FolderDepth(&self) -> ::windows::core::Result<FolderDepth> {
        let this = self;
        unsafe {
            let mut result__: FolderDepth = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FolderDepth)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FolderDepth>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetFolderDepth(&self, value: FolderDepth) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFolderDepth)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn ApplicationSearchFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationSearchFilter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetApplicationSearchFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetApplicationSearchFilter)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn UserSearchFilter(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserSearchFilter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetUserSearchFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUserSearchFilter)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn IndexerOption(&self) -> ::windows::core::Result<IndexerOption> {
        let this = self;
        unsafe {
            let mut result__: IndexerOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexerOption)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IndexerOption>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetIndexerOption(&self, value: IndexerOption) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIndexerOption)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SortOrder(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SortEntry>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SortOrder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<SortEntry>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn GroupPropertyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GroupPropertyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn DateStackOption(&self) -> ::windows::core::Result<DateStackOption> {
        let this = self;
        unsafe {
            let mut result__: DateStackOption = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DateStackOption)(::core::mem::transmute_copy(this), &mut result__).from_abi::<DateStackOption>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SaveToString(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SaveToString)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn LoadFromString<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).LoadFromString)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn SetThumbnailPrefetch(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThumbnailPrefetch)(::core::mem::transmute_copy(this), mode, requestedsize, options).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_FileProperties"))]
    pub fn SetPropertyPrefetch<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(&self, options: super::FileProperties::PropertyPrefetchOptions, propertiestoretrieve: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPropertyPrefetch)(::core::mem::transmute_copy(this), options, propertiestoretrieve.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateCommonFileQuery<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>>(query: CommonFileQuery, filetypefilter: Param1) -> ::windows::core::Result<QueryOptions> {
        Self::IQueryOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCommonFileQuery)(::core::mem::transmute_copy(this), query, filetypefilter.into_param().abi(), &mut result__).from_abi::<QueryOptions>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn CreateCommonFolderQuery(query: CommonFolderQuery) -> ::windows::core::Result<QueryOptions> {
        Self::IQueryOptionsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateCommonFolderQuery)(::core::mem::transmute_copy(this), query, &mut result__).from_abi::<QueryOptions>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageProviderIdFilter(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = &::windows::core::Interface::cast::<IQueryOptionsWithProviderFilter>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StorageProviderIdFilter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc(hidden)]
    pub fn IQueryOptionsFactory<R, F: FnOnce(&IQueryOptionsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<QueryOptions, IQueryOptionsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for QueryOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for QueryOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.QueryOptions;{1e5e46ee-0f45-4838-a8e9-d0479d446c30})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for QueryOptions {
    type Vtable = IQueryOptions_Vtbl;
    const IID: ::windows::core::GUID = <IQueryOptions as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for QueryOptions {
    const NAME: &'static str = "Windows.Storage.Search.QueryOptions";
}
impl ::core::convert::From<QueryOptions> for ::windows::core::IUnknown {
    fn from(value: QueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QueryOptions> for ::windows::core::IUnknown {
    fn from(value: &QueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for QueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a QueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<QueryOptions> for ::windows::core::IInspectable {
    fn from(value: QueryOptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&QueryOptions> for ::windows::core::IInspectable {
    fn from(value: &QueryOptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for QueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a QueryOptions {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for QueryOptions {}
unsafe impl ::core::marker::Sync for QueryOptions {}
#[repr(C)]
#[doc = "*Required features: `\"Storage_Search\"`*"]
pub struct SortEntry {
    pub PropertyName: ::windows::core::HSTRING,
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
unsafe impl ::windows::core::Abi for SortEntry {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
unsafe impl ::windows::core::RuntimeType for SortEntry {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Storage.Search.SortEntry;string;b1)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(from.clone())
    }
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
#[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct SortEntryVector(::windows::core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl SortEntryVector {
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterator<SortEntry>> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::Collections::IIterable<SortEntry>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IIterator<SortEntry>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<SortEntry> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<SortEntry> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<SortEntry>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<SortEntry>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<SortEntry>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, SortEntry>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, SortEntry>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, SortEntry>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, SortEntry>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [SortEntry]) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[SortEntry]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceAll)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for SortEntryVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for SortEntryVector {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.SortEntryVector;pinterface({913337e9-11a1-4345-a3a2-4e7f956e222d};struct(Windows.Storage.Search.SortEntry;string;b1)))");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows::core::Interface for SortEntryVector {
    type Vtable = super::super::Foundation::Collections::IVector_Vtbl<SortEntry>;
    const IID: ::windows::core::GUID = <super::super::Foundation::Collections::IVector<SortEntry> as ::windows::core::Interface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows::core::RuntimeName for SortEntryVector {
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
        super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SortEntryVector> for ::windows::core::IUnknown {
    fn from(value: SortEntryVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SortEntryVector> for ::windows::core::IUnknown {
    fn from(value: &SortEntryVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SortEntryVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SortEntryVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<SortEntryVector> for ::windows::core::IInspectable {
    fn from(value: SortEntryVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::From<&SortEntryVector> for ::windows::core::IInspectable {
    fn from(value: &SortEntryVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SortEntryVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SortEntryVector {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SortEntryVector> for super::super::Foundation::Collections::IIterable<SortEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: SortEntryVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SortEntryVector> for super::super::Foundation::Collections::IIterable<SortEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SortEntryVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<SortEntry>> for SortEntryVector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<SortEntry>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<SortEntry>> for &SortEntryVector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IIterable<SortEntry>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IIterable<SortEntry>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<SortEntryVector> for super::super::Foundation::Collections::IVector<SortEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: SortEntryVector) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&SortEntryVector> for super::super::Foundation::Collections::IVector<SortEntry> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SortEntryVector) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<SortEntry>> for SortEntryVector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVector<SortEntry>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IVector<SortEntry>> for &SortEntryVector {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::Foundation::Collections::IVector<SortEntry>> {
        ::core::convert::TryInto::<super::super::Foundation::Collections::IVector<SortEntry>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageFileQueryResult(::windows::core::IUnknown);
impl StorageFileQueryResult {
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsync)(::core::mem::transmute_copy(this), startindex, maxnumberofitems, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Data_Text\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "Data_Text", feature = "Foundation_Collections"))]
    pub fn GetMatchingPropertiesWithRanges<'a, Param0: ::windows::core::IntoParam<'a, super::StorageFile>>(&self, file: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>> {
        let this = &::windows::core::Interface::cast::<IStorageFileQueryResult2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMatchingPropertiesWithRanges)(::core::mem::transmute_copy(this), file.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, super::super::Foundation::Collections::IVectorView<super::super::Data::Text::TextSegment>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemCountAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Folder(&self) -> ::windows::core::Result<super::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContentsChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OptionsChanged)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOptionsChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindStartIndexAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindStartIndexAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<QueryOptions> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentQueryOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<QueryOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn ApplyNewQueryOptions<'a, Param0: ::windows::core::IntoParam<'a, QueryOptions>>(&self, newqueryoptions: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ApplyNewQueryOptions)(::core::mem::transmute_copy(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageFileQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for StorageFileQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageFileQueryResult;{52fda447-2baa-412c-b29f-d4b1778efa1e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageFileQueryResult {
    type Vtable = IStorageFileQueryResult_Vtbl;
    const IID: ::windows::core::GUID = <IStorageFileQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageFileQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageFileQueryResult";
}
impl ::core::convert::From<StorageFileQueryResult> for ::windows::core::IUnknown {
    fn from(value: StorageFileQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFileQueryResult> for ::windows::core::IUnknown {
    fn from(value: &StorageFileQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageFileQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageFileQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageFileQueryResult> for ::windows::core::IInspectable {
    fn from(value: StorageFileQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFileQueryResult> for ::windows::core::IInspectable {
    fn from(value: &StorageFileQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageFileQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageFileQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageFileQueryResult> for IStorageQueryResultBase {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFileQueryResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFileQueryResult> for IStorageQueryResultBase {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFileQueryResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageQueryResultBase> for StorageFileQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageQueryResultBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageQueryResultBase> for &StorageFileQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageQueryResultBase> {
        ::core::convert::TryInto::<IStorageQueryResultBase>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageFolderQueryResult(::windows::core::IUnknown);
impl StorageFolderQueryResult {
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsync)(::core::mem::transmute_copy(this), startindex, maxnumberofitems, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemCountAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Folder(&self) -> ::windows::core::Result<super::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContentsChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OptionsChanged)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOptionsChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindStartIndexAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindStartIndexAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<QueryOptions> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentQueryOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<QueryOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn ApplyNewQueryOptions<'a, Param0: ::windows::core::IntoParam<'a, QueryOptions>>(&self, newqueryoptions: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ApplyNewQueryOptions)(::core::mem::transmute_copy(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageFolderQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for StorageFolderQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageFolderQueryResult;{6654c911-7d66-46fa-aecf-e4a4baa93ab8})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageFolderQueryResult {
    type Vtable = IStorageFolderQueryResult_Vtbl;
    const IID: ::windows::core::GUID = <IStorageFolderQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageFolderQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageFolderQueryResult";
}
impl ::core::convert::From<StorageFolderQueryResult> for ::windows::core::IUnknown {
    fn from(value: StorageFolderQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFolderQueryResult> for ::windows::core::IUnknown {
    fn from(value: &StorageFolderQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageFolderQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageFolderQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageFolderQueryResult> for ::windows::core::IInspectable {
    fn from(value: StorageFolderQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageFolderQueryResult> for ::windows::core::IInspectable {
    fn from(value: &StorageFolderQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageFolderQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageFolderQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageFolderQueryResult> for IStorageQueryResultBase {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageFolderQueryResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageFolderQueryResult> for IStorageQueryResultBase {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageFolderQueryResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageQueryResultBase> for StorageFolderQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageQueryResultBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageQueryResultBase> for &StorageFolderQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageQueryResultBase> {
        ::core::convert::TryInto::<IStorageQueryResultBase>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageItemQueryResult(::windows::core::IUnknown);
impl StorageItemQueryResult {
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxnumberofitems: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsync)(::core::mem::transmute_copy(this), startindex, maxnumberofitems, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsyncDefaultStartAndCount)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemCountAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemCountAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Folder(&self) -> ::windows::core::Result<super::StorageFolder> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ContentsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentsChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveContentsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContentsChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OptionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageQueryResultBase, ::windows::core::IInspectable>>>(&self, changedhandler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).OptionsChanged)(::core::mem::transmute_copy(this), changedhandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveOptionsChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, eventcookie: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveOptionsChanged)(::core::mem::transmute_copy(this), eventcookie.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FindStartIndexAsync<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<u32>> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FindStartIndexAsync)(::core::mem::transmute_copy(this), value.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<u32>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn GetCurrentQueryOptions(&self) -> ::windows::core::Result<QueryOptions> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentQueryOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<QueryOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn ApplyNewQueryOptions<'a, Param0: ::windows::core::IntoParam<'a, QueryOptions>>(&self, newqueryoptions: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageQueryResultBase>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ApplyNewQueryOptions)(::core::mem::transmute_copy(this), newqueryoptions.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageItemQueryResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for StorageItemQueryResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageItemQueryResult;{e8948079-9d58-47b8-b2b2-41b07f4795f9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageItemQueryResult {
    type Vtable = IStorageItemQueryResult_Vtbl;
    const IID: ::windows::core::GUID = <IStorageItemQueryResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageItemQueryResult {
    const NAME: &'static str = "Windows.Storage.Search.StorageItemQueryResult";
}
impl ::core::convert::From<StorageItemQueryResult> for ::windows::core::IUnknown {
    fn from(value: StorageItemQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemQueryResult> for ::windows::core::IUnknown {
    fn from(value: &StorageItemQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageItemQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageItemQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageItemQueryResult> for ::windows::core::IInspectable {
    fn from(value: StorageItemQueryResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageItemQueryResult> for ::windows::core::IInspectable {
    fn from(value: &StorageItemQueryResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageItemQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageItemQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<StorageItemQueryResult> for IStorageQueryResultBase {
    type Error = ::windows::core::Error;
    fn try_from(value: StorageItemQueryResult) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&StorageItemQueryResult> for IStorageQueryResultBase {
    type Error = ::windows::core::Error;
    fn try_from(value: &StorageItemQueryResult) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageQueryResultBase> for StorageItemQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageQueryResultBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IStorageQueryResultBase> for &StorageItemQueryResult {
    fn into_param(self) -> ::windows::core::Param<'a, IStorageQueryResultBase> {
        ::core::convert::TryInto::<IStorageQueryResultBase>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageLibraryChangeTrackerTriggerDetails(::windows::core::IUnknown);
impl StorageLibraryChangeTrackerTriggerDetails {
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Folder(&self) -> ::windows::core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn ChangeTracker(&self) -> ::windows::core::Result<super::StorageLibraryChangeTracker> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ChangeTracker)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageLibraryChangeTracker>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageLibraryChangeTrackerTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for StorageLibraryChangeTrackerTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageLibraryChangeTrackerTriggerDetails;{1dc7a369-b7a3-4df2-9d61-eba85a0343d2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryChangeTrackerTriggerDetails {
    type Vtable = IStorageLibraryChangeTrackerTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IStorageLibraryChangeTrackerTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryChangeTrackerTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.StorageLibraryChangeTrackerTriggerDetails";
}
impl ::core::convert::From<StorageLibraryChangeTrackerTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryChangeTrackerTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryChangeTrackerTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryChangeTrackerTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryChangeTrackerTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryChangeTrackerTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryChangeTrackerTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryChangeTrackerTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryChangeTrackerTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryChangeTrackerTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryChangeTrackerTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct StorageLibraryContentChangedTriggerDetails(::windows::core::IUnknown);
impl StorageLibraryContentChangedTriggerDetails {
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Folder(&self) -> ::windows::core::Result<super::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateModifiedSinceQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::DateTime>>(&self, lastquerytime: Param0) -> ::windows::core::Result<StorageItemQueryResult> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateModifiedSinceQuery)(::core::mem::transmute_copy(this), lastquerytime.into_param().abi(), &mut result__).from_abi::<StorageItemQueryResult>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageLibraryContentChangedTriggerDetails {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for StorageLibraryContentChangedTriggerDetails {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.StorageLibraryContentChangedTriggerDetails;{2a371977-abbf-4e1d-8aa5-6385d8884799})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageLibraryContentChangedTriggerDetails {
    type Vtable = IStorageLibraryContentChangedTriggerDetails_Vtbl;
    const IID: ::windows::core::GUID = <IStorageLibraryContentChangedTriggerDetails as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageLibraryContentChangedTriggerDetails {
    const NAME: &'static str = "Windows.Storage.Search.StorageLibraryContentChangedTriggerDetails";
}
impl ::core::convert::From<StorageLibraryContentChangedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: StorageLibraryContentChangedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryContentChangedTriggerDetails> for ::windows::core::IUnknown {
    fn from(value: &StorageLibraryContentChangedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageLibraryContentChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageLibraryContentChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageLibraryContentChangedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: StorageLibraryContentChangedTriggerDetails) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageLibraryContentChangedTriggerDetails> for ::windows::core::IInspectable {
    fn from(value: &StorageLibraryContentChangedTriggerDetails) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageLibraryContentChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageLibraryContentChangedTriggerDetails {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Search\"`*"]
#[repr(transparent)]
pub struct ValueAndLanguage(::windows::core::IUnknown);
impl ValueAndLanguage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ValueAndLanguage, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for ValueAndLanguage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for ValueAndLanguage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Search.ValueAndLanguage;{b9914881-a1ee-4bc4-92a5-466968e30436})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ValueAndLanguage {
    type Vtable = IValueAndLanguage_Vtbl;
    const IID: ::windows::core::GUID = <IValueAndLanguage as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ValueAndLanguage {
    const NAME: &'static str = "Windows.Storage.Search.ValueAndLanguage";
}
impl ::core::convert::From<ValueAndLanguage> for ::windows::core::IUnknown {
    fn from(value: ValueAndLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValueAndLanguage> for ::windows::core::IUnknown {
    fn from(value: &ValueAndLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ValueAndLanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ValueAndLanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ValueAndLanguage> for ::windows::core::IInspectable {
    fn from(value: ValueAndLanguage) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ValueAndLanguage> for ::windows::core::IInspectable {
    fn from(value: &ValueAndLanguage) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ValueAndLanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ValueAndLanguage {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ValueAndLanguage {}
unsafe impl ::core::marker::Sync for ValueAndLanguage {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
