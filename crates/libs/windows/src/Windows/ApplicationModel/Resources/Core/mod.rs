#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct INamedResource(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for INamedResource {
    type Vtable = INamedResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for INamedResource {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1c98c219_0b13_4240_89a5_d495dc189a00);
}
#[repr(C)]
#[doc(hidden)]
pub struct INamedResource_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Candidates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Candidates: usize,
    pub Resolve: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub ResolveForContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcecontext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ResolveAllForContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcecontext: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResolveAllForContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceCandidate(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceCandidate {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaf5207d9_c433_4764_b3fd_8fa6bfbcbadc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Qualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Qualifiers: usize,
    pub IsMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMatchAsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub ValueAsString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub GetValueAsFileAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage")))]
    GetValueAsFileAsync: usize,
    pub GetQualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiername: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceCandidate2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCandidate2 {
    type Vtable = IResourceCandidate2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceCandidate2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x69e5b468_f6fc_4013_aaa2_d53f1757d3b5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub GetValueAsStreamAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    GetValueAsStreamAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceCandidate3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceCandidate3 {
    type Vtable = IResourceCandidate3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceCandidate3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x08ae97f8_517a_4674_958c_4a3c7cd2cc6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceCandidate3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Kind: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ResourceCandidateKind) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceContext(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContext {
    type Vtable = IResourceContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceContext {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2fa22f4b_707e_4b27_ad0d_d0d8cd468fd2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContext_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub QualifierValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    QualifierValues: usize,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetQualifierValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiernames: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetQualifierValues: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub OverrideToMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    OverrideToMatch: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Languages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Languages: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SetLanguages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languages: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SetLanguages: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceContextStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContextStatics {
    type Vtable = IResourceContextStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceContextStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x98be9d6c_6338_4b31_99df_b2b442f17149);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub CreateMatchingContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CreateMatchingContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceContextStatics2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContextStatics2 {
    type Vtable = IResourceContextStatics2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceContextStatics2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x41f752ef_12af_41b9_ab36_b1eb4b512460);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub SetGlobalQualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ResetGlobalQualifierValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub ResetGlobalQualifierValuesForSpecifiedQualifiers: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, qualifiernames: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ResetGlobalQualifierValuesForSpecifiedQualifiers: usize,
    pub GetForViewIndependentUse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceContextStatics3(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContextStatics3 {
    type Vtable = IResourceContextStatics3_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceContextStatics3 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x20cf492c_af0f_450b_9da6_106dd0c29a39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics3_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub SetGlobalQualifierValueWithPersistence: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, key: ::std::mem::MaybeUninit<::windows_core::HSTRING>, value: ::std::mem::MaybeUninit<::windows_core::HSTRING>, persistence: ResourceQualifierPersistence) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceContextStatics4(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceContextStatics4 {
    type Vtable = IResourceContextStatics4_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceContextStatics4 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x22eb9ccd_fb31_4bfa_b86b_df9d9d7bdc39);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceContextStatics4_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub GetForUIContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    GetForUIContext: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceManager(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceManager {
    type Vtable = IResourceManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceManager {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf744d97b_9988_44fb_abd6_5378844cfa8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub MainResourceMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub AllResourceMaps: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    AllResourceMaps: usize,
    pub DefaultContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub LoadPriFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    LoadPriFiles: usize,
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub UnloadPriFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, files: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Storage")))]
    UnloadPriFiles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceManager2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceManager2 {
    type Vtable = IResourceManager2_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceManager2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x9d66fe6c_a4d7_4c23_9e85_675f304c252d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManager2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllNamedResourcesForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllNamedResourcesForPackage: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetAllSubtreesForPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagename: ::std::mem::MaybeUninit<::windows_core::HSTRING>, resourcelayoutinfo: ResourceLayoutInfo, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetAllSubtreesForPackage: usize,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceManagerStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceManagerStatics {
    type Vtable = IResourceManagerStatics_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceManagerStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x1cc0fdfc_69ee_4e43_9901_47f12687baf7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceManagerStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub IsResourceReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcereference: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut bool) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceMap(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceMap {
    type Vtable = IResourceMap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceMap {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x72284824_db8c_42f8_b08c_53ff357dad82);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceMap_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetValueForContext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: ::std::mem::MaybeUninit<::windows_core::HSTRING>, context: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub GetSubtree: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, reference: ::std::mem::MaybeUninit<::windows_core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct IResourceQualifier(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IResourceQualifier {
    type Vtable = IResourceQualifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for IResourceQualifier {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x785da5b2_4afd_4376_a888_c5f9a6b7a05c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResourceQualifier_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub QualifierName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub QualifierValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub IsDefault: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub IsMatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub Score: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct NamedResource(::windows_core::IUnknown);
impl NamedResource {
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Candidates(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Candidates)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Resolve(&self) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Resolve)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ResolveForContext<P0>(&self, resourcecontext: P0) -> ::windows_core::Result<ResourceCandidate>
    where
        P0: ::windows_core::IntoParam<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolveForContext)(::windows_core::Interface::as_raw(this), resourcecontext.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResolveAll(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolveAll)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResolveAllForContext<P0>(&self, resourcecontext: P0) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>>
    where
        P0: ::windows_core::IntoParam<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ResolveAllForContext)(::windows_core::Interface::as_raw(this), resourcecontext.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for NamedResource {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.NamedResource;{1c98c219-0b13-4240-89a5-d495dc189a00})");
}
unsafe impl ::windows_core::Interface for NamedResource {
    type Vtable = INamedResource_Vtbl;
}
unsafe impl ::windows_core::ComInterface for NamedResource {
    const IID: ::windows_core::GUID = <INamedResource as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for NamedResource {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.NamedResource";
}
::windows_core::imp::interface_hierarchy!(NamedResource, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for NamedResource {}
unsafe impl ::core::marker::Sync for NamedResource {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceCandidate(::windows_core::IUnknown);
impl ResourceCandidate {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Qualifiers(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Qualifiers)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMatch(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMatch)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMatchAsDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMatchAsDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ValueAsString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ValueAsString)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage"))]
    pub fn GetValueAsFileAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValueAsFileAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetQualifierValue(&self, qualifiername: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetQualifierValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(qualifiername), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation\"`, `\"Storage_Streams\"`"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn GetValueAsStreamAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Storage::Streams::IRandomAccessStream>> {
        let this = &::windows_core::ComInterface::cast::<IResourceCandidate2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValueAsStreamAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Kind(&self) -> ::windows_core::Result<ResourceCandidateKind> {
        let this = &::windows_core::ComInterface::cast::<IResourceCandidate3>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Kind)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ResourceCandidate {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceCandidate;{af5207d9-c433-4764-b3fd-8fa6bfbcbadc})");
}
unsafe impl ::windows_core::Interface for ResourceCandidate {
    type Vtable = IResourceCandidate_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceCandidate {
    const IID: ::windows_core::GUID = <IResourceCandidate as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceCandidate {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidate";
}
::windows_core::imp::interface_hierarchy!(ResourceCandidate, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ResourceCandidate {}
unsafe impl ::core::marker::Sync for ResourceCandidate {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceCandidateVectorView(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceCandidateVectorView {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<ResourceCandidate>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<ResourceCandidate>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<ResourceCandidate>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ResourceCandidate>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for ResourceCandidateVectorView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.ApplicationModel.Resources.Core.ResourceCandidate;{af5207d9-c433-4764-b3fd-8fa6bfbcbadc})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for ResourceCandidateVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<ResourceCandidate>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for ResourceCandidateVectorView {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVectorView<ResourceCandidate> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ResourceCandidateVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceCandidateVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceCandidateVectorView {
    type Item = ResourceCandidate;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(ResourceCandidateVectorView, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<ResourceCandidate>> for ResourceCandidateVectorView {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<ResourceCandidate>> for ResourceCandidateVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceCandidateVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceCandidateVectorView {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceContext(::windows_core::IUnknown);
impl ResourceContext {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceContext, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn QualifierValues(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).QualifierValues)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Reset(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Reset)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetQualifierValues<P0>(&self, qualifiernames: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ResetQualifierValues)(::windows_core::Interface::as_raw(this), qualifiernames.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn OverrideToMatch<P0>(&self, result: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OverrideToMatch)(::windows_core::Interface::as_raw(this), result.try_into_param()?.abi()).ok() }
    }
    pub fn Clone(&self) -> ::windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Clone)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Languages(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Languages)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetLanguages<P0>(&self, languages: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetLanguages)(::windows_core::Interface::as_raw(this), languages.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateMatchingContext<P0>(result: P0) -> ::windows_core::Result<ResourceContext>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>,
    {
        Self::IResourceContextStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateMatchingContext)(::windows_core::Interface::as_raw(this), result.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    pub fn GetForCurrentView() -> ::windows_core::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForCurrentView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetGlobalQualifierValue(key: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).SetGlobalQualifierValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(value)).ok() })
    }
    pub fn ResetGlobalQualifierValues() -> ::windows_core::Result<()> {
        Self::IResourceContextStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).ResetGlobalQualifierValues)(::windows_core::Interface::as_raw(this)).ok() })
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ResetGlobalQualifierValuesForSpecifiedQualifiers<P0>(qualifiernames: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>,
    {
        Self::IResourceContextStatics2(|this| unsafe { (::windows_core::Interface::vtable(this).ResetGlobalQualifierValuesForSpecifiedQualifiers)(::windows_core::Interface::as_raw(this), qualifiernames.try_into_param()?.abi()).ok() })
    }
    pub fn GetForViewIndependentUse() -> ::windows_core::Result<ResourceContext> {
        Self::IResourceContextStatics2(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForViewIndependentUse)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn SetGlobalQualifierValueWithPersistence(key: &::windows_core::HSTRING, value: &::windows_core::HSTRING, persistence: ResourceQualifierPersistence) -> ::windows_core::Result<()> {
        Self::IResourceContextStatics3(|this| unsafe { (::windows_core::Interface::vtable(this).SetGlobalQualifierValueWithPersistence)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(value), persistence).ok() })
    }
    #[doc = "Required features: `\"UI\"`"]
    #[cfg(feature = "UI")]
    pub fn GetForUIContext<P0>(context: P0) -> ::windows_core::Result<ResourceContext>
    where
        P0: ::windows_core::IntoParam<super::super::super::UI::UIContext>,
    {
        Self::IResourceContextStatics4(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetForUIContext)(::windows_core::Interface::as_raw(this), context.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceContextStatics<R, F: FnOnce(&IResourceContextStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceContext, IResourceContextStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceContextStatics2<R, F: FnOnce(&IResourceContextStatics2) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceContext, IResourceContextStatics2> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceContextStatics3<R, F: FnOnce(&IResourceContextStatics3) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceContext, IResourceContextStatics3> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IResourceContextStatics4<R, F: FnOnce(&IResourceContextStatics4) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceContext, IResourceContextStatics4> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ResourceContext {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceContext;{2fa22f4b-707e-4b27-ad0d-d0d8cd468fd2})");
}
unsafe impl ::windows_core::Interface for ResourceContext {
    type Vtable = IResourceContext_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceContext {
    const IID: ::windows_core::GUID = <IResourceContext as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceContext {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContext";
}
::windows_core::imp::interface_hierarchy!(ResourceContext, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ResourceContext {}
unsafe impl ::core::marker::Sync for ResourceContext {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceContextLanguagesVectorView(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceContextLanguagesVectorView {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf(&self, value: &::windows_core::HSTRING, index: &mut u32) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(value), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::windows_core::HSTRING]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for ResourceContextLanguagesVectorView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for ResourceContextLanguagesVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for ResourceContextLanguagesVectorView {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ResourceContextLanguagesVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceContextLanguagesVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceContextLanguagesVectorView {
    type Item = ::windows_core::HSTRING;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceContextLanguagesVectorView {
    type Item = ::windows_core::HSTRING;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(ResourceContextLanguagesVectorView, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<::windows_core::HSTRING>> for ResourceContextLanguagesVectorView {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<::windows_core::HSTRING>> for ResourceContextLanguagesVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceContextLanguagesVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceContextLanguagesVectorView {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceManager(::windows_core::IUnknown);
impl ResourceManager {
    pub fn MainResourceMap(&self) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MainResourceMap)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn AllResourceMaps(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AllResourceMaps)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultContext(&self) -> ::windows_core::Result<ResourceContext> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DefaultContext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn LoadPriFiles<P0>(&self, files: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).LoadPriFiles)(::windows_core::Interface::as_raw(this), files.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`, `\"Storage\"`"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage"))]
    pub fn UnloadPriFiles<P0>(&self, files: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<super::super::super::Storage::IStorageFile>>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnloadPriFiles)(::windows_core::Interface::as_raw(this), files.try_into_param()?.abi()).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllNamedResourcesForPackage(&self, packagename: &::windows_core::HSTRING, resourcelayoutinfo: ResourceLayoutInfo) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<NamedResource>> {
        let this = &::windows_core::ComInterface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllNamedResourcesForPackage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagename), resourcelayoutinfo, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAllSubtreesForPackage(&self, packagename: &::windows_core::HSTRING, resourcelayoutinfo: ResourceLayoutInfo) -> ::windows_core::Result<super::super::super::Foundation::Collections::IVectorView<ResourceMap>> {
        let this = &::windows_core::ComInterface::cast::<IResourceManager2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAllSubtreesForPackage)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(packagename), resourcelayoutinfo, &mut result__).from_abi(result__)
        }
    }
    pub fn Current() -> ::windows_core::Result<ResourceManager> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn IsResourceReference(resourcereference: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        Self::IResourceManagerStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsResourceReference)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(resourcereference), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IResourceManagerStatics<R, F: FnOnce(&IResourceManagerStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<ResourceManager, IResourceManagerStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for ResourceManager {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceManager;{f744d97b-9988-44fb-abd6-5378844cfa8b})");
}
unsafe impl ::windows_core::Interface for ResourceManager {
    type Vtable = IResourceManager_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceManager {
    const IID: ::windows_core::GUID = <IResourceManager as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceManager {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceManager";
}
::windows_core::imp::interface_hierarchy!(ResourceManager, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ResourceManager {}
unsafe impl ::core::marker::Sync for ResourceManager {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceMap(::windows_core::IUnknown);
impl ResourceMap {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<NamedResource> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
    #[doc = "Required features: `\"Foundation\"`"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Uri)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn GetValue(&self, resource: &::windows_core::HSTRING) -> ::windows_core::Result<ResourceCandidate> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValue)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), &mut result__).from_abi(result__)
        }
    }
    pub fn GetValueForContext<P0>(&self, resource: &::windows_core::HSTRING, context: P0) -> ::windows_core::Result<ResourceCandidate>
    where
        P0: ::windows_core::IntoParam<ResourceContext>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetValueForContext)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(resource), context.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    pub fn GetSubtree(&self, reference: &::windows_core::HSTRING) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetSubtree)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(reference), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ResourceMap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82})");
}
unsafe impl ::windows_core::Interface for ResourceMap {
    type Vtable = IResourceMap_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceMap {
    const IID: ::windows_core::GUID = <IResourceMap as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMap";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
::windows_core::imp::interface_hierarchy!(ResourceMap, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> for ResourceMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, NamedResource>> for ResourceMap {}
unsafe impl ::core::marker::Send for ResourceMap {}
unsafe impl ::core::marker::Sync for ResourceMap {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceMapIterator(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapIterator {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for ResourceMapIterator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;rc(Windows.ApplicationModel.Resources.Core.NamedResource;{1c98c219-0b13-4240-89a5-d495dc189a00}))))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for ResourceMapIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for ResourceMapIterator {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ResourceMapIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapIterator";
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(ResourceMapIterator, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, NamedResource>>> for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapIterator {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceMapMapView(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapMapView {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<ResourceMap> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for ResourceMapMapView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapMapView;pinterface({e480ce40-a338-4ada-adcf-272272e48cb9};string;rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for ResourceMapMapView {
    type Vtable = super::super::super::Foundation::Collections::IMapView_Vtbl<::windows_core::HSTRING, ResourceMap>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for ResourceMapMapView {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ResourceMapMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceMapMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceMapMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(ResourceMapMapView, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ResourceMap>> for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapMapView {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceMapMapViewIterator(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceMapMapViewIterator {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Current(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Current)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasCurrent(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasCurrent)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MoveNext(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MoveNext)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, items: &mut [::core::option::Option<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for ResourceMapMapViewIterator {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator;pinterface({6a79e863-4300-459a-9966-cbb660963ee1};pinterface({02b51929-c1c4-4a7e-8940-0312b5c18500};string;rc(Windows.ApplicationModel.Resources.Core.ResourceMap;{72284824-db8c-42f8-b08c-53ff357dad82}))))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for ResourceMapMapViewIterator {
    type Vtable = super::super::super::Foundation::Collections::IIterator_Vtbl<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for ResourceMapMapViewIterator {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ResourceMapMapViewIterator {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceMapMapViewIterator";
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(ResourceMapMapViewIterator, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ResourceMap>>> for ResourceMapMapViewIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceMapMapViewIterator {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceMapMapViewIterator {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceQualifier(::windows_core::IUnknown);
impl ResourceQualifier {
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
    pub fn IsDefault(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsDefault)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn IsMatch(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsMatch)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Score(&self) -> ::windows_core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Score)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::windows_core::RuntimeType for ResourceQualifier {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifier;{785da5b2-4afd-4376-a888-c5f9a6b7a05c})");
}
unsafe impl ::windows_core::Interface for ResourceQualifier {
    type Vtable = IResourceQualifier_Vtbl;
}
unsafe impl ::windows_core::ComInterface for ResourceQualifier {
    const IID: ::windows_core::GUID = <IResourceQualifier as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for ResourceQualifier {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifier";
}
::windows_core::imp::interface_hierarchy!(ResourceQualifier, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for ResourceQualifier {}
unsafe impl ::core::marker::Sync for ResourceQualifier {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceQualifierMapView(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierMapView {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Split(&self, first: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>, second: &mut ::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>>) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Split)(::windows_core::Interface::as_raw(this), first as *mut _ as _, second as *mut _ as _).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for ResourceQualifierMapView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView;pinterface({e480ce40-a338-4ada-adcf-272272e48cb9};string;string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for ResourceQualifierMapView {
    type Vtable = super::super::super::Foundation::Collections::IMapView_Vtbl<::windows_core::HSTRING, ::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for ResourceQualifierMapView {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ResourceQualifierMapView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierMapView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceQualifierMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceQualifierMapView {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(ResourceQualifierMapView, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierMapView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierMapView {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceQualifierObservableMap(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierObservableMap {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Lookup(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Lookup)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn HasKey(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).HasKey)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IMapView<::windows_core::HSTRING, ::windows_core::HSTRING>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetView)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Insert(&self, key: &::windows_core::HSTRING, value: &::windows_core::HSTRING) -> ::windows_core::Result<bool> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Insert)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Remove(&self, key: &::windows_core::HSTRING) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Remove)(::windows_core::Interface::as_raw(this), ::core::mem::transmute_copy(key)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Clear)(::windows_core::Interface::as_raw(this)).ok() }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MapChanged<P0>(&self, vhnd: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::Collections::MapChangedEventHandler<::windows_core::HSTRING, ::windows_core::HSTRING>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).MapChanged)(::windows_core::Interface::as_raw(this), vhnd.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveMapChanged(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveMapChanged)(::windows_core::Interface::as_raw(this), token).ok() }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for ResourceQualifierObservableMap {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap;pinterface({65df2bf5-bf39-41b5-aebc-5a9d865e472b};string;string))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for ResourceQualifierObservableMap {
    type Vtable = super::super::super::Foundation::Collections::IObservableMap_Vtbl<::windows_core::HSTRING, ::windows_core::HSTRING>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for ResourceQualifierObservableMap {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ResourceQualifierObservableMap {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierObservableMap";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceQualifierObservableMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceQualifierObservableMap {
    type Item = super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>;
    type IntoIter = super::super::super::Foundation::Collections::IIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.First().unwrap()
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(ResourceQualifierObservableMap, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<super::super::super::Foundation::Collections::IKeyValuePair<::windows_core::HSTRING, ::windows_core::HSTRING>>> for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IObservableMap<::windows_core::HSTRING, ::windows_core::HSTRING>> for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierObservableMap {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierObservableMap {}
#[doc = "Required features: `\"Foundation_Collections\"`"]
#[cfg(feature = "Foundation_Collections")]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct ResourceQualifierVectorView(::windows_core::IUnknown);
#[cfg(feature = "Foundation_Collections")]
impl ResourceQualifierVectorView {
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<ResourceQualifier>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<ResourceQualifier> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::IntoParam<ResourceQualifier>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.into_param().abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "Required features: `\"Foundation_Collections\"`"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ResourceQualifier>]) -> ::windows_core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len().try_into().unwrap(), ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeType for ResourceQualifierVectorView {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView;pinterface({bbe1fa4c-b0e3-4583-baef-1f1b2e483e56};rc(Windows.ApplicationModel.Resources.Core.ResourceQualifier;{785da5b2-4afd-4376-a888-c5f9a6b7a05c})))");
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::Interface for ResourceQualifierVectorView {
    type Vtable = super::super::super::Foundation::Collections::IVectorView_Vtbl<ResourceQualifier>;
}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::windows_core::ComInterface for ResourceQualifierVectorView {
    const IID: ::windows_core::GUID = <super::super::super::Foundation::Collections::IVectorView<ResourceQualifier> as ::windows_core::ComInterface>::IID;
}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::RuntimeName for ResourceQualifierVectorView {
    const NAME: &'static str = "Windows.ApplicationModel.Resources.Core.ResourceQualifierVectorView";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ResourceQualifierVectorView {
    type Item = ResourceQualifier;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
#[cfg(feature = "Foundation_Collections")]
::windows_core::imp::interface_hierarchy!(ResourceQualifierVectorView, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<ResourceQualifier>> for ResourceQualifierVectorView {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<ResourceQualifier>> for ResourceQualifierVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Send for ResourceQualifierVectorView {}
#[cfg(feature = "Foundation_Collections")]
unsafe impl ::core::marker::Sync for ResourceQualifierVectorView {}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ResourceCandidateKind(pub i32);
impl ResourceCandidateKind {
    pub const String: Self = Self(0i32);
    pub const File: Self = Self(1i32);
    pub const EmbeddedData: Self = Self(2i32);
}
impl ::core::marker::Copy for ResourceCandidateKind {}
impl ::core::clone::Clone for ResourceCandidateKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourceCandidateKind {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ResourceCandidateKind {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ResourceCandidateKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceCandidateKind").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ResourceCandidateKind {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceCandidateKind;i4)");
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ResourceQualifierPersistence(pub i32);
impl ResourceQualifierPersistence {
    pub const None: Self = Self(0i32);
    pub const LocalMachine: Self = Self(1i32);
}
impl ::core::marker::Copy for ResourceQualifierPersistence {}
impl ::core::clone::Clone for ResourceQualifierPersistence {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourceQualifierPersistence {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for ResourceQualifierPersistence {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for ResourceQualifierPersistence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceQualifierPersistence").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ResourceQualifierPersistence {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.Resources.Core.ResourceQualifierPersistence;i4)");
}
#[repr(C)]
pub struct ResourceLayoutInfo {
    pub MajorVersion: u32,
    pub MinorVersion: u32,
    pub ResourceSubtreeCount: u32,
    pub NamedResourceCount: u32,
    pub Checksum: i32,
}
impl ::core::marker::Copy for ResourceLayoutInfo {}
impl ::core::clone::Clone for ResourceLayoutInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ResourceLayoutInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ResourceLayoutInfo").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).field("ResourceSubtreeCount", &self.ResourceSubtreeCount).field("NamedResourceCount", &self.NamedResourceCount).field("Checksum", &self.Checksum).finish()
    }
}
impl ::windows_core::TypeKind for ResourceLayoutInfo {
    type TypeKind = ::windows_core::CopyType;
}
impl ::windows_core::RuntimeType for ResourceLayoutInfo {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.ApplicationModel.Resources.Core.ResourceLayoutInfo;u4;u4;u4;u4;i4)");
}
impl ::core::cmp::PartialEq for ResourceLayoutInfo {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion && self.ResourceSubtreeCount == other.ResourceSubtreeCount && self.NamedResourceCount == other.NamedResourceCount && self.Checksum == other.Checksum
    }
}
impl ::core::cmp::Eq for ResourceLayoutInfo {}
impl ::core::default::Default for ResourceLayoutInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
