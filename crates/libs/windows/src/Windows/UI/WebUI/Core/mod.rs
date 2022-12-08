#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBar {
    type Vtable = IWebUICommandBar_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4fc0016_dbe5_41ad_8d7b_14698bd6911d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBar_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Opacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetOpacity: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
    pub ForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub SetForegroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    pub BackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::Color) -> ::windows::core::HRESULT,
    pub SetBackgroundColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::Color) -> ::windows::core::HRESULT,
    pub ClosedDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut WebUICommandBarClosedDisplayMode) -> ::windows::core::HRESULT,
    pub SetClosedDisplayMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::HRESULT,
    pub IsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub PrimaryCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrimaryCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SecondaryCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SecondaryCommands: usize,
    #[cfg(feature = "Foundation")]
    pub MenuOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MenuOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMenuOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMenuOpened: usize,
    #[cfg(feature = "Foundation")]
    pub MenuClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MenuClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMenuClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMenuClosed: usize,
    #[cfg(feature = "Foundation")]
    pub SizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SizeChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSizeChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIcon(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIcon_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarBitmapIcon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x858f4f45_08d8_4a46_81ec_00015b0b1c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIcon_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIconFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarBitmapIconFactory {
    type Vtable = IWebUICommandBarBitmapIconFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarBitmapIconFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3f7d78a_7673_444a_be62_ac12d31c2231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIconFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarConfirmationButton(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButton_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarConfirmationButton {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e7824a_e3d5_4eb6_b2ff_8f018a172105);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarConfirmationButton_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemInvoked: usize,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct IWebUICommandBarElement(::windows::core::IUnknown);
impl IWebUICommandBarElement {}
::windows::core::interface_hierarchy!(IWebUICommandBarElement, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IWebUICommandBarElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebUICommandBarElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebUICommandBarElement {}
impl ::core::fmt::Debug for IWebUICommandBarElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebUICommandBarElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebUICommandBarElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c9069ec2-284a-4633-8aad-637a27e282c3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IWebUICommandBarElement {
    type Vtable = IWebUICommandBarElement_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarElement {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9069ec2_284a_4633_8aad_637a27e282c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarElement_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct IWebUICommandBarIcon(::windows::core::IUnknown);
impl IWebUICommandBarIcon {}
::windows::core::interface_hierarchy!(IWebUICommandBarIcon, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::clone::Clone for IWebUICommandBarIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebUICommandBarIcon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebUICommandBarIcon {}
impl ::core::fmt::Debug for IWebUICommandBarIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebUICommandBarIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebUICommandBarIcon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d587655d-2014-42be-969a-7d14ca6c8a49}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for IWebUICommandBarIcon {
    type Vtable = IWebUICommandBarIcon_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarIcon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd587655d_2014_42be_969a_7d14ca6c8a49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIcon_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarIconButton(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButton_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarIconButton {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1bc93a_3a7c_4842_a0cf_aff6ea308586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIconButton_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IsToggleButton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsToggleButton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ItemInvoked: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveItemInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveItemInvoked: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarItemInvokedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarItemInvokedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x304edbdd_e741_41ef_bdc4_a45cea2a4f70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarItemInvokedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub IsPrimaryCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSizeChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarSizeChangedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf1e2f6_3029_4719_8378_92f82b87af1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSizeChangedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub Size: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::Size) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Size: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarStatics {
    type Vtable = IWebUICommandBarStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1449cdb9_a506_45be_8f42_b2837e2fe0c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIcon(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIcon_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarSymbolIcon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4935477_fd26_46ed_8658_1a3f4400e7b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIcon_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Symbol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSymbol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIconFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IWebUICommandBarSymbolIconFactory {
    type Vtable = IWebUICommandBarSymbolIconFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IWebUICommandBarSymbolIconFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51be1a1f_3730_429e_b622_14e2b7bf6a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIconFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbol: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBar(::windows::core::IUnknown);
impl WebUICommandBar {
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Visible)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetVisible)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Opacity)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetOpacity)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ForegroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetForegroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).BackgroundColor)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetBackgroundColor)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn ClosedDisplayMode(&self) -> ::windows::core::Result<WebUICommandBarClosedDisplayMode> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ClosedDisplayMode)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetClosedDisplayMode(&self, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetClosedDisplayMode)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsOpen)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsOpen)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrimaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PrimaryCommands)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SecondaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SecondaryCommands)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MenuOpened(&self, handler: &MenuOpenedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MenuOpened)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMenuOpened(&self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMenuOpened)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MenuClosed(&self, handler: &MenuClosedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).MenuClosed)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMenuClosed(&self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveMenuClosed)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeChanged(&self, handler: &SizeChangedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).SizeChanged)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSizeChanged(&self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveSizeChanged)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<WebUICommandBar> {
        Self::IWebUICommandBarStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).GetForCurrentView)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarStatics<R, F: FnOnce(&IWebUICommandBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUICommandBar, IWebUICommandBarStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WebUICommandBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBar {}
impl ::core::fmt::Debug for WebUICommandBar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBar {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBar;{a4fc0016-dbe5-41ad-8d7b-14698bd6911d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebUICommandBar {
    type Vtable = IWebUICommandBar_Vtbl;
}
unsafe impl ::windows::core::Interface for WebUICommandBar {
    const IID: ::windows::core::GUID = <IWebUICommandBar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBar {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBar";
}
::windows::core::interface_hierarchy!(WebUICommandBar, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebUICommandBar {}
unsafe impl ::core::marker::Sync for WebUICommandBar {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarBitmapIcon(::windows::core::IUnknown);
impl WebUICommandBarBitmapIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUICommandBarBitmapIcon, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Uri)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetUri)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create(uri: &super::super::super::Foundation::Uri) -> ::windows::core::Result<WebUICommandBarBitmapIcon> {
        Self::IWebUICommandBarBitmapIconFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(uri), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarBitmapIconFactory<R, F: FnOnce(&IWebUICommandBarBitmapIconFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUICommandBarBitmapIcon, IWebUICommandBarBitmapIconFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WebUICommandBarBitmapIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarBitmapIcon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarBitmapIcon {}
impl ::core::fmt::Debug for WebUICommandBarBitmapIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarBitmapIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarBitmapIcon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon;{858f4f45-08d8-4a46-81ec-00015b0b1c6c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIcon_Vtbl;
}
unsafe impl ::windows::core::Interface for WebUICommandBarBitmapIcon {
    const IID: ::windows::core::GUID = <IWebUICommandBarBitmapIcon as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarBitmapIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon";
}
::windows::core::interface_hierarchy!(WebUICommandBarBitmapIcon, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<WebUICommandBarBitmapIcon> for IWebUICommandBarIcon {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandBarBitmapIcon) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarBitmapIcon> for IWebUICommandBarIcon {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarBitmapIcon) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarBitmapIcon> for ::windows::core::InParam<IWebUICommandBarIcon> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarBitmapIcon) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarBitmapIcon {}
unsafe impl ::core::marker::Sync for WebUICommandBarBitmapIcon {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarConfirmationButton(::windows::core::IUnknown);
impl WebUICommandBarConfirmationButton {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUICommandBarConfirmationButton, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Text)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetText)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemInvoked(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemInvoked)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemInvoked(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveItemInvoked)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for WebUICommandBarConfirmationButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarConfirmationButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarConfirmationButton {}
impl ::core::fmt::Debug for WebUICommandBarConfirmationButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarConfirmationButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarConfirmationButton {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton;{86e7824a-e3d5-4eb6-b2ff-8f018a172105})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButton_Vtbl;
}
unsafe impl ::windows::core::Interface for WebUICommandBarConfirmationButton {
    const IID: ::windows::core::GUID = <IWebUICommandBarConfirmationButton as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarConfirmationButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton";
}
::windows::core::interface_hierarchy!(WebUICommandBarConfirmationButton, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<WebUICommandBarConfirmationButton> for IWebUICommandBarElement {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandBarConfirmationButton) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarConfirmationButton> for IWebUICommandBarElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarConfirmationButton) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarConfirmationButton> for ::windows::core::InParam<IWebUICommandBarElement> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarConfirmationButton) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarConfirmationButton {}
unsafe impl ::core::marker::Sync for WebUICommandBarConfirmationButton {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarIconButton(::windows::core::IUnknown);
impl WebUICommandBarIconButton {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUICommandBarIconButton, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Enabled)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetEnabled)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Label)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetLabel)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsToggleButton(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsToggleButton)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsToggleButton(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsToggleButton)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn IsChecked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsChecked)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIsChecked)(::windows::core::Vtable::as_raw(this), value).ok() }
    }
    pub fn Icon(&self) -> ::windows::core::Result<IWebUICommandBarIcon> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Icon)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetIcon<P0, E0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<IWebUICommandBarIcon>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetIcon)(::windows::core::Vtable::as_raw(this), value.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemInvoked(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).ItemInvoked)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemInvoked(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveItemInvoked)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
}
impl ::core::clone::Clone for WebUICommandBarIconButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarIconButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarIconButton {}
impl ::core::fmt::Debug for WebUICommandBarIconButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarIconButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarIconButton {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarIconButton;{8f1bc93a-3a7c-4842-a0cf-aff6ea308586})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButton_Vtbl;
}
unsafe impl ::windows::core::Interface for WebUICommandBarIconButton {
    const IID: ::windows::core::GUID = <IWebUICommandBarIconButton as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarIconButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarIconButton";
}
::windows::core::interface_hierarchy!(WebUICommandBarIconButton, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<WebUICommandBarIconButton> for IWebUICommandBarElement {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandBarIconButton) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarIconButton> for IWebUICommandBarElement {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarIconButton) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarIconButton> for ::windows::core::InParam<IWebUICommandBarElement> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarIconButton) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarIconButton {}
unsafe impl ::core::marker::Sync for WebUICommandBarIconButton {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarItemInvokedEventArgs(::windows::core::IUnknown);
impl WebUICommandBarItemInvokedEventArgs {
    pub fn IsPrimaryCommand(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).IsPrimaryCommand)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebUICommandBarItemInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarItemInvokedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarItemInvokedEventArgs {}
impl ::core::fmt::Debug for WebUICommandBarItemInvokedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarItemInvokedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarItemInvokedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs;{304edbdd-e741-41ef-bdc4-a45cea2a4f70})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebUICommandBarItemInvokedEventArgs {
    const IID: ::windows::core::GUID = <IWebUICommandBarItemInvokedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarItemInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs";
}
::windows::core::interface_hierarchy!(WebUICommandBarItemInvokedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebUICommandBarItemInvokedEventArgs {}
unsafe impl ::core::marker::Sync for WebUICommandBarItemInvokedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarSizeChangedEventArgs(::windows::core::IUnknown);
impl WebUICommandBarSizeChangedEventArgs {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Size)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for WebUICommandBarSizeChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarSizeChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarSizeChangedEventArgs {}
impl ::core::fmt::Debug for WebUICommandBarSizeChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarSizeChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarSizeChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs;{fbf1e2f6-3029-4719-8378-92f82b87af1e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for WebUICommandBarSizeChangedEventArgs {
    const IID: ::windows::core::GUID = <IWebUICommandBarSizeChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs";
}
::windows::core::interface_hierarchy!(WebUICommandBarSizeChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebUICommandBarSizeChangedEventArgs {}
unsafe impl ::core::marker::Sync for WebUICommandBarSizeChangedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarSymbolIcon(::windows::core::IUnknown);
impl WebUICommandBarSymbolIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUICommandBarSymbolIcon, ::windows::core::IGenericFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Symbol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Symbol)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn SetSymbol(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).SetSymbol)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(symbol: &::windows::core::HSTRING) -> ::windows::core::Result<WebUICommandBarSymbolIcon> {
        Self::IWebUICommandBarSymbolIconFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Create)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(symbol), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarSymbolIconFactory<R, F: FnOnce(&IWebUICommandBarSymbolIconFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUICommandBarSymbolIcon, IWebUICommandBarSymbolIconFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WebUICommandBarSymbolIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUICommandBarSymbolIcon {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUICommandBarSymbolIcon {}
impl ::core::fmt::Debug for WebUICommandBarSymbolIcon {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarSymbolIcon").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarSymbolIcon {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon;{d4935477-fd26-46ed-8658-1a3f4400e7b3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for WebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIcon_Vtbl;
}
unsafe impl ::windows::core::Interface for WebUICommandBarSymbolIcon {
    const IID: ::windows::core::GUID = <IWebUICommandBarSymbolIcon as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarSymbolIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon";
}
::windows::core::interface_hierarchy!(WebUICommandBarSymbolIcon, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::core::convert::TryFrom<WebUICommandBarSymbolIcon> for IWebUICommandBarIcon {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandBarSymbolIcon) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarSymbolIcon> for IWebUICommandBarIcon {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarSymbolIcon) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl ::core::convert::TryFrom<&WebUICommandBarSymbolIcon> for ::windows::core::InParam<IWebUICommandBarIcon> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandBarSymbolIcon) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarSymbolIcon {}
unsafe impl ::core::marker::Sync for WebUICommandBarSymbolIcon {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WebUICommandBarClosedDisplayMode(pub i32);
impl WebUICommandBarClosedDisplayMode {
    pub const Default: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
    pub const Compact: Self = Self(2i32);
}
impl ::core::marker::Copy for WebUICommandBarClosedDisplayMode {}
impl ::core::clone::Clone for WebUICommandBarClosedDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WebUICommandBarClosedDisplayMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WebUICommandBarClosedDisplayMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for WebUICommandBarClosedDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarClosedDisplayMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUICommandBarClosedDisplayMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.Core.WebUICommandBarClosedDisplayMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct MenuClosedEventHandler(pub ::windows::core::IUnknown);
impl MenuClosedEventHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MenuClosedEventHandlerBox::<F> { vtable: &MenuClosedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct MenuClosedEventHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MenuClosedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MenuClosedEventHandlerBox<F> {
    const VTABLE: MenuClosedEventHandler_Vtbl = MenuClosedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MenuClosedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for MenuClosedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MenuClosedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MenuClosedEventHandler {}
impl ::core::fmt::Debug for MenuClosedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MenuClosedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for MenuClosedEventHandler {
    type Vtable = MenuClosedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for MenuClosedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x435387c8_4dd0_4c52_9489_d390ce7721d2);
}
unsafe impl ::windows::core::RuntimeType for MenuClosedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{435387c8-4dd0-4c52-9489-d390ce7721d2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct MenuClosedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct MenuOpenedEventHandler(pub ::windows::core::IUnknown);
impl MenuOpenedEventHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MenuOpenedEventHandlerBox::<F> { vtable: &MenuOpenedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct MenuOpenedEventHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MenuOpenedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MenuOpenedEventHandlerBox<F> {
    const VTABLE: MenuOpenedEventHandler_Vtbl = MenuOpenedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MenuOpenedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
    }
}
impl ::core::clone::Clone for MenuOpenedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MenuOpenedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MenuOpenedEventHandler {}
impl ::core::fmt::Debug for MenuOpenedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MenuOpenedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for MenuOpenedEventHandler {
    type Vtable = MenuOpenedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for MenuOpenedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18dc0ad3_678f_4c19_8963_cc1c49a5ef9e);
}
unsafe impl ::windows::core::RuntimeType for MenuOpenedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{18dc0ad3-678f-4c19-8963-cc1c49a5ef9e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct MenuOpenedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct SizeChangedEventHandler(pub ::windows::core::IUnknown);
impl SizeChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SizeChangedEventHandlerBox::<F> { vtable: &SizeChangedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, eventargs: &WebUICommandBarSizeChangedEventArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(eventargs)).ok() }
    }
}
#[repr(C)]
struct SizeChangedEventHandlerBox<F: FnMut(&::core::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SizeChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SizeChangedEventHandlerBox<F> {
    const VTABLE: SizeChangedEventHandler_Vtbl = SizeChangedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<SizeChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
            let _ = ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&eventargs)).into()
    }
}
impl ::core::clone::Clone for SizeChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SizeChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SizeChangedEventHandler {}
impl ::core::fmt::Debug for SizeChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SizeChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for SizeChangedEventHandler {
    type Vtable = SizeChangedEventHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for SizeChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd49cfe3c_dd2e_4c28_b627_303a7f911af5);
}
unsafe impl ::windows::core::RuntimeType for SizeChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d49cfe3c-dd2e-4c28-b627-303a7f911af5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct SizeChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
