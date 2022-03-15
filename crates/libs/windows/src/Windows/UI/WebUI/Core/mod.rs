#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBar(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBar {
    type Vtable = IWebUICommandBar_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa4fc0016_dbe5_41ad_8d7b_14698bd6911d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBar_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    pub PrimaryCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    PrimaryCommands: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub SecondaryCommands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    SecondaryCommands: usize,
    #[cfg(feature = "Foundation")]
    pub MenuOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MenuOpened: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMenuOpened: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMenuOpened: usize,
    #[cfg(feature = "Foundation")]
    pub MenuClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    MenuClosed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveMenuClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveMenuClosed: usize,
    #[cfg(feature = "Foundation")]
    pub SizeChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x858f4f45_08d8_4a46_81ec_00015b0b1c6c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIcon_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Uri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Uri: usize,
    #[cfg(feature = "Foundation")]
    pub SetUri: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    SetUri: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIconFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarBitmapIconFactory {
    type Vtable = IWebUICommandBarBitmapIconFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3f7d78a_7673_444a_be62_ac12d31c2231);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarBitmapIconFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Create: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarConfirmationButton(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButton_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e7824a_e3d5_4eb6_b2ff_8f018a172105);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarConfirmationButton_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Text: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
impl ::core::convert::From<IWebUICommandBarElement> for ::windows::core::IUnknown {
    fn from(value: IWebUICommandBarElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUICommandBarElement> for ::windows::core::IUnknown {
    fn from(value: &IWebUICommandBarElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebUICommandBarElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebUICommandBarElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebUICommandBarElement> for ::windows::core::IInspectable {
    fn from(value: IWebUICommandBarElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUICommandBarElement> for ::windows::core::IInspectable {
    fn from(value: &IWebUICommandBarElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebUICommandBarElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebUICommandBarElement {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWebUICommandBarElement {
    type Vtable = IWebUICommandBarElement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc9069ec2_284a_4633_8aad_637a27e282c3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarElement_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct IWebUICommandBarIcon(::windows::core::IUnknown);
impl IWebUICommandBarIcon {}
impl ::core::convert::From<IWebUICommandBarIcon> for ::windows::core::IUnknown {
    fn from(value: IWebUICommandBarIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUICommandBarIcon> for ::windows::core::IUnknown {
    fn from(value: &IWebUICommandBarIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWebUICommandBarIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWebUICommandBarIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWebUICommandBarIcon> for ::windows::core::IInspectable {
    fn from(value: IWebUICommandBarIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUICommandBarIcon> for ::windows::core::IInspectable {
    fn from(value: &IWebUICommandBarIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWebUICommandBarIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWebUICommandBarIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IWebUICommandBarIcon {
    type Vtable = IWebUICommandBarIcon_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd587655d_2014_42be_969a_7d14ca6c8a49);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIcon_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarIconButton(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButton_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f1bc93a_3a7c_4842_a0cf_aff6ea308586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarIconButton_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub IsToggleButton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsToggleButton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsChecked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub Icon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ItemInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x304edbdd_e741_41ef_bdc4_a45cea2a4f70);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarItemInvokedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub IsPrimaryCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSizeChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfbf1e2f6_3029_4719_8378_92f82b87af1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSizeChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
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
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1449cdb9_a506_45be_8f42_b2837e2fe0c9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetForCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIcon(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIcon_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4935477_fd26_46ed_8658_1a3f4400e7b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIcon_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Symbol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetSymbol: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIconFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUICommandBarSymbolIconFactory {
    type Vtable = IWebUICommandBarSymbolIconFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x51be1a1f_3730_429e_b622_14e2b7bf6a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUICommandBarSymbolIconFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, symbol: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct MenuClosedEventHandler(pub ::windows::core::IUnknown);
impl MenuClosedEventHandler {
    pub fn new<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = MenuClosedEventHandlerBox::<F> { vtable: &MenuClosedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[repr(C)]
struct MenuClosedEventHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MenuClosedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MenuClosedEventHandlerBox<F> {
    const VTABLE: MenuClosedEventHandler_Vtbl = MenuClosedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MenuClosedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
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
unsafe impl ::windows::core::Interface for MenuClosedEventHandler {
    type Vtable = MenuClosedEventHandler_Vtbl;
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
    pub base: ::windows::core::IUnknownVtbl,
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
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this)).ok() }
    }
}
#[repr(C)]
struct MenuOpenedEventHandlerBox<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const MenuOpenedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut() -> ::windows::core::Result<()> + ::core::marker::Send + 'static> MenuOpenedEventHandlerBox<F> {
    const VTABLE: MenuOpenedEventHandler_Vtbl = MenuOpenedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<MenuOpenedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
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
unsafe impl ::windows::core::Interface for MenuOpenedEventHandler {
    type Vtable = MenuOpenedEventHandler_Vtbl;
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
    pub base: ::windows::core::IUnknownVtbl,
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
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, WebUICommandBarSizeChangedEventArgs>>(&self, eventargs: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), eventargs.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct SizeChangedEventHandlerBox<F: FnMut(&::core::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SizeChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<WebUICommandBarSizeChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SizeChangedEventHandlerBox<F> {
    const VTABLE: SizeChangedEventHandler_Vtbl = SizeChangedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<SizeChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, eventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
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
unsafe impl ::windows::core::Interface for SizeChangedEventHandler {
    type Vtable = SizeChangedEventHandler_Vtbl;
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
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventargs: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBar(::windows::core::IUnknown);
impl WebUICommandBar {
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Visible(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Visible)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetVisible(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVisible)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Opacity(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Opacity)(::core::mem::transmute_copy(this), &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetOpacity(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOpacity)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn ForegroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ForegroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetForegroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetForegroundColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn BackgroundColor(&self) -> ::windows::core::Result<super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::Color = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundColor)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetBackgroundColor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Color>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBackgroundColor)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn ClosedDisplayMode(&self) -> ::windows::core::Result<WebUICommandBarClosedDisplayMode> {
        let this = self;
        unsafe {
            let mut result__: WebUICommandBarClosedDisplayMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ClosedDisplayMode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebUICommandBarClosedDisplayMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetClosedDisplayMode(&self, value: WebUICommandBarClosedDisplayMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetClosedDisplayMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn IsOpen(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsOpen)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetIsOpen(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsOpen)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn PrimaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrimaryCommands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SecondaryCommands(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SecondaryCommands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableVector<IWebUICommandBarElement>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MenuOpened<'a, Param0: ::windows::core::IntoParam<'a, MenuOpenedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MenuOpened)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMenuOpened<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMenuOpened)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MenuClosed<'a, Param0: ::windows::core::IntoParam<'a, MenuClosedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MenuClosed)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveMenuClosed<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveMenuClosed)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SizeChanged<'a, Param0: ::windows::core::IntoParam<'a, SizeChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SizeChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSizeChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveSizeChanged)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn GetForCurrentView() -> ::windows::core::Result<WebUICommandBar> {
        Self::IWebUICommandBarStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetForCurrentView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<WebUICommandBar>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarStatics<R, F: FnOnce(&IWebUICommandBarStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBar, IWebUICommandBarStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for WebUICommandBar {
    type Vtable = IWebUICommandBar_Vtbl;
    const IID: ::windows::core::GUID = <IWebUICommandBar as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBar {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBar";
}
impl ::core::convert::From<WebUICommandBar> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBar> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebUICommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBar> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBar) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBar> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBar) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebUICommandBar {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebUICommandBar {}
unsafe impl ::core::marker::Sync for WebUICommandBar {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarBitmapIcon(::windows::core::IUnknown);
impl WebUICommandBarBitmapIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarBitmapIcon, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn SetUri<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetUri)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Uri>>(uri: Param0) -> ::windows::core::Result<WebUICommandBarBitmapIcon> {
        Self::IWebUICommandBarBitmapIconFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), uri.into_param().abi(), &mut result__).from_abi::<WebUICommandBarBitmapIcon>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarBitmapIconFactory<R, F: FnOnce(&IWebUICommandBarBitmapIconFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarBitmapIcon, IWebUICommandBarBitmapIconFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for WebUICommandBarBitmapIcon {
    type Vtable = IWebUICommandBarBitmapIcon_Vtbl;
    const IID: ::windows::core::GUID = <IWebUICommandBarBitmapIcon as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarBitmapIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarBitmapIcon";
}
impl ::core::convert::From<WebUICommandBarBitmapIcon> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarBitmapIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarBitmapIcon> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarBitmapIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarBitmapIcon> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarBitmapIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarBitmapIcon> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarBitmapIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarIcon> for WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarIcon> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarIcon> for &WebUICommandBarBitmapIcon {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarIcon> {
        ::core::convert::TryInto::<IWebUICommandBarIcon>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarBitmapIcon {}
unsafe impl ::core::marker::Sync for WebUICommandBarBitmapIcon {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
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
pub struct WebUICommandBarConfirmationButton(::windows::core::IUnknown);
impl WebUICommandBarConfirmationButton {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarConfirmationButton, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Text)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetText)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebUICommandBarConfirmationButton, WebUICommandBarItemInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemInvoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveItemInvoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
unsafe impl ::windows::core::Interface for WebUICommandBarConfirmationButton {
    type Vtable = IWebUICommandBarConfirmationButton_Vtbl;
    const IID: ::windows::core::GUID = <IWebUICommandBarConfirmationButton as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarConfirmationButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarConfirmationButton";
}
impl ::core::convert::From<WebUICommandBarConfirmationButton> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarConfirmationButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarConfirmationButton> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarConfirmationButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarConfirmationButton> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarConfirmationButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarConfirmationButton> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarConfirmationButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarElement> for WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarElement> for &WebUICommandBarConfirmationButton {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarElement> {
        ::core::convert::TryInto::<IWebUICommandBarElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
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
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarIconButton, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Enabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Enabled)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEnabled)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn IsToggleButton(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsToggleButton)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetIsToggleButton(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsToggleButton)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn IsChecked(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsChecked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetIsChecked(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsChecked)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Icon(&self) -> ::windows::core::Result<IWebUICommandBarIcon> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Icon)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IWebUICommandBarIcon>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetIcon<'a, Param0: ::windows::core::IntoParam<'a, IWebUICommandBarIcon>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIcon)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ItemInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::TypedEventHandler<WebUICommandBarIconButton, WebUICommandBarItemInvokedEventArgs>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemInvoked)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveItemInvoked<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveItemInvoked)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
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
unsafe impl ::windows::core::Interface for WebUICommandBarIconButton {
    type Vtable = IWebUICommandBarIconButton_Vtbl;
    const IID: ::windows::core::GUID = <IWebUICommandBarIconButton as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarIconButton {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarIconButton";
}
impl ::core::convert::From<WebUICommandBarIconButton> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarIconButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarIconButton> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarIconButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarIconButton> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarIconButton) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarIconButton> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarIconButton) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarElement> for WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarElement> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarElement> for &WebUICommandBarIconButton {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarElement> {
        ::core::convert::TryInto::<IWebUICommandBarElement>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarIconButton {}
unsafe impl ::core::marker::Sync for WebUICommandBarIconButton {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarItemInvokedEventArgs(::windows::core::IUnknown);
impl WebUICommandBarItemInvokedEventArgs {
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn IsPrimaryCommand(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsPrimaryCommand)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
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
unsafe impl ::windows::core::Interface for WebUICommandBarItemInvokedEventArgs {
    type Vtable = IWebUICommandBarItemInvokedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebUICommandBarItemInvokedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarItemInvokedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarItemInvokedEventArgs";
}
impl ::core::convert::From<WebUICommandBarItemInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarItemInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarItemInvokedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarItemInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarItemInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebUICommandBarItemInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarItemInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarItemInvokedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarItemInvokedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarItemInvokedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarItemInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebUICommandBarItemInvokedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarItemInvokedEventArgs {}
unsafe impl ::core::marker::Sync for WebUICommandBarItemInvokedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarSizeChangedEventArgs(::windows::core::IUnknown);
impl WebUICommandBarSizeChangedEventArgs {
    #[doc = "*Required features: `\"UI_WebUI_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Size(&self) -> ::windows::core::Result<super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::Size = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Size>(result__)
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
unsafe impl ::windows::core::Interface for WebUICommandBarSizeChangedEventArgs {
    type Vtable = IWebUICommandBarSizeChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebUICommandBarSizeChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarSizeChangedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSizeChangedEventArgs";
}
impl ::core::convert::From<WebUICommandBarSizeChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarSizeChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarSizeChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarSizeChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebUICommandBarSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarSizeChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarSizeChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarSizeChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarSizeChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebUICommandBarSizeChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarSizeChangedEventArgs {}
unsafe impl ::core::marker::Sync for WebUICommandBarSizeChangedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
#[repr(transparent)]
pub struct WebUICommandBarSymbolIcon(::windows::core::IUnknown);
impl WebUICommandBarSymbolIcon {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarSymbolIcon, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Symbol(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Symbol)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn SetSymbol<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSymbol)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_WebUI_Core\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(symbol: Param0) -> ::windows::core::Result<WebUICommandBarSymbolIcon> {
        Self::IWebUICommandBarSymbolIconFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), symbol.into_param().abi(), &mut result__).from_abi::<WebUICommandBarSymbolIcon>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUICommandBarSymbolIconFactory<R, F: FnOnce(&IWebUICommandBarSymbolIconFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<WebUICommandBarSymbolIcon, IWebUICommandBarSymbolIconFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::Interface for WebUICommandBarSymbolIcon {
    type Vtable = IWebUICommandBarSymbolIcon_Vtbl;
    const IID: ::windows::core::GUID = <IWebUICommandBarSymbolIcon as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUICommandBarSymbolIcon {
    const NAME: &'static str = "Windows.UI.WebUI.Core.WebUICommandBarSymbolIcon";
}
impl ::core::convert::From<WebUICommandBarSymbolIcon> for ::windows::core::IUnknown {
    fn from(value: WebUICommandBarSymbolIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarSymbolIcon> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandBarSymbolIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<WebUICommandBarSymbolIcon> for ::windows::core::IInspectable {
    fn from(value: WebUICommandBarSymbolIcon) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUICommandBarSymbolIcon> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandBarSymbolIcon) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarIcon> for WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarIcon> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IWebUICommandBarIcon> for &WebUICommandBarSymbolIcon {
    fn into_param(self) -> ::windows::core::Param<'a, IWebUICommandBarIcon> {
        ::core::convert::TryInto::<IWebUICommandBarIcon>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for WebUICommandBarSymbolIcon {}
unsafe impl ::core::marker::Sync for WebUICommandBarSymbolIcon {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
