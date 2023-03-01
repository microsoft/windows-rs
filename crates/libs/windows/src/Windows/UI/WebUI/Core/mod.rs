#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBar {
    type Vtable = IWebUICommandBar_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBar {
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
unsafe impl ::windows::core::Interface for IWebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIcon_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarBitmapIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarBitmapIcon {
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
unsafe impl ::windows::core::Interface for IWebUICommandBarBitmapIconFactory {
    type Vtable = IWebUICommandBarBitmapIconFactory_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarBitmapIconFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarBitmapIconFactory {
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
unsafe impl ::windows::core::Interface for IWebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButton_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarConfirmationButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarConfirmationButton {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e7824a_e3d5_4eb6_b2ff_8f018a172105);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarConfirmationButton_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
::windows::imp::interface_hierarchy!(IWebUICommandBarElement, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IWebUICommandBarElement {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{c9069ec2-284a-4633-8aad-637a27e282c3}");
}
unsafe impl ::windows::core::Interface for IWebUICommandBarElement {
    type Vtable = IWebUICommandBarElement_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarElement {
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
::windows::imp::interface_hierarchy!(IWebUICommandBarIcon, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IWebUICommandBarIcon {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d587655d-2014-42be-969a-7d14ca6c8a49}");
}
unsafe impl ::windows::core::Interface for IWebUICommandBarIcon {
    type Vtable = IWebUICommandBarIcon_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarIcon {
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
unsafe impl ::windows::core::Interface for IWebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButton_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarIconButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarIconButton {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1bc93a_3a7c_4842_a0cf_aff6ea308586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIconButton_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
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
unsafe impl ::windows::core::Interface for IWebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarItemInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarItemInvokedEventArgs {
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
unsafe impl ::windows::core::Interface for IWebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarSizeChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarSizeChangedEventArgs {
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
unsafe impl ::windows::core::Interface for IWebUICommandBarStatics {
    type Vtable = IWebUICommandBarStatics_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarStatics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarStatics {
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
unsafe impl ::windows::core::Interface for IWebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIcon_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarSymbolIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarSymbolIcon {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4935477_fd26_46ed_8658_1a3f4400e7b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIcon_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Symbol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSymbol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIconFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarSymbolIconFactory {
    type Vtable = IWebUICommandBarSymbolIconFactory_Vtbl;
}
impl ::core::clone::Clone for IWebUICommandBarSymbolIconFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IWebUICommandBarSymbolIconFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51be1a1f_3730_429e_b622_14e2b7bf6a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIconFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbol: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBar(::windows::core::IUnknown);
impl WebUICommandBar {
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Visible)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVisible)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<f64>();
            (::windows::core::Interface::vtable(this).Opacity)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOpacity)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Color>();
            (::windows::core::Interface::vtable(this).ForegroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetForegroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForegroundColor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Color>();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetBackgroundColor(&self, value: super::super::Color) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundColor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ClosedDisplayMode(&self) -> ::windows::core::Result<WebUICommandBarClosedDisplayMode> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<WebUICommandBarClosedDisplayMode>();
            (::windows::core::Interface::vtable(this).ClosedDisplayMode)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetClosedDisplayMode(&self, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClosedDisplayMode)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsOpen)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsOpen)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrimaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>();
            (::windows::core::Interface::vtable(this).PrimaryCommands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SecondaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>();
            (::windows::core::Interface::vtable(this).SecondaryCommands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MenuOpened(&self, handler: &MenuOpenedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MenuOpened)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMenuOpened(&self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMenuOpened)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MenuClosed(&self, handler: &MenuClosedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).MenuClosed)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMenuClosed(&self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMenuClosed)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeChanged(&self, handler: &SizeChangedEventHandler) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).SizeChanged)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSizeChanged(&self, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSizeChanged)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn GetForCurrentView() -> ::windows::core::Result<WebUICommandBar> {
        Self::IWebUICommandBarStatics(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebUICommandBar>();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarStatics<R, F: FnOnce(&IWebUICommandBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebUICommandBar, IWebUICommandBarStatics> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for WebUICommandBar {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBar;{a4fc0016-dbe5-41ad-8d7b-14698bd6911d})");
}
impl ::core::clone::Clone for WebUICommandBar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebUICommandBar {
    type Vtable = IWebUICommandBar_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebUICommandBar {
    const IID: ::windows::core::GUID = <IWebUICommandBar as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBar {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBar";
}
::windows::imp::interface_hierarchy!(WebUICommandBar, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebUICommandBar {}
unsafe impl ::core::marker::Sync for WebUICommandBar {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarBitmapIcon(::windows::core::IUnknown);
impl WebUICommandBarBitmapIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebUICommandBarBitmapIcon, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Uri>();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri(&self, value: &super::super::super::Foundation::Uri) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create(uri: &super::super::super::Foundation::Uri) -> ::windows::core::Result<WebUICommandBarBitmapIcon> {
        Self::IWebUICommandBarBitmapIconFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebUICommandBarBitmapIcon>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(uri), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarBitmapIconFactory<R, F: FnOnce(&IWebUICommandBarBitmapIconFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebUICommandBarBitmapIcon, IWebUICommandBarBitmapIconFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for WebUICommandBarBitmapIcon {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon;{858f4f45-08d8-4a46-81ec-00015b0b1c6c})");
}
impl ::core::clone::Clone for WebUICommandBarBitmapIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIcon_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebUICommandBarBitmapIcon {
    const IID: ::windows::core::GUID = <IWebUICommandBarBitmapIcon as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarBitmapIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon";
}
::windows::imp::interface_hierarchy!(WebUICommandBarBitmapIcon, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IWebUICommandBarIcon> for WebUICommandBarBitmapIcon {}
unsafe impl ::core::marker::Send for WebUICommandBarBitmapIcon {}
unsafe impl ::core::marker::Sync for WebUICommandBarBitmapIcon {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarConfirmationButton(::windows::core::IUnknown);
impl WebUICommandBarConfirmationButton {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebUICommandBarConfirmationButton, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Text)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemInvoked(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ItemInvoked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemInvoked(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveItemInvoked)(::windows::core::Interface::as_raw(this), token).ok() }
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
impl ::windows::core::RuntimeType for WebUICommandBarConfirmationButton {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton;{86e7824a-e3d5-4eb6-b2ff-8f018a172105})");
}
impl ::core::clone::Clone for WebUICommandBarConfirmationButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButton_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebUICommandBarConfirmationButton {
    const IID: ::windows::core::GUID = <IWebUICommandBarConfirmationButton as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarConfirmationButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton";
}
::windows::imp::interface_hierarchy!(WebUICommandBarConfirmationButton, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IWebUICommandBarElement> for WebUICommandBarConfirmationButton {}
unsafe impl ::core::marker::Send for WebUICommandBarConfirmationButton {}
unsafe impl ::core::marker::Sync for WebUICommandBarConfirmationButton {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarIconButton(::windows::core::IUnknown);
impl WebUICommandBarIconButton {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebUICommandBarIconButton, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).Enabled)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Label)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn IsToggleButton(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsToggleButton)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsToggleButton(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsToggleButton)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn IsChecked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsChecked)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsChecked)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Icon(&self) -> ::windows::core::Result<IWebUICommandBarIcon> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<IWebUICommandBarIcon>();
            (::windows::core::Interface::vtable(this).Icon)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIcon<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IWebUICommandBarIcon>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIcon)(::windows::core::Interface::as_raw(this), value.try_into_param()?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemInvoked(&self, handler: &super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::EventRegistrationToken>();
            (::windows::core::Interface::vtable(this).ItemInvoked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(handler), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemInvoked(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveItemInvoked)(::windows::core::Interface::as_raw(this), token).ok() }
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
impl ::windows::core::RuntimeType for WebUICommandBarIconButton {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarIconButton;{8f1bc93a-3a7c-4842-a0cf-aff6ea308586})");
}
impl ::core::clone::Clone for WebUICommandBarIconButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButton_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebUICommandBarIconButton {
    const IID: ::windows::core::GUID = <IWebUICommandBarIconButton as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarIconButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarIconButton";
}
::windows::imp::interface_hierarchy!(WebUICommandBarIconButton, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IWebUICommandBarElement> for WebUICommandBarIconButton {}
unsafe impl ::core::marker::Send for WebUICommandBarIconButton {}
unsafe impl ::core::marker::Sync for WebUICommandBarIconButton {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarItemInvokedEventArgs(::windows::core::IUnknown);
impl WebUICommandBarItemInvokedEventArgs {
    pub fn IsPrimaryCommand(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<bool>();
            (::windows::core::Interface::vtable(this).IsPrimaryCommand)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for WebUICommandBarItemInvokedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs;{304edbdd-e741-41ef-bdc4-a45cea2a4f70})");
}
impl ::core::clone::Clone for WebUICommandBarItemInvokedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebUICommandBarItemInvokedEventArgs {
    const IID: ::windows::core::GUID = <IWebUICommandBarItemInvokedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarItemInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs";
}
::windows::imp::interface_hierarchy!(WebUICommandBarItemInvokedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
            let mut result__ = ::windows::core::zeroed::<super::super::super::Foundation::Size>();
            (::windows::core::Interface::vtable(this).Size)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for WebUICommandBarSizeChangedEventArgs {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs;{fbf1e2f6-3029-4719-8378-92f82b87af1e})");
}
impl ::core::clone::Clone for WebUICommandBarSizeChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgs_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebUICommandBarSizeChangedEventArgs {
    const IID: ::windows::core::GUID = <IWebUICommandBarSizeChangedEventArgs as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs";
}
::windows::imp::interface_hierarchy!(WebUICommandBarSizeChangedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for WebUICommandBarSizeChangedEventArgs {}
unsafe impl ::core::marker::Sync for WebUICommandBarSizeChangedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarSymbolIcon(::windows::core::IUnknown);
impl WebUICommandBarSymbolIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebUICommandBarSymbolIcon, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    pub fn Symbol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Symbol)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetSymbol(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSymbol)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Create(symbol: &::windows::core::HSTRING) -> ::windows::core::Result<WebUICommandBarSymbolIcon> {
        Self::IWebUICommandBarSymbolIconFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<WebUICommandBarSymbolIcon>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(symbol), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarSymbolIconFactory<R, F: FnOnce(&IWebUICommandBarSymbolIconFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<WebUICommandBarSymbolIcon, IWebUICommandBarSymbolIconFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for WebUICommandBarSymbolIcon {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon;{d4935477-fd26-46ed-8658-1a3f4400e7b3})");
}
impl ::core::clone::Clone for WebUICommandBarSymbolIcon {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for WebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIcon_Vtbl;
}
unsafe impl ::windows::core::ComInterface for WebUICommandBarSymbolIcon {
    const IID: ::windows::core::GUID = <IWebUICommandBarSymbolIcon as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarSymbolIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon";
}
::windows::imp::interface_hierarchy!(WebUICommandBarSymbolIcon, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IWebUICommandBarIcon> for WebUICommandBarSymbolIcon {}
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
impl ::windows::core::TypeKind for WebUICommandBarClosedDisplayMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WebUICommandBarClosedDisplayMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandBarClosedDisplayMode").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for WebUICommandBarClosedDisplayMode {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.Core.WebUICommandBarClosedDisplayMode;i4)");
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct MenuClosedEventHandler(pub ::windows::core::IUnknown);
impl MenuClosedEventHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MenuClosedEventHandlerBox::<F> { vtable: &MenuClosedEventHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct MenuClosedEventHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MenuClosedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MenuClosedEventHandlerBox<F> {
    const VTABLE: MenuClosedEventHandler_Vtbl = MenuClosedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MenuClosedEventHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
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
unsafe impl ::windows::core::Interface for MenuClosedEventHandler {
    type Vtable = MenuClosedEventHandler_Vtbl;
}
impl ::core::clone::Clone for MenuClosedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for MenuClosedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x435387c8_4dd0_4c52_9489_d390ce7721d2);
}
impl ::windows::core::RuntimeType for MenuClosedEventHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{435387c8-4dd0-4c52-9489-d390ce7721d2}");
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
        let com = MenuOpenedEventHandlerBox::<F> { vtable: &MenuOpenedEventHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[repr(C)]
struct MenuOpenedEventHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MenuOpenedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MenuOpenedEventHandlerBox<F> {
    const VTABLE: MenuOpenedEventHandler_Vtbl = MenuOpenedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<MenuOpenedEventHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)().into()
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
unsafe impl ::windows::core::Interface for MenuOpenedEventHandler {
    type Vtable = MenuOpenedEventHandler_Vtbl;
}
impl ::core::clone::Clone for MenuOpenedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for MenuOpenedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18dc0ad3_678f_4c19_8963_cc1c49a5ef9e);
}
impl ::windows::core::RuntimeType for MenuOpenedEventHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{18dc0ad3-678f-4c19-8963-cc1c49a5ef9e}");
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
    pub fn new<F: FnMut(::core::option::Option<&WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SizeChangedEventHandlerBox::<F> { vtable: &SizeChangedEventHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, eventargs: &WebUICommandBarSizeChangedEventArgs) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(eventargs)).ok() }
    }
}
#[repr(C)]
struct SizeChangedEventHandlerBox<F: FnMut(::core::option::Option<&WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SizeChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SizeChangedEventHandlerBox<F> {
    const VTABLE: SizeChangedEventHandler_Vtbl = SizeChangedEventHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<SizeChangedEventHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&eventargs)).into()
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
unsafe impl ::windows::core::Interface for SizeChangedEventHandler {
    type Vtable = SizeChangedEventHandler_Vtbl;
}
impl ::core::clone::Clone for SizeChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for SizeChangedEventHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd49cfe3c_dd2e_4c28_b627_303a7f911af5);
}
impl ::windows::core::RuntimeType for SizeChangedEventHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{d49cfe3c-dd2e-4c28-b627-303a7f911af5}");
}
#[repr(C)]
#[doc(hidden)]
pub struct SizeChangedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
