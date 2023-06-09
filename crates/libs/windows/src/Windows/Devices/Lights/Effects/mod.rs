#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffect_Vtbl;
}
impl ::core::clone::Clone for ILampArrayBitmapEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayBitmapEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x3238e065_d877_4627_89e5_2a88f7052fa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub StartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUpdateInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SuggestedBitmapSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SuggestedBitmapSize: usize,
    #[cfg(feature = "Foundation")]
    pub BitmapRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    BitmapRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBitmapRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBitmapRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBitmapEffectFactory {
    type Vtable = ILampArrayBitmapEffectFactory_Vtbl;
}
impl ::core::clone::Clone for ILampArrayBitmapEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayBitmapEffectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x13608090_e336_4c8f_9053_a92407ca7b1d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBitmapRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILampArrayBitmapRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayBitmapRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xc8b4af9e_fe63_4d51_babd_619defb454ba);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBitmapRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SinceStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SinceStarted: usize,
    #[cfg(feature = "Graphics_Imaging")]
    pub UpdateBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bitmap: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Graphics_Imaging"))]
    UpdateBitmap: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBlinkEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffect_Vtbl;
}
impl ::core::clone::Clone for ILampArrayBlinkEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayBlinkEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xebbf35f6_2fc5_4bb3_b3c3_6221a7680d13);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub AttackDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttackDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetAttackDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetAttackDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SustainDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SustainDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetSustainDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetSustainDuration: usize,
    #[cfg(feature = "Foundation")]
    pub DecayDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    DecayDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDecayDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDecayDuration: usize,
    #[cfg(feature = "Foundation")]
    pub RepetitionDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RepetitionDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetRepetitionDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRepetitionDelay: usize,
    #[cfg(feature = "Foundation")]
    pub StartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDelay: usize,
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub RepetitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayRepetitionMode) -> ::windows_core::HRESULT,
    pub SetRepetitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayRepetitionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayBlinkEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayBlinkEffectFactory {
    type Vtable = ILampArrayBlinkEffectFactory_Vtbl;
}
impl ::core::clone::Clone for ILampArrayBlinkEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayBlinkEffectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x879f1d97_9f50_49b2_a56f_013aa08d55e0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayBlinkEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayColorRampEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffect_Vtbl;
}
impl ::core::clone::Clone for ILampArrayColorRampEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayColorRampEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x2b004437_40a7_432e_a0b9_0d570c2153ff);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub RampDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RampDuration: usize,
    #[cfg(feature = "Foundation")]
    pub SetRampDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetRampDuration: usize,
    #[cfg(feature = "Foundation")]
    pub StartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDelay: usize,
    pub CompletionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows_core::HRESULT,
    pub SetCompletionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectCompletionBehavior) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayColorRampEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayColorRampEffectFactory {
    type Vtable = ILampArrayColorRampEffectFactory_Vtbl;
}
impl ::core::clone::Clone for ILampArrayColorRampEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayColorRampEffectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x520bd133_0c74_4df5_bea7_4899e0266b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayColorRampEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayCustomEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffect_Vtbl;
}
impl ::core::clone::Clone for ILampArrayCustomEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayCustomEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xec579170_3c34_4876_818b_5765f78b0ee4);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayCustomEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateInterval: usize,
    #[cfg(feature = "Foundation")]
    pub SetUpdateInterval: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUpdateInterval: usize,
    #[cfg(feature = "Foundation")]
    pub UpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateRequested: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveUpdateRequested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveUpdateRequested: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayCustomEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayCustomEffectFactory {
    type Vtable = ILampArrayCustomEffectFactory_Vtbl;
}
impl ::core::clone::Clone for ILampArrayCustomEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayCustomEffectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x68b4774d_63e5_4af0_a58b_3e535b94e8c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayCustomEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct ILampArrayEffect(::windows_core::IUnknown);
impl ILampArrayEffect {
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
}
::windows_core::imp::interface_hierarchy!(ILampArrayEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::core::cmp::PartialEq for ILampArrayEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILampArrayEffect {}
impl ::core::fmt::Debug for ILampArrayEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILampArrayEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for ILampArrayEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{11d45590-57fb-4546-b1ce-863107f740df}");
}
unsafe impl ::windows_core::Interface for ILampArrayEffect {
    type Vtable = ILampArrayEffect_Vtbl;
}
impl ::core::clone::Clone for ILampArrayEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x11d45590_57fb_4546_b1ce_863107f740df);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub ZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayEffectPlaylist(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylist_Vtbl;
}
impl ::core::clone::Clone for ILampArrayEffectPlaylist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayEffectPlaylist {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7de58bfe_6f61_4103_98c7_d6632f7b9169);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylist_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Append: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, effect: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub OverrideZIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zindex: i32) -> ::windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    pub EffectStartMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectStartMode) -> ::windows_core::HRESULT,
    pub SetEffectStartMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectStartMode) -> ::windows_core::HRESULT,
    pub Occurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows_core::HRESULT,
    pub SetOccurrences: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT,
    pub RepetitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayRepetitionMode) -> ::windows_core::HRESULT,
    pub SetRepetitionMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayRepetitionMode) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayEffectPlaylistStatics(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayEffectPlaylistStatics {
    type Vtable = ILampArrayEffectPlaylistStatics_Vtbl;
}
impl ::core::clone::Clone for ILampArrayEffectPlaylistStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayEffectPlaylistStatics {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfb15235c_ea35_4c7f_a016_f3bfc6a6c47d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayEffectPlaylistStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub StartAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StartAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub StopAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    StopAll: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PauseAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PauseAll: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArraySolidEffect(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArraySolidEffect {
    type Vtable = ILampArraySolidEffect_Vtbl;
}
impl ::core::clone::Clone for ILampArraySolidEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArraySolidEffect {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x441f8213_43cc_4b33_80eb_c6ddde7dc8ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArraySolidEffect_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "UI")]
    pub Color: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    Color: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "Foundation")]
    pub Duration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Duration: usize,
    #[cfg(feature = "Foundation")]
    pub SetDuration: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetDuration: usize,
    #[cfg(feature = "Foundation")]
    pub StartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartDelay: usize,
    #[cfg(feature = "Foundation")]
    pub SetStartDelay: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetStartDelay: usize,
    pub CompletionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut LampArrayEffectCompletionBehavior) -> ::windows_core::HRESULT,
    pub SetCompletionBehavior: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: LampArrayEffectCompletionBehavior) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArraySolidEffectFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArraySolidEffectFactory {
    type Vtable = ILampArraySolidEffectFactory_Vtbl;
}
impl ::core::clone::Clone for ILampArraySolidEffectFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArraySolidEffectFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf862a32c_5576_4341_961b_aee1f13cf9dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArraySolidEffectFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lamparray: *mut ::core::ffi::c_void, lampIndexes_array_size: u32, lampindexes: *const i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ILampArrayUpdateRequestedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for ILampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgs_Vtbl;
}
impl ::core::clone::Clone for ILampArrayUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for ILampArrayUpdateRequestedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x73560d6a_576a_48af_8539_67ffa0ab3516);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILampArrayUpdateRequestedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub SinceStarted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SinceStarted: usize,
    #[cfg(feature = "UI")]
    pub SetColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColor: usize,
    #[cfg(feature = "UI")]
    pub SetColorForIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lampindex: i32, desiredcolor: super::super::super::UI::Color) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorForIndex: usize,
    #[cfg(feature = "UI")]
    pub SetSingleColorForIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredcolor: super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetSingleColorForIndices: usize,
    #[cfg(feature = "UI")]
    pub SetColorsForIndices: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desiredColors_array_size: u32, desiredcolors: *const super::super::super::UI::Color, lampIndexes_array_size: u32, lampindexes: *const i32) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "UI"))]
    SetColorsForIndices: usize,
}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayBitmapEffect(::windows_core::IUnknown);
impl LampArrayBitmapEffect {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDelay(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartDelay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDelay(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateInterval(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUpdateInterval(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUpdateInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SuggestedBitmapSize(&self) -> ::windows_core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SuggestedBitmapSize)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn BitmapRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<LampArrayBitmapEffect, LampArrayBitmapRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).BitmapRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBitmapRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveBitmapRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> ::windows_core::Result<LampArrayBitmapEffect>
    where
        P0: ::windows_core::IntoParam<super::LampArray>,
    {
        Self::ILampArrayBitmapEffectFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, lampindexes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayBitmapEffectFactory<R, F: FnOnce(&ILampArrayBitmapEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LampArrayBitmapEffect, ILampArrayBitmapEffectFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LampArrayBitmapEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBitmapEffect {}
impl ::core::fmt::Debug for LampArrayBitmapEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBitmapEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayBitmapEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBitmapEffect;{3238e065-d877-4627-89e5-2a88f7052fa6})");
}
impl ::core::clone::Clone for LampArrayBitmapEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LampArrayBitmapEffect {
    type Vtable = ILampArrayBitmapEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LampArrayBitmapEffect {
    const IID: ::windows_core::GUID = <ILampArrayBitmapEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayBitmapEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapEffect";
}
::windows_core::imp::interface_hierarchy!(LampArrayBitmapEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILampArrayEffect> for LampArrayBitmapEffect {}
unsafe impl ::core::marker::Send for LampArrayBitmapEffect {}
unsafe impl ::core::marker::Sync for LampArrayBitmapEffect {}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayBitmapRequestedEventArgs(::windows_core::IUnknown);
impl LampArrayBitmapRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SinceStarted(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SinceStarted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Imaging\"`*"]
    #[cfg(feature = "Graphics_Imaging")]
    pub fn UpdateBitmap<P0>(&self, bitmap: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<super::super::super::Graphics::Imaging::SoftwareBitmap>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UpdateBitmap)(::windows_core::Interface::as_raw(this), bitmap.into_param().abi()).ok() }
    }
}
impl ::core::cmp::PartialEq for LampArrayBitmapRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBitmapRequestedEventArgs {}
impl ::core::fmt::Debug for LampArrayBitmapRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBitmapRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayBitmapRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs;{c8b4af9e-fe63-4d51-babd-619defb454ba})");
}
impl ::core::clone::Clone for LampArrayBitmapRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LampArrayBitmapRequestedEventArgs {
    type Vtable = ILampArrayBitmapRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LampArrayBitmapRequestedEventArgs {
    const IID: ::windows_core::GUID = <ILampArrayBitmapRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayBitmapRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBitmapRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(LampArrayBitmapRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LampArrayBitmapRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LampArrayBitmapRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayBlinkEffect(::windows_core::IUnknown);
impl LampArrayBlinkEffect {
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttackDuration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AttackDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetAttackDuration(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetAttackDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SustainDuration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SustainDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetSustainDuration(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSustainDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DecayDuration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DecayDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDecayDuration(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDecayDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RepetitionDelay(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RepetitionDelay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRepetitionDelay(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepetitionDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDelay(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartDelay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDelay(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Occurrences)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOccurrences(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOccurrences)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepetitionMode(&self) -> ::windows_core::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RepetitionMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepetitionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> ::windows_core::Result<LampArrayBlinkEffect>
    where
        P0: ::windows_core::IntoParam<super::LampArray>,
    {
        Self::ILampArrayBlinkEffectFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, lampindexes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayBlinkEffectFactory<R, F: FnOnce(&ILampArrayBlinkEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LampArrayBlinkEffect, ILampArrayBlinkEffectFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LampArrayBlinkEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayBlinkEffect {}
impl ::core::fmt::Debug for LampArrayBlinkEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayBlinkEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayBlinkEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayBlinkEffect;{ebbf35f6-2fc5-4bb3-b3c3-6221a7680d13})");
}
impl ::core::clone::Clone for LampArrayBlinkEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LampArrayBlinkEffect {
    type Vtable = ILampArrayBlinkEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LampArrayBlinkEffect {
    const IID: ::windows_core::GUID = <ILampArrayBlinkEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayBlinkEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayBlinkEffect";
}
::windows_core::imp::interface_hierarchy!(LampArrayBlinkEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILampArrayEffect> for LampArrayBlinkEffect {}
unsafe impl ::core::marker::Send for LampArrayBlinkEffect {}
unsafe impl ::core::marker::Sync for LampArrayBlinkEffect {}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayColorRampEffect(::windows_core::IUnknown);
impl LampArrayColorRampEffect {
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RampDuration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RampDuration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetRampDuration(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRampDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDelay(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartDelay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDelay(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletionBehavior(&self) -> ::windows_core::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompletionBehavior)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletionBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> ::windows_core::Result<LampArrayColorRampEffect>
    where
        P0: ::windows_core::IntoParam<super::LampArray>,
    {
        Self::ILampArrayColorRampEffectFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, lampindexes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayColorRampEffectFactory<R, F: FnOnce(&ILampArrayColorRampEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LampArrayColorRampEffect, ILampArrayColorRampEffectFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LampArrayColorRampEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayColorRampEffect {}
impl ::core::fmt::Debug for LampArrayColorRampEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayColorRampEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayColorRampEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayColorRampEffect;{2b004437-40a7-432e-a0b9-0d570c2153ff})");
}
impl ::core::clone::Clone for LampArrayColorRampEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LampArrayColorRampEffect {
    type Vtable = ILampArrayColorRampEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LampArrayColorRampEffect {
    const IID: ::windows_core::GUID = <ILampArrayColorRampEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayColorRampEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayColorRampEffect";
}
::windows_core::imp::interface_hierarchy!(LampArrayColorRampEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILampArrayEffect> for LampArrayColorRampEffect {}
unsafe impl ::core::marker::Send for LampArrayColorRampEffect {}
unsafe impl ::core::marker::Sync for LampArrayColorRampEffect {}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayCustomEffect(::windows_core::IUnknown);
impl LampArrayCustomEffect {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateInterval(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateInterval)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUpdateInterval(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetUpdateInterval)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateRequested<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<LampArrayCustomEffect, LampArrayUpdateRequestedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateRequested)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdateRequested(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveUpdateRequested)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> ::windows_core::Result<LampArrayCustomEffect>
    where
        P0: ::windows_core::IntoParam<super::LampArray>,
    {
        Self::ILampArrayCustomEffectFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, lampindexes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc(hidden)]
    pub fn ILampArrayCustomEffectFactory<R, F: FnOnce(&ILampArrayCustomEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LampArrayCustomEffect, ILampArrayCustomEffectFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LampArrayCustomEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayCustomEffect {}
impl ::core::fmt::Debug for LampArrayCustomEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayCustomEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayCustomEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayCustomEffect;{ec579170-3c34-4876-818b-5765f78b0ee4})");
}
impl ::core::clone::Clone for LampArrayCustomEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LampArrayCustomEffect {
    type Vtable = ILampArrayCustomEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LampArrayCustomEffect {
    const IID: ::windows_core::GUID = <ILampArrayCustomEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayCustomEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayCustomEffect";
}
::windows_core::imp::interface_hierarchy!(LampArrayCustomEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILampArrayEffect> for LampArrayCustomEffect {}
unsafe impl ::core::marker::Send for LampArrayCustomEffect {}
unsafe impl ::core::marker::Sync for LampArrayCustomEffect {}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayEffectPlaylist(::windows_core::IUnknown);
impl LampArrayEffectPlaylist {
    pub fn new() -> ::windows_core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows_core::imp::IGenericFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LampArrayEffectPlaylist, ::windows_core::imp::IGenericFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows_core::Result<super::super::super::Foundation::Collections::IIterator<ILampArrayEffect>> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).First)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Append<P0>(&self, effect: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<ILampArrayEffect>,
    {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Append)(::windows_core::Interface::as_raw(this), effect.try_into_param()?.abi()).ok() }
    }
    pub fn OverrideZIndex(&self, zindex: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).OverrideZIndex)(::windows_core::Interface::as_raw(this), zindex).ok() }
    }
    pub fn Start(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Start)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Stop(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Stop)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Pause(&self) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).Pause)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn EffectStartMode(&self) -> ::windows_core::Result<LampArrayEffectStartMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).EffectStartMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEffectStartMode(&self, value: LampArrayEffectStartMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetEffectStartMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn Occurrences(&self) -> ::windows_core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Occurrences)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOccurrences(&self, value: i32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetOccurrences)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RepetitionMode(&self) -> ::windows_core::Result<LampArrayRepetitionMode> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).RepetitionMode)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetRepetitionMode(&self, value: LampArrayRepetitionMode) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetRepetitionMode)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StartAll<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>,
    {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows_core::Interface::vtable(this).StartAll)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn StopAll<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>,
    {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows_core::Interface::vtable(this).StopAll)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PauseAll<P0>(value: P0) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::TryIntoParam<super::super::super::Foundation::Collections::IIterable<LampArrayEffectPlaylist>>,
    {
        Self::ILampArrayEffectPlaylistStatics(|this| unsafe { (::windows_core::Interface::vtable(this).PauseAll)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() })
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows_core::Result<ILampArrayEffect> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetAt)(::windows_core::Interface::as_raw(this), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Size)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<P0>(&self, value: P0, index: &mut u32) -> ::windows_core::Result<bool>
    where
        P0: ::windows_core::TryIntoParam<ILampArrayEffect>,
    {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IndexOf)(::windows_core::Interface::as_raw(this), value.try_into_param()?.abi(), index, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<ILampArrayEffect>]) -> ::windows_core::Result<u32> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetMany)(::windows_core::Interface::as_raw(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi(result__)
        }
    }
    #[doc(hidden)]
    pub fn ILampArrayEffectPlaylistStatics<R, F: FnOnce(&ILampArrayEffectPlaylistStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LampArrayEffectPlaylist, ILampArrayEffectPlaylistStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LampArrayEffectPlaylist {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayEffectPlaylist {}
impl ::core::fmt::Debug for LampArrayEffectPlaylist {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectPlaylist").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayEffectPlaylist {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayEffectPlaylist;{7de58bfe-6f61-4103-98c7-d6632f7b9169})");
}
impl ::core::clone::Clone for LampArrayEffectPlaylist {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LampArrayEffectPlaylist {
    type Vtable = ILampArrayEffectPlaylist_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LampArrayEffectPlaylist {
    const IID: ::windows_core::GUID = <ILampArrayEffectPlaylist as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayEffectPlaylist {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayEffectPlaylist";
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &LampArrayEffectPlaylist {
    type Item = ILampArrayEffect;
    type IntoIter = super::super::super::Foundation::Collections::VectorViewIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorViewIterator::new(::windows_core::ComInterface::cast(self).ok())
    }
}
::windows_core::imp::interface_hierarchy!(LampArrayEffectPlaylist, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IIterable<ILampArrayEffect>> for LampArrayEffectPlaylist {}
#[cfg(feature = "Foundation_Collections")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::Collections::IVectorView<ILampArrayEffect>> for LampArrayEffectPlaylist {}
unsafe impl ::core::marker::Send for LampArrayEffectPlaylist {}
unsafe impl ::core::marker::Sync for LampArrayEffectPlaylist {}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArraySolidEffect(::windows_core::IUnknown);
impl LampArraySolidEffect {
    pub fn ZIndex(&self) -> ::windows_core::Result<i32> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ZIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetZIndex(&self, value: i32) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<ILampArrayEffect>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).SetZIndex)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn Color(&self) -> ::windows_core::Result<super::super::super::UI::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Color)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, value: super::super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Duration(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Duration)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetDuration(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetDuration)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartDelay(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartDelay)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetStartDelay(&self, value: super::super::super::Foundation::TimeSpan) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetStartDelay)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CompletionBehavior(&self) -> ::windows_core::Result<LampArrayEffectCompletionBehavior> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CompletionBehavior)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCompletionBehavior(&self, value: LampArrayEffectCompletionBehavior) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetCompletionBehavior)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn CreateInstance<P0>(lamparray: P0, lampindexes: &[i32]) -> ::windows_core::Result<LampArraySolidEffect>
    where
        P0: ::windows_core::IntoParam<super::LampArray>,
    {
        Self::ILampArraySolidEffectFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), lamparray.into_param().abi(), lampindexes.len() as u32, lampindexes.as_ptr(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ILampArraySolidEffectFactory<R, F: FnOnce(&ILampArraySolidEffectFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<LampArraySolidEffect, ILampArraySolidEffectFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::cmp::PartialEq for LampArraySolidEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArraySolidEffect {}
impl ::core::fmt::Debug for LampArraySolidEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArraySolidEffect").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArraySolidEffect {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArraySolidEffect;{441f8213-43cc-4b33-80eb-c6ddde7dc8ed})");
}
impl ::core::clone::Clone for LampArraySolidEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LampArraySolidEffect {
    type Vtable = ILampArraySolidEffect_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LampArraySolidEffect {
    const IID: ::windows_core::GUID = <ILampArraySolidEffect as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LampArraySolidEffect {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArraySolidEffect";
}
::windows_core::imp::interface_hierarchy!(LampArraySolidEffect, ::windows_core::IUnknown, ::windows_core::IInspectable);
impl ::windows_core::CanTryInto<ILampArrayEffect> for LampArraySolidEffect {}
unsafe impl ::core::marker::Send for LampArraySolidEffect {}
unsafe impl ::core::marker::Sync for LampArraySolidEffect {}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
pub struct LampArrayUpdateRequestedEventArgs(::windows_core::IUnknown);
impl LampArrayUpdateRequestedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SinceStarted(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).SinceStarted)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColor(&self, desiredcolor: super::super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColor)(::windows_core::Interface::as_raw(this), desiredcolor).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColorForIndex(&self, lampindex: i32, desiredcolor: super::super::super::UI::Color) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorForIndex)(::windows_core::Interface::as_raw(this), lampindex, desiredcolor).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetSingleColorForIndices(&self, desiredcolor: super::super::super::UI::Color, lampindexes: &[i32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetSingleColorForIndices)(::windows_core::Interface::as_raw(this), desiredcolor, lampindexes.len() as u32, lampindexes.as_ptr()).ok() }
    }
    #[doc = "*Required features: `\"UI\"`*"]
    #[cfg(feature = "UI")]
    pub fn SetColorsForIndices(&self, desiredcolors: &[super::super::super::UI::Color], lampindexes: &[i32]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetColorsForIndices)(::windows_core::Interface::as_raw(this), desiredcolors.len() as u32, desiredcolors.as_ptr(), lampindexes.len() as u32, lampindexes.as_ptr()).ok() }
    }
}
impl ::core::cmp::PartialEq for LampArrayUpdateRequestedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for LampArrayUpdateRequestedEventArgs {}
impl ::core::fmt::Debug for LampArrayUpdateRequestedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayUpdateRequestedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayUpdateRequestedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs;{73560d6a-576a-48af-8539-67ffa0ab3516})");
}
impl ::core::clone::Clone for LampArrayUpdateRequestedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for LampArrayUpdateRequestedEventArgs {
    type Vtable = ILampArrayUpdateRequestedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for LampArrayUpdateRequestedEventArgs {
    const IID: ::windows_core::GUID = <ILampArrayUpdateRequestedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for LampArrayUpdateRequestedEventArgs {
    const NAME: &'static str = "Windows.Devices.Lights.Effects.LampArrayUpdateRequestedEventArgs";
}
::windows_core::imp::interface_hierarchy!(LampArrayUpdateRequestedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for LampArrayUpdateRequestedEventArgs {}
unsafe impl ::core::marker::Sync for LampArrayUpdateRequestedEventArgs {}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LampArrayEffectCompletionBehavior(pub i32);
impl LampArrayEffectCompletionBehavior {
    pub const ClearState: Self = Self(0i32);
    pub const KeepState: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectCompletionBehavior {}
impl ::core::clone::Clone for LampArrayEffectCompletionBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampArrayEffectCompletionBehavior {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LampArrayEffectCompletionBehavior {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LampArrayEffectCompletionBehavior {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectCompletionBehavior").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayEffectCompletionBehavior {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectCompletionBehavior;i4)");
}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LampArrayEffectStartMode(pub i32);
impl LampArrayEffectStartMode {
    pub const Sequential: Self = Self(0i32);
    pub const Simultaneous: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayEffectStartMode {}
impl ::core::clone::Clone for LampArrayEffectStartMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampArrayEffectStartMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LampArrayEffectStartMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LampArrayEffectStartMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayEffectStartMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayEffectStartMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayEffectStartMode;i4)");
}
#[doc = "*Required features: `\"Devices_Lights_Effects\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LampArrayRepetitionMode(pub i32);
impl LampArrayRepetitionMode {
    pub const Occurrences: Self = Self(0i32);
    pub const Forever: Self = Self(1i32);
}
impl ::core::marker::Copy for LampArrayRepetitionMode {}
impl ::core::clone::Clone for LampArrayRepetitionMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LampArrayRepetitionMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows_core::TypeKind for LampArrayRepetitionMode {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for LampArrayRepetitionMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LampArrayRepetitionMode").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for LampArrayRepetitionMode {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Devices.Lights.Effects.LampArrayRepetitionMode;i4)");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
