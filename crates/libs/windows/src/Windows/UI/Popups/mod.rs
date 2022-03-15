#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageDialog(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageDialog {
    type Vtable = IMessageDialog_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f59b01_5325_43ab_9ab3_bdae440e4121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialog_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CancelCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCancelCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    pub Options: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut MessageDialogOptions) -> ::windows::core::HRESULT,
    pub SetOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: MessageDialogOptions) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageDialogFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageDialogFactory {
    type Vtable = IMessageDialogFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d161777_a66f_4ea5_bb87_793ffa4941f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialogFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWithTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupMenu(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopupMenu {
    type Vtable = IPopupMenu_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e9bc6dc_880d_47fc_a0a1_72b639e62559);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupMenu_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsyncWithRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsyncWithRect: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsyncWithRectAndPlacement: usize,
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct IUICommand(::windows::core::IUnknown);
impl IUICommand {
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetInvoked<'a, Param0: ::windows::core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvoked)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IUICommand> for ::windows::core::IUnknown {
    fn from(value: IUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICommand> for ::windows::core::IUnknown {
    fn from(value: &IUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IUICommand> for ::windows::core::IInspectable {
    fn from(value: IUICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IUICommand> for ::windows::core::IInspectable {
    fn from(value: &IUICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IUICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IUICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUICommand {}
impl ::core::fmt::Debug for IUICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IUICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IUICommand {
    type Vtable = IUICommand_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ff93a75_4145_47ff_ac7f_dff1c1fa5b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommand_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUICommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUICommandFactory {
    type Vtable = IUICommandFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa21a8189_26b0_4676_ae94_54041bc125e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommandFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWithHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, action: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWithHandlerAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, action: ::windows::core::RawPtr, commandid: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct MessageDialog(::windows::core::IUnknown);
impl MessageDialog {
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Title)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Commands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DefaultCommandIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultCommandIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn CancelCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CancelCommandIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetCancelCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancelCommandIndex)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Content)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContent)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Options(&self) -> ::windows::core::Result<MessageDialogOptions> {
        let this = self;
        unsafe {
            let mut result__: MessageDialogOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Options)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MessageDialogOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetOptions(&self, value: MessageDialogOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOptions)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(content: Param0) -> ::windows::core::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<MessageDialog>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn CreateWithTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(content: Param0, title: Param1) -> ::windows::core::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithTitle)(::core::mem::transmute_copy(this), content.into_param().abi(), title.into_param().abi(), &mut result__).from_abi::<MessageDialog>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMessageDialogFactory<R, F: FnOnce(&IMessageDialogFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MessageDialog, IMessageDialogFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for MessageDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for MessageDialog {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MessageDialog {}
impl ::core::fmt::Debug for MessageDialog {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageDialog").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for MessageDialog {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.MessageDialog;{33f59b01-5325-43ab-9ab3-bdae440e4121})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for MessageDialog {
    type Vtable = IMessageDialog_Vtbl;
    const IID: ::windows::core::GUID = <IMessageDialog as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for MessageDialog {
    const NAME: &'static str = "Windows.UI.Popups.MessageDialog";
}
impl ::core::convert::From<MessageDialog> for ::windows::core::IUnknown {
    fn from(value: MessageDialog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageDialog> for ::windows::core::IUnknown {
    fn from(value: &MessageDialog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MessageDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MessageDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<MessageDialog> for ::windows::core::IInspectable {
    fn from(value: MessageDialog) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&MessageDialog> for ::windows::core::IInspectable {
    fn from(value: &MessageDialog) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MessageDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MessageDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MessageDialogOptions(pub u32);
impl MessageDialogOptions {
    pub const None: Self = Self(0u32);
    pub const AcceptUserInputAfterDelay: Self = Self(1u32);
}
impl ::core::marker::Copy for MessageDialogOptions {}
impl ::core::clone::Clone for MessageDialogOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MessageDialogOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MessageDialogOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for MessageDialogOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageDialogOptions").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MessageDialogOptions {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MessageDialogOptions {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MessageDialogOptions {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MessageDialogOptions {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MessageDialogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for MessageDialogOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.MessageDialogOptions;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Placement(pub i32);
impl Placement {
    pub const Default: Self = Self(0i32);
    pub const Above: Self = Self(1i32);
    pub const Below: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
}
impl ::core::marker::Copy for Placement {}
impl ::core::clone::Clone for Placement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Placement {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Placement {
    type Abi = Self;
}
impl ::core::fmt::Debug for Placement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Placement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Placement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.Placement;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct PopupMenu(::windows::core::IUnknown);
impl PopupMenu {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopupMenu, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Popups\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Commands)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, invocationpoint: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAsync)(::core::mem::transmute_copy(this), invocationpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsyncWithRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAsyncWithRect)(::core::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsyncWithRectAndPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ShowAsyncWithRectAndPlacement)(::core::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
}
impl ::core::clone::Clone for PopupMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PopupMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PopupMenu {}
impl ::core::fmt::Debug for PopupMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PopupMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PopupMenu {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.PopupMenu;{4e9bc6dc-880d-47fc-a0a1-72b639e62559})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PopupMenu {
    type Vtable = IPopupMenu_Vtbl;
    const IID: ::windows::core::GUID = <IPopupMenu as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PopupMenu {
    const NAME: &'static str = "Windows.UI.Popups.PopupMenu";
}
impl ::core::convert::From<PopupMenu> for ::windows::core::IUnknown {
    fn from(value: PopupMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupMenu> for ::windows::core::IUnknown {
    fn from(value: &PopupMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopupMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PopupMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PopupMenu> for ::windows::core::IInspectable {
    fn from(value: PopupMenu) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PopupMenu> for ::windows::core::IInspectable {
    fn from(value: &PopupMenu) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopupMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PopupMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct UICommand(::windows::core::IUnknown);
impl UICommand {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UICommand, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetInvoked<'a, Param0: ::windows::core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvoked)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(label: Param0) -> ::windows::core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Create)(::core::mem::transmute_copy(this), label.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn CreateWithHandler<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, UICommandInvokedHandler>>(label: Param0, action: Param1) -> ::windows::core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithHandler)(::core::mem::transmute_copy(this), label.into_param().abi(), action.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn CreateWithHandlerAndId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, UICommandInvokedHandler>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(label: Param0, action: Param1, commandid: Param2) -> ::windows::core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithHandlerAndId)(::core::mem::transmute_copy(this), label.into_param().abi(), action.into_param().abi(), commandid.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUICommandFactory<R, F: FnOnce(&IUICommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UICommand, IUICommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for UICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UICommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UICommand {}
impl ::core::fmt::Debug for UICommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UICommand").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommand;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UICommand {
    type Vtable = IUICommand_Vtbl;
    const IID: ::windows::core::GUID = <IUICommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UICommand {
    const NAME: &'static str = "Windows.UI.Popups.UICommand";
}
impl ::core::convert::From<UICommand> for ::windows::core::IUnknown {
    fn from(value: UICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommand> for ::windows::core::IUnknown {
    fn from(value: &UICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UICommand> for ::windows::core::IInspectable {
    fn from(value: UICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommand> for ::windows::core::IInspectable {
    fn from(value: &UICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UICommand> for IUICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: UICommand) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UICommand> for IUICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: &UICommand) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUICommand> for UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, IUICommand> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUICommand> for &UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, IUICommand> {
        ::core::convert::TryInto::<IUICommand>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UICommand {}
unsafe impl ::core::marker::Sync for UICommand {}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct UICommandInvokedHandler(pub ::windows::core::IUnknown);
impl UICommandInvokedHandler {
    pub fn new<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = UICommandInvokedHandlerBox::<F> { vtable: &UICommandInvokedHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IUICommand>>(&self, command: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), command.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct UICommandInvokedHandlerBox<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const UICommandInvokedHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> UICommandInvokedHandlerBox<F> {
    const VTABLE: UICommandInvokedHandler_Vtbl = UICommandInvokedHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<UICommandInvokedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&command)).into()
    }
}
impl ::core::clone::Clone for UICommandInvokedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UICommandInvokedHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UICommandInvokedHandler {}
impl ::core::fmt::Debug for UICommandInvokedHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UICommandInvokedHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for UICommandInvokedHandler {
    type Vtable = UICommandInvokedHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaf77a4f_c27a_4298_9ac6_2922c45e7da6);
}
unsafe impl ::windows::core::RuntimeType for UICommandInvokedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{daf77a4f-c27a-4298-9ac6-2922c45e7da6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct UICommandInvokedHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct UICommandSeparator(::windows::core::IUnknown);
impl UICommandSeparator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UICommandSeparator, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Label)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLabel)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Invoked)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetInvoked<'a, Param0: ::windows::core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvoked)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Id)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Popups\"`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
impl ::core::clone::Clone for UICommandSeparator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for UICommandSeparator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for UICommandSeparator {}
impl ::core::fmt::Debug for UICommandSeparator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UICommandSeparator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UICommandSeparator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommandSeparator;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for UICommandSeparator {
    type Vtable = IUICommand_Vtbl;
    const IID: ::windows::core::GUID = <IUICommand as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for UICommandSeparator {
    const NAME: &'static str = "Windows.UI.Popups.UICommandSeparator";
}
impl ::core::convert::From<UICommandSeparator> for ::windows::core::IUnknown {
    fn from(value: UICommandSeparator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommandSeparator> for ::windows::core::IUnknown {
    fn from(value: &UICommandSeparator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<UICommandSeparator> for ::windows::core::IInspectable {
    fn from(value: UICommandSeparator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommandSeparator> for ::windows::core::IInspectable {
    fn from(value: &UICommandSeparator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<UICommandSeparator> for IUICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: UICommandSeparator) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&UICommandSeparator> for IUICommand {
    type Error = ::windows::core::Error;
    fn try_from(value: &UICommandSeparator) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUICommand> for UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, IUICommand> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUICommand> for &UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, IUICommand> {
        ::core::convert::TryInto::<IUICommand>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for UICommandSeparator {}
unsafe impl ::core::marker::Sync for UICommandSeparator {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
