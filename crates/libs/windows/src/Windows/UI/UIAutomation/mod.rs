#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[doc = "*Required features: `\"UI_UIAutomation\"`*"]
#[repr(transparent)]
pub struct AutomationConnection(::windows::core::IUnknown);
impl AutomationConnection {
    pub fn IsRemoteSystem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsRemoteSystem)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppUserModelId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExecutableFileName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AutomationConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationConnection {}
impl ::core::fmt::Debug for AutomationConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationConnection;{aad262ed-0ef4-5d43-97be-a834e27b65b9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AutomationConnection {
    type Vtable = IAutomationConnection_Vtbl;
    const IID: ::windows::core::GUID = <IAutomationConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AutomationConnection {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnection";
}
impl ::core::convert::From<AutomationConnection> for ::windows::core::IUnknown {
    fn from(value: AutomationConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationConnection> for ::windows::core::IUnknown {
    fn from(value: &AutomationConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AutomationConnection> for &::windows::core::IUnknown {
    fn from(value: &AutomationConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AutomationConnection> for ::windows::core::IInspectable {
    fn from(value: AutomationConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationConnection> for ::windows::core::IInspectable {
    fn from(value: &AutomationConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AutomationConnection> for &::windows::core::IInspectable {
    fn from(value: &AutomationConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AutomationConnection {}
unsafe impl ::core::marker::Sync for AutomationConnection {}
#[doc = "*Required features: `\"UI_UIAutomation\"`*"]
#[repr(transparent)]
pub struct AutomationConnectionBoundObject(::windows::core::IUnknown);
impl AutomationConnectionBoundObject {
    pub fn Connection(&self) -> ::windows::core::Result<AutomationConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Connection)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<AutomationConnection>(result__)
        }
    }
}
impl ::core::clone::Clone for AutomationConnectionBoundObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationConnectionBoundObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationConnectionBoundObject {}
impl ::core::fmt::Debug for AutomationConnectionBoundObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationConnectionBoundObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationConnectionBoundObject {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationConnectionBoundObject;{5e8558fb-ca52-5b65-9830-dd2905816093})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObject_Vtbl;
    const IID: ::windows::core::GUID = <IAutomationConnectionBoundObject as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AutomationConnectionBoundObject {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnectionBoundObject";
}
impl ::core::convert::From<AutomationConnectionBoundObject> for ::windows::core::IUnknown {
    fn from(value: AutomationConnectionBoundObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationConnectionBoundObject> for ::windows::core::IUnknown {
    fn from(value: &AutomationConnectionBoundObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AutomationConnectionBoundObject> for &::windows::core::IUnknown {
    fn from(value: &AutomationConnectionBoundObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AutomationConnectionBoundObject> for ::windows::core::IInspectable {
    fn from(value: AutomationConnectionBoundObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationConnectionBoundObject> for ::windows::core::IInspectable {
    fn from(value: &AutomationConnectionBoundObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AutomationConnectionBoundObject> for &::windows::core::IInspectable {
    fn from(value: &AutomationConnectionBoundObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AutomationConnectionBoundObject {}
unsafe impl ::core::marker::Sync for AutomationConnectionBoundObject {}
#[doc = "*Required features: `\"UI_UIAutomation\"`*"]
#[repr(transparent)]
pub struct AutomationElement(::windows::core::IUnknown);
impl AutomationElement {
    pub fn IsRemoteSystem(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsRemoteSystem)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppUserModelId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ExecutableFileName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ExecutableFileName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::clone::Clone for AutomationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationElement {}
impl ::core::fmt::Debug for AutomationElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationElement {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationElement;{a1898370-2c07-56fd-993f-61a72a08058c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AutomationElement {
    type Vtable = IAutomationElement_Vtbl;
    const IID: ::windows::core::GUID = <IAutomationElement as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AutomationElement {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationElement";
}
impl ::core::convert::From<AutomationElement> for ::windows::core::IUnknown {
    fn from(value: AutomationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationElement> for ::windows::core::IUnknown {
    fn from(value: &AutomationElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AutomationElement> for &::windows::core::IUnknown {
    fn from(value: &AutomationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AutomationElement> for ::windows::core::IInspectable {
    fn from(value: AutomationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationElement> for ::windows::core::IInspectable {
    fn from(value: &AutomationElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AutomationElement> for &::windows::core::IInspectable {
    fn from(value: &AutomationElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AutomationElement {}
unsafe impl ::core::marker::Sync for AutomationElement {}
#[doc = "*Required features: `\"UI_UIAutomation\"`*"]
#[repr(transparent)]
pub struct AutomationTextRange(::windows::core::IUnknown);
impl AutomationTextRange {}
impl ::core::clone::Clone for AutomationTextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AutomationTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationTextRange {}
impl ::core::fmt::Debug for AutomationTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AutomationTextRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationTextRange;{7e101b65-40d3-5994-85a9-0a0cb9a4ec98})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AutomationTextRange {
    type Vtable = IAutomationTextRange_Vtbl;
    const IID: ::windows::core::GUID = <IAutomationTextRange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AutomationTextRange {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationTextRange";
}
impl ::core::convert::From<AutomationTextRange> for ::windows::core::IUnknown {
    fn from(value: AutomationTextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationTextRange> for ::windows::core::IUnknown {
    fn from(value: &AutomationTextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AutomationTextRange> for &::windows::core::IUnknown {
    fn from(value: &AutomationTextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<AutomationTextRange> for ::windows::core::IInspectable {
    fn from(value: AutomationTextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AutomationTextRange> for ::windows::core::IInspectable {
    fn from(value: &AutomationTextRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&AutomationTextRange> for &::windows::core::IInspectable {
    fn from(value: &AutomationTextRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for AutomationTextRange {}
unsafe impl ::core::marker::Sync for AutomationTextRange {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationConnection {
    type Vtable = IAutomationConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaad262ed_0ef4_5d43_97be_a834e27b65b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationConnection_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsRemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationConnectionBoundObject(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e8558fb_ca52_5b65_9830_dd2905816093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationConnectionBoundObject_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElement(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationElement {
    type Vtable = IAutomationElement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1898370_2c07_56fd_993f_61a72a08058c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElement_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsRemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationTextRange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAutomationTextRange {
    type Vtable = IAutomationTextRange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e101b65_40d3_5994_85a9_0a0cb9a4ec98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationTextRange_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
