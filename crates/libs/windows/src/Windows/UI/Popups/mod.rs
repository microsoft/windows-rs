#[doc(hidden)]
#[repr(transparent)]
pub struct IMessageDialog(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IMessageDialog {
    type Vtable = IMessageDialog_Vtbl;
}
impl ::core::clone::Clone for IMessageDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageDialog {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x33f59b01_5325_43ab_9ab3_bdae440e4121);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialog_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Title: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
    pub DefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetDefaultCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub CancelCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub SetCancelCommandIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
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
}
impl ::core::clone::Clone for IMessageDialogFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageDialogFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d161777_a66f_4ea5_bb87_793ffa4941f2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDialogFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, content: ::std::mem::MaybeUninit<::windows::core::HSTRING>, title: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPopupMenu(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPopupMenu {
    type Vtable = IPopupMenu_Vtbl;
}
impl ::core::clone::Clone for IPopupMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPopupMenu {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e9bc6dc_880d_47fc_a0a1_72b639e62559);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPopupMenu_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub Commands: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    Commands: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, invocationpoint: super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsync: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsyncWithRect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsyncWithRect: usize,
    #[cfg(feature = "Foundation")]
    pub ShowAsyncWithRectAndPlacement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selection: super::super::Foundation::Rect, preferredplacement: Placement, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ShowAsyncWithRectAndPlacement: usize,
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct IUICommand(::windows::core::IUnknown);
impl IUICommand {
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
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UICommandInvokedHandler>();
            (::windows::core::Interface::vtable(this).Invoked)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInvoked(&self, value: &UICommandInvokedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvoked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
::windows::imp::interface_hierarchy!(IUICommand, ::windows::core::IUnknown, ::windows::core::IInspectable);
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
impl ::windows::core::RuntimeType for IUICommand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f}");
}
unsafe impl ::windows::core::Interface for IUICommand {
    type Vtable = IUICommand_Vtbl;
}
impl ::core::clone::Clone for IUICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUICommand {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4ff93a75_4145_47ff_ac7f_dff1c1fa5b0f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommand_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Label: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetLabel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Invoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetInvoked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Id: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IUICommandFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IUICommandFactory {
    type Vtable = IUICommandFactory_Vtbl;
}
impl ::core::clone::Clone for IUICommandFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IUICommandFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa21a8189_26b0_4676_ae94_54041bc125e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUICommandFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithHandler: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows::core::HSTRING>, action: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateWithHandlerAndId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, label: ::std::mem::MaybeUninit<::windows::core::HSTRING>, action: *mut ::core::ffi::c_void, commandid: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct MessageDialog(::windows::core::IUnknown);
impl MessageDialog {
    pub fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Title)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<IUICommand>>();
            (::windows::core::Interface::vtable(this).Commands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn DefaultCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).DefaultCommandIndex)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetDefaultCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultCommandIndex)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn CancelCommandIndex(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<u32>();
            (::windows::core::Interface::vtable(this).CancelCommandIndex)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetCancelCommandIndex(&self, value: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancelCommandIndex)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Content(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::HSTRING>();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetContent(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContent)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<IUICommand>>();
            (::windows::core::Interface::vtable(this).ShowAsync)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn Options(&self) -> ::windows::core::Result<MessageDialogOptions> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<MessageDialogOptions>();
            (::windows::core::Interface::vtable(this).Options)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetOptions(&self, value: MessageDialogOptions) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetOptions)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn Create(content: &::windows::core::HSTRING) -> ::windows::core::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<MessageDialog>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(content), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithTitle(content: &::windows::core::HSTRING, title: &::windows::core::HSTRING) -> ::windows::core::Result<MessageDialog> {
        Self::IMessageDialogFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<MessageDialog>();
            (::windows::core::Interface::vtable(this).CreateWithTitle)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(content), ::core::mem::transmute_copy(title), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IMessageDialogFactory<R, F: FnOnce(&IMessageDialogFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<MessageDialog, IMessageDialogFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for MessageDialog {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.MessageDialog;{33f59b01-5325-43ab-9ab3-bdae440e4121})");
}
impl ::core::clone::Clone for MessageDialog {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for MessageDialog {
    type Vtable = IMessageDialog_Vtbl;
}
unsafe impl ::windows::core::ComInterface for MessageDialog {
    const IID: ::windows::core::GUID = <IMessageDialog as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for MessageDialog {
    const NAME: &'static str = "Windows.UI.Popups.MessageDialog";
}
::windows::imp::interface_hierarchy!(MessageDialog, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct PopupMenu(::windows::core::IUnknown);
impl PopupMenu {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<PopupMenu, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Commands(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::Collections::IVector<IUICommand>>();
            (::windows::core::Interface::vtable(this).Commands)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsync(&self, invocationpoint: super::super::Foundation::Point) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<IUICommand>>();
            (::windows::core::Interface::vtable(this).ShowAsync)(::windows::core::Interface::as_raw(this), invocationpoint, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsyncWithRect(&self, selection: super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<IUICommand>>();
            (::windows::core::Interface::vtable(this).ShowAsyncWithRect)(::windows::core::Interface::as_raw(this), selection, &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ShowAsyncWithRectAndPlacement(&self, selection: super::super::Foundation::Rect, preferredplacement: Placement) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<IUICommand>> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<super::super::Foundation::IAsyncOperation<IUICommand>>();
            (::windows::core::Interface::vtable(this).ShowAsyncWithRectAndPlacement)(::windows::core::Interface::as_raw(this), selection, preferredplacement, &mut result__).from_abi(result__)
        }
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
impl ::windows::core::RuntimeType for PopupMenu {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.PopupMenu;{4e9bc6dc-880d-47fc-a0a1-72b639e62559})");
}
impl ::core::clone::Clone for PopupMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for PopupMenu {
    type Vtable = IPopupMenu_Vtbl;
}
unsafe impl ::windows::core::ComInterface for PopupMenu {
    const IID: ::windows::core::GUID = <IPopupMenu as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for PopupMenu {
    const NAME: &'static str = "Windows.UI.Popups.PopupMenu";
}
::windows::imp::interface_hierarchy!(PopupMenu, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct UICommand(::windows::core::IUnknown);
impl UICommand {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UICommand, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UICommandInvokedHandler>();
            (::windows::core::Interface::vtable(this).Invoked)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInvoked(&self, value: &UICommandInvokedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvoked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
    pub fn Create(label: &::windows::core::HSTRING) -> ::windows::core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UICommand>();
            (::windows::core::Interface::vtable(this).Create)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(label), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithHandler(label: &::windows::core::HSTRING, action: &UICommandInvokedHandler) -> ::windows::core::Result<UICommand> {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UICommand>();
            (::windows::core::Interface::vtable(this).CreateWithHandler)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(label), ::core::mem::transmute_copy(action), &mut result__).from_abi(result__)
        })
    }
    pub fn CreateWithHandlerAndId<P0>(label: &::windows::core::HSTRING, action: &UICommandInvokedHandler, commandid: P0) -> ::windows::core::Result<UICommand>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        Self::IUICommandFactory(|this| unsafe {
            let mut result__ = ::windows::core::zeroed::<UICommand>();
            (::windows::core::Interface::vtable(this).CreateWithHandlerAndId)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(label), ::core::mem::transmute_copy(action), commandid.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IUICommandFactory<R, F: FnOnce(&IUICommandFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UICommand, IUICommandFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows::core::RuntimeType for UICommand {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommand;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
}
impl ::core::clone::Clone for UICommand {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UICommand {
    type Vtable = IUICommand_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UICommand {
    const IID: ::windows::core::GUID = <IUICommand as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UICommand {
    const NAME: &'static str = "Windows.UI.Popups.UICommand";
}
::windows::imp::interface_hierarchy!(UICommand, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IUICommand> for UICommand {}
unsafe impl ::core::marker::Send for UICommand {}
unsafe impl ::core::marker::Sync for UICommand {}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct UICommandSeparator(::windows::core::IUnknown);
impl UICommandSeparator {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::imp::IGenericFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::imp::FactoryCache<UICommandSeparator, ::windows::imp::IGenericFactory> = ::windows::imp::FactoryCache::new();
        SHARED.call(callback)
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
    pub fn Invoked(&self) -> ::windows::core::Result<UICommandInvokedHandler> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<UICommandInvokedHandler>();
            (::windows::core::Interface::vtable(this).Invoked)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetInvoked(&self, value: &UICommandInvokedHandler) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetInvoked)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(value)).ok() }
    }
    pub fn Id(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::windows::core::zeroed::<::windows::core::IInspectable>();
            (::windows::core::Interface::vtable(this).Id)(::windows::core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetId<P0>(&self, value: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IInspectable>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetId)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
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
impl ::windows::core::RuntimeType for UICommandSeparator {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"rc(Windows.UI.Popups.UICommandSeparator;{4ff93a75-4145-47ff-ac7f-dff1c1fa5b0f})");
}
impl ::core::clone::Clone for UICommandSeparator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::Interface for UICommandSeparator {
    type Vtable = IUICommand_Vtbl;
}
unsafe impl ::windows::core::ComInterface for UICommandSeparator {
    const IID: ::windows::core::GUID = <IUICommand as ::windows::core::ComInterface>::IID;
}
impl ::windows::core::RuntimeName for UICommandSeparator {
    const NAME: &'static str = "Windows.UI.Popups.UICommandSeparator";
}
::windows::imp::interface_hierarchy!(UICommandSeparator, ::windows::core::IUnknown, ::windows::core::IInspectable);
impl ::windows::core::CanTryInto<IUICommand> for UICommandSeparator {}
unsafe impl ::core::marker::Send for UICommandSeparator {}
unsafe impl ::core::marker::Sync for UICommandSeparator {}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for MessageDialogOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MessageDialogOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MessageDialogOptions").field(&self.0).finish()
    }
}
impl MessageDialogOptions {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
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
impl ::windows::core::RuntimeType for MessageDialogOptions {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.MessageDialogOptions;u4)");
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::windows::core::TypeKind for Placement {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for Placement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Placement").field(&self.0).finish()
    }
}
impl ::windows::core::RuntimeType for Placement {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Popups.Placement;i4)");
}
#[doc = "*Required features: `\"UI_Popups\"`*"]
#[repr(transparent)]
pub struct UICommandInvokedHandler(pub ::windows::core::IUnknown);
impl UICommandInvokedHandler {
    pub fn new<F: FnMut(::core::option::Option<&IUICommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = UICommandInvokedHandlerBox::<F> { vtable: &UICommandInvokedHandlerBox::<F>::VTABLE, count: ::windows::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<P0>(&self, command: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::TryIntoParam<IUICommand>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), command.try_into_param()?.abi()).ok() }
    }
}
#[repr(C)]
struct UICommandInvokedHandlerBox<F: FnMut(::core::option::Option<&IUICommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const UICommandInvokedHandler_Vtbl,
    invoke: F,
    count: ::windows::imp::RefCount,
}
impl<F: FnMut(::core::option::Option<&IUICommand>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> UICommandInvokedHandlerBox<F> {
    const VTABLE: UICommandInvokedHandler_Vtbl = UICommandInvokedHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<UICommandInvokedHandler as ::windows::core::ComInterface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::ComInterface>::IID || iid == &<::windows::imp::IAgileObject as ::windows::core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::windows::core::from_raw_borrowed(&command)).into()
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
}
impl ::core::clone::Clone for UICommandInvokedHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for UICommandInvokedHandler {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdaf77a4f_c27a_4298_9ac6_2922c45e7da6);
}
impl ::windows::core::RuntimeType for UICommandInvokedHandler {
    const SIGNATURE: ::windows::imp::ConstBuffer = ::windows::imp::ConstBuffer::from_slice(b"{daf77a4f-c27a-4298-9ac6-2922c45e7da6}");
}
#[repr(C)]
#[doc(hidden)]
pub struct UICommandInvokedHandler_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
