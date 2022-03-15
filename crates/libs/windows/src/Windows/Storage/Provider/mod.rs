#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CachedFileOptions(pub u32);
impl CachedFileOptions {
    pub const None: Self = Self(0u32);
    pub const RequireUpdateOnAccess: Self = Self(1u32);
    pub const UseCachedFileWhenOffline: Self = Self(2u32);
    pub const DenyAccessWhenOffline: Self = Self(4u32);
}
impl ::core::marker::Copy for CachedFileOptions {}
impl ::core::clone::Clone for CachedFileOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CachedFileOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CachedFileOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for CachedFileOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CachedFileOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CachedFileOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CachedFileOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CachedFileOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CachedFileOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CachedFileTarget(pub i32);
impl CachedFileTarget {
    pub const Local: Self = Self(0i32);
    pub const Remote: Self = Self(1i32);
}
impl ::core::marker::Copy for CachedFileTarget {}
impl ::core::clone::Clone for CachedFileTarget {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CachedFileTarget {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CachedFileTarget {
    type Abi = Self;
}
impl ::core::fmt::Debug for CachedFileTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileTarget").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileTarget {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.CachedFileTarget;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct CachedFileUpdater {}
impl CachedFileUpdater {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetUpdateInformation<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(file: Param0, contentid: Param1, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::Result<()> {
        Self::ICachedFileUpdaterStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetUpdateInformation)(::core::mem::transmute_copy(this), file.into_param().abi(), contentid.into_param().abi(), readmode, writemode, options).ok() })
    }
    #[doc(hidden)]
    pub fn ICachedFileUpdaterStatics<R, F: FnOnce(&ICachedFileUpdaterStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CachedFileUpdater, ICachedFileUpdaterStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for CachedFileUpdater {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdater";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct CachedFileUpdaterUI(::windows::core::IUnknown);
impl CachedFileUpdaterUI {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn UpdateTarget(&self) -> ::windows::core::Result<CachedFileTarget> {
        let this = self;
        unsafe {
            let mut result__: CachedFileTarget = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateTarget)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CachedFileTarget>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FileUpdateRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, FileUpdateRequestedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileUpdateRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveFileUpdateRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFileUpdateRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UIRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<CachedFileUpdaterUI, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UIRequested)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUIRequested<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUIRequested)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn UIStatus(&self) -> ::windows::core::Result<UIStatus> {
        let this = self;
        unsafe {
            let mut result__: UIStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UIStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UIStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn UpdateRequest(&self) -> ::windows::core::Result<FileUpdateRequest> {
        let this = &::windows::core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateRequest)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateRequest>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral> {
        let this = &::windows::core::Interface::cast::<ICachedFileUpdaterUI2>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateRequestDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for CachedFileUpdaterUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CachedFileUpdaterUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CachedFileUpdaterUI {}
impl ::core::fmt::Debug for CachedFileUpdaterUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CachedFileUpdaterUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CachedFileUpdaterUI {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.CachedFileUpdaterUI;{9e6f41e6-baf2-4a97-b600-9333f5df80fd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_Vtbl;
    const IID: ::windows::core::GUID = <ICachedFileUpdaterUI as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CachedFileUpdaterUI {
    const NAME: &'static str = "Windows.Storage.Provider.CachedFileUpdaterUI";
}
impl ::core::convert::From<CachedFileUpdaterUI> for ::windows::core::IUnknown {
    fn from(value: CachedFileUpdaterUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterUI> for ::windows::core::IUnknown {
    fn from(value: &CachedFileUpdaterUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CachedFileUpdaterUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CachedFileUpdaterUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CachedFileUpdaterUI> for ::windows::core::IInspectable {
    fn from(value: CachedFileUpdaterUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CachedFileUpdaterUI> for ::windows::core::IInspectable {
    fn from(value: &CachedFileUpdaterUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CachedFileUpdaterUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CachedFileUpdaterUI {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequest(::windows::core::IUnknown);
impl FileUpdateRequest {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn File(&self) -> ::windows::core::Result<super::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).File)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::StorageFile>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<FileUpdateStatus> {
        let this = self;
        unsafe {
            let mut result__: FileUpdateStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetStatus(&self, value: FileUpdateStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn GetDeferral(&self) -> ::windows::core::Result<FileUpdateRequestDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateRequestDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn UpdateLocalFile<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFile>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdateLocalFile)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn UserInputNeededMessage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UserInputNeededMessage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetUserInputNeededMessage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IFileUpdateRequest2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetUserInputNeededMessage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for FileUpdateRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequest {}
impl ::core::fmt::Debug for FileUpdateRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequest").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequest {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequest;{40c82536-c1fe-4d93-a792-1e736bc70837})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileUpdateRequest {
    type Vtable = IFileUpdateRequest_Vtbl;
    const IID: ::windows::core::GUID = <IFileUpdateRequest as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequest {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequest";
}
impl ::core::convert::From<FileUpdateRequest> for ::windows::core::IUnknown {
    fn from(value: FileUpdateRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUpdateRequest> for ::windows::core::IUnknown {
    fn from(value: &FileUpdateRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileUpdateRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileUpdateRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileUpdateRequest> for ::windows::core::IInspectable {
    fn from(value: FileUpdateRequest) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUpdateRequest> for ::windows::core::IInspectable {
    fn from(value: &FileUpdateRequest) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileUpdateRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileUpdateRequest {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequestDeferral(::windows::core::IUnknown);
impl FileUpdateRequestDeferral {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for FileUpdateRequestDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequestDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequestDeferral {}
impl ::core::fmt::Debug for FileUpdateRequestDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequestDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequestDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequestDeferral;{ffcedb2b-8ade-44a5-bb00-164c4e72f13a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IFileUpdateRequestDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequestDeferral {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestDeferral";
}
impl ::core::convert::From<FileUpdateRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: FileUpdateRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUpdateRequestDeferral> for ::windows::core::IUnknown {
    fn from(value: &FileUpdateRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileUpdateRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileUpdateRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileUpdateRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: FileUpdateRequestDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUpdateRequestDeferral> for ::windows::core::IInspectable {
    fn from(value: &FileUpdateRequestDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileUpdateRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileUpdateRequestDeferral {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct FileUpdateRequestedEventArgs(::windows::core::IUnknown);
impl FileUpdateRequestedEventArgs {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Request(&self) -> ::windows::core::Result<FileUpdateRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Request)(::core::mem::transmute_copy(this), &mut result__).from_abi::<FileUpdateRequest>(result__)
        }
    }
}
impl ::core::clone::Clone for FileUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileUpdateRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileUpdateRequestedEventArgs {}
impl ::core::fmt::Debug for FileUpdateRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateRequestedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateRequestedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.FileUpdateRequestedEventArgs;{7b0a9342-3905-438d-aaef-78ae265f8dd2})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IFileUpdateRequestedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Storage.Provider.FileUpdateRequestedEventArgs";
}
impl ::core::convert::From<FileUpdateRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: FileUpdateRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUpdateRequestedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &FileUpdateRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for FileUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a FileUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<FileUpdateRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: FileUpdateRequestedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileUpdateRequestedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &FileUpdateRequestedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for FileUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a FileUpdateRequestedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct FileUpdateStatus(pub i32);
impl FileUpdateStatus {
    pub const Incomplete: Self = Self(0i32);
    pub const Complete: Self = Self(1i32);
    pub const UserInputNeeded: Self = Self(2i32);
    pub const CurrentlyUnavailable: Self = Self(3i32);
    pub const Failed: Self = Self(4i32);
    pub const CompleteAndRenamed: Self = Self(5i32);
}
impl ::core::marker::Copy for FileUpdateStatus {}
impl ::core::clone::Clone for FileUpdateStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FileUpdateStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FileUpdateStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for FileUpdateStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileUpdateStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileUpdateStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.FileUpdateStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterStatics {
    type Vtable = ICachedFileUpdaterStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc90920_7bcf_4888_a81e_102d7034d7ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetUpdateInformation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::windows::core::RawPtr, contentid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, readmode: ReadActivationMode, writemode: WriteActivationMode, options: CachedFileOptions) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterUI(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterUI {
    type Vtable = ICachedFileUpdaterUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e6f41e6_baf2_4a97_b600_9333f5df80fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub UpdateTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut CachedFileTarget) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub FileUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FileUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveFileUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveFileUpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub UIRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UIRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUIRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUIRequested: usize,
    pub UIStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UIStatus) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICachedFileUpdaterUI2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICachedFileUpdaterUI2 {
    type Vtable = ICachedFileUpdaterUI2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8856a21c_8699_4340_9f49_f7cad7fe8991);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICachedFileUpdaterUI2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UpdateRequest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUpdateRequest {
    type Vtable = IFileUpdateRequest_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x40c82536_c1fe_4d93_a792_1e736bc70837);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub File: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut FileUpdateStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: FileUpdateStatus) -> ::windows::core::HRESULT,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UpdateLocalFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequest2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUpdateRequest2 {
    type Vtable = IFileUpdateRequest2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82484648_bdbe_447b_a2ee_7afe6a032a94);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequest2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub UserInputNeededMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetUserInputNeededMessage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequestDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUpdateRequestDeferral {
    type Vtable = IFileUpdateRequestDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xffcedb2b_8ade_44a5_bb00_164c4e72f13a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestDeferral_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileUpdateRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileUpdateRequestedEventArgs {
    type Vtable = IFileUpdateRequestedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0a9342_3905_438d_aaef_78ae265f8dd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileUpdateRequestedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderError(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderError {
    type Vtable = IStorageProviderError_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47f2780b_ef7f_5910_bf83_331d89256615);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderError_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetFilePath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PrimaryAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPrimaryAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SecondaryAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetSecondaryAction: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InformationalLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetInformationalLink: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderErrorCommand(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderErrorCommand {
    type Vtable = IStorageProviderErrorCommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6b18aed_bb65_5f26_86e4_1d3e34d54477);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorCommand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ActionUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ActionUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderErrorCommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderErrorCommandFactory {
    type Vtable = IStorageProviderErrorCommandFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xecc1f555_3ab4_556f_8bb2_7e5515eed8dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorCommandFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, actionuri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateInstance: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderErrorFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderErrorFactory {
    type Vtable = IStorageProviderErrorFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x97d6f240_61ab_51dc_9921_18bd0dbef79e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderErrorFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1955b9c1_0184_5a88_87df_4544f464365d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FileExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderFileTypeInfoFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderFileTypeInfoFactory {
    type Vtable = IStorageProviderFileTypeInfoFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3fa12c6f_cce6_5d5d_80b1_389e7cf92dbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderFileTypeInfoFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fileextension: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, iconresource: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderGetContentInfoForPathResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2564711d_aa89_4d12_82e3_f72a92e33966);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetContentInfoForPathResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub ContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContentId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderGetPathForContentUriResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x63711a9d_4118_45a6_acb6_22c49d019f40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderGetPathForContentUriResult_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Status: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub SetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderUriSourceStatus) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderHandlerFactory(::windows::core::IUnknown);
impl IStorageProviderHandlerFactory {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn GetStatusSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, syncrootid: Param0) -> ::windows::core::Result<IStorageProviderStatusSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatusSource)(::core::mem::transmute_copy(this), syncrootid.into_param().abi(), &mut result__).from_abi::<IStorageProviderStatusSource>(result__)
        }
    }
}
impl ::core::convert::From<IStorageProviderHandlerFactory> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderHandlerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderHandlerFactory> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderHandlerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageProviderHandlerFactory> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderHandlerFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderHandlerFactory> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderHandlerFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderHandlerFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageProviderHandlerFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderHandlerFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderHandlerFactory {}
impl ::core::fmt::Debug for IStorageProviderHandlerFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderHandlerFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderHandlerFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6154dc3a-fc1d-5aae-9e23-e8659a22c5f6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageProviderHandlerFactory {
    type Vtable = IStorageProviderHandlerFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6154dc3a_fc1d_5aae_9e23_e8659a22c5f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderHandlerFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetStatusSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootid: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemPropertiesStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertiesStatics {
    type Vtable = IStorageProviderItemPropertiesStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d2c1c97_2704_4729_8fa9_7e6b8e158c2f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertiesStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub SetAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr, itemproperties: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemProperty(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x476cb558_730b_4188_b7b5_63b716ed476d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemProperty_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderItemPropertyDefinition(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc5b383bb_ff1f_4298_831e_ff1c08089690);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertyDefinition_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderItemPropertySource(::windows::core::IUnknown);
impl IStorageProviderItemPropertySource {
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, itempath: Param0) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetItemProperties)(::core::mem::transmute_copy(this), itempath.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>(result__)
        }
    }
}
impl ::core::convert::From<IStorageProviderItemPropertySource> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderItemPropertySource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderItemPropertySource> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderItemPropertySource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderItemPropertySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderItemPropertySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageProviderItemPropertySource> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderItemPropertySource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderItemPropertySource> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderItemPropertySource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderItemPropertySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderItemPropertySource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageProviderItemPropertySource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderItemPropertySource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderItemPropertySource {}
impl ::core::fmt::Debug for IStorageProviderItemPropertySource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderItemPropertySource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderItemPropertySource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8f6f9c3e-f632-4a9b-8d99-d2d7a11df56a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageProviderItemPropertySource {
    type Vtable = IStorageProviderItemPropertySource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f6f9c3e_f632_4a9b_8d99_d2d7a11df56a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderItemPropertySource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itempath: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemProperties: usize,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderPropertyCapabilities(::windows::core::IUnknown);
impl IStorageProviderPropertyCapabilities {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn IsPropertySupported<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, propertycanonicalname: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPropertySupported)(::core::mem::transmute_copy(this), propertycanonicalname.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IStorageProviderPropertyCapabilities> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderPropertyCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderPropertyCapabilities> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderPropertyCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderPropertyCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderPropertyCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageProviderPropertyCapabilities> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderPropertyCapabilities) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderPropertyCapabilities> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderPropertyCapabilities) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderPropertyCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderPropertyCapabilities {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageProviderPropertyCapabilities {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderPropertyCapabilities {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderPropertyCapabilities {}
impl ::core::fmt::Debug for IStorageProviderPropertyCapabilities {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderPropertyCapabilities").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderPropertyCapabilities {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{658d2f0e-63b7-4567-acf9-51abe301dda5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageProviderPropertyCapabilities {
    type Vtable = IStorageProviderPropertyCapabilities_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x658d2f0e_63b7_4567_acf9_51abe301dda5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderPropertyCapabilities_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPropertySupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertycanonicalname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderStatus(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderStatus {
    type Vtable = IStorageProviderStatus_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff6e761d_fb8b_56c3_9e7a_05309d191fb4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatus_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Message: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub State: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderState) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ErrorMessages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ErrorMessages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderStatusFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderStatusFactory {
    type Vtable = IStorageProviderStatusFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd64828c5_9b7a_5fa4_b126_90bd18936c7f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: StorageProviderState, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateInstance2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: StorageProviderState, message: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, errormessages: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateInstance2: usize,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderStatusSource(::windows::core::IUnknown);
impl IStorageProviderStatusSource {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn GetStatus(&self) -> ::windows::core::Result<StorageProviderStatus> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStatus)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Changed<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::TypedEventHandler<IStorageProviderStatusSource, ::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Changed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IStorageProviderStatusSource> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderStatusSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderStatusSource> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderStatusSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderStatusSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderStatusSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageProviderStatusSource> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderStatusSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderStatusSource> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderStatusSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderStatusSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderStatusSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageProviderStatusSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderStatusSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderStatusSource {}
impl ::core::fmt::Debug for IStorageProviderStatusSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderStatusSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderStatusSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2e316bb2-fd43-5335-b3c4-a962ee31d17e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageProviderStatusSource {
    type Vtable = IStorageProviderStatusSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e316bb2_fd43_5335_b3c4_a962ee31d17e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderStatusSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Changed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Changed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c1305c4_99f9_41ac_8904_ab055d654926);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub Context: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    Context: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetContext: usize,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetDisplayNameResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetIconResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub HydrationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHydrationPolicy) -> ::windows::core::HRESULT,
    pub SetHydrationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHydrationPolicy) -> ::windows::core::HRESULT,
    pub HydrationPolicyModifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT,
    pub SetHydrationPolicyModifier: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::HRESULT,
    pub PopulationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderPopulationPolicy) -> ::windows::core::HRESULT,
    pub SetPopulationPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderPopulationPolicy) -> ::windows::core::HRESULT,
    pub InSyncPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderInSyncPolicy) -> ::windows::core::HRESULT,
    pub SetInSyncPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderInSyncPolicy) -> ::windows::core::HRESULT,
    pub HardlinkPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT,
    pub SetHardlinkPolicy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderHardlinkPolicy) -> ::windows::core::HRESULT,
    pub ShowSiblingsAsGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShowSiblingsAsGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ProtectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut StorageProviderProtectionMode) -> ::windows::core::HRESULT,
    pub SetProtectionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: StorageProviderProtectionMode) -> ::windows::core::HRESULT,
    pub AllowPinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetAllowPinning: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub StorageProviderItemPropertyDefinitions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StorageProviderItemPropertyDefinitions: usize,
    #[cfg(feature = "Foundation")]
    pub RecycleBinUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RecycleBinUri: usize,
    #[cfg(feature = "Foundation")]
    pub SetRecycleBinUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRecycleBinUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo2 {
    type Vtable = IStorageProviderSyncRootInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf51b023_7cf1_5166_bdba_efd95f529e31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub ProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub SetProviderId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootInfo3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootInfo3 {
    type Vtable = IStorageProviderSyncRootInfo3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x507a6617_bef6_56fd_855e_75ace2e45cf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootInfo3_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub FallbackFileTypeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    FallbackFileTypeInfo: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootManagerStatics {
    type Vtable = IStorageProviderSyncRootManagerStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e99fbbf_8fe3_4b40_abc7_f6fc3d74c98e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Register: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, syncrootinformation: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Unregister: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetSyncRootInformationForFolder: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, folder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetSyncRootInformationForId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetCurrentSyncRoots: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetCurrentSyncRoots: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IStorageProviderSyncRootManagerStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IStorageProviderSyncRootManagerStatics2 {
    type Vtable = IStorageProviderSyncRootManagerStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xefb6cfee_1374_544e_9df1_5598d2e9cfdd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderSyncRootManagerStatics2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsSupported: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct IStorageProviderUriSource(::windows::core::IUnknown);
impl IStorageProviderUriSource {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn GetPathForContentUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, StorageProviderGetPathForContentUriResult>>(&self, contenturi: Param0, result: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetPathForContentUri)(::core::mem::transmute_copy(this), contenturi.into_param().abi(), result.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn GetContentInfoForPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, StorageProviderGetContentInfoForPathResult>>(&self, path: Param0, result: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetContentInfoForPath)(::core::mem::transmute_copy(this), path.into_param().abi(), result.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IStorageProviderUriSource> for ::windows::core::IUnknown {
    fn from(value: IStorageProviderUriSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderUriSource> for ::windows::core::IUnknown {
    fn from(value: &IStorageProviderUriSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStorageProviderUriSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStorageProviderUriSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStorageProviderUriSource> for ::windows::core::IInspectable {
    fn from(value: IStorageProviderUriSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageProviderUriSource> for ::windows::core::IInspectable {
    fn from(value: &IStorageProviderUriSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStorageProviderUriSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStorageProviderUriSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStorageProviderUriSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageProviderUriSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageProviderUriSource {}
impl ::core::fmt::Debug for IStorageProviderUriSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageProviderUriSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageProviderUriSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{b29806d1-8be0-4962-8bb6-0d4c2e14d47a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageProviderUriSource {
    type Vtable = IStorageProviderUriSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb29806d1_8be0_4962_8bb6_0d4c2e14d47a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageProviderUriSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetPathForContentUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenturi: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub GetContentInfoForPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ReadActivationMode(pub i32);
impl ReadActivationMode {
    pub const NotNeeded: Self = Self(0i32);
    pub const BeforeAccess: Self = Self(1i32);
}
impl ::core::marker::Copy for ReadActivationMode {}
impl ::core::clone::Clone for ReadActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ReadActivationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ReadActivationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for ReadActivationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ReadActivationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ReadActivationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.ReadActivationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderError(::windows::core::IUnknown);
impl StorageProviderError {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn FilePath(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FilePath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetFilePath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetFilePath)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn PrimaryAction(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryAction)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetPrimaryAction<'a, Param0: ::windows::core::IntoParam<'a, StorageProviderErrorCommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrimaryAction)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SecondaryAction(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SecondaryAction)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetSecondaryAction<'a, Param0: ::windows::core::IntoParam<'a, StorageProviderErrorCommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSecondaryAction)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn InformationalLink(&self) -> ::windows::core::Result<StorageProviderErrorCommand> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InformationalLink)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderErrorCommand>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetInformationalLink<'a, Param0: ::windows::core::IntoParam<'a, StorageProviderErrorCommand>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInformationalLink)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0, title: Param1, message: Param2) -> ::windows::core::Result<StorageProviderError> {
        Self::IStorageProviderErrorFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), id.into_param().abi(), title.into_param().abi(), message.into_param().abi(), &mut result__).from_abi::<StorageProviderError>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderErrorFactory<R, F: FnOnce(&IStorageProviderErrorFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderError, IStorageProviderErrorFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageProviderError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderError {}
impl ::core::fmt::Debug for StorageProviderError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderError {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderError;{47f2780b-ef7f-5910-bf83-331d89256615})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderError {
    type Vtable = IStorageProviderError_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderError as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderError {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderError";
}
impl ::core::convert::From<StorageProviderError> for ::windows::core::IUnknown {
    fn from(value: StorageProviderError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderError> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderError> for ::windows::core::IInspectable {
    fn from(value: StorageProviderError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderError> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderError {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderError {}
unsafe impl ::core::marker::Sync for StorageProviderError {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderErrorCommand(::windows::core::IUnknown);
impl StorageProviderErrorCommand {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ActionUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ActionUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(label: Param0, actionuri: Param1) -> ::windows::core::Result<StorageProviderErrorCommand> {
        Self::IStorageProviderErrorCommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), label.into_param().abi(), actionuri.into_param().abi(), &mut result__).from_abi::<StorageProviderErrorCommand>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderErrorCommandFactory<R, F: FnOnce(&IStorageProviderErrorCommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderErrorCommand, IStorageProviderErrorCommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageProviderErrorCommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderErrorCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderErrorCommand {}
impl ::core::fmt::Debug for StorageProviderErrorCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderErrorCommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderErrorCommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderErrorCommand;{b6b18aed-bb65-5f26-86e4-1d3e34d54477})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderErrorCommand {
    type Vtable = IStorageProviderErrorCommand_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderErrorCommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderErrorCommand {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderErrorCommand";
}
impl ::core::convert::From<StorageProviderErrorCommand> for ::windows::core::IUnknown {
    fn from(value: StorageProviderErrorCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderErrorCommand> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderErrorCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderErrorCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderErrorCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderErrorCommand> for ::windows::core::IInspectable {
    fn from(value: StorageProviderErrorCommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderErrorCommand> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderErrorCommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderErrorCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderErrorCommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderErrorCommand {}
unsafe impl ::core::marker::Sync for StorageProviderErrorCommand {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderFileTypeInfo(::windows::core::IUnknown);
impl StorageProviderFileTypeInfo {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn FileExtension(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FileExtension)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IconResource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(fileextension: Param0, iconresource: Param1) -> ::windows::core::Result<StorageProviderFileTypeInfo> {
        Self::IStorageProviderFileTypeInfoFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), fileextension.into_param().abi(), iconresource.into_param().abi(), &mut result__).from_abi::<StorageProviderFileTypeInfo>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderFileTypeInfoFactory<R, F: FnOnce(&IStorageProviderFileTypeInfoFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderFileTypeInfo, IStorageProviderFileTypeInfoFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageProviderFileTypeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderFileTypeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderFileTypeInfo {}
impl ::core::fmt::Debug for StorageProviderFileTypeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderFileTypeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderFileTypeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderFileTypeInfo;{1955b9c1-0184-5a88-87df-4544f464365d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderFileTypeInfo {
    type Vtable = IStorageProviderFileTypeInfo_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderFileTypeInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderFileTypeInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderFileTypeInfo";
}
impl ::core::convert::From<StorageProviderFileTypeInfo> for ::windows::core::IUnknown {
    fn from(value: StorageProviderFileTypeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderFileTypeInfo> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderFileTypeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderFileTypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderFileTypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderFileTypeInfo> for ::windows::core::IInspectable {
    fn from(value: StorageProviderFileTypeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderFileTypeInfo> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderFileTypeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderFileTypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderFileTypeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderFileTypeInfo {}
unsafe impl ::core::marker::Sync for StorageProviderFileTypeInfo {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderGetContentInfoForPathResult(::windows::core::IUnknown);
impl StorageProviderGetContentInfoForPathResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderGetContentInfoForPathResult, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderUriSourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderUriSourceStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn ContentUri(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetContentUri<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn ContentId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ContentId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetContentId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContentId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageProviderGetContentInfoForPathResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderGetContentInfoForPathResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderGetContentInfoForPathResult {}
impl ::core::fmt::Debug for StorageProviderGetContentInfoForPathResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderGetContentInfoForPathResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderGetContentInfoForPathResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult;{2564711d-aa89-4d12-82e3-f72a92e33966})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderGetContentInfoForPathResult {
    type Vtable = IStorageProviderGetContentInfoForPathResult_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderGetContentInfoForPathResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderGetContentInfoForPathResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetContentInfoForPathResult";
}
impl ::core::convert::From<StorageProviderGetContentInfoForPathResult> for ::windows::core::IUnknown {
    fn from(value: StorageProviderGetContentInfoForPathResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderGetContentInfoForPathResult> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderGetContentInfoForPathResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderGetContentInfoForPathResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderGetContentInfoForPathResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderGetContentInfoForPathResult> for ::windows::core::IInspectable {
    fn from(value: StorageProviderGetContentInfoForPathResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderGetContentInfoForPathResult> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderGetContentInfoForPathResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderGetContentInfoForPathResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderGetContentInfoForPathResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderGetContentInfoForPathResult {}
unsafe impl ::core::marker::Sync for StorageProviderGetContentInfoForPathResult {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderGetPathForContentUriResult(::windows::core::IUnknown);
impl StorageProviderGetPathForContentUriResult {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderGetPathForContentUriResult, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Status(&self) -> ::windows::core::Result<StorageProviderUriSourceStatus> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderUriSourceStatus = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Status)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderUriSourceStatus>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetStatus(&self, value: StorageProviderUriSourceStatus) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetStatus)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPath)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageProviderGetPathForContentUriResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderGetPathForContentUriResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderGetPathForContentUriResult {}
impl ::core::fmt::Debug for StorageProviderGetPathForContentUriResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderGetPathForContentUriResult").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderGetPathForContentUriResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderGetPathForContentUriResult;{63711a9d-4118-45a6-acb6-22c49d019f40})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderGetPathForContentUriResult {
    type Vtable = IStorageProviderGetPathForContentUriResult_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderGetPathForContentUriResult as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderGetPathForContentUriResult {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderGetPathForContentUriResult";
}
impl ::core::convert::From<StorageProviderGetPathForContentUriResult> for ::windows::core::IUnknown {
    fn from(value: StorageProviderGetPathForContentUriResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderGetPathForContentUriResult> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderGetPathForContentUriResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderGetPathForContentUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderGetPathForContentUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderGetPathForContentUriResult> for ::windows::core::IInspectable {
    fn from(value: StorageProviderGetPathForContentUriResult) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderGetPathForContentUriResult> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderGetPathForContentUriResult) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderGetPathForContentUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderGetPathForContentUriResult {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderGetPathForContentUriResult {}
unsafe impl ::core::marker::Sync for StorageProviderGetPathForContentUriResult {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageProviderHardlinkPolicy(pub u32);
impl StorageProviderHardlinkPolicy {
    pub const None: Self = Self(0u32);
    pub const Allowed: Self = Self(1u32);
}
impl ::core::marker::Copy for StorageProviderHardlinkPolicy {}
impl ::core::clone::Clone for StorageProviderHardlinkPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHardlinkPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHardlinkPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderHardlinkPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHardlinkPolicy").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderHardlinkPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderHardlinkPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderHardlinkPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHardlinkPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHardlinkPolicy;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageProviderHydrationPolicy(pub i32);
impl StorageProviderHydrationPolicy {
    pub const Partial: Self = Self(0i32);
    pub const Progressive: Self = Self(1i32);
    pub const Full: Self = Self(2i32);
    pub const AlwaysFull: Self = Self(3i32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicy {}
impl ::core::clone::Clone for StorageProviderHydrationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHydrationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHydrationPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderHydrationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHydrationPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHydrationPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageProviderHydrationPolicyModifier(pub u32);
impl StorageProviderHydrationPolicyModifier {
    pub const None: Self = Self(0u32);
    pub const ValidationRequired: Self = Self(1u32);
    pub const StreamingAllowed: Self = Self(2u32);
    pub const AutoDehydrationAllowed: Self = Self(4u32);
    pub const AllowFullRestartHydration: Self = Self(8u32);
}
impl ::core::marker::Copy for StorageProviderHydrationPolicyModifier {}
impl ::core::clone::Clone for StorageProviderHydrationPolicyModifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderHydrationPolicyModifier {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderHydrationPolicyModifier {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderHydrationPolicyModifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderHydrationPolicyModifier").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderHydrationPolicyModifier {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderHydrationPolicyModifier {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderHydrationPolicyModifier {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderHydrationPolicyModifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderHydrationPolicyModifier;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageProviderInSyncPolicy(pub u32);
impl StorageProviderInSyncPolicy {
    pub const Default: Self = Self(0u32);
    pub const FileCreationTime: Self = Self(1u32);
    pub const FileReadOnlyAttribute: Self = Self(2u32);
    pub const FileHiddenAttribute: Self = Self(4u32);
    pub const FileSystemAttribute: Self = Self(8u32);
    pub const DirectoryCreationTime: Self = Self(16u32);
    pub const DirectoryReadOnlyAttribute: Self = Self(32u32);
    pub const DirectoryHiddenAttribute: Self = Self(64u32);
    pub const DirectorySystemAttribute: Self = Self(128u32);
    pub const FileLastWriteTime: Self = Self(256u32);
    pub const DirectoryLastWriteTime: Self = Self(512u32);
    pub const PreserveInsyncForSyncEngine: Self = Self(2147483648u32);
}
impl ::core::marker::Copy for StorageProviderInSyncPolicy {}
impl ::core::clone::Clone for StorageProviderInSyncPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderInSyncPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderInSyncPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderInSyncPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderInSyncPolicy").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for StorageProviderInSyncPolicy {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for StorageProviderInSyncPolicy {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for StorageProviderInSyncPolicy {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for StorageProviderInSyncPolicy {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderInSyncPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderInSyncPolicy;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct StorageProviderItemProperties {}
impl StorageProviderItemProperties {
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAsync<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageItem>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StorageProviderItemProperty>>>(item: Param0, itemproperties: Param1) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        Self::IStorageProviderItemPropertiesStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SetAsync)(::core::mem::transmute_copy(this), item.into_param().abi(), itemproperties.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderItemPropertiesStatics<R, F: FnOnce(&IStorageProviderItemPropertiesStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderItemProperties, IStorageProviderItemPropertiesStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StorageProviderItemProperties {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperties";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderItemProperty(::windows::core::IUnknown);
impl StorageProviderItemProperty {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderItemProperty, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetIconResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIconResource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IconResource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageProviderItemProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderItemProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderItemProperty {}
impl ::core::fmt::Debug for StorageProviderItemProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderItemProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderItemProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderItemProperty;{476cb558-730b-4188-b7b5-63b716ed476d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderItemProperty {
    type Vtable = IStorageProviderItemProperty_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderItemProperty as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderItemProperty {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemProperty";
}
impl ::core::convert::From<StorageProviderItemProperty> for ::windows::core::IUnknown {
    fn from(value: StorageProviderItemProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderItemProperty> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderItemProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderItemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderItemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderItemProperty> for ::windows::core::IInspectable {
    fn from(value: StorageProviderItemProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderItemProperty> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderItemProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderItemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderItemProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderItemProperty {}
unsafe impl ::core::marker::Sync for StorageProviderItemProperty {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderItemPropertyDefinition(::windows::core::IUnknown);
impl StorageProviderItemPropertyDefinition {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderItemPropertyDefinition, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetId(&self, value: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayNameResource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetDisplayNameResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayNameResource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for StorageProviderItemPropertyDefinition {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderItemPropertyDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderItemPropertyDefinition {}
impl ::core::fmt::Debug for StorageProviderItemPropertyDefinition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderItemPropertyDefinition").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderItemPropertyDefinition {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderItemPropertyDefinition;{c5b383bb-ff1f-4298-831e-ff1c08089690})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderItemPropertyDefinition {
    type Vtable = IStorageProviderItemPropertyDefinition_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderItemPropertyDefinition as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderItemPropertyDefinition {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderItemPropertyDefinition";
}
impl ::core::convert::From<StorageProviderItemPropertyDefinition> for ::windows::core::IUnknown {
    fn from(value: StorageProviderItemPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderItemPropertyDefinition> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderItemPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderItemPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderItemPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderItemPropertyDefinition> for ::windows::core::IInspectable {
    fn from(value: StorageProviderItemPropertyDefinition) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderItemPropertyDefinition> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderItemPropertyDefinition) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderItemPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderItemPropertyDefinition {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderItemPropertyDefinition {}
unsafe impl ::core::marker::Sync for StorageProviderItemPropertyDefinition {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageProviderPopulationPolicy(pub i32);
impl StorageProviderPopulationPolicy {
    pub const Full: Self = Self(1i32);
    pub const AlwaysFull: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderPopulationPolicy {}
impl ::core::clone::Clone for StorageProviderPopulationPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderPopulationPolicy {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderPopulationPolicy {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderPopulationPolicy {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderPopulationPolicy").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderPopulationPolicy {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderPopulationPolicy;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageProviderProtectionMode(pub i32);
impl StorageProviderProtectionMode {
    pub const Unknown: Self = Self(0i32);
    pub const Personal: Self = Self(1i32);
}
impl ::core::marker::Copy for StorageProviderProtectionMode {}
impl ::core::clone::Clone for StorageProviderProtectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderProtectionMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderProtectionMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderProtectionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderProtectionMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderProtectionMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderProtectionMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageProviderState(pub i32);
impl StorageProviderState {
    pub const InSync: Self = Self(0i32);
    pub const Syncing: Self = Self(1i32);
    pub const Paused: Self = Self(2i32);
    pub const Error: Self = Self(3i32);
    pub const Warning: Self = Self(4i32);
    pub const Offline: Self = Self(5i32);
}
impl ::core::marker::Copy for StorageProviderState {}
impl ::core::clone::Clone for StorageProviderState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderState {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderState {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderState;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderStatus(::windows::core::IUnknown);
impl StorageProviderStatus {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Message(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Message)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn State(&self) -> ::windows::core::Result<StorageProviderState> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderState = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).State)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderState>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ErrorMessages(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderError>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ErrorMessages)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StorageProviderError>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn CreateInstance<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(state: StorageProviderState, message: Param1) -> ::windows::core::Result<StorageProviderStatus> {
        Self::IStorageProviderStatusFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), state, message.into_param().abi(), &mut result__).from_abi::<StorageProviderStatus>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateInstance2<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::Collections::IIterable<StorageProviderError>>>(state: StorageProviderState, message: Param1, errormessages: Param2) -> ::windows::core::Result<StorageProviderStatus> {
        Self::IStorageProviderStatusFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance2)(::core::mem::transmute_copy(this), state, message.into_param().abi(), errormessages.into_param().abi(), &mut result__).from_abi::<StorageProviderStatus>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderStatusFactory<R, F: FnOnce(&IStorageProviderStatusFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderStatus, IStorageProviderStatusFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for StorageProviderStatus {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderStatus {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderStatus {}
impl ::core::fmt::Debug for StorageProviderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderStatus;{ff6e761d-fb8b-56c3-9e7a-05309d191fb4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderStatus {
    type Vtable = IStorageProviderStatus_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderStatus as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderStatus {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderStatus";
}
impl ::core::convert::From<StorageProviderStatus> for ::windows::core::IUnknown {
    fn from(value: StorageProviderStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderStatus> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderStatus> for ::windows::core::IInspectable {
    fn from(value: StorageProviderStatus) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderStatus> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderStatus) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderStatus {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderStatus {}
unsafe impl ::core::marker::Sync for StorageProviderStatus {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
pub struct StorageProviderSyncRootInfo(::windows::core::IUnknown);
impl StorageProviderSyncRootInfo {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootInfo, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn Context(&self) -> ::windows::core::Result<super::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Context)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Streams::IBuffer>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetContext<'a, Param0: ::windows::core::IntoParam<'a, super::Streams::IBuffer>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContext)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<super::IStorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IStorageFolder>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPath)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn DisplayNameResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DisplayNameResource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetDisplayNameResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDisplayNameResource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn IconResource(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IconResource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetIconResource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIconResource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn HydrationPolicy(&self) -> ::windows::core::Result<StorageProviderHydrationPolicy> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderHydrationPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HydrationPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderHydrationPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetHydrationPolicy(&self, value: StorageProviderHydrationPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHydrationPolicy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn HydrationPolicyModifier(&self) -> ::windows::core::Result<StorageProviderHydrationPolicyModifier> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderHydrationPolicyModifier = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HydrationPolicyModifier)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderHydrationPolicyModifier>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetHydrationPolicyModifier(&self, value: StorageProviderHydrationPolicyModifier) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHydrationPolicyModifier)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn PopulationPolicy(&self) -> ::windows::core::Result<StorageProviderPopulationPolicy> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderPopulationPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PopulationPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderPopulationPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetPopulationPolicy(&self, value: StorageProviderPopulationPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPopulationPolicy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn InSyncPolicy(&self) -> ::windows::core::Result<StorageProviderInSyncPolicy> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderInSyncPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).InSyncPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderInSyncPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetInSyncPolicy(&self, value: StorageProviderInSyncPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInSyncPolicy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn HardlinkPolicy(&self) -> ::windows::core::Result<StorageProviderHardlinkPolicy> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderHardlinkPolicy = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HardlinkPolicy)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderHardlinkPolicy>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetHardlinkPolicy(&self, value: StorageProviderHardlinkPolicy) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetHardlinkPolicy)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn ShowSiblingsAsGroup(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowSiblingsAsGroup)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetShowSiblingsAsGroup(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShowSiblingsAsGroup)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Version)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetVersion<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVersion)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn ProtectionMode(&self) -> ::windows::core::Result<StorageProviderProtectionMode> {
        let this = self;
        unsafe {
            let mut result__: StorageProviderProtectionMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProtectionMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<StorageProviderProtectionMode>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetProtectionMode(&self, value: StorageProviderProtectionMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetProtectionMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn AllowPinning(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AllowPinning)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetAllowPinning(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetAllowPinning)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StorageProviderItemPropertyDefinitions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).StorageProviderItemPropertyDefinitions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<StorageProviderItemPropertyDefinition>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RecycleBinUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RecycleBinUri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRecycleBinUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRecycleBinUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn ProviderId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe {
            let mut result__: ::windows::core::GUID = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ProviderId)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn SetProviderId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProviderId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn FallbackFileTypeInfo(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>> {
        let this = &::windows::core::Interface::cast::<IStorageProviderSyncRootInfo3>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FallbackFileTypeInfo)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<StorageProviderFileTypeInfo>>(result__)
        }
    }
}
impl ::core::clone::Clone for StorageProviderSyncRootInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for StorageProviderSyncRootInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for StorageProviderSyncRootInfo {}
impl ::core::fmt::Debug for StorageProviderSyncRootInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderSyncRootInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderSyncRootInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.Provider.StorageProviderSyncRootInfo;{7c1305c4-99f9-41ac-8904-ab055d654926})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for StorageProviderSyncRootInfo {
    type Vtable = IStorageProviderSyncRootInfo_Vtbl;
    const IID: ::windows::core::GUID = <IStorageProviderSyncRootInfo as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for StorageProviderSyncRootInfo {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootInfo";
}
impl ::core::convert::From<StorageProviderSyncRootInfo> for ::windows::core::IUnknown {
    fn from(value: StorageProviderSyncRootInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderSyncRootInfo> for ::windows::core::IUnknown {
    fn from(value: &StorageProviderSyncRootInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for StorageProviderSyncRootInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a StorageProviderSyncRootInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<StorageProviderSyncRootInfo> for ::windows::core::IInspectable {
    fn from(value: StorageProviderSyncRootInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&StorageProviderSyncRootInfo> for ::windows::core::IInspectable {
    fn from(value: &StorageProviderSyncRootInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for StorageProviderSyncRootInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a StorageProviderSyncRootInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for StorageProviderSyncRootInfo {}
unsafe impl ::core::marker::Sync for StorageProviderSyncRootInfo {}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
pub struct StorageProviderSyncRootManager {}
impl StorageProviderSyncRootManager {
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Register<'a, Param0: ::windows::core::IntoParam<'a, StorageProviderSyncRootInfo>>(syncrootinformation: Param0) -> ::windows::core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Register)(::core::mem::transmute_copy(this), syncrootinformation.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn Unregister<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<()> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe { (::windows::core::Interface::vtable(this).Unregister)(::core::mem::transmute_copy(this), id.into_param().abi()).ok() })
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn GetSyncRootInformationForFolder<'a, Param0: ::windows::core::IntoParam<'a, super::IStorageFolder>>(folder: Param0) -> ::windows::core::Result<StorageProviderSyncRootInfo> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSyncRootInformationForFolder)(::core::mem::transmute_copy(this), folder.into_param().abi(), &mut result__).from_abi::<StorageProviderSyncRootInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn GetSyncRootInformationForId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(id: Param0) -> ::windows::core::Result<StorageProviderSyncRootInfo> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSyncRootInformationForId)(::core::mem::transmute_copy(this), id.into_param().abi(), &mut result__).from_abi::<StorageProviderSyncRootInfo>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Provider\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetCurrentSyncRoots() -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>> {
        Self::IStorageProviderSyncRootManagerStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCurrentSyncRoots)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVectorView<StorageProviderSyncRootInfo>>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_Provider\"`*"]
    pub fn IsSupported() -> ::windows::core::Result<bool> {
        Self::IStorageProviderSyncRootManagerStatics2(|this| unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSupported)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IStorageProviderSyncRootManagerStatics<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IStorageProviderSyncRootManagerStatics2<R, F: FnOnce(&IStorageProviderSyncRootManagerStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<StorageProviderSyncRootManager, IStorageProviderSyncRootManagerStatics2> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::core::RuntimeName for StorageProviderSyncRootManager {
    const NAME: &'static str = "Windows.Storage.Provider.StorageProviderSyncRootManager";
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct StorageProviderUriSourceStatus(pub i32);
impl StorageProviderUriSourceStatus {
    pub const Success: Self = Self(0i32);
    pub const NoSyncRoot: Self = Self(1i32);
    pub const FileNotFound: Self = Self(2i32);
}
impl ::core::marker::Copy for StorageProviderUriSourceStatus {}
impl ::core::clone::Clone for StorageProviderUriSourceStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for StorageProviderUriSourceStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for StorageProviderUriSourceStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for StorageProviderUriSourceStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("StorageProviderUriSourceStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for StorageProviderUriSourceStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.StorageProviderUriSourceStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UIStatus(pub i32);
impl UIStatus {
    pub const Unavailable: Self = Self(0i32);
    pub const Hidden: Self = Self(1i32);
    pub const Visible: Self = Self(2i32);
    pub const Complete: Self = Self(3i32);
}
impl ::core::marker::Copy for UIStatus {}
impl ::core::clone::Clone for UIStatus {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UIStatus {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UIStatus {
    type Abi = Self;
}
impl ::core::fmt::Debug for UIStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UIStatus").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UIStatus {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.UIStatus;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Storage_Provider\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WriteActivationMode(pub i32);
impl WriteActivationMode {
    pub const ReadOnly: Self = Self(0i32);
    pub const NotNeeded: Self = Self(1i32);
    pub const AfterWrite: Self = Self(2i32);
}
impl ::core::marker::Copy for WriteActivationMode {}
impl ::core::clone::Clone for WriteActivationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WriteActivationMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WriteActivationMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for WriteActivationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WriteActivationMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WriteActivationMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Storage.Provider.WriteActivationMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
