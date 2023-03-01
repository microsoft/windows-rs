#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DManager(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DManager {
    type Vtable = IPrint3DManager_Vtbl;
}
impl ::core::clone::Clone for IPrint3DManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4d2fcb0a_7366_4971_8bd5_17c4e3e8c6c0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DManager_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TaskRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveTaskRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveTaskRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DManagerStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DManagerStatics {
    type Vtable = IPrint3DManagerStatics_Vtbl;
}
impl ::core::clone::Clone for IPrint3DManagerStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DManagerStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0ef1cafe_a9ad_4c08_a917_1d1f863eabcb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DManagerStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShowPrintUIAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowPrintUIAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DTask(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DTask {
    type Vtable = IPrint3DTask_Vtbl;
}
impl ::core::clone::Clone for IPrint3DTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DTask {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ce3d080_2118_4c28_80de_f426d70191ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTask_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Submitting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Submitting: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSubmitting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSubmitting: usize,
    #[cfg(feature = "Foundation")]
    pub Completed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Completed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCompleted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCompleted: usize,
    #[cfg(feature = "Foundation")]
    pub SourceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SourceChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSourceChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSourceChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DTaskCompletedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DTaskCompletedEventArgs {
    type Vtable = IPrint3DTaskCompletedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrint3DTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DTaskCompletedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc1914af_2614_4f1d_accc_d6fc4fda5455);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskCompletedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Completion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Print3DTaskCompletion) -> ::windows::core::HRESULT,
    pub ExtendedStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Print3DTaskDetail) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DTaskRequest(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DTaskRequest {
    type Vtable = IPrint3DTaskRequest_Vtbl;
}
impl ::core::clone::Clone for IPrint3DTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DTaskRequest {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2595c46f_2245_4c5a_8731_0d604dc6bc3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskRequest_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub CreateTask: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::std::mem::MaybeUninit<::windows::core::HSTRING>, printerid: ::std::mem::MaybeUninit<::windows::core::HSTRING>, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DTaskRequestedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DTaskRequestedEventArgs {
    type Vtable = IPrint3DTaskRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrint3DTaskRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DTaskRequestedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x150cb77f_18c5_40d7_9f40_fab3096e05a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskRequestedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Request: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DTaskSourceChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DTaskSourceChangedEventArgs {
    type Vtable = IPrint3DTaskSourceChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IPrint3DTaskSourceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DTaskSourceChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5bcd34af_24e9_4c10_8d07_14c346ba3fcf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskSourceChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrint3DTaskSourceRequestedArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrint3DTaskSourceRequestedArgs {
    type Vtable = IPrint3DTaskSourceRequestedArgs_Vtbl;
}
impl ::core::clone::Clone for IPrint3DTaskSourceRequestedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrint3DTaskSourceRequestedArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc77c9aba_24af_424d_a3bf_92250c355602);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrint3DTaskSourceRequestedArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, source: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3D3MFPackage(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3D3MFPackage {
    type Vtable = IPrinting3D3MFPackage_Vtbl;
}
impl ::core::clone::Clone for IPrinting3D3MFPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3D3MFPackage {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf64dd5c8_2ab7_45a9_a1b7_267e948d5b18);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackage_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub SaveAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    SaveAsync: usize,
    #[cfg(feature = "Storage_Streams")]
    pub PrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PrintTicket: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetPrintTicket: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetPrintTicket: usize,
    #[cfg(feature = "Storage_Streams")]
    pub ModelPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    ModelPart: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetModelPart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetModelPart: usize,
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Textures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Textures: usize,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadModelFromPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadModelFromPackageAsync: usize,
    #[cfg(feature = "Foundation")]
    pub SaveModelToPackageAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SaveModelToPackageAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3D3MFPackage2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3D3MFPackage2 {
    type Vtable = IPrinting3D3MFPackage2_Vtbl;
}
impl ::core::clone::Clone for IPrinting3D3MFPackage2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3D3MFPackage2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x965c7ac4_93cb_4430_92b8_789cd454f883);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackage2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Compression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DPackageCompression) -> ::windows::core::HRESULT,
    pub SetCompression: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DPackageCompression) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3D3MFPackageStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3D3MFPackageStatics {
    type Vtable = IPrinting3D3MFPackageStatics_Vtbl;
}
impl ::core::clone::Clone for IPrinting3D3MFPackageStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3D3MFPackageStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7058d9af_7a9a_4787_b817_f6f459214823);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3D3MFPackageStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub LoadAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Storage_Streams")))]
    LoadAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DBaseMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DBaseMaterial {
    type Vtable = IPrinting3DBaseMaterial_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DBaseMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DBaseMaterial {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0f0e743_c50c_4bcb_9d04_fc16adcea2c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DBaseMaterialGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DBaseMaterialGroup {
    type Vtable = IPrinting3DBaseMaterialGroup_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DBaseMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DBaseMaterialGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x94f070b8_2515_4a8d_a1f0_d0fc13d06021);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Bases: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Bases: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DBaseMaterialGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DBaseMaterialGroupFactory {
    type Vtable = IPrinting3DBaseMaterialGroupFactory_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DBaseMaterialGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DBaseMaterialGroupFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5c1546dc_8697_4193_976b_84bb4116e5bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialGroupFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DBaseMaterialStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DBaseMaterialStatics {
    type Vtable = IPrinting3DBaseMaterialStatics_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DBaseMaterialStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DBaseMaterialStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x815a47bc_374a_476d_be92_3ecfd1cb9776);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DBaseMaterialStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Abs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Pla: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DColorMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DColorMaterial {
    type Vtable = IPrinting3DColorMaterial_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DColorMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DColorMaterial {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe1899928_7ce7_4285_a35d_f145c9510c7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DColorMaterial2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DColorMaterial2 {
    type Vtable = IPrinting3DColorMaterial2_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DColorMaterial2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DColorMaterial2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfab0e852_0aef_44e9_9ddd_36eeea5acd44);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterial2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::UI::Color) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DColorMaterialGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DColorMaterialGroup {
    type Vtable = IPrinting3DColorMaterialGroup_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DColorMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DColorMaterialGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x001a6bd0_aadf_4226_afe9_f369a0b45004);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterialGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Colors: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Colors: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DColorMaterialGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DColorMaterialGroupFactory {
    type Vtable = IPrinting3DColorMaterialGroupFactory_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DColorMaterialGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DColorMaterialGroupFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71d38d6d_b1ea_4a5b_bc54_19c65f3df044);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DColorMaterialGroupFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DComponent(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DComponent {
    type Vtable = IPrinting3DComponent_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DComponent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e287845_bf7f_4cdb_a27f_30a01437fede);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DComponent_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Mesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMesh: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetThumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DObjectType) -> ::windows::core::HRESULT,
    pub SetType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DObjectType) -> ::windows::core::HRESULT,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub PartNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetPartNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DComponentWithMatrix(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DComponentWithMatrix {
    type Vtable = IPrinting3DComponentWithMatrix_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DComponentWithMatrix {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DComponentWithMatrix {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3279f335_0ef0_456b_9a21_49bebe8b51c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DComponentWithMatrix_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Component: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Numerics")]
    pub Matrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    Matrix: usize,
    #[cfg(feature = "Foundation_Numerics")]
    pub SetMatrix: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Numerics"))]
    SetMatrix: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DCompositeMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DCompositeMaterial {
    type Vtable = IPrinting3DCompositeMaterial_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DCompositeMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DCompositeMaterial {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x462238dd_562e_4f6c_882d_f4d841fd63c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Values: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Values: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DCompositeMaterialGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DCompositeMaterialGroup {
    type Vtable = IPrinting3DCompositeMaterialGroup_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DCompositeMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DCompositeMaterialGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d946a5b_40f1_496d_a5fb_340a5a678e30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Composites: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Composites: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialIndices: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DCompositeMaterialGroup2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DCompositeMaterialGroup2 {
    type Vtable = IPrinting3DCompositeMaterialGroup2_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DCompositeMaterialGroup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DCompositeMaterialGroup2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x06e86d62_7d3b_41e1_944c_bafde4555483);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroup2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub BaseMaterialGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBaseMaterialGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DCompositeMaterialGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DCompositeMaterialGroupFactory {
    type Vtable = IPrinting3DCompositeMaterialGroupFactory_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DCompositeMaterialGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DCompositeMaterialGroupFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd08ecd13_92ff_43aa_a627_8d43c22c817e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DCompositeMaterialGroupFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DFaceReductionOptions(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DFaceReductionOptions {
    type Vtable = IPrinting3DFaceReductionOptions_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DFaceReductionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DFaceReductionOptions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbbfed397_2d74_46f7_be85_99a67bbb6629);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DFaceReductionOptions_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub MaxReductionArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetMaxReductionArea: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub TargetTriangleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetTargetTriangleCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub MaxEdgeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetMaxEdgeLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DMaterial {
    type Vtable = IPrinting3DMaterial_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DMaterial {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x378db256_ed62_4952_b85b_03567d7c465e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub BaseGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BaseGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ColorGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ColorGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Texture2CoordGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Texture2CoordGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub CompositeGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CompositeGroups: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MultiplePropertyGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MultiplePropertyGroups: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DMesh(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DMesh {
    type Vtable = IPrinting3DMesh_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DMesh {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x192e90dc_0228_2e01_bc20_c5290cbf32c4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMesh_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub VertexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetVertexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub IndexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetIndexCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub VertexPositionsDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DBufferDescription) -> ::windows::core::HRESULT,
    pub SetVertexPositionsDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DBufferDescription) -> ::windows::core::HRESULT,
    pub VertexNormalsDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DBufferDescription) -> ::windows::core::HRESULT,
    pub SetVertexNormalsDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DBufferDescription) -> ::windows::core::HRESULT,
    pub TriangleIndicesDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DBufferDescription) -> ::windows::core::HRESULT,
    pub SetTriangleIndicesDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DBufferDescription) -> ::windows::core::HRESULT,
    pub TriangleMaterialIndicesDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DBufferDescription) -> ::windows::core::HRESULT,
    pub SetTriangleMaterialIndicesDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DBufferDescription) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetVertexPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetVertexPositions: usize,
    pub CreateVertexPositions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetVertexNormals: usize,
    pub CreateVertexNormals: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTriangleIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTriangleIndices: usize,
    pub CreateTriangleIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub GetTriangleMaterialIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    GetTriangleMaterialIndices: usize,
    pub CreateTriangleMaterialIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub BufferDescriptionSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BufferDescriptionSet: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub BufferSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    BufferSet: usize,
    #[cfg(feature = "Foundation")]
    pub VerifyAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DMeshVerificationMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    VerifyAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DMeshVerificationResult(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DMeshVerificationResult {
    type Vtable = IPrinting3DMeshVerificationResult_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DMeshVerificationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DMeshVerificationResult {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x195671ba_e93a_4e8a_a46f_dea8e852197e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMeshVerificationResult_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsValid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub NonmanifoldTriangles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    NonmanifoldTriangles: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub ReversedNormalTriangles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    ReversedNormalTriangles: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DModel(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DModel {
    type Vtable = IPrinting3DModel_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DModel {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d012ef0_52fb_919a_77b0_4b1a3b80324f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DModel_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Unit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DModelUnit) -> ::windows::core::HRESULT,
    pub SetUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DModelUnit) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Textures: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Textures: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Meshes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Meshes: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Components: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Components: usize,
    pub Material: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetMaterial: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Build: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetBuild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub RequiredExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RequiredExtensions: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub Metadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Metadata: usize,
    #[cfg(feature = "Foundation")]
    pub RepairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepairAsync: usize,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DModel2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DModel2 {
    type Vtable = IPrinting3DModel2_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DModel2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DModel2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc92069c7_c841_47f3_a84e_a149fd08b657);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DModel2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub TryPartialRepairAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPartialRepairAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryPartialRepairWithTimeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxwaittime: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryPartialRepairWithTimeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryReduceFacesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryReduceFacesAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryReduceFacesWithOptionsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printing3dfacereductionoptions: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryReduceFacesWithOptionsAsync: usize,
    #[cfg(feature = "Foundation")]
    pub TryReduceFacesWithOptionsAndTimeAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, printing3dfacereductionoptions: *mut ::core::ffi::c_void, maxwait: super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    TryReduceFacesWithOptionsAndTimeAsync: usize,
    #[cfg(feature = "Foundation")]
    pub RepairWithProgressAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepairWithProgressAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DModelTexture(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DModelTexture {
    type Vtable = IPrinting3DModelTexture_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DModelTexture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DModelTexture {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5dafcf01_b59d_483c_97bb_a4d546d1c75c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DModelTexture_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub TextureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTextureResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TileStyleU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DTextureEdgeBehavior) -> ::windows::core::HRESULT,
    pub SetTileStyleU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DTextureEdgeBehavior) -> ::windows::core::HRESULT,
    pub TileStyleV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Printing3DTextureEdgeBehavior) -> ::windows::core::HRESULT,
    pub SetTileStyleV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: Printing3DTextureEdgeBehavior) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DMultiplePropertyMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DMultiplePropertyMaterial {
    type Vtable = IPrinting3DMultiplePropertyMaterial_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DMultiplePropertyMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DMultiplePropertyMaterial {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x25a6254b_c6e9_484d_a214_a25e5776ba62);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialIndices: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DMultiplePropertyMaterialGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DMultiplePropertyMaterialGroup {
    type Vtable = IPrinting3DMultiplePropertyMaterialGroup_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DMultiplePropertyMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DMultiplePropertyMaterialGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0950519_aeb9_4515_a39b_a088fbbb277c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterialGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub MultipleProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MultipleProperties: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub MaterialGroupIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    MaterialGroupIndices: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DMultiplePropertyMaterialGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DMultiplePropertyMaterialGroupFactory {
    type Vtable = IPrinting3DMultiplePropertyMaterialGroupFactory_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DMultiplePropertyMaterialGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DMultiplePropertyMaterialGroupFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x323e196e_d4c6_451e_a814_4d78a210fe53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DMultiplePropertyMaterialGroupFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DTexture2CoordMaterial(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DTexture2CoordMaterial {
    type Vtable = IPrinting3DTexture2CoordMaterial_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DTexture2CoordMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DTexture2CoordMaterial {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d844bfb_07e9_4986_9833_8dd3d48c6859);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterial_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Texture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub U: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetU: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub V: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetV: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DTexture2CoordMaterialGroup(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DTexture2CoordMaterialGroup {
    type Vtable = IPrinting3DTexture2CoordMaterialGroup_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DTexture2CoordMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DTexture2CoordMaterialGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x627d7ca7_6d90_4fb9_9fc4_9feff3dfa892);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroup_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Texture2Coords: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Texture2Coords: usize,
    pub MaterialGroupId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DTexture2CoordMaterialGroup2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DTexture2CoordMaterialGroup2 {
    type Vtable = IPrinting3DTexture2CoordMaterialGroup2_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DTexture2CoordMaterialGroup2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DTexture2CoordMaterialGroup2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69fbdbba_b12e_429b_8386_df5284f6e80f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroup2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Texture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTexture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DTexture2CoordMaterialGroupFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DTexture2CoordMaterialGroupFactory {
    type Vtable = IPrinting3DTexture2CoordMaterialGroupFactory_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DTexture2CoordMaterialGroupFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DTexture2CoordMaterialGroupFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcbb049b0_468a_4c6f_b2a2_8eb8ba8dea48);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTexture2CoordMaterialGroupFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, materialgroupid: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrinting3DTextureResource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrinting3DTextureResource {
    type Vtable = IPrinting3DTextureResource_Vtbl;
}
impl ::core::clone::Clone for IPrinting3DTextureResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPrinting3DTextureResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa70df32d_6ab1_44ae_bc45_a27382c0d38c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DTextureResource_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Storage_Streams")]
    pub TextureData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    TextureData: usize,
    #[cfg(feature = "Storage_Streams")]
    pub SetTextureData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    SetTextureData: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DManager(::windows::core::IUnknown);
impl Print3DManager {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TaskRequested(&self, eventhandler: &super::super::Foundation::TypedEventHandler<Print3DManager, Print3DTaskRequestedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).TaskRequested)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveTaskRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveTaskRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<Print3DManager> {
        Self::IPrint3DManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Print3DManager>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowPrintUIAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IPrint3DManagerStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).ShowPrintUIAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrint3DManagerStatics<R, F: FnOnce(&IPrint3DManagerStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Print3DManager, IPrint3DManagerStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Print3DManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DManager {}
impl ::core::fmt::Debug for Print3DManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DManager").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DManager {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DManager;{4d2fcb0a-7366-4971-8bd5-17c4e3e8c6c0})");
}
impl ::core::clone::Clone for Print3DManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Print3DManager {
    type Vtable = IPrint3DManager_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Print3DManager {
    const IID: ::windows::core::GUID = <IPrint3DManager as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Print3DManager {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DManager";
}
::windows::imp::interface_hierarchy!(Print3DManager, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DManager {}
unsafe impl ::core::marker::Sync for Print3DManager {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTask(::windows::core::IUnknown);
impl Print3DTask {
    pub fn Source(&self) -> ::windows::core::Result<Printing3D3MFPackage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3D3MFPackage>();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Submitting(&self, eventhandler: &super::super::Foundation::TypedEventHandler<Print3DTask, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Submitting)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSubmitting(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSubmitting)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Completed(&self, eventhandler: &super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskCompletedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).Completed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCompleted(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCompleted)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SourceChanged(&self, eventhandler: &super::super::Foundation::TypedEventHandler<Print3DTask, Print3DTaskSourceChangedEventArgs>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SourceChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventhandler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSourceChanged(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSourceChanged)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
}
impl ::core::cmp::PartialEq for Print3DTask {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTask {}
impl ::core::fmt::Debug for Print3DTask {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTask").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DTask {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTask;{8ce3d080-2118-4c28-80de-f426d70191ae})");
}
impl ::core::clone::Clone for Print3DTask {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Print3DTask {
    type Vtable = IPrint3DTask_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Print3DTask {
    const IID: ::windows::core::GUID = <IPrint3DTask as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Print3DTask {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTask";
}
::windows::imp::interface_hierarchy!(Print3DTask, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DTask {}
unsafe impl ::core::marker::Sync for Print3DTask {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTaskCompletedEventArgs(::windows::core::IUnknown);
impl Print3DTaskCompletedEventArgs {
    pub fn Completion(&self) -> ::windows::core::Result<Print3DTaskCompletion> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Print3DTaskCompletion>();
            (::windows::core::Interface::vtable(this).Completion)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExtendedStatus(&self) -> ::windows::core::Result<Print3DTaskDetail> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Print3DTaskDetail>();
            (::windows::core::Interface::vtable(this).ExtendedStatus)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Print3DTaskCompletedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskCompletedEventArgs {}
impl ::core::fmt::Debug for Print3DTaskCompletedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskCompletedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DTaskCompletedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs;{cc1914af-2614-4f1d-accc-d6fc4fda5455})");
}
impl ::core::clone::Clone for Print3DTaskCompletedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Print3DTaskCompletedEventArgs {
    type Vtable = IPrint3DTaskCompletedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Print3DTaskCompletedEventArgs {
    const IID: ::windows::core::GUID = <IPrint3DTaskCompletedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Print3DTaskCompletedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskCompletedEventArgs";
}
::windows::imp::interface_hierarchy!(Print3DTaskCompletedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DTaskCompletedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DTaskCompletedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTaskRequest(::windows::core::IUnknown);
impl Print3DTaskRequest {
    pub fn CreateTask(&self, title: &::windows::core::HSTRING, printerid: &::windows::core::HSTRING, handler: &Print3DTaskSourceRequestedHandler) -> ::windows::core::Result<Print3DTask> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Print3DTask>();
            (::windows::core::Interface::vtable(this).CreateTask)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(title), ::core::mem::transmute_copy(printerid), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Print3DTaskRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskRequest {}
impl ::core::fmt::Debug for Print3DTaskRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskRequest").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DTaskRequest {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskRequest;{2595c46f-2245-4c5a-8731-0d604dc6bc3c})");
}
impl ::core::clone::Clone for Print3DTaskRequest {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Print3DTaskRequest {
    type Vtable = IPrint3DTaskRequest_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Print3DTaskRequest {
    const IID: ::windows::core::GUID = <IPrint3DTaskRequest as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Print3DTaskRequest {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskRequest";
}
::windows::imp::interface_hierarchy!(Print3DTaskRequest, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DTaskRequest {}
unsafe impl ::core::marker::Sync for Print3DTaskRequest {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTaskRequestedEventArgs(::windows::core::IUnknown);
impl Print3DTaskRequestedEventArgs {
    pub fn Request(&self) -> ::windows::core::Result<Print3DTaskRequest> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Print3DTaskRequest>();
            (::windows::core::Interface::vtable(this).Request)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Print3DTaskRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskRequestedEventArgs {}
impl ::core::fmt::Debug for Print3DTaskRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DTaskRequestedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs;{150cb77f-18c5-40d7-9f40-fab3096e05a9})");
}
impl ::core::clone::Clone for Print3DTaskRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Print3DTaskRequestedEventArgs {
    type Vtable = IPrint3DTaskRequestedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Print3DTaskRequestedEventArgs {
    const IID: ::windows::core::GUID = <IPrint3DTaskRequestedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Print3DTaskRequestedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskRequestedEventArgs";
}
::windows::imp::interface_hierarchy!(Print3DTaskRequestedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DTaskRequestedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DTaskRequestedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTaskSourceChangedEventArgs(::windows::core::IUnknown);
impl Print3DTaskSourceChangedEventArgs {
    pub fn Source(&self) -> ::windows::core::Result<Printing3D3MFPackage> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3D3MFPackage>();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Print3DTaskSourceChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskSourceChangedEventArgs {}
impl ::core::fmt::Debug for Print3DTaskSourceChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskSourceChangedEventArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DTaskSourceChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs;{5bcd34af-24e9-4c10-8d07-14c346ba3fcf})");
}
impl ::core::clone::Clone for Print3DTaskSourceChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Print3DTaskSourceChangedEventArgs {
    type Vtable = IPrint3DTaskSourceChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Print3DTaskSourceChangedEventArgs {
    const IID: ::windows::core::GUID = <IPrint3DTaskSourceChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Print3DTaskSourceChangedEventArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskSourceChangedEventArgs";
}
::windows::imp::interface_hierarchy!(Print3DTaskSourceChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DTaskSourceChangedEventArgs {}
unsafe impl ::core::marker::Sync for Print3DTaskSourceChangedEventArgs {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTaskSourceRequestedArgs(::windows::core::IUnknown);
impl Print3DTaskSourceRequestedArgs {
    pub fn SetSource(&self, source: &Printing3D3MFPackage) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(source)).ok() }
    }
}
impl ::core::cmp::PartialEq for Print3DTaskSourceRequestedArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskSourceRequestedArgs {}
impl ::core::fmt::Debug for Print3DTaskSourceRequestedArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskSourceRequestedArgs").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DTaskSourceRequestedArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs;{c77c9aba-24af-424d-a3bf-92250c355602})");
}
impl ::core::clone::Clone for Print3DTaskSourceRequestedArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Print3DTaskSourceRequestedArgs {
    type Vtable = IPrint3DTaskSourceRequestedArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Print3DTaskSourceRequestedArgs {
    const IID: ::windows::core::GUID = <IPrint3DTaskSourceRequestedArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Print3DTaskSourceRequestedArgs {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Print3DTaskSourceRequestedArgs";
}
::windows::imp::interface_hierarchy!(Print3DTaskSourceRequestedArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Print3DTaskSourceRequestedArgs {}
unsafe impl ::core::marker::Sync for Print3DTaskSourceRequestedArgs {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3D3MFPackage(::windows::core::IUnknown);
impl Printing3D3MFPackage {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3D3MFPackage, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn SaveAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<super::super::Storage::Streams::IRandomAccessStream>>();
            (::windows::core::Interface::vtable(this).SaveAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn PrintTicket(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStream>();
            (::windows::core::Interface::vtable(this).PrintTicket)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetPrintTicket<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPrintTicket)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn ModelPart(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStream> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStream>();
            (::windows::core::Interface::vtable(this).ModelPart)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetModelPart<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetModelPart)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Thumbnail(&self) -> ::windows::core::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DTextureResource>();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetThumbnail(&self, value: &Printing3DTextureResource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThumbnail)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Textures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTextureResource>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DTextureResource>>();
            (::windows::core::Interface::vtable(this).Textures)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadModelFromPackageAsync<P0>(&self, value: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3DModel>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Printing3DModel>>();
            (::windows::core::Interface::vtable(this).LoadModelFromPackageAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SaveModelToPackageAsync(&self, value: &Printing3DModel) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).SaveModelToPackageAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value), &mut result__).from_abi(result__)
        }
    }
    pub fn Compression(&self) -> ::windows::core::Result<Printing3DPackageCompression> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3D3MFPackage2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DPackageCompression>();
            (::windows::core::Interface::vtable(this).Compression)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompression(&self, value: Printing3DPackageCompression) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3D3MFPackage2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetCompression)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn LoadAsync<P0>(value: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3D3MFPackage>>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStream>,
    {
        Self::IPrinting3D3MFPackageStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Printing3D3MFPackage>>();
            (::windows::core::Interface::vtable(this).LoadAsync)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3D3MFPackageStatics<R, F: FnOnce(&IPrinting3D3MFPackageStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3D3MFPackage, IPrinting3D3MFPackageStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Printing3D3MFPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3D3MFPackage {}
impl ::core::fmt::Debug for Printing3D3MFPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3D3MFPackage").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3D3MFPackage {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3D3MFPackage;{f64dd5c8-2ab7-45a9-a1b7-267e948d5b18})");
}
impl ::core::clone::Clone for Printing3D3MFPackage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3D3MFPackage {
    type Vtable = IPrinting3D3MFPackage_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3D3MFPackage {
    const IID: ::windows::core::GUID = <IPrinting3D3MFPackage as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3D3MFPackage {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3D3MFPackage";
}
::windows::imp::interface_hierarchy!(Printing3D3MFPackage, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3D3MFPackage {}
unsafe impl ::core::marker::Sync for Printing3D3MFPackage {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DBaseMaterial(::windows::core::IUnknown);
impl Printing3DBaseMaterial {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DBaseMaterial, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Color(&self) -> ::windows::core::Result<Printing3DColorMaterial> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DColorMaterial>();
            (::windows::core::Interface::vtable(this).Color)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetColor(&self, value: &Printing3DColorMaterial) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetColor)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Abs() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPrinting3DBaseMaterialStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Abs)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    pub fn Pla() -> ::windows::core::Result<::windows::core::HSTRING> {
        Self::IPrinting3DBaseMaterialStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Pla)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DBaseMaterialStatics<R, F: FnOnce(&IPrinting3DBaseMaterialStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DBaseMaterial, IPrinting3DBaseMaterialStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Printing3DBaseMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DBaseMaterial {}
impl ::core::fmt::Debug for Printing3DBaseMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DBaseMaterial").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DBaseMaterial {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DBaseMaterial;{d0f0e743-c50c-4bcb-9d04-fc16adcea2c9})");
}
impl ::core::clone::Clone for Printing3DBaseMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DBaseMaterial {
    type Vtable = IPrinting3DBaseMaterial_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DBaseMaterial {
    const IID: ::windows::core::GUID = <IPrinting3DBaseMaterial as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DBaseMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DBaseMaterial";
}
::windows::imp::interface_hierarchy!(Printing3DBaseMaterial, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DBaseMaterial {}
unsafe impl ::core::marker::Sync for Printing3DBaseMaterial {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DBaseMaterialGroup(::windows::core::IUnknown);
impl Printing3DBaseMaterialGroup {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Bases(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DBaseMaterial>>();
            (::windows::core::Interface::vtable(this).Bases)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaterialGroupId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaterialGroupId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(materialgroupid: u32) -> ::windows::core::Result<Printing3DBaseMaterialGroup> {
        Self::IPrinting3DBaseMaterialGroupFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DBaseMaterialGroup>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), materialgroupid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DBaseMaterialGroupFactory<R, F: FnOnce(&IPrinting3DBaseMaterialGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DBaseMaterialGroup, IPrinting3DBaseMaterialGroupFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Printing3DBaseMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DBaseMaterialGroup {}
impl ::core::fmt::Debug for Printing3DBaseMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DBaseMaterialGroup").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DBaseMaterialGroup {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup;{94f070b8-2515-4a8d-a1f0-d0fc13d06021})");
}
impl ::core::clone::Clone for Printing3DBaseMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DBaseMaterialGroup {
    type Vtable = IPrinting3DBaseMaterialGroup_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DBaseMaterialGroup {
    const IID: ::windows::core::GUID = <IPrinting3DBaseMaterialGroup as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DBaseMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DBaseMaterialGroup";
}
::windows::imp::interface_hierarchy!(Printing3DBaseMaterialGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DBaseMaterialGroup {}
unsafe impl ::core::marker::Sync for Printing3DBaseMaterialGroup {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DColorMaterial(::windows::core::IUnknown);
impl Printing3DColorMaterial {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DColorMaterial, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Value(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetValue(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows::core::Result<super::super::UI::Color> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DColorMaterial2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::UI::Color>();
            (::windows::core::Interface::vtable(this).Color)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::UI::Color) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DColorMaterial2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetColor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for Printing3DColorMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DColorMaterial {}
impl ::core::fmt::Debug for Printing3DColorMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DColorMaterial").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DColorMaterial {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DColorMaterial;{e1899928-7ce7-4285-a35d-f145c9510c7b})");
}
impl ::core::clone::Clone for Printing3DColorMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DColorMaterial {
    type Vtable = IPrinting3DColorMaterial_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DColorMaterial {
    const IID: ::windows::core::GUID = <IPrinting3DColorMaterial as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DColorMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DColorMaterial";
}
::windows::imp::interface_hierarchy!(Printing3DColorMaterial, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DColorMaterial {}
unsafe impl ::core::marker::Sync for Printing3DColorMaterial {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DColorMaterialGroup(::windows::core::IUnknown);
impl Printing3DColorMaterialGroup {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Colors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DColorMaterial>>();
            (::windows::core::Interface::vtable(this).Colors)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaterialGroupId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaterialGroupId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(materialgroupid: u32) -> ::windows::core::Result<Printing3DColorMaterialGroup> {
        Self::IPrinting3DColorMaterialGroupFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DColorMaterialGroup>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), materialgroupid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DColorMaterialGroupFactory<R, F: FnOnce(&IPrinting3DColorMaterialGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DColorMaterialGroup, IPrinting3DColorMaterialGroupFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Printing3DColorMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DColorMaterialGroup {}
impl ::core::fmt::Debug for Printing3DColorMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DColorMaterialGroup").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DColorMaterialGroup {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DColorMaterialGroup;{001a6bd0-aadf-4226-afe9-f369a0b45004})");
}
impl ::core::clone::Clone for Printing3DColorMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DColorMaterialGroup {
    type Vtable = IPrinting3DColorMaterialGroup_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DColorMaterialGroup {
    const IID: ::windows::core::GUID = <IPrinting3DColorMaterialGroup as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DColorMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DColorMaterialGroup";
}
::windows::imp::interface_hierarchy!(Printing3DColorMaterialGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DColorMaterialGroup {}
unsafe impl ::core::marker::Sync for Printing3DColorMaterialGroup {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DComponent(::windows::core::IUnknown);
impl Printing3DComponent {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DComponent, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Mesh(&self) -> ::windows::core::Result<Printing3DMesh> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DMesh>();
            (::windows::core::Interface::vtable(this).Mesh)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMesh(&self, value: &Printing3DMesh) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMesh)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Components(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DComponentWithMatrix>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DComponentWithMatrix>>();
            (::windows::core::Interface::vtable(this).Components)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Thumbnail(&self) -> ::windows::core::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DTextureResource>();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetThumbnail(&self, value: &Printing3DTextureResource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetThumbnail)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Type(&self) -> ::windows::core::Result<Printing3DObjectType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DObjectType>();
            (::windows::core::Interface::vtable(this).Type)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetType(&self, value: Printing3DObjectType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetType)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn PartNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).PartNumber)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetPartNumber(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPartNumber)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for Printing3DComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DComponent {}
impl ::core::fmt::Debug for Printing3DComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DComponent").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DComponent {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DComponent;{7e287845-bf7f-4cdb-a27f-30a01437fede})");
}
impl ::core::clone::Clone for Printing3DComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DComponent {
    type Vtable = IPrinting3DComponent_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DComponent {
    const IID: ::windows::core::GUID = <IPrinting3DComponent as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DComponent {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DComponent";
}
::windows::imp::interface_hierarchy!(Printing3DComponent, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DComponent {}
unsafe impl ::core::marker::Sync for Printing3DComponent {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DComponentWithMatrix(::windows::core::IUnknown);
impl Printing3DComponentWithMatrix {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DComponentWithMatrix, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Component(&self) -> ::windows::core::Result<Printing3DComponent> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DComponent>();
            (::windows::core::Interface::vtable(this).Component)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetComponent(&self, value: &Printing3DComponent) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetComponent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn Matrix(&self) -> ::windows::core::Result<super::super::Foundation::Numerics::Matrix4x4> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Numerics::Matrix4x4>();
            (::windows::core::Interface::vtable(this).Matrix)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Numerics\"`*"]
    #[cfg(feature = "Foundation_Numerics")]
    pub fn SetMatrix(&self, value: super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMatrix)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for Printing3DComponentWithMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DComponentWithMatrix {}
impl ::core::fmt::Debug for Printing3DComponentWithMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DComponentWithMatrix").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DComponentWithMatrix {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DComponentWithMatrix;{3279f335-0ef0-456b-9a21-49bebe8b51c2})");
}
impl ::core::clone::Clone for Printing3DComponentWithMatrix {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DComponentWithMatrix {
    type Vtable = IPrinting3DComponentWithMatrix_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DComponentWithMatrix {
    const IID: ::windows::core::GUID = <IPrinting3DComponentWithMatrix as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DComponentWithMatrix {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DComponentWithMatrix";
}
::windows::imp::interface_hierarchy!(Printing3DComponentWithMatrix, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DComponentWithMatrix {}
unsafe impl ::core::marker::Sync for Printing3DComponentWithMatrix {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DCompositeMaterial(::windows::core::IUnknown);
impl Printing3DCompositeMaterial {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DCompositeMaterial, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Values(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<f64>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<f64>>();
            (::windows::core::Interface::vtable(this).Values)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Printing3DCompositeMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DCompositeMaterial {}
impl ::core::fmt::Debug for Printing3DCompositeMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DCompositeMaterial").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DCompositeMaterial {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DCompositeMaterial;{462238dd-562e-4f6c-882d-f4d841fd63c7})");
}
impl ::core::clone::Clone for Printing3DCompositeMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DCompositeMaterial {
    type Vtable = IPrinting3DCompositeMaterial_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DCompositeMaterial {
    const IID: ::windows::core::GUID = <IPrinting3DCompositeMaterial as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DCompositeMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DCompositeMaterial";
}
::windows::imp::interface_hierarchy!(Printing3DCompositeMaterial, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DCompositeMaterial {}
unsafe impl ::core::marker::Sync for Printing3DCompositeMaterial {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DCompositeMaterialGroup(::windows::core::IUnknown);
impl Printing3DCompositeMaterialGroup {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Composites(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterial>>();
            (::windows::core::Interface::vtable(this).Composites)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaterialGroupId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaterialGroupId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MaterialIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<u32>>();
            (::windows::core::Interface::vtable(this).MaterialIndices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn BaseMaterialGroup(&self) -> ::windows::core::Result<Printing3DBaseMaterialGroup> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DCompositeMaterialGroup2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DBaseMaterialGroup>();
            (::windows::core::Interface::vtable(this).BaseMaterialGroup)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBaseMaterialGroup(&self, value: &Printing3DBaseMaterialGroup) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DCompositeMaterialGroup2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetBaseMaterialGroup)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(materialgroupid: u32) -> ::windows::core::Result<Printing3DCompositeMaterialGroup> {
        Self::IPrinting3DCompositeMaterialGroupFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DCompositeMaterialGroup>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), materialgroupid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DCompositeMaterialGroupFactory<R, F: FnOnce(&IPrinting3DCompositeMaterialGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DCompositeMaterialGroup, IPrinting3DCompositeMaterialGroupFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Printing3DCompositeMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DCompositeMaterialGroup {}
impl ::core::fmt::Debug for Printing3DCompositeMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DCompositeMaterialGroup").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DCompositeMaterialGroup {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup;{8d946a5b-40f1-496d-a5fb-340a5a678e30})");
}
impl ::core::clone::Clone for Printing3DCompositeMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DCompositeMaterialGroup {
    type Vtable = IPrinting3DCompositeMaterialGroup_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DCompositeMaterialGroup {
    const IID: ::windows::core::GUID = <IPrinting3DCompositeMaterialGroup as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DCompositeMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DCompositeMaterialGroup";
}
::windows::imp::interface_hierarchy!(Printing3DCompositeMaterialGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DCompositeMaterialGroup {}
unsafe impl ::core::marker::Sync for Printing3DCompositeMaterialGroup {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DFaceReductionOptions(::windows::core::IUnknown);
impl Printing3DFaceReductionOptions {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DFaceReductionOptions, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn MaxReductionArea(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MaxReductionArea)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxReductionArea(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxReductionArea)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TargetTriangleCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).TargetTriangleCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTargetTriangleCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetTriangleCount)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn MaxEdgeLength(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).MaxEdgeLength)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaxEdgeLength(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaxEdgeLength)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for Printing3DFaceReductionOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DFaceReductionOptions {}
impl ::core::fmt::Debug for Printing3DFaceReductionOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DFaceReductionOptions").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DFaceReductionOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DFaceReductionOptions;{bbfed397-2d74-46f7-be85-99a67bbb6629})");
}
impl ::core::clone::Clone for Printing3DFaceReductionOptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DFaceReductionOptions {
    type Vtable = IPrinting3DFaceReductionOptions_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DFaceReductionOptions {
    const IID: ::windows::core::GUID = <IPrinting3DFaceReductionOptions as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DFaceReductionOptions {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DFaceReductionOptions";
}
::windows::imp::interface_hierarchy!(Printing3DFaceReductionOptions, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DFaceReductionOptions {}
unsafe impl ::core::marker::Sync for Printing3DFaceReductionOptions {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DMaterial(::windows::core::IUnknown);
impl Printing3DMaterial {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DMaterial, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BaseGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DBaseMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DBaseMaterialGroup>>();
            (::windows::core::Interface::vtable(this).BaseGroups)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ColorGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DColorMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DColorMaterialGroup>>();
            (::windows::core::Interface::vtable(this).ColorGroups)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Texture2CoordGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterialGroup>>();
            (::windows::core::Interface::vtable(this).Texture2CoordGroups)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CompositeGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DCompositeMaterialGroup>>();
            (::windows::core::Interface::vtable(this).CompositeGroups)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MultiplePropertyGroups(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterialGroup>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterialGroup>>();
            (::windows::core::Interface::vtable(this).MultiplePropertyGroups)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Printing3DMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMaterial {}
impl ::core::fmt::Debug for Printing3DMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMaterial").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DMaterial {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMaterial;{378db256-ed62-4952-b85b-03567d7c465e})");
}
impl ::core::clone::Clone for Printing3DMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DMaterial {
    type Vtable = IPrinting3DMaterial_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DMaterial {
    const IID: ::windows::core::GUID = <IPrinting3DMaterial as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMaterial";
}
::windows::imp::interface_hierarchy!(Printing3DMaterial, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DMaterial {}
unsafe impl ::core::marker::Sync for Printing3DMaterial {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DMesh(::windows::core::IUnknown);
impl Printing3DMesh {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DMesh, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn VertexCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).VertexCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVertexCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVertexCount)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IndexCount(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).IndexCount)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIndexCount(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIndexCount)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn VertexPositionsDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DBufferDescription>();
            (::windows::core::Interface::vtable(this).VertexPositionsDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVertexPositionsDescription(&self, value: Printing3DBufferDescription) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVertexPositionsDescription)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn VertexNormalsDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DBufferDescription>();
            (::windows::core::Interface::vtable(this).VertexNormalsDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVertexNormalsDescription(&self, value: Printing3DBufferDescription) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVertexNormalsDescription)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriangleIndicesDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DBufferDescription>();
            (::windows::core::Interface::vtable(this).TriangleIndicesDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTriangleIndicesDescription(&self, value: Printing3DBufferDescription) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTriangleIndicesDescription)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TriangleMaterialIndicesDescription(&self) -> ::windows::core::Result<Printing3DBufferDescription> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DBufferDescription>();
            (::windows::core::Interface::vtable(this).TriangleMaterialIndicesDescription)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTriangleMaterialIndicesDescription(&self, value: Printing3DBufferDescription) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTriangleMaterialIndicesDescription)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetVertexPositions(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).GetVertexPositions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateVertexPositions(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CreateVertexPositions)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetVertexNormals(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).GetVertexNormals)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateVertexNormals(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CreateVertexNormals)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTriangleIndices(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).GetTriangleIndices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateTriangleIndices(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CreateTriangleIndices)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn GetTriangleMaterialIndices(&self) -> ::windows::core::Result<super::super::Storage::Streams::IBuffer> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IBuffer>();
            (::windows::core::Interface::vtable(this).GetTriangleMaterialIndices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CreateTriangleMaterialIndices(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).CreateTriangleMaterialIndices)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BufferDescriptionSet(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).BufferDescriptionSet)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn BufferSet(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IPropertySet> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IPropertySet>();
            (::windows::core::Interface::vtable(this).BufferSet)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn VerifyAsync(&self, value: Printing3DMeshVerificationMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Printing3DMeshVerificationResult>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<Printing3DMeshVerificationResult>>();
            (::windows::core::Interface::vtable(this).VerifyAsync)(::windows::core::Interface::as_raw(this), value, &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Printing3DMesh {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMesh {}
impl ::core::fmt::Debug for Printing3DMesh {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMesh").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DMesh {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMesh;{192e90dc-0228-2e01-bc20-c5290cbf32c4})");
}
impl ::core::clone::Clone for Printing3DMesh {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DMesh {
    type Vtable = IPrinting3DMesh_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DMesh {
    const IID: ::windows::core::GUID = <IPrinting3DMesh as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DMesh {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMesh";
}
::windows::imp::interface_hierarchy!(Printing3DMesh, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DMesh {}
unsafe impl ::core::marker::Sync for Printing3DMesh {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DMeshVerificationResult(::windows::core::IUnknown);
impl Printing3DMeshVerificationResult {
    pub fn IsValid(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsValid)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn NonmanifoldTriangles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).NonmanifoldTriangles)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReversedNormalTriangles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVectorView<u32>>();
            (::windows::core::Interface::vtable(this).ReversedNormalTriangles)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Printing3DMeshVerificationResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMeshVerificationResult {}
impl ::core::fmt::Debug for Printing3DMeshVerificationResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMeshVerificationResult").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DMeshVerificationResult {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMeshVerificationResult;{195671ba-e93a-4e8a-a46f-dea8e852197e})");
}
impl ::core::clone::Clone for Printing3DMeshVerificationResult {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DMeshVerificationResult {
    type Vtable = IPrinting3DMeshVerificationResult_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DMeshVerificationResult {
    const IID: ::windows::core::GUID = <IPrinting3DMeshVerificationResult as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DMeshVerificationResult {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMeshVerificationResult";
}
::windows::imp::interface_hierarchy!(Printing3DMeshVerificationResult, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DMeshVerificationResult {}
unsafe impl ::core::marker::Sync for Printing3DMeshVerificationResult {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DModel(::windows::core::IUnknown);
impl Printing3DModel {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DModel, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Unit(&self) -> ::windows::core::Result<Printing3DModelUnit> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DModelUnit>();
            (::windows::core::Interface::vtable(this).Unit)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetUnit(&self, value: Printing3DModelUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUnit)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Textures(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DModelTexture>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DModelTexture>>();
            (::windows::core::Interface::vtable(this).Textures)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Meshes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMesh>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DMesh>>();
            (::windows::core::Interface::vtable(this).Meshes)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Components(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DComponent>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DComponent>>();
            (::windows::core::Interface::vtable(this).Components)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Material(&self) -> ::windows::core::Result<Printing3DMaterial> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DMaterial>();
            (::windows::core::Interface::vtable(this).Material)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetMaterial(&self, value: &Printing3DMaterial) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMaterial)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Build(&self) -> ::windows::core::Result<Printing3DComponent> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DComponent>();
            (::windows::core::Interface::vtable(this).Build)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBuild(&self, value: &Printing3DComponent) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBuild)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Version)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVersion)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RequiredExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).RequiredExtensions)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Metadata(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IMap<::windows::core::HSTRING, ::windows::core::HSTRING>>();
            (::windows::core::Interface::vtable(this).Metadata)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RepairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncAction>();
            (::windows::core::Interface::vtable(this).RepairAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Clone(&self) -> ::windows::core::Result<Printing3DModel> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DModel>();
            (::windows::core::Interface::vtable(this).Clone)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPartialRepairAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryPartialRepairAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryPartialRepairWithTimeAsync(&self, maxwaittime: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<bool>>();
            (::windows::core::Interface::vtable(this).TryPartialRepairWithTimeAsync)(::windows::core::Interface::as_raw(this), maxwaittime, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryReduceFacesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>();
            (::windows::core::Interface::vtable(this).TryReduceFacesAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryReduceFacesWithOptionsAsync(&self, printing3dfacereductionoptions: &Printing3DFaceReductionOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>();
            (::windows::core::Interface::vtable(this).TryReduceFacesWithOptionsAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(printing3dfacereductionoptions), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryReduceFacesWithOptionsAndTimeAsync(&self, printing3dfacereductionoptions: &Printing3DFaceReductionOptions, maxwait: super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>();
            (::windows::core::Interface::vtable(this).TryReduceFacesWithOptionsAndTimeAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(printing3dfacereductionoptions), maxwait, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RepairWithProgressAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DModel2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperationWithProgress<bool, f64>>();
            (::windows::core::Interface::vtable(this).RepairWithProgressAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Printing3DModel {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DModel {}
impl ::core::fmt::Debug for Printing3DModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DModel").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DModel {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DModel;{2d012ef0-52fb-919a-77b0-4b1a3b80324f})");
}
impl ::core::clone::Clone for Printing3DModel {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DModel {
    type Vtable = IPrinting3DModel_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DModel {
    const IID: ::windows::core::GUID = <IPrinting3DModel as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DModel {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DModel";
}
::windows::imp::interface_hierarchy!(Printing3DModel, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DModel {}
unsafe impl ::core::marker::Sync for Printing3DModel {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DModelTexture(::windows::core::IUnknown);
impl Printing3DModelTexture {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DModelTexture, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn TextureResource(&self) -> ::windows::core::Result<Printing3DTextureResource> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DTextureResource>();
            (::windows::core::Interface::vtable(this).TextureResource)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTextureResource(&self, value: &Printing3DTextureResource) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextureResource)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn TileStyleU(&self) -> ::windows::core::Result<Printing3DTextureEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DTextureEdgeBehavior>();
            (::windows::core::Interface::vtable(this).TileStyleU)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTileStyleU(&self, value: Printing3DTextureEdgeBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTileStyleU)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TileStyleV(&self) -> ::windows::core::Result<Printing3DTextureEdgeBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DTextureEdgeBehavior>();
            (::windows::core::Interface::vtable(this).TileStyleV)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTileStyleV(&self, value: Printing3DTextureEdgeBehavior) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTileStyleV)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for Printing3DModelTexture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DModelTexture {}
impl ::core::fmt::Debug for Printing3DModelTexture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DModelTexture").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DModelTexture {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DModelTexture;{5dafcf01-b59d-483c-97bb-a4d546d1c75c})");
}
impl ::core::clone::Clone for Printing3DModelTexture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DModelTexture {
    type Vtable = IPrinting3DModelTexture_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DModelTexture {
    const IID: ::windows::core::GUID = <IPrinting3DModelTexture as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DModelTexture {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DModelTexture";
}
::windows::imp::interface_hierarchy!(Printing3DModelTexture, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DModelTexture {}
unsafe impl ::core::marker::Sync for Printing3DModelTexture {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DMultiplePropertyMaterial(::windows::core::IUnknown);
impl Printing3DMultiplePropertyMaterial {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DMultiplePropertyMaterial, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MaterialIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<u32>>();
            (::windows::core::Interface::vtable(this).MaterialIndices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for Printing3DMultiplePropertyMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMultiplePropertyMaterial {}
impl ::core::fmt::Debug for Printing3DMultiplePropertyMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMultiplePropertyMaterial").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DMultiplePropertyMaterial {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial;{25a6254b-c6e9-484d-a214-a25e5776ba62})");
}
impl ::core::clone::Clone for Printing3DMultiplePropertyMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DMultiplePropertyMaterial {
    type Vtable = IPrinting3DMultiplePropertyMaterial_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DMultiplePropertyMaterial {
    const IID: ::windows::core::GUID = <IPrinting3DMultiplePropertyMaterial as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DMultiplePropertyMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterial";
}
::windows::imp::interface_hierarchy!(Printing3DMultiplePropertyMaterial, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DMultiplePropertyMaterial {}
unsafe impl ::core::marker::Sync for Printing3DMultiplePropertyMaterial {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DMultiplePropertyMaterialGroup(::windows::core::IUnknown);
impl Printing3DMultiplePropertyMaterialGroup {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MultipleProperties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DMultiplePropertyMaterial>>();
            (::windows::core::Interface::vtable(this).MultipleProperties)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn MaterialGroupIndices(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<u32>>();
            (::windows::core::Interface::vtable(this).MaterialGroupIndices)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaterialGroupId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaterialGroupId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Create(materialgroupid: u32) -> ::windows::core::Result<Printing3DMultiplePropertyMaterialGroup> {
        Self::IPrinting3DMultiplePropertyMaterialGroupFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DMultiplePropertyMaterialGroup>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), materialgroupid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DMultiplePropertyMaterialGroupFactory<R, F: FnOnce(&IPrinting3DMultiplePropertyMaterialGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DMultiplePropertyMaterialGroup, IPrinting3DMultiplePropertyMaterialGroupFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Printing3DMultiplePropertyMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DMultiplePropertyMaterialGroup {}
impl ::core::fmt::Debug for Printing3DMultiplePropertyMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMultiplePropertyMaterialGroup").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DMultiplePropertyMaterialGroup {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup;{f0950519-aeb9-4515-a39b-a088fbbb277c})");
}
impl ::core::clone::Clone for Printing3DMultiplePropertyMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DMultiplePropertyMaterialGroup {
    type Vtable = IPrinting3DMultiplePropertyMaterialGroup_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DMultiplePropertyMaterialGroup {
    const IID: ::windows::core::GUID = <IPrinting3DMultiplePropertyMaterialGroup as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DMultiplePropertyMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DMultiplePropertyMaterialGroup";
}
::windows::imp::interface_hierarchy!(Printing3DMultiplePropertyMaterialGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DMultiplePropertyMaterialGroup {}
unsafe impl ::core::marker::Sync for Printing3DMultiplePropertyMaterialGroup {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DTexture2CoordMaterial(::windows::core::IUnknown);
impl Printing3DTexture2CoordMaterial {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DTexture2CoordMaterial, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Texture(&self) -> ::windows::core::Result<Printing3DModelTexture> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DModelTexture>();
            (::windows::core::Interface::vtable(this).Texture)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTexture(&self, value: &Printing3DModelTexture) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTexture)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn U(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).U)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetU(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetU)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn V(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).V)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetV(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetV)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::cmp::PartialEq for Printing3DTexture2CoordMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DTexture2CoordMaterial {}
impl ::core::fmt::Debug for Printing3DTexture2CoordMaterial {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DTexture2CoordMaterial").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DTexture2CoordMaterial {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial;{8d844bfb-07e9-4986-9833-8dd3d48c6859})");
}
impl ::core::clone::Clone for Printing3DTexture2CoordMaterial {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DTexture2CoordMaterial {
    type Vtable = IPrinting3DTexture2CoordMaterial_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DTexture2CoordMaterial {
    const IID: ::windows::core::GUID = <IPrinting3DTexture2CoordMaterial as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DTexture2CoordMaterial {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterial";
}
::windows::imp::interface_hierarchy!(Printing3DTexture2CoordMaterial, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DTexture2CoordMaterial {}
unsafe impl ::core::marker::Sync for Printing3DTexture2CoordMaterial {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DTexture2CoordMaterialGroup(::windows::core::IUnknown);
impl Printing3DTexture2CoordMaterialGroup {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Texture2Coords(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterial>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<Printing3DTexture2CoordMaterial>>();
            (::windows::core::Interface::vtable(this).Texture2Coords)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn MaterialGroupId(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).MaterialGroupId)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Texture(&self) -> ::windows::core::Result<Printing3DModelTexture> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DTexture2CoordMaterialGroup2>(self)?;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DModelTexture>();
            (::windows::core::Interface::vtable(this).Texture)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTexture(&self, value: &Printing3DModelTexture) -> ::windows::core::Result<()> {
        let this = &::windows::core::ComInterface::cast::<IPrinting3DTexture2CoordMaterialGroup2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTexture)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(materialgroupid: u32) -> ::windows::core::Result<Printing3DTexture2CoordMaterialGroup> {
        Self::IPrinting3DTexture2CoordMaterialGroupFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<Printing3DTexture2CoordMaterialGroup>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), materialgroupid, &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrinting3DTexture2CoordMaterialGroupFactory<R, F: FnOnce(&IPrinting3DTexture2CoordMaterialGroupFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DTexture2CoordMaterialGroup, IPrinting3DTexture2CoordMaterialGroupFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for Printing3DTexture2CoordMaterialGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DTexture2CoordMaterialGroup {}
impl ::core::fmt::Debug for Printing3DTexture2CoordMaterialGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DTexture2CoordMaterialGroup").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DTexture2CoordMaterialGroup {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup;{627d7ca7-6d90-4fb9-9fc4-9feff3dfa892})");
}
impl ::core::clone::Clone for Printing3DTexture2CoordMaterialGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DTexture2CoordMaterialGroup {
    type Vtable = IPrinting3DTexture2CoordMaterialGroup_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DTexture2CoordMaterialGroup {
    const IID: ::windows::core::GUID = <IPrinting3DTexture2CoordMaterialGroup as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DTexture2CoordMaterialGroup {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTexture2CoordMaterialGroup";
}
::windows::imp::interface_hierarchy!(Printing3DTexture2CoordMaterialGroup, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DTexture2CoordMaterialGroup {}
unsafe impl ::core::marker::Sync for Printing3DTexture2CoordMaterialGroup {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Printing3DTextureResource(::windows::core::IUnknown);
impl Printing3DTextureResource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<Printing3DTextureResource, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn TextureData(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamWithContentType> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Storage::Streams::IRandomAccessStreamWithContentType>();
            (::windows::core::Interface::vtable(this).TextureData)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Streams\"`*"]
    #[cfg(feature = "Storage_Streams")]
    pub fn SetTextureData<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<super::super::Storage::Streams::IRandomAccessStreamWithContentType>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTextureData)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
}
impl ::core::cmp::PartialEq for Printing3DTextureResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Printing3DTextureResource {}
impl ::core::fmt::Debug for Printing3DTextureResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DTextureResource").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DTextureResource {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.Graphics.Printing3D.Printing3DTextureResource;{a70df32d-6ab1-44ae-bc45-a27382c0d38c})");
}
impl ::core::clone::Clone for Printing3DTextureResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for Printing3DTextureResource {
    type Vtable = IPrinting3DTextureResource_Vtbl;
}
unsafe impl ::windows::core::ComInterface for Printing3DTextureResource {
    const IID: ::windows::core::GUID = <IPrinting3DTextureResource as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for Printing3DTextureResource {
    const NAME: &'static str = "Windows.Graphics.Printing3D.Printing3DTextureResource";
}
::windows::imp::interface_hierarchy!(Printing3DTextureResource, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for Printing3DTextureResource {}
unsafe impl ::core::marker::Sync for Printing3DTextureResource {}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Print3DTaskCompletion(pub i32);
impl Print3DTaskCompletion {
    pub const Abandoned: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const Failed: Self = Self(2i32);
    pub const Slicing: Self = Self(3i32);
    pub const Submitted: Self = Self(4i32);
}
impl ::core::marker::Copy for Print3DTaskCompletion {}
impl ::core::clone::Clone for Print3DTaskCompletion {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Print3DTaskCompletion {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Print3DTaskCompletion {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Print3DTaskCompletion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskCompletion").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DTaskCompletion {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Print3DTaskCompletion;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Print3DTaskDetail(pub i32);
impl Print3DTaskDetail {
    pub const Unknown: Self = Self(0i32);
    pub const ModelExceedsPrintBed: Self = Self(1i32);
    pub const UploadFailed: Self = Self(2i32);
    pub const InvalidMaterialSelection: Self = Self(3i32);
    pub const InvalidModel: Self = Self(4i32);
    pub const ModelNotManifold: Self = Self(5i32);
    pub const InvalidPrintTicket: Self = Self(6i32);
}
impl ::core::marker::Copy for Print3DTaskDetail {}
impl ::core::clone::Clone for Print3DTaskDetail {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Print3DTaskDetail {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Print3DTaskDetail {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Print3DTaskDetail {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskDetail").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Print3DTaskDetail {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Print3DTaskDetail;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Printing3DBufferFormat(pub i32);
impl Printing3DBufferFormat {
    pub const Unknown: Self = Self(0i32);
    pub const R32G32B32A32Float: Self = Self(2i32);
    pub const R32G32B32A32UInt: Self = Self(3i32);
    pub const R32G32B32Float: Self = Self(6i32);
    pub const R32G32B32UInt: Self = Self(7i32);
    pub const Printing3DDouble: Self = Self(500i32);
    pub const Printing3DUInt: Self = Self(501i32);
}
impl ::core::marker::Copy for Printing3DBufferFormat {}
impl ::core::clone::Clone for Printing3DBufferFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Printing3DBufferFormat {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Printing3DBufferFormat {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Printing3DBufferFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DBufferFormat").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DBufferFormat {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DBufferFormat;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Printing3DMeshVerificationMode(pub i32);
impl Printing3DMeshVerificationMode {
    pub const FindFirstError: Self = Self(0i32);
    pub const FindAllErrors: Self = Self(1i32);
}
impl ::core::marker::Copy for Printing3DMeshVerificationMode {}
impl ::core::clone::Clone for Printing3DMeshVerificationMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Printing3DMeshVerificationMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Printing3DMeshVerificationMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Printing3DMeshVerificationMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DMeshVerificationMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DMeshVerificationMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DMeshVerificationMode;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Printing3DModelUnit(pub i32);
impl Printing3DModelUnit {
    pub const Meter: Self = Self(0i32);
    pub const Micron: Self = Self(1i32);
    pub const Millimeter: Self = Self(2i32);
    pub const Centimeter: Self = Self(3i32);
    pub const Inch: Self = Self(4i32);
    pub const Foot: Self = Self(5i32);
}
impl ::core::marker::Copy for Printing3DModelUnit {}
impl ::core::clone::Clone for Printing3DModelUnit {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Printing3DModelUnit {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Printing3DModelUnit {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Printing3DModelUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DModelUnit").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DModelUnit {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DModelUnit;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Printing3DObjectType(pub i32);
impl Printing3DObjectType {
    pub const Model: Self = Self(0i32);
    pub const Support: Self = Self(1i32);
    pub const Others: Self = Self(2i32);
}
impl ::core::marker::Copy for Printing3DObjectType {}
impl ::core::clone::Clone for Printing3DObjectType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Printing3DObjectType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Printing3DObjectType {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Printing3DObjectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DObjectType").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DObjectType {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DObjectType;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Printing3DPackageCompression(pub i32);
impl Printing3DPackageCompression {
    pub const Low: Self = Self(0i32);
    pub const Medium: Self = Self(1i32);
    pub const High: Self = Self(2i32);
}
impl ::core::marker::Copy for Printing3DPackageCompression {}
impl ::core::clone::Clone for Printing3DPackageCompression {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Printing3DPackageCompression {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Printing3DPackageCompression {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Printing3DPackageCompression {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DPackageCompression").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DPackageCompression {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DPackageCompression;i4)");
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct Printing3DTextureEdgeBehavior(pub i32);
impl Printing3DTextureEdgeBehavior {
    pub const None: Self = Self(0i32);
    pub const Wrap: Self = Self(1i32);
    pub const Mirror: Self = Self(2i32);
    pub const Clamp: Self = Self(3i32);
}
impl ::core::marker::Copy for Printing3DTextureEdgeBehavior {}
impl ::core::clone::Clone for Printing3DTextureEdgeBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Printing3DTextureEdgeBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for Printing3DTextureEdgeBehavior {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Printing3DTextureEdgeBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Printing3DTextureEdgeBehavior").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Printing3DTextureEdgeBehavior {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.Graphics.Printing3D.Printing3DTextureEdgeBehavior;i4)");
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
pub struct Printing3DBufferDescription {
    pub Format: Printing3DBufferFormat,
    pub Stride: u32,
}
impl ::core::marker::Copy for Printing3DBufferDescription {}
impl ::core::clone::Clone for Printing3DBufferDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Printing3DBufferDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Printing3DBufferDescription").field("Format", &self.Format).field("Stride", &self.Stride).finish()
    }
}
impl ::windows::core::TypeKind for Printing3DBufferDescription {
    type TypeKind = ::windows::core::CopyType;
}
impl ::windows::core::RuntimeType for Printing3DBufferDescription {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"struct(Windows.Graphics.Printing3D.Printing3DBufferDescription;enum(Windows.Graphics.Printing3D.Printing3DBufferFormat;i4);u4)");
}
impl ::core::cmp::PartialEq for Printing3DBufferDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Stride == other.Stride
    }
}
impl ::core::cmp::Eq for Printing3DBufferDescription {}
impl ::core::default::Default for Printing3DBufferDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Graphics_Printing3D\"`*"]
#[repr(transparent)]
pub struct Print3DTaskSourceRequestedHandler(pub ::windows::core::IUnknown);
impl Print3DTaskSourceRequestedHandler {
    pub fn new<F: FnMut(::core::option::Option<&Print3DTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = Print3DTaskSourceRequestedHandlerBox::<F> { vtable: &Print3DTaskSourceRequestedHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, args: &Print3DTaskSourceRequestedArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(args)).ok() }
    }
}
#[repr(C)]
struct Print3DTaskSourceRequestedHandlerBox<F: FnMut(::core::option::Option<&Print3DTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const Print3DTaskSourceRequestedHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&Print3DTaskSourceRequestedArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> Print3DTaskSourceRequestedHandlerBox<F> {
    const VTABLE: Print3DTaskSourceRequestedHandler_Vtbl = Print3DTaskSourceRequestedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<Print3DTaskSourceRequestedHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: *mut ::core::ffi::c_void) -> u32 {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            let _ = ::std::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&args)).into()
    }
}
impl ::core::cmp::PartialEq for Print3DTaskSourceRequestedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Print3DTaskSourceRequestedHandler {}
impl ::core::fmt::Debug for Print3DTaskSourceRequestedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Print3DTaskSourceRequestedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for Print3DTaskSourceRequestedHandler {
    type Vtable = Print3DTaskSourceRequestedHandler_Vtbl;
}
impl ::core::clone::Clone for Print3DTaskSourceRequestedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for Print3DTaskSourceRequestedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9175e70_c917_46de_bb51_d9a94db3711f);
}
impl ::windows::core::RuntimeType for Print3DTaskSourceRequestedHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{e9175e70-c917-46de-bb51-d9a94db3711f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct Print3DTaskSourceRequestedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, args: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
