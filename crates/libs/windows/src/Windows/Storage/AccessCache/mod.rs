#[doc(hidden)]
#[repr(transparent)]
pub struct IItemRemovedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IItemRemovedEventArgs {
    type Vtable = IItemRemovedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IItemRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IItemRemovedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x59677e5c_55be_4c66_ba66_5eaea79d2631);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemRemovedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub RemovedEntry: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<AccessListEntry>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageApplicationPermissionsStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageApplicationPermissionsStatics {
    type Vtable = IStorageApplicationPermissionsStatics_Vtbl;
}
impl ::core::clone::Clone for IStorageApplicationPermissionsStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageApplicationPermissionsStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4391dfaa_d033_48f9_8060_3ec847d2e3f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageApplicationPermissionsStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub FutureAccessList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub MostRecentlyUsedList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageApplicationPermissionsStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageApplicationPermissionsStatics2 {
    type Vtable = IStorageApplicationPermissionsStatics2_Vtbl;
}
impl ::core::clone::Clone for IStorageApplicationPermissionsStatics2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageApplicationPermissionsStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x072716ec_aa05_4294_9a11_1a3d04519ad0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageApplicationPermissionsStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "System")]
    pub GetFutureAccessListForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetFutureAccessListForUser: usize,
    #[cfg(feature = "System")]
    pub GetMostRecentlyUsedListForUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "System"))]
    GetMostRecentlyUsedListForUser: usize,
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct IStorageItemAccessList(::windows_core::IUnknown);
impl IStorageItemAccessList {
    pub fn AddOverloadDefaultMetadata<P0>(&self, file: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), ::core::mem::transmute_copy(metadata), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P0>(&self, token: &::windows_core::HSTRING, file: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), file.try_into_param()?.abi()).ok() }
    }
    pub fn AddOrReplace<P0>(&self, token: &::windows_core::HSTRING, file: P0, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), file.try_into_param()?.abi(), ::core::mem::transmute_copy(metadata)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    pub fn Remove(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token)).ok() }
    }
    pub fn ContainsItem(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsItem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CheckAccess<P0>(&self, file: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows_core::Result<AccessListEntryView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Entries)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaximumItemsAllowed(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaximumItemsAllowed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
::windows_core::imp::interface_hierarchy!(IStorageItemAccessList, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for IStorageItemAccessList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemAccessList {}
impl ::core::fmt::Debug for IStorageItemAccessList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemAccessList").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for IStorageItemAccessList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{2caff6ad-de90-47f5-b2c3-dd36c9fdd453}");
}
unsafe impl ::windows_core::Interface for IStorageItemAccessList {
    type Vtable = IStorageItemAccessList_Vtbl;
}
impl ::core::clone::Clone for IStorageItemAccessList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageItemAccessList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2caff6ad_de90_47f5_b2c3_dd36c9fdd453);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemAccessList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddOverloadDefaultMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, metadata: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddOrReplaceOverloadDefaultMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, file: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub AddOrReplace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, file: *mut ::core::ffi::c_void, metadata: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetItemAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetItemWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetItemWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFileWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFileWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub GetFolderWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, options: AccessCacheOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetFolderWithOptionsAsync: usize,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ContainsItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub CheckAccess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Entries: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Entries: usize,
    pub MaximumItemsAllowed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemMostRecentlyUsedList(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageItemMostRecentlyUsedList {
    type Vtable = IStorageItemMostRecentlyUsedList_Vtbl;
}
impl ::core::clone::Clone for IStorageItemMostRecentlyUsedList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageItemMostRecentlyUsedList {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x016239d5_510d_411e_8cf1_c3d1effa4c33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub ItemRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemRemoved: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemRemoved: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageItemMostRecentlyUsedList2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IStorageItemMostRecentlyUsedList2 {
    type Vtable = IStorageItemMostRecentlyUsedList2_Vtbl;
}
impl ::core::clone::Clone for IStorageItemMostRecentlyUsedList2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IStorageItemMostRecentlyUsedList2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xda481ea0_ed8d_4731_a1db_e44ee2204093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemMostRecentlyUsedList2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub AddWithMetadataAndVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::core::ffi::c_void, metadata: ::std::mem::MaybeUninit<::windows_core::HSTRING>, visibility: RecentStorageItemVisibility, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub AddOrReplaceWithMetadataAndVisibility: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: ::std::mem::MaybeUninit<::windows_core::HSTRING>, file: *mut ::core::ffi::c_void, metadata: ::std::mem::MaybeUninit<::windows_core::HSTRING>, visibility: RecentStorageItemVisibility) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Storage_AccessCache\"`, `\"Foundation_Collections\"`*"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
pub struct AccessListEntryView(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl AccessListEntryView {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::Foundation::Collections::IIterator<AccessListEntry>> {
        let this = &::windows_core::ComInterface::cast::<super::super::Foundation::Collections::IIterable<AccessListEntry>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<AccessListEntry> {
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
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<AccessListEntry>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [AccessListEntry]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::PartialEq for AccessListEntryView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::cmp::Eq for AccessListEntryView {}
#[cfg(feature = "Foundation_Collections")]
impl ::core::fmt::Debug for AccessListEntryView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessListEntryView").field(&self.0).finish()
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for AccessListEntryView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.AccessListEntryView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};struct(Windows.Storage.AccessCache.AccessListEntry;string;string)))");
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::clone::Clone for AccessListEntryView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for AccessListEntryView {
    type Vtable = super::super::Foundation::Collections::IVectorView_Vtbl<AccessListEntry>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for AccessListEntryView {
    const IID: ::windows_core::GUID = <super::super::Foundation::Collections::IVectorView<AccessListEntry> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for AccessListEntryView {
    const NAME: &'static str = "Windows.Storage.AccessCache.AccessListEntryView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &AccessListEntryView {
    type Item = AccessListEntry;
    type IntoIter = super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(AccessListEntryView, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IIterable<AccessListEntry>> for AccessListEntryView {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::Foundation::Collections::IVectorView<AccessListEntry>> for AccessListEntryView {}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct ItemRemovedEventArgs(::windows_core::IUnknown);
impl ItemRemovedEventArgs {
    pub fn RemovedEntry(&self) -> ::windows_core::Result<AccessListEntry> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RemovedEntry)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for ItemRemovedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItemRemovedEventArgs {}
impl ::core::fmt::Debug for ItemRemovedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItemRemovedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ItemRemovedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.ItemRemovedEventArgs;{59677e5c-55be-4c66-ba66-5eaea79d2631})");
}
impl ::core::clone::Clone for ItemRemovedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for ItemRemovedEventArgs {
    type Vtable = IItemRemovedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ItemRemovedEventArgs {
    const IID: ::windows_core::GUID = <IItemRemovedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ItemRemovedEventArgs {
    const NAME: &'static str = "Windows.Storage.AccessCache.ItemRemovedEventArgs";
}
::windows_core::imp::interface_hierarchy!(ItemRemovedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
pub struct StorageApplicationPermissions;
impl StorageApplicationPermissions {
    pub fn FutureAccessList() -> ::windows_core::Result<StorageItemAccessList> {
        Self::IStorageApplicationPermissionsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FutureAccessList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn MostRecentlyUsedList() -> ::windows_core::Result<StorageItemMostRecentlyUsedList> {
        Self::IStorageApplicationPermissionsStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MostRecentlyUsedList)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetFutureAccessListForUser<P0>(user: P0) -> ::windows_core::Result<StorageItemAccessList>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IStorageApplicationPermissionsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFutureAccessListForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"System\"`*"]
    #[cfg(feature = "System")]
    pub fn GetMostRecentlyUsedListForUser<P0>(user: P0) -> ::windows_core::Result<StorageItemMostRecentlyUsedList>
    where
        P0: ::windows_core::IntoParam<super::super::System::User>,
    {
        Self::IStorageApplicationPermissionsStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMostRecentlyUsedListForUser)(::windows_core::Interface::as_raw(this), user.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageApplicationPermissionsStatics<R, F: FnOnce(&IStorageApplicationPermissionsStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IStorageApplicationPermissionsStatics2<R, F: FnOnce(&IStorageApplicationPermissionsStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<StorageApplicationPermissions, IStorageApplicationPermissionsStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeName for StorageApplicationPermissions {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageApplicationPermissions";
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct StorageItemAccessList(::windows_core::IUnknown);
impl StorageItemAccessList {
    pub fn AddOverloadDefaultMetadata<P0>(&self, file: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), ::core::mem::transmute_copy(metadata), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P0>(&self, token: &::windows_core::HSTRING, file: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), file.try_into_param()?.abi()).ok() }
    }
    pub fn AddOrReplace<P0>(&self, token: &::windows_core::HSTRING, file: P0, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), file.try_into_param()?.abi(), ::core::mem::transmute_copy(metadata)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    pub fn Remove(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token)).ok() }
    }
    pub fn ContainsItem(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsItem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CheckAccess<P0>(&self, file: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows_core::Result<AccessListEntryView> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Entries)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaximumItemsAllowed(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaximumItemsAllowed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for StorageItemAccessList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageItemAccessList {}
impl ::core::fmt::Debug for StorageItemAccessList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemAccessList").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StorageItemAccessList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.StorageItemAccessList;{2caff6ad-de90-47f5-b2c3-dd36c9fdd453})");
}
impl ::core::clone::Clone for StorageItemAccessList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StorageItemAccessList {
    type Vtable = IStorageItemAccessList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageItemAccessList {
    const IID: ::windows_core::GUID = <IStorageItemAccessList as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageItemAccessList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemAccessList";
}
::windows_core::imp::interface_hierarchy!(StorageItemAccessList, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IStorageItemAccessList> for StorageItemAccessList {}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
pub struct StorageItemMostRecentlyUsedList(::windows_core::IUnknown);
impl StorageItemMostRecentlyUsedList {
    pub fn AddOverloadDefaultMetadata<P0>(&self, file: P0) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn Add<P0>(&self, file: P0, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Add)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), ::core::mem::transmute_copy(metadata), &mut result__).from_abi(result__)
        }
    }
    pub fn AddOrReplaceOverloadDefaultMetadata<P0>(&self, token: &::windows_core::HSTRING, file: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplaceOverloadDefaultMetadata)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), file.try_into_param()?.abi()).ok() }
    }
    pub fn AddOrReplace<P0>(&self, token: &::windows_core::HSTRING, file: P0, metadata: &::windows_core::HSTRING) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplace)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), file.try_into_param()?.abi(), ::core::mem::transmute_copy(metadata)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFileAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetItemWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFileWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderWithOptionsAsync(&self, token: &::windows_core::HSTRING, options: AccessCacheOptions) -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetFolderWithOptionsAsync)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), options, &mut result__).from_abi(result__)
        }
    }
    pub fn Remove(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token)).ok() }
    }
    pub fn ContainsItem(&self, token: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ContainsItem)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), &mut result__).from_abi(result__)
        }
    }
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn CheckAccess<P0>(&self, file: P0) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CheckAccess)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Entries(&self) -> ::windows_core::Result<AccessListEntryView> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Entries)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaximumItemsAllowed(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<IStorageItemAccessList>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MaximumItemsAllowed)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemRemoved<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::Foundation::TypedEventHandler<StorageItemMostRecentlyUsedList, ItemRemovedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ItemRemoved)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemRemoved(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveItemRemoved)(::windows_core::Interface::as_raw(this), eventcookie).ok() }
    }
    pub fn AddWithMetadataAndVisibility<P0>(&self, file: P0, metadata: &::windows_core::HSTRING, visibility: RecentStorageItemVisibility) -> ::windows_core::Result<::windows_core::HSTRING>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AddWithMetadataAndVisibility)(::windows_core::Interface::as_raw(this), file.try_into_param()?.abi(), ::core::mem::transmute_copy(metadata), visibility, &mut result__).from_abi(result__)
        }
    }
    pub fn AddOrReplaceWithMetadataAndVisibility<P0>(&self, token: &::windows_core::HSTRING, file: P0, metadata: &::windows_core::HSTRING, visibility: RecentStorageItemVisibility) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::IStorageItem>,
    {
        let this = &::windows_core::ComInterface::cast::<IStorageItemMostRecentlyUsedList2>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).AddOrReplaceWithMetadataAndVisibility)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(token), file.try_into_param()?.abi(), ::core::mem::transmute_copy(metadata), visibility).ok() }
    }
}
impl ::core::cmp::PartialEq for StorageItemMostRecentlyUsedList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageItemMostRecentlyUsedList {}
impl ::core::fmt::Debug for StorageItemMostRecentlyUsedList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageItemMostRecentlyUsedList").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for StorageItemMostRecentlyUsedList {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Storage.AccessCache.StorageItemMostRecentlyUsedList;{016239d5-510d-411e-8cf1-c3d1effa4c33})");
}
impl ::core::clone::Clone for StorageItemMostRecentlyUsedList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for StorageItemMostRecentlyUsedList {
    type Vtable = IStorageItemMostRecentlyUsedList_Vtbl;
}
unsafe impl ::windows_core::ComInterface for StorageItemMostRecentlyUsedList {
    const IID: ::windows_core::GUID = <IStorageItemMostRecentlyUsedList as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for StorageItemMostRecentlyUsedList {
    const NAME: &'static str = "Windows.Storage.AccessCache.StorageItemMostRecentlyUsedList";
}
::windows_core::imp::interface_hierarchy!(StorageItemMostRecentlyUsedList, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<IStorageItemAccessList> for StorageItemMostRecentlyUsedList {}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AccessCacheOptions(pub u32);
impl AccessCacheOptions {
    pub const None: Self = Self(0u32);
    pub const DisallowUserInput: Self = Self(1u32);
    pub const FastLocationsOnly: Self = Self(2u32);
    pub const UseReadOnlyCachedCopy: Self = Self(4u32);
    pub const SuppressAccessTimeUpdate: Self = Self(8u32);
}
impl ::core::marker::Copy for AccessCacheOptions {}
impl ::core::clone::Clone for AccessCacheOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AccessCacheOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for AccessCacheOptions {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for AccessCacheOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AccessCacheOptions").field(&self.0).finish()
    }
}
impl AccessCacheOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for AccessCacheOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for AccessCacheOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for AccessCacheOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for AccessCacheOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for AccessCacheOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::windows_core::RuntimeType for AccessCacheOptions {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.AccessCacheOptions;u4)");
}
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RecentStorageItemVisibility(pub i32);
impl RecentStorageItemVisibility {
    pub const AppOnly: Self = Self(0i32);
    pub const AppAndSystem: Self = Self(1i32);
}
impl ::core::marker::Copy for RecentStorageItemVisibility {}
impl ::core::clone::Clone for RecentStorageItemVisibility {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RecentStorageItemVisibility {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for RecentStorageItemVisibility {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for RecentStorageItemVisibility {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RecentStorageItemVisibility").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for RecentStorageItemVisibility {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Storage.AccessCache.RecentStorageItemVisibility;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Storage_AccessCache\"`*"]
pub struct AccessListEntry {
    pub Token: ::windows_core::HSTRING,
    pub Metadata: ::windows_core::HSTRING,
}
impl ::core::clone::Clone for AccessListEntry {
    fn clone(&self) -> Self {
        Self { Token: self.Token.clone(), Metadata: self.Metadata.clone() }
    }
}
impl ::core::fmt::Debug for AccessListEntry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AccessListEntry").field("Token", &self.Token).field("Metadata", &self.Metadata).finish()
    }
}
impl ::windows_core::TypeKind for AccessListEntry {
    type TypeKind = ::windows_core::ValueType;
}
impl ::windows_core::RuntimeType for AccessListEntry {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Storage.AccessCache.AccessListEntry;string;string)");
}
impl ::core::cmp::PartialEq for AccessListEntry {
    fn eq(&self, other: &Self) -> bool {
        self.Token == other.Token && self.Metadata == other.Metadata
    }
}
impl ::core::cmp::Eq for AccessListEntry {}
impl ::core::default::Default for AccessListEntry {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
