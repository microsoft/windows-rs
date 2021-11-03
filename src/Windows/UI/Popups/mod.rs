#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[doc(hidden)]
pub struct IMessageDialog(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMessageDialog {
    type Vtable = IMessageDialog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(871734017, 21285, 17323, [154, 179, 189, 174, 68, 14, 65, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialog_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut MessageDialogOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: MessageDialogOptions) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IMessageDialogFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IMessageDialogFactory {
    type Vtable = IMessageDialogFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(756422519, 42607, 20133, [187, 135, 121, 63, 250, 73, 65, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialogFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, content: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, title: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IPopupMenu(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IPopupMenu {
    type Vtable = IPopupMenu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1318831836, 34829, 18428, [160, 161, 114, 182, 57, 230, 37, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupMenu_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, invocationpoint: super::super::Foundation::Point, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, selection: super::super::Foundation::Rect, preferredplacement: Placement, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Popups`*"]
pub struct IUICommand(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUICommand {
    type Vtable = IUICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1341733493, 16709, 18431, [172, 127, 223, 241, 193, 250, 91, 15]);
}
impl IUICommand {
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Invoked(&self) -> ::windows::runtime::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IUICommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommand_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IUICommandFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IUICommandFactory {
    type Vtable = IUICommandFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2719646089, 9904, 18038, [174, 148, 84, 4, 27, 193, 37, 232]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommandFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, action: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, label: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, action: ::windows::runtime::RawPtr, commandid: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct MessageDialog(::windows::runtime::IInspectable);
impl MessageDialog {
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Title(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Popups`, `Foundation_Collections`*"]
    pub fn Commands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn DefaultCommandIndex(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn CancelCommandIndex(&self) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetCancelCommandIndex(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Content(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetContent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Popups`, `Foundation`*"]
    pub fn ShowAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Options(&self) -> ::windows::runtime::Result<MessageDialogOptions> {
        let this = self;
        unsafe {
            let mut result__: MessageDialogOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<MessageDialogOptions>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetOptions(&self, value: MessageDialogOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(content: Param0) -> ::windows::runtime::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), content.into_param().abi(), &mut result__).from_abi::<MessageDialog>(result__)
        })
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn CreateWithTitle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(content: Param0, title: Param1) -> ::windows::runtime::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), content.into_param().abi(), title.into_param().abi(), &mut result__).from_abi::<MessageDialog>(result__)
        })
    }
    pub fn IMessageDialogFactory<R, F: FnOnce(&IMessageDialogFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<MessageDialog, IMessageDialogFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for MessageDialog {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.MessageDialog;{33f59b01-5325-43ab-9ab3-bdae440e4121})");
}
unsafe impl ::windows::runtime::Interface for MessageDialog {
    type Vtable = IMessageDialog_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(871734017, 21285, 17323, [154, 179, 189, 174, 68, 14, 65, 33]);
}
impl ::windows::runtime::RuntimeName for MessageDialog {
    const NAME: &'static str = "Windows.UI.Popups.MessageDialog";
}
#[doc = "*Required features: `UI_Popups`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MessageDialogOptions(pub u32);
impl MessageDialogOptions {
    pub const None: MessageDialogOptions = MessageDialogOptions(0u32);
    pub const AcceptUserInputAfterDelay: MessageDialogOptions = MessageDialogOptions(1u32);
}
impl ::std::convert::From<u32> for MessageDialogOptions {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MessageDialogOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for MessageDialogOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.MessageDialogOptions;u4)");
}
impl ::windows::runtime::DefaultType for MessageDialogOptions {
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MessageDialogOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MessageDialogOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MessageDialogOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MessageDialogOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MessageDialogOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `UI_Popups`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct Placement(pub i32);
impl Placement {
    pub const Default: Placement = Placement(0i32);
    pub const Above: Placement = Placement(1i32);
    pub const Below: Placement = Placement(2i32);
    pub const Left: Placement = Placement(3i32);
    pub const Right: Placement = Placement(4i32);
}
impl ::std::convert::From<i32> for Placement {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Placement {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Placement {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.Placement;i4)");
}
impl ::windows::runtime::DefaultType for Placement {
    type DefaultType = Self;
}
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct PopupMenu(::windows::runtime::IInspectable);
impl PopupMenu {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<PopupMenu, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation_Collections")]
    #[doc = "*Required features: `UI_Popups`, `Foundation_Collections`*"]
    pub fn Commands(&self) -> ::windows::runtime::Result<super::super::Foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::Collections::IVector<IUICommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Popups`, `Foundation`*"]
    pub fn ShowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Point>>(&self, invocationpoint: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), invocationpoint.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Popups`, `Foundation`*"]
    pub fn ShowAsyncWithRect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), selection.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Popups`, `Foundation`*"]
    pub fn ShowAsyncWithRectAndPlacement<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Rect>>(&self, selection: Param0, preferredplacement: Placement) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), selection.into_param().abi(), preferredplacement, &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<IUICommand>>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for PopupMenu {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.PopupMenu;{4e9bc6dc-880d-47fc-a0a1-72b639e62559})");
}
unsafe impl ::windows::runtime::Interface for PopupMenu {
    type Vtable = IPopupMenu_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1318831836, 34829, 18428, [160, 161, 114, 182, 57, 230, 37, 89]);
}
impl ::windows::runtime::RuntimeName for PopupMenu {
    const NAME: &'static str = "Windows.UI.Popups.PopupMenu";
}
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UICommand(::windows::runtime::IInspectable);
impl UICommand {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UICommand, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Invoked(&self) -> ::windows::runtime::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(label: Param0) -> ::windows::runtime::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), label.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn CreateWithHandler<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, UICommandInvokedHandler>>(label: Param0, action: Param1) -> ::windows::runtime::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), label.into_param().abi(), action.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn CreateWithHandlerAndId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, UICommandInvokedHandler>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(label: Param0, action: Param1, commandid: Param2) -> ::windows::runtime::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), label.into_param().abi(), action.into_param().abi(), commandid.into_param().abi(), &mut result__).from_abi::<UICommand>(result__)
        })
    }
    pub fn IUICommandFactory<R, F: FnOnce(&IUICommandFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UICommand, IUICommandFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UICommand {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommand;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
}
unsafe impl ::windows::runtime::Interface for UICommand {
    type Vtable = IUICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1341733493, 16709, 18431, [172, 127, 223, 241, 193, 250, 91, 15]);
}
impl ::windows::runtime::RuntimeName for UICommand {
    const NAME: &'static str = "Windows.UI.Popups.UICommand";
}
impl ::std::convert::From<UICommand> for IUICommand {
    fn from(value: UICommand) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UICommand> for IUICommand {
    fn from(value: &UICommand) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUICommand> for UICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUICommand> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IUICommand>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUICommand> for &UICommand {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUICommand> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IUICommand>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for UICommand {}
unsafe impl ::std::marker::Sync for UICommand {}
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct UICommandInvokedHandler(::windows::runtime::IUnknown);
impl UICommandInvokedHandler {
    pub fn new<F: FnMut(&::std::option::Option<IUICommand>) -> ::windows::runtime::Result<()> + 'static>(invoke: F) -> Self {
        let com = UICommandInvokedHandler_box::<F> {
            vtable: &UICommandInvokedHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Invoke<'a, Param0: ::windows::runtime::IntoParam<'a, IUICommand>>(&self, command: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), command.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UICommandInvokedHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({daf77a4f-c27a-4298-9ac6-2922c45e7da6})");
}
unsafe impl ::windows::runtime::Interface for UICommandInvokedHandler {
    type Vtable = UICommandInvokedHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3673651791, 49786, 17048, [154, 198, 41, 34, 196, 94, 125, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct UICommandInvokedHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct UICommandInvokedHandler_box<F: FnMut(&::std::option::Option<IUICommand>) -> ::windows::runtime::Result<()> + 'static> {
    vtable: *const UICommandInvokedHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&::std::option::Option<IUICommand>) -> ::windows::runtime::Result<()> + 'static> UICommandInvokedHandler_box<F> {
    const VTABLE: UICommandInvokedHandler_abi = UICommandInvokedHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<UICommandInvokedHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, command: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ((*this).invoke)(&*(&command as *const <IUICommand as ::windows::runtime::Abi>::Abi as *const <IUICommand as ::windows::runtime::DefaultType>::DefaultType)).into()
    }
}
#[doc = "*Required features: `UI_Popups`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct UICommandSeparator(::windows::runtime::IInspectable);
impl UICommandSeparator {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<UICommandSeparator, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Label(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetLabel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Invoked(&self) -> ::windows::runtime::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<UICommandInvokedHandler>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetInvoked<'a, Param0: ::windows::runtime::IntoParam<'a, UICommandInvokedHandler>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Popups`*"]
    pub fn SetId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for UICommandSeparator {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommandSeparator;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
}
unsafe impl ::windows::runtime::Interface for UICommandSeparator {
    type Vtable = IUICommand_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1341733493, 16709, 18431, [172, 127, 223, 241, 193, 250, 91, 15]);
}
impl ::windows::runtime::RuntimeName for UICommandSeparator {
    const NAME: &'static str = "Windows.UI.Popups.UICommandSeparator";
}
impl ::std::convert::From<UICommandSeparator> for IUICommand {
    fn from(value: UICommandSeparator) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&UICommandSeparator> for IUICommand {
    fn from(value: &UICommandSeparator) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUICommand> for UICommandSeparator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUICommand> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IUICommand>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IUICommand> for &UICommandSeparator {
    fn into_param(self) -> ::windows::runtime::Param<'a, IUICommand> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IUICommand>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for UICommandSeparator {}
unsafe impl ::std::marker::Sync for UICommandSeparator {}
