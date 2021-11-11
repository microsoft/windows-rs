#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IMessageDialog(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMessageDialog {
    type Vtable = IMessageDialog_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f59b01_5325_43ab_9ab3_bdae440e4121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialog_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut MessageDialogOptions) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: MessageDialogOptions) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMessageDialogFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IMessageDialogFactory {
    type Vtable = IMessageDialogFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d161777_a66f_4ea5_bb87_793ffa4941f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialogFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, content: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, title: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopupMenu(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IPopupMenu {
    type Vtable = IPopupMenu_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e9bc6dc_880d_47fc_a0a1_72b639e62559);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupMenu_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: Placement, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `UI_Popups`*"]
pub struct IUICommand(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUICommand {
    type Vtable = IUICommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ff93a75_4145_47ff_ac7f_dff1c1fa5b0f);
}
impl IUICommand {
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetInvoked<'a, Param0: ::windows::core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for IUICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f}");
}
impl ::core::convert::From<IUICommand> for ::windows::core::IUnknown {
    fn from(value: IUICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IUICommand> for ::windows::core::IUnknown {
    fn from(value: &IUICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IUICommand> for ::windows::core::IInspectable {
    fn from(value: IUICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IUICommand> for ::windows::core::IInspectable {
    fn from(value: &IUICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IUICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommand_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUICommandFactory(pub ::windows::core::IInspectable);
unsafe impl ::windows::core::Interface for IUICommandFactory {
    type Vtable = IUICommandFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa21a8189_26b0_4676_ae94_54041bc125e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, action: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, label: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, action: ::windows::core::RawPtr, commandid: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct MessageDialog(pub ::windows::core::IInspectable);
impl MessageDialog {
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Popups`, `Foundation_Collections`*"]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn CancelCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetCancelCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).13)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetContent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).14)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Popups`, `Foundation`*"]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).15)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Options(&self) -> ::windows::core::Result<MessageDialogOptions> {
        let this = self;
        unsafe {
            let mut result__: MessageDialogOptions = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<MessageDialogOptions>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetOptions(&self, value: MessageDialogOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(content: Param0) -> ::windows::core::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<MessageDialog>(result__)
        })
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn CreateWithTitle<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(content: Param0, title: Param1) -> ::windows::core::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), content.into_param().abi(), title.into_param().abi(), &mut result__).from_abi::<MessageDialog>(result__)
        })
    }
    pub fn IMessageDialogFactory<R, F: FnOnce(&IMessageDialogFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<MessageDialog, IMessageDialogFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for MessageDialog {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.MessageDialog;{33f59b01-5325-43ab-9ab3-bdae440e4121})");
}
unsafe impl ::windows::core::Interface for MessageDialog {
    type Vtable = IMessageDialog_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f59b01_5325_43ab_9ab3_bdae440e4121);
}
impl ::windows::core::RuntimeName for MessageDialog {
    const NAME: &'static str = "Windows.UI.Popups.MessageDialog";
}
impl ::core::convert::From<MessageDialog> for ::windows::core::IUnknown {
    fn from(value: MessageDialog) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&MessageDialog> for ::windows::core::IUnknown {
    fn from(value: &MessageDialog) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for MessageDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a MessageDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<MessageDialog> for ::windows::core::IInspectable {
    fn from(value: MessageDialog) -> Self {
        value.0
    }
}
impl ::core::convert::From<&MessageDialog> for ::windows::core::IInspectable {
    fn from(value: &MessageDialog) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for MessageDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a MessageDialog {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `UI_Popups`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct MessageDialogOptions(pub u32);
impl MessageDialogOptions {
    pub const None: MessageDialogOptions = MessageDialogOptions(0u32);
    pub const AcceptUserInputAfterDelay: MessageDialogOptions = MessageDialogOptions(1u32);
}
impl ::core::convert::From<u32> for MessageDialogOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for MessageDialogOptions {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for MessageDialogOptions {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.MessageDialogOptions;u4)");
}
impl ::windows::core::DefaultType for MessageDialogOptions {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for MessageDialogOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for MessageDialogOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for MessageDialogOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for MessageDialogOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for MessageDialogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Popups`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Placement(pub i32);
impl Placement {
    pub const Default: Placement = Placement(0i32);
    pub const Above: Placement = Placement(1i32);
    pub const Below: Placement = Placement(2i32);
    pub const Left: Placement = Placement(3i32);
    pub const Right: Placement = Placement(4i32);
}
impl ::core::convert::From<i32> for Placement {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for Placement {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Placement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.Placement;i4)");
}
impl ::windows::core::DefaultType for Placement {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct PopupMenu(pub ::windows::core::IInspectable);
impl PopupMenu {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PopupMenu, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Popups`, `Foundation_Collections`*"]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<IUICommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Popups`, `Foundation`*"]
    pub fn ShowAsync<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Point>>(&self, invocationpoint: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), invocationpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Popups`, `Foundation`*"]
    pub fn ShowAsyncWithRect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Popups`, `Foundation`*"]
    pub fn ShowAsyncWithRectAndPlacement<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
}
unsafe impl ::windows::core::RuntimeType for PopupMenu {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.PopupMenu;{4e9bc6dc-880d-47fc-a0a1-72b639e62559})");
}
unsafe impl ::windows::core::Interface for PopupMenu {
    type Vtable = IPopupMenu_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e9bc6dc_880d_47fc_a0a1_72b639e62559);
}
impl ::windows::core::RuntimeName for PopupMenu {
    const NAME: &'static str = "Windows.UI.Popups.PopupMenu";
}
impl ::core::convert::From<PopupMenu> for ::windows::core::IUnknown {
    fn from(value: PopupMenu) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&PopupMenu> for ::windows::core::IUnknown {
    fn from(value: &PopupMenu) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PopupMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PopupMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<PopupMenu> for ::windows::core::IInspectable {
    fn from(value: PopupMenu) -> Self {
        value.0
    }
}
impl ::core::convert::From<&PopupMenu> for ::windows::core::IInspectable {
    fn from(value: &PopupMenu) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PopupMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PopupMenu {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UICommand(pub ::windows::core::IInspectable);
impl UICommand {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UICommand, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetInvoked<'a, Param0: ::windows::core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Create<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(label: Param0) -> ::windows::core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), label.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn CreateWithHandler<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, UICommandInvokedHandler>>(label: Param0, action: Param1) -> ::windows::core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), label.into_param().abi(), action.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn CreateWithHandlerAndId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, UICommandInvokedHandler>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(label: Param0, action: Param1, commandid: Param2) -> ::windows::core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), label.into_param().abi(), action.into_param().abi(), commandid.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    pub fn IUICommandFactory<R, F: FnOnce(&IUICommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UICommand, IUICommandFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::core::RuntimeType for UICommand {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommand;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
}
unsafe impl ::windows::core::Interface for UICommand {
    type Vtable = IUICommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ff93a75_4145_47ff_ac7f_dff1c1fa5b0f);
}
impl ::windows::core::RuntimeName for UICommand {
    const NAME: &'static str = "Windows.UI.Popups.UICommand";
}
impl ::core::convert::From<UICommand> for ::windows::core::IUnknown {
    fn from(value: UICommand) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UICommand> for ::windows::core::IUnknown {
    fn from(value: &UICommand) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UICommand> for ::windows::core::IInspectable {
    fn from(value: UICommand) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UICommand> for ::windows::core::IInspectable {
    fn from(value: &UICommand) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<UICommand> for IUICommand {
    fn from(value: UICommand) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommand> for IUICommand {
    fn from(value: &UICommand) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUICommand> for UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, IUICommand> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUICommand> for &UICommand {
    fn into_param(self) -> ::windows::core::Param<'a, IUICommand> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UICommand {}
unsafe impl ::core::marker::Sync for UICommand {}
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UICommandInvokedHandler(::windows::core::IUnknown);
impl UICommandInvokedHandler {
    pub fn new<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows::core::Result<()> + 'static>(invoke: F) -> Self {
        let com = UICommandInvokedHandler_box::<F> {
            vtable: &UICommandInvokedHandler_box::<F>::VTABLE,
            count: ::windows::core::RefCount::new(1),
            invoke,
        };
        unsafe { core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, IUICommand>>(&self, command: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), command.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UICommandInvokedHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"delegate({daf77a4f-c27a-4298-9ac6-2922c45e7da6})");
}
unsafe impl ::windows::core::Interface for UICommandInvokedHandler {
    type Vtable = UICommandInvokedHandler_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaf77a4f_c27a_4298_9ac6_2922c45e7da6);
}
#[repr(C)]
#[doc(hidden)]
pub struct UICommandInvokedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, command: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(C)]
struct UICommandInvokedHandler_box<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows::core::Result<()> + 'static> {
    vtable: *const UICommandInvokedHandler_abi,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<IUICommand>) -> ::windows::core::Result<()> + 'static> UICommandInvokedHandler_box<F> {
    const VTABLE: UICommandInvokedHandler_abi = UICommandInvokedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<UICommandInvokedHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::core::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::core::HRESULT(0x8000_4002)
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
    unsafe extern "system" fn Invoke(this: ::windows::core::RawPtr, command: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(&*(&command as *const <IUICommand as ::windows::core::Abi>::Abi as *const <IUICommand as ::windows::core::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct UICommandSeparator(pub ::windows::core::IInspectable);
impl UICommandSeparator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<UICommandSeparator, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetLabel<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetInvoked<'a, Param0: ::windows::core::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetId<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::core::RuntimeType for UICommandSeparator {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommandSeparator;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
}
unsafe impl ::windows::core::Interface for UICommandSeparator {
    type Vtable = IUICommand_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ff93a75_4145_47ff_ac7f_dff1c1fa5b0f);
}
impl ::windows::core::RuntimeName for UICommandSeparator {
    const NAME: &'static str = "Windows.UI.Popups.UICommandSeparator";
}
impl ::core::convert::From<UICommandSeparator> for ::windows::core::IUnknown {
    fn from(value: UICommandSeparator) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&UICommandSeparator> for ::windows::core::IUnknown {
    fn from(value: &UICommandSeparator) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<UICommandSeparator> for ::windows::core::IInspectable {
    fn from(value: UICommandSeparator) -> Self {
        value.0
    }
}
impl ::core::convert::From<&UICommandSeparator> for ::windows::core::IInspectable {
    fn from(value: &UICommandSeparator) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::From<UICommandSeparator> for IUICommand {
    fn from(value: UICommandSeparator) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&UICommandSeparator> for IUICommand {
    fn from(value: &UICommandSeparator) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUICommand> for UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, IUICommand> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IUICommand> for &UICommandSeparator {
    fn into_param(self) -> ::windows::core::Param<'a, IUICommand> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for UICommandSeparator {}
unsafe impl ::core::marker::Sync for UICommandSeparator {}
