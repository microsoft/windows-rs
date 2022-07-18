#[cfg(feature = "UI_WebUI_Core")]
pub mod Core;
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct ActivatedDeferral(::windows::core::IUnknown);
impl ActivatedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for ActivatedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivatedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivatedDeferral {}
impl ::core::fmt::Debug for ActivatedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivatedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivatedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.ActivatedDeferral;{c3bd1978-a431-49d8-a76a-395a4e03dcf3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ActivatedDeferral {
    type Vtable = IActivatedDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IActivatedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedDeferral";
}
impl ::core::convert::From<ActivatedDeferral> for ::windows::core::IUnknown {
    fn from(value: ActivatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivatedDeferral> for ::windows::core::IUnknown {
    fn from(value: &ActivatedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ActivatedDeferral> for &::windows::core::IUnknown {
    fn from(value: &ActivatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ActivatedDeferral> for ::windows::core::IInspectable {
    fn from(value: ActivatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivatedDeferral> for ::windows::core::IInspectable {
    fn from(value: &ActivatedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ActivatedDeferral> for &::windows::core::IInspectable {
    fn from(value: &ActivatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct ActivatedEventHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl ActivatedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ActivatedEventHandlerBox::<F> { vtable: &ActivatedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Invoke<'a, P0, P1, E1>(&self, sender: P0, eventargs: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), sender.into().abi(), eventargs.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct ActivatedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ActivatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ActivatedEventHandlerBox<F> {
    const VTABLE: ActivatedEventHandler_Vtbl = ActivatedEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<ActivatedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&eventargs)).into()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for ActivatedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for ActivatedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for ActivatedEventHandler {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for ActivatedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivatedEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for ActivatedEventHandler {
    type Vtable = ActivatedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50f1e730_c5d1_4b6b_9adb_8a11756be29c);
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for ActivatedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{50f1e730-c5d1-4b6b-9adb-8a11756be29c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct ActivatedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct ActivatedOperation(::windows::core::IUnknown);
impl ActivatedOperation {
    pub fn GetDeferral(&self) -> ::windows::core::Result<ActivatedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for ActivatedOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ActivatedOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ActivatedOperation {}
impl ::core::fmt::Debug for ActivatedOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ActivatedOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ActivatedOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.ActivatedOperation;{b6a0b4bc-c6ca-42fd-9818-71904e45fed7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ActivatedOperation {
    type Vtable = IActivatedOperation_Vtbl;
    const IID: ::windows::core::GUID = <IActivatedOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ActivatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedOperation";
}
impl ::core::convert::From<ActivatedOperation> for ::windows::core::IUnknown {
    fn from(value: ActivatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivatedOperation> for ::windows::core::IUnknown {
    fn from(value: &ActivatedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ActivatedOperation> for &::windows::core::IUnknown {
    fn from(value: &ActivatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<ActivatedOperation> for ::windows::core::IInspectable {
    fn from(value: ActivatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ActivatedOperation> for ::windows::core::IInspectable {
    fn from(value: &ActivatedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&ActivatedOperation> for &::windows::core::IInspectable {
    fn from(value: &ActivatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct BackgroundActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Background\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Background"))]
    pub fn TaskInstance(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTaskInstance> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TaskInstance)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for BackgroundActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for BackgroundActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for BackgroundActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for BackgroundActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.BackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&BackgroundActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<BackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&BackgroundActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&BackgroundActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<BackgroundActivatedEventArgs> for super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: BackgroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&BackgroundActivatedEventArgs> for super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&BackgroundActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &BackgroundActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for BackgroundActivatedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct BackgroundActivatedEventHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = BackgroundActivatedEventHandlerBox::<F> { vtable: &BackgroundActivatedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Invoke<'a, P0, P1, E1>(&self, sender: P0, eventargs: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), sender.into().abi(), eventargs.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct BackgroundActivatedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const BackgroundActivatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> BackgroundActivatedEventHandlerBox<F> {
    const VTABLE: BackgroundActivatedEventHandler_Vtbl = BackgroundActivatedEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<BackgroundActivatedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&eventargs)).into()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for BackgroundActivatedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for BackgroundActivatedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for BackgroundActivatedEventHandler {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for BackgroundActivatedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BackgroundActivatedEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for BackgroundActivatedEventHandler {
    type Vtable = BackgroundActivatedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xedb19fbb_0761_47cc_9a77_24d7072965ca);
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for BackgroundActivatedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{edb19fbb-0761-47cc-9a77-24d7072965ca}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundActivatedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, eventargs: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel\"`*"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct EnteredBackgroundEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for EnteredBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for EnteredBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for EnteredBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnteredBackgroundEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::RuntimeType for EnteredBackgroundEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.EnteredBackgroundEventArgs;{f722dcc2-9827-403d-aaed-ecca9ac17398})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::Interface for EnteredBackgroundEventArgs {
    type Vtable = super::super::ApplicationModel::IEnteredBackgroundEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::IEnteredBackgroundEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::core::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.EnteredBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<EnteredBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&EnteredBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&EnteredBackgroundEventArgs> for &::windows::core::IUnknown {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<EnteredBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&EnteredBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&EnteredBackgroundEventArgs> for &::windows::core::IInspectable {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<EnteredBackgroundEventArgs> for super::super::ApplicationModel::IEnteredBackgroundEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: EnteredBackgroundEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&EnteredBackgroundEventArgs> for super::super::ApplicationModel::IEnteredBackgroundEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &EnteredBackgroundEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::core::convert::TryFrom<&EnteredBackgroundEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::IEnteredBackgroundEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &EnteredBackgroundEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Send for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Sync for EnteredBackgroundEventArgs {}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel\"`*"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct EnteredBackgroundEventHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = EnteredBackgroundEventHandlerBox::<F> { vtable: &EnteredBackgroundEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<'a, P0, P1, E1>(&self, sender: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::ApplicationModel::IEnteredBackgroundEventArgs>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), sender.into().abi(), e.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct EnteredBackgroundEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const EnteredBackgroundEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::IEnteredBackgroundEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> EnteredBackgroundEventHandlerBox<F> {
    const VTABLE: EnteredBackgroundEventHandler_Vtbl = EnteredBackgroundEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<EnteredBackgroundEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for EnteredBackgroundEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for EnteredBackgroundEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for EnteredBackgroundEventHandler {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for EnteredBackgroundEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EnteredBackgroundEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::Interface for EnteredBackgroundEventHandler {
    type Vtable = EnteredBackgroundEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2b09a173_b68e_4def_88c1_8de84e5aab2f);
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::RuntimeType for EnteredBackgroundEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2b09a173-b68e-4def-88c1-8de84e5aab2f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct EnteredBackgroundEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct HtmlPrintDocumentSource(::windows::core::IUnknown);
impl HtmlPrintDocumentSource {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn Content(&self) -> ::windows::core::Result<PrintContent> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Content)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<PrintContent>(result__)
        }
    }
    pub fn SetContent(&self, value: PrintContent) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetContent)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn LeftMargin(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LeftMargin)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetLeftMargin(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetLeftMargin)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn TopMargin(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TopMargin)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetTopMargin(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetTopMargin)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RightMargin(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RightMargin)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetRightMargin(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRightMargin)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn BottomMargin(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BottomMargin)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetBottomMargin(&self, value: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetBottomMargin)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn EnableHeaderFooter(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnableHeaderFooter)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableHeaderFooter(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetEnableHeaderFooter)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn ShrinkToFit(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShrinkToFit)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetShrinkToFit(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetShrinkToFit)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn PercentScale(&self) -> ::windows::core::Result<f32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PercentScale)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f32>(result__)
        }
    }
    pub fn SetPercentScale(&self, scalepercent: f32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPercentScale)(::windows::core::Interface::as_raw(this), scalepercent).ok() }
    }
    pub fn PageRange(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PageRange)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn TrySetPageRange(&self, strpagerange: &::windows::core::HSTRING) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TrySetPageRange)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(strpagerange), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::clone::Clone for HtmlPrintDocumentSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for HtmlPrintDocumentSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for HtmlPrintDocumentSource {}
impl ::core::fmt::Debug for HtmlPrintDocumentSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HtmlPrintDocumentSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for HtmlPrintDocumentSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.HtmlPrintDocumentSource;{cea6469a-0e05-467a-abc9-36ec1d4cdcb6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for HtmlPrintDocumentSource {
    type Vtable = IHtmlPrintDocumentSource_Vtbl;
    const IID: ::windows::core::GUID = <IHtmlPrintDocumentSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for HtmlPrintDocumentSource {
    const NAME: &'static str = "Windows.UI.WebUI.HtmlPrintDocumentSource";
}
impl ::core::convert::From<HtmlPrintDocumentSource> for ::windows::core::IUnknown {
    fn from(value: HtmlPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HtmlPrintDocumentSource> for ::windows::core::IUnknown {
    fn from(value: &HtmlPrintDocumentSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&HtmlPrintDocumentSource> for &::windows::core::IUnknown {
    fn from(value: &HtmlPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<HtmlPrintDocumentSource> for ::windows::core::IInspectable {
    fn from(value: HtmlPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&HtmlPrintDocumentSource> for ::windows::core::IInspectable {
    fn from(value: &HtmlPrintDocumentSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&HtmlPrintDocumentSource> for &::windows::core::IInspectable {
    fn from(value: &HtmlPrintDocumentSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<HtmlPrintDocumentSource> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: HtmlPrintDocumentSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&HtmlPrintDocumentSource> for super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &HtmlPrintDocumentSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&HtmlPrintDocumentSource> for ::windows::core::InParam<'a, super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HtmlPrintDocumentSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::core::convert::TryFrom<HtmlPrintDocumentSource> for super::super::Graphics::Printing::IPrintDocumentSource {
    type Error = ::windows::core::Error;
    fn try_from(value: HtmlPrintDocumentSource) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::core::convert::TryFrom<&HtmlPrintDocumentSource> for super::super::Graphics::Printing::IPrintDocumentSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &HtmlPrintDocumentSource) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl<'a> ::core::convert::TryFrom<&HtmlPrintDocumentSource> for ::windows::core::InParam<'a, super::super::Graphics::Printing::IPrintDocumentSource> {
    type Error = ::windows::core::Error;
    fn try_from(value: &HtmlPrintDocumentSource) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for HtmlPrintDocumentSource {}
unsafe impl ::core::marker::Sync for HtmlPrintDocumentSource {}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivatedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivatedDeferral {
    type Vtable = IActivatedDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3bd1978_a431_49d8_a76a_395a4e03dcf3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedDeferral_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct IActivatedEventArgsDeferral(::windows::core::IUnknown);
impl IActivatedEventArgsDeferral {
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
}
impl ::core::convert::From<IActivatedEventArgsDeferral> for ::windows::core::IUnknown {
    fn from(value: IActivatedEventArgsDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActivatedEventArgsDeferral> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IActivatedEventArgsDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsDeferral> for ::windows::core::IUnknown {
    fn from(value: &IActivatedEventArgsDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IActivatedEventArgsDeferral> for ::windows::core::IInspectable {
    fn from(value: IActivatedEventArgsDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IActivatedEventArgsDeferral> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IActivatedEventArgsDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IActivatedEventArgsDeferral> for ::windows::core::IInspectable {
    fn from(value: &IActivatedEventArgsDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IActivatedEventArgsDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IActivatedEventArgsDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActivatedEventArgsDeferral {}
impl ::core::fmt::Debug for IActivatedEventArgsDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActivatedEventArgsDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IActivatedEventArgsDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ca6d5f74-63c2-44a6-b97b-d9a03c20bc9b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IActivatedEventArgsDeferral {
    type Vtable = IActivatedEventArgsDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca6d5f74_63c2_44a6_b97b_d9a03c20bc9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsDeferral_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ActivatedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IActivatedOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IActivatedOperation {
    type Vtable = IActivatedOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6a0b4bc_c6ca_42fd_9818_71904e45fed7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedOperation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IHtmlPrintDocumentSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IHtmlPrintDocumentSource {
    type Vtable = IHtmlPrintDocumentSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcea6469a_0e05_467a_abc9_36ec1d4cdcb6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlPrintDocumentSource_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Content: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut PrintContent) -> ::windows::core::HRESULT,
    pub SetContent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: PrintContent) -> ::windows::core::HRESULT,
    pub LeftMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetLeftMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub TopMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetTopMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub RightMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetRightMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub BottomMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetBottomMargin: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f32) -> ::windows::core::HRESULT,
    pub EnableHeaderFooter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetEnableHeaderFooter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ShrinkToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetShrinkToFit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub PercentScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f32) -> ::windows::core::HRESULT,
    pub SetPercentScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scalepercent: f32) -> ::windows::core::HRESULT,
    pub PageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub TrySetPageRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, strpagerange: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct INewWebUIViewCreatedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for INewWebUIViewCreatedEventArgs {
    type Vtable = INewWebUIViewCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8e1b216_be2b_4c9e_85e7_083143ec4be7);
}
#[repr(C)]
#[doc(hidden)]
pub struct INewWebUIViewCreatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub WebUIView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "ApplicationModel_Activation")]
    pub ActivatedEventArgs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))]
    ActivatedEventArgs: usize,
    pub HasPendingNavigate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetDeferral: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIActivationStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUIActivationStatics {
    type Vtable = IWebUIActivationStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x351b86bd_43b3_482b_85db_35d87b517ad9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub Suspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    Suspending: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveSuspending: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveSuspending: usize,
    #[cfg(feature = "Foundation")]
    pub Resuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Resuming: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveResuming: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveResuming: usize,
    #[cfg(feature = "Foundation")]
    pub Navigated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Navigated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNavigated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNavigated: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIActivationStatics2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUIActivationStatics2 {
    type Vtable = IWebUIActivationStatics2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc8e88696_4d78_4aa4_8f06_2a9eadc6c40a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub LeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    LeavingBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveLeavingBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveLeavingBackground: usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub EnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))]
    EnteredBackground: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveEnteredBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveEnteredBackground: usize,
    pub EnablePrelaunch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIActivationStatics3(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUIActivationStatics3 {
    type Vtable = IWebUIActivationStatics3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91abb686_1af5_4445_b49f_9459f40fc8de);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics3_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub RequestRestartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))]
    RequestRestartAsync: usize,
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System"))]
    pub RequestRestartForUserAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, user: *mut ::core::ffi::c_void, launcharguments: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System")))]
    RequestRestartForUserAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIActivationStatics4(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUIActivationStatics4 {
    type Vtable = IWebUIActivationStatics4_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5e391429_183f_478d_8a25_67f80d03935b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics4_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub NewWebUIViewCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    NewWebUIViewCreated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveNewWebUIViewCreated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveNewWebUIViewCreated: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub BackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    BackgroundActivated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveBackgroundActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveBackgroundActivated: usize,
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct IWebUIBackgroundTaskInstance(::windows::core::IUnknown);
impl IWebUIBackgroundTaskInstance {
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Succeeded)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSucceeded)(::windows::core::Interface::as_raw(this), succeeded).ok() }
    }
}
impl ::core::convert::From<IWebUIBackgroundTaskInstance> for ::windows::core::IUnknown {
    fn from(value: IWebUIBackgroundTaskInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebUIBackgroundTaskInstance> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWebUIBackgroundTaskInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUIBackgroundTaskInstance> for ::windows::core::IUnknown {
    fn from(value: &IWebUIBackgroundTaskInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWebUIBackgroundTaskInstance> for ::windows::core::IInspectable {
    fn from(value: IWebUIBackgroundTaskInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebUIBackgroundTaskInstance> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWebUIBackgroundTaskInstance) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUIBackgroundTaskInstance> for ::windows::core::IInspectable {
    fn from(value: &IWebUIBackgroundTaskInstance) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWebUIBackgroundTaskInstance {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebUIBackgroundTaskInstance {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebUIBackgroundTaskInstance {}
impl ::core::fmt::Debug for IWebUIBackgroundTaskInstance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebUIBackgroundTaskInstance").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebUIBackgroundTaskInstance {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{23f12c25-e2f7-4741-bc9c-394595de24dc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebUIBackgroundTaskInstance {
    type Vtable = IWebUIBackgroundTaskInstance_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x23f12c25_e2f7_4741_bc9c_394595de24dc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstance_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Succeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetSucceeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, succeeded: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIBackgroundTaskInstanceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUIBackgroundTaskInstanceStatics {
    type Vtable = IWebUIBackgroundTaskInstanceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9c7a5291_19ae_4ca3_b94b_fe4ec744a740);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstanceStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Current: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUINavigatedDeferral(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUINavigatedDeferral {
    type Vtable = IWebUINavigatedDeferral_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd804204d_831f_46e2_b432_3afce211f962);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedDeferral_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Complete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct IWebUINavigatedEventArgs(::windows::core::IUnknown);
impl IWebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> ::windows::core::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NavigatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebUINavigatedOperation>(result__)
        }
    }
}
impl ::core::convert::From<IWebUINavigatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: IWebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebUINavigatedEventArgs> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IWebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUINavigatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &IWebUINavigatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IWebUINavigatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: IWebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IWebUINavigatedEventArgs> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IWebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWebUINavigatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &IWebUINavigatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IWebUINavigatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWebUINavigatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWebUINavigatedEventArgs {}
impl ::core::fmt::Debug for IWebUINavigatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWebUINavigatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWebUINavigatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a75841b8-2499-4030-a69d-15d2d9cfe524}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWebUINavigatedEventArgs {
    type Vtable = IWebUINavigatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa75841b8_2499_4030_a69d_15d2d9cfe524);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub NavigatedOperation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUINavigatedOperation(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUINavigatedOperation {
    type Vtable = IWebUINavigatedOperation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a965f08_8182_4a89_ab67_8492e8750d4b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedOperation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetDeferral: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIView(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUIView {
    type Vtable = IWebUIView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6783f64f_52da_4fd7_be69_8ef6284b423c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIView_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ApplicationViewId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub Closed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Closed: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveClosed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveClosed: usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub Activated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))]
    Activated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveActivated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveActivated: usize,
    pub IgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIgnoreApplicationContentUriRulesNavigationRestrictions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IWebUIViewStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IWebUIViewStatics {
    type Vtable = IWebUIViewStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb591e668_8e59_44f9_8803_1b24c9149d30);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIViewStatics_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateAsync: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWithUriAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, uri: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWithUriAsync: usize,
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel\"`*"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct LeavingBackgroundEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for LeavingBackgroundEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for LeavingBackgroundEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for LeavingBackgroundEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LeavingBackgroundEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::RuntimeType for LeavingBackgroundEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.LeavingBackgroundEventArgs;{39c6ec9a-ae6e-46f9-a07a-cfc23f88733e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::Interface for LeavingBackgroundEventArgs {
    type Vtable = super::super::ApplicationModel::ILeavingBackgroundEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::ILeavingBackgroundEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::core::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.LeavingBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<LeavingBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&LeavingBackgroundEventArgs> for ::windows::core::IUnknown {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&LeavingBackgroundEventArgs> for &::windows::core::IUnknown {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<LeavingBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&LeavingBackgroundEventArgs> for ::windows::core::IInspectable {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&LeavingBackgroundEventArgs> for &::windows::core::IInspectable {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<LeavingBackgroundEventArgs> for super::super::ApplicationModel::ILeavingBackgroundEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: LeavingBackgroundEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&LeavingBackgroundEventArgs> for super::super::ApplicationModel::ILeavingBackgroundEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &LeavingBackgroundEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::core::convert::TryFrom<&LeavingBackgroundEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::ILeavingBackgroundEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &LeavingBackgroundEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Send for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::core::marker::Sync for LeavingBackgroundEventArgs {}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel\"`*"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct LeavingBackgroundEventHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = LeavingBackgroundEventHandlerBox::<F> { vtable: &LeavingBackgroundEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<'a, P0, P1, E1>(&self, sender: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::ApplicationModel::ILeavingBackgroundEventArgs>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), sender.into().abi(), e.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct LeavingBackgroundEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const LeavingBackgroundEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::ILeavingBackgroundEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> LeavingBackgroundEventHandlerBox<F> {
    const VTABLE: LeavingBackgroundEventHandler_Vtbl = LeavingBackgroundEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<LeavingBackgroundEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for LeavingBackgroundEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for LeavingBackgroundEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for LeavingBackgroundEventHandler {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for LeavingBackgroundEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LeavingBackgroundEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::Interface for LeavingBackgroundEventHandler {
    type Vtable = LeavingBackgroundEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00b4ccd9_7a9c_4b6b_9ac4_13474f268bc4);
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::RuntimeType for LeavingBackgroundEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{00b4ccd9-7a9c-4b6b-9ac4-13474f268bc4}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct LeavingBackgroundEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct NavigatedEventHandler(pub ::windows::core::IUnknown);
impl NavigatedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<IWebUINavigatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = NavigatedEventHandlerBox::<F> { vtable: &NavigatedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0, P1, E1>(&self, sender: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, IWebUINavigatedEventArgs>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), sender.into().abi(), e.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[repr(C)]
struct NavigatedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<IWebUINavigatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const NavigatedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<IWebUINavigatedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> NavigatedEventHandlerBox<F> {
    const VTABLE: NavigatedEventHandler_Vtbl = NavigatedEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<NavigatedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for NavigatedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NavigatedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NavigatedEventHandler {}
impl ::core::fmt::Debug for NavigatedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NavigatedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7af46fe6_40ca_4e49_a7d6_dbdb330cd1a3);
}
unsafe impl ::windows::core::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7af46fe6-40ca-4e49-a7d6-dbdb330cd1a3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct NewWebUIViewCreatedEventArgs(::windows::core::IUnknown);
impl NewWebUIViewCreatedEventArgs {
    pub fn WebUIView(&self) -> ::windows::core::Result<WebUIView> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WebUIView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebUIView>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ActivatedEventArgs(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::IActivatedEventArgs> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedEventArgs)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(result__)
        }
    }
    pub fn HasPendingNavigate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).HasPendingNavigate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
impl ::core::clone::Clone for NewWebUIViewCreatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for NewWebUIViewCreatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for NewWebUIViewCreatedEventArgs {}
impl ::core::fmt::Debug for NewWebUIViewCreatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NewWebUIViewCreatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for NewWebUIViewCreatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.NewWebUIViewCreatedEventArgs;{e8e1b216-be2b-4c9e-85e7-083143ec4be7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for NewWebUIViewCreatedEventArgs {
    type Vtable = INewWebUIViewCreatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <INewWebUIViewCreatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for NewWebUIViewCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.NewWebUIViewCreatedEventArgs";
}
impl ::core::convert::From<NewWebUIViewCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: NewWebUIViewCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NewWebUIViewCreatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &NewWebUIViewCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&NewWebUIViewCreatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &NewWebUIViewCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<NewWebUIViewCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: NewWebUIViewCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&NewWebUIViewCreatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &NewWebUIViewCreatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&NewWebUIViewCreatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &NewWebUIViewCreatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PrintContent(pub i32);
impl PrintContent {
    pub const AllPages: Self = Self(0i32);
    pub const CurrentPage: Self = Self(1i32);
    pub const CustomPageRange: Self = Self(2i32);
    pub const CurrentSelection: Self = Self(3i32);
}
impl ::core::marker::Copy for PrintContent {}
impl ::core::clone::Clone for PrintContent {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PrintContent {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PrintContent {
    type Abi = Self;
}
impl ::core::fmt::Debug for PrintContent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintContent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintContent {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.PrintContent;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct ResumingEventHandler(pub ::windows::core::IUnknown);
impl ResumingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = ResumingEventHandlerBox::<F> { vtable: &ResumingEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke<'a, P0>(&self, sender: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), sender.into().abi()).ok() }
    }
}
#[repr(C)]
struct ResumingEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const ResumingEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> ResumingEventHandlerBox<F> {
    const VTABLE: ResumingEventHandler_Vtbl = ResumingEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<ResumingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender)).into()
    }
}
impl ::core::clone::Clone for ResumingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ResumingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ResumingEventHandler {}
impl ::core::fmt::Debug for ResumingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResumingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ResumingEventHandler {
    type Vtable = ResumingEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x26599ba9_a22d_4806_a728_acadc1d075fa);
}
unsafe impl ::windows::core::RuntimeType for ResumingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{26599ba9-a22d-4806-a728-acadc1d075fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ResumingEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel\"`*"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct SuspendingDeferral(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingDeferral {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for SuspendingDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for SuspendingDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for SuspendingDeferral {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for SuspendingDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingDeferral").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::RuntimeType for SuspendingDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.SuspendingDeferral;{59140509-8bc9-4eb4-b636-dabdc4f46f66})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::Interface for SuspendingDeferral {
    type Vtable = super::super::ApplicationModel::ISuspendingDeferral_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::ISuspendingDeferral as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::core::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingDeferral";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingDeferral> for ::windows::core::IUnknown {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingDeferral> for ::windows::core::IUnknown {
    fn from(value: &SuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingDeferral> for &::windows::core::IUnknown {
    fn from(value: &SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingDeferral> for ::windows::core::IInspectable {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingDeferral> for ::windows::core::IInspectable {
    fn from(value: &SuspendingDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingDeferral> for &::windows::core::IInspectable {
    fn from(value: &SuspendingDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<SuspendingDeferral> for super::super::ApplicationModel::ISuspendingDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: SuspendingDeferral) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&SuspendingDeferral> for super::super::ApplicationModel::ISuspendingDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingDeferral) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::core::convert::TryFrom<&SuspendingDeferral> for ::windows::core::InParam<'a, super::super::ApplicationModel::ISuspendingDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingDeferral) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel\"`*"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct SuspendingEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventArgs {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn SuspendingOperation(&self) -> ::windows::core::Result<super::super::ApplicationModel::SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SuspendingOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::SuspendingOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for SuspendingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for SuspendingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for SuspendingEventArgs {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for SuspendingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::RuntimeType for SuspendingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.SuspendingEventArgs;{96061c05-2dba-4d08-b0bd-2b30a131c6aa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::Interface for SuspendingEventArgs {
    type Vtable = super::super::ApplicationModel::ISuspendingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::ISuspendingEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::core::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingEventArgs> for ::windows::core::IUnknown {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingEventArgs> for &::windows::core::IUnknown {
    fn from(value: &SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingEventArgs> for ::windows::core::IInspectable {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingEventArgs> for &::windows::core::IInspectable {
    fn from(value: &SuspendingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<SuspendingEventArgs> for super::super::ApplicationModel::ISuspendingEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: SuspendingEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&SuspendingEventArgs> for super::super::ApplicationModel::ISuspendingEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::core::convert::TryFrom<&SuspendingEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::ISuspendingEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel\"`*"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct SuspendingEventHandler(pub ::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::ISuspendingEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SuspendingEventHandlerBox::<F> { vtable: &SuspendingEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<'a, P0, P1, E1>(&self, sender: P0, e: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::ApplicationModel::ISuspendingEventArgs>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), sender.into().abi(), e.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct SuspendingEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::ISuspendingEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SuspendingEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<super::super::ApplicationModel::ISuspendingEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SuspendingEventHandlerBox<F> {
    const VTABLE: SuspendingEventHandler_Vtbl = SuspendingEventHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<SuspendingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for SuspendingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for SuspendingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for SuspendingEventHandler {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for SuspendingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingEventHandler").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::Interface for SuspendingEventHandler {
    type Vtable = SuspendingEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x509c429c_78e2_4883_abc8_8960dcde1b5c);
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::RuntimeType for SuspendingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{509c429c-78e2-4883-abc8-8960dcde1b5c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct SuspendingEventHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "ApplicationModel")]
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "ApplicationModel"))]
    Invoke: usize,
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel\"`*"]
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
pub struct SuspendingOperation(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingOperation {
    #[doc = "*Required features: `\"ApplicationModel\"`*"]
    #[cfg(feature = "ApplicationModel")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::ApplicationModel::SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::SuspendingDeferral>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn Deadline(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Deadline)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::clone::Clone for SuspendingOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::PartialEq for SuspendingOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::cmp::Eq for SuspendingOperation {}
#[cfg(feature = "ApplicationModel")]
impl ::core::fmt::Debug for SuspendingOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SuspendingOperation").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::RuntimeType for SuspendingOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.SuspendingOperation;{9da4ca41-20e1-4e9b-9f65-a9f435340c3a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::core::Interface for SuspendingOperation {
    type Vtable = super::super::ApplicationModel::ISuspendingOperation_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::ISuspendingOperation as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::core::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingOperation";
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingOperation> for ::windows::core::IUnknown {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingOperation> for ::windows::core::IUnknown {
    fn from(value: &SuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingOperation> for &::windows::core::IUnknown {
    fn from(value: &SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<SuspendingOperation> for ::windows::core::IInspectable {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingOperation> for ::windows::core::IInspectable {
    fn from(value: &SuspendingOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::From<&SuspendingOperation> for &::windows::core::IInspectable {
    fn from(value: &SuspendingOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<SuspendingOperation> for super::super::ApplicationModel::ISuspendingOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: SuspendingOperation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::core::convert::TryFrom<&SuspendingOperation> for super::super::ApplicationModel::ISuspendingOperation {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingOperation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::core::convert::TryFrom<&SuspendingOperation> for ::windows::core::InParam<'a, super::super::ApplicationModel::ISuspendingOperation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &SuspendingOperation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
pub struct WebUIApplication;
impl WebUIApplication {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ActivatedEventHandler>>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Activated)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveActivated)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn Suspending<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, SuspendingEventHandler>>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Suspending)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveSuspending(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveSuspending)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Resuming<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ResumingEventHandler>>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Resuming)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveResuming(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveResuming)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Navigated<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, NavigatedEventHandler>>,
    {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Navigated)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigated(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveNavigated)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn LeavingBackground<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, LeavingBackgroundEventHandler>>,
    {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LeavingBackground)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveLeavingBackground(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveLeavingBackground)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn EnteredBackground<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, EnteredBackgroundEventHandler>>,
    {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnteredBackground)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnteredBackground(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveEnteredBackground)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    pub fn EnablePrelaunch(value: bool) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe { (::windows::core::Interface::vtable(this).EnablePrelaunch)(::windows::core::Interface::as_raw(this), value).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestRestartAsync(launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>> {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestRestartAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(launcharguments), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>(result__)
        })
    }
    #[doc = "*Required features: `\"ApplicationModel_Core\"`, `\"Foundation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation", feature = "System"))]
    pub fn RequestRestartForUserAsync<'a, P0>(user: P0, launcharguments: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::System::User>>,
    {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RequestRestartForUserAsync)(::windows::core::Interface::as_raw(this), user.into().abi(), ::core::mem::transmute_copy(launcharguments), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::Core::AppRestartFailureReason>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn NewWebUIViewCreated<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs>>>,
    {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NewWebUIViewCreated)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveNewWebUIViewCreated(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveNewWebUIViewCreated)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated<'a, P0>(handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, BackgroundActivatedEventHandler>>,
    {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BackgroundActivated)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated(token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe { (::windows::core::Interface::vtable(this).RemoveBackgroundActivated)(::windows::core::Interface::as_raw(this), token).ok() })
    }
    #[doc(hidden)]
    pub fn IWebUIActivationStatics<R, F: FnOnce(&IWebUIActivationStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUIApplication, IWebUIActivationStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebUIActivationStatics2<R, F: FnOnce(&IWebUIActivationStatics2) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUIApplication, IWebUIActivationStatics2> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebUIActivationStatics3<R, F: FnOnce(&IWebUIActivationStatics3) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUIApplication, IWebUIActivationStatics3> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
    #[doc(hidden)]
    pub fn IWebUIActivationStatics4<R, F: FnOnce(&IWebUIActivationStatics4) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUIApplication, IWebUIActivationStatics4> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for WebUIApplication {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIApplication";
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderAddAppointmentActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn AddAppointmentOperation(&self) -> ::windows::core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::AddAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AddAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Appointments::AppointmentsProvider::AddAppointmentOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderAddAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderAddAppointmentActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn RemoveAppointmentOperation(&self) -> ::windows::core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::RemoveAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RemoveAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Appointments::AppointmentsProvider::RemoveAppointmentOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderRemoveAppointmentActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Appointments_AppointmentsProvider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Appointments_AppointmentsProvider"))]
    pub fn ReplaceAppointmentOperation(&self) -> ::windows::core::Result<super::super::ApplicationModel::Appointments::AppointmentsProvider::ReplaceAppointmentOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ReplaceAppointmentOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Appointments::AppointmentsProvider::ReplaceAppointmentOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderReplaceAppointmentActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn InstanceStartDate(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InstanceStartDate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn LocalId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LocalId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn RoamingId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RoamingId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn TimeToShow(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TimeToShow)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Duration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IAppointmentsProviderShowTimeFrameActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
pub struct WebUIBackgroundTaskInstance;
impl WebUIBackgroundTaskInstance {
    pub fn Current() -> ::windows::core::Result<IWebUIBackgroundTaskInstance> {
        Self::IWebUIBackgroundTaskInstanceStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Current)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IWebUIBackgroundTaskInstance>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IWebUIBackgroundTaskInstanceStatics<R, F: FnOnce(&IWebUIBackgroundTaskInstanceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUIBackgroundTaskInstance, IWebUIBackgroundTaskInstanceStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows::core::RuntimeName for WebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstance";
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct WebUIBackgroundTaskInstanceRuntimeClass(::windows::core::IUnknown);
impl WebUIBackgroundTaskInstanceRuntimeClass {
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn InstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InstanceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Task(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::BackgroundTaskRegistration> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Task)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Background::BackgroundTaskRegistration>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Progress(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Progress)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn SetProgress(&self, value: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetProgress)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TriggerDetails(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TriggerDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation"))]
    pub fn Canceled<'a, P0>(&self, cancelhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::ApplicationModel::Background::BackgroundTaskCanceledEventHandler>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Canceled)(::windows::core::Interface::as_raw(this), cancelhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation"))]
    pub fn RemoveCanceled(&self, cookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCanceled)(::windows::core::Interface::as_raw(this), cookie).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn SuspendedCount(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SuspendedCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Background\"`*"]
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn GetDeferral(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::BackgroundTaskDeferral> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Background::BackgroundTaskDeferral>(result__)
        }
    }
    pub fn Succeeded(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Succeeded)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSucceeded)(::windows::core::Interface::as_raw(this), succeeded).ok() }
    }
}
impl ::core::clone::Clone for WebUIBackgroundTaskInstanceRuntimeClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUIBackgroundTaskInstanceRuntimeClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUIBackgroundTaskInstanceRuntimeClass {}
impl ::core::fmt::Debug for WebUIBackgroundTaskInstanceRuntimeClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIBackgroundTaskInstanceRuntimeClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUIBackgroundTaskInstanceRuntimeClass {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass;{23f12c25-e2f7-4741-bc9c-394595de24dc})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebUIBackgroundTaskInstanceRuntimeClass {
    type Vtable = IWebUIBackgroundTaskInstance_Vtbl;
    const IID: ::windows::core::GUID = <IWebUIBackgroundTaskInstance as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUIBackgroundTaskInstanceRuntimeClass {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass";
}
impl ::core::convert::From<WebUIBackgroundTaskInstanceRuntimeClass> for ::windows::core::IUnknown {
    fn from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass> for ::windows::core::IUnknown {
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass> for &::windows::core::IUnknown {
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebUIBackgroundTaskInstanceRuntimeClass> for ::windows::core::IInspectable {
    fn from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass> for ::windows::core::IInspectable {
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass> for &::windows::core::IInspectable {
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::TryFrom<WebUIBackgroundTaskInstanceRuntimeClass> for super::super::ApplicationModel::Background::IBackgroundTaskInstance {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::core::convert::TryFrom<&WebUIBackgroundTaskInstanceRuntimeClass> for super::super::ApplicationModel::Background::IBackgroundTaskInstance {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a> ::core::convert::TryFrom<&WebUIBackgroundTaskInstanceRuntimeClass> for ::windows::core::InParam<'a, super::super::ApplicationModel::Background::IBackgroundTaskInstance> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<WebUIBackgroundTaskInstanceRuntimeClass> for IWebUIBackgroundTaskInstance {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUIBackgroundTaskInstanceRuntimeClass> for IWebUIBackgroundTaskInstance {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebUIBackgroundTaskInstanceRuntimeClass> for ::windows::core::InParam<'a, IWebUIBackgroundTaskInstance> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIBarcodeScannerPreviewActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIBarcodeScannerPreviewActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ConnectionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ConnectionId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIBarcodeScannerPreviewActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIBarcodeScannerPreviewActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIBarcodeScannerPreviewActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs> for super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUICachedFileUpdaterActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICachedFileUpdaterActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Provider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Provider"))]
    pub fn CachedFileUpdaterUI(&self) -> ::windows::core::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CachedFileUpdaterUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUICachedFileUpdaterActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUICachedFileUpdaterActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUICachedFileUpdaterActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUICachedFileUpdaterActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICachedFileUpdaterActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUICachedFileUpdaterActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICachedFileUpdaterActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICachedFileUpdaterActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICachedFileUpdaterActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICachedFileUpdaterActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs> for super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUICameraSettingsActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICameraSettingsActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn VideoDeviceController(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceController)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn VideoDeviceExtension(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoDeviceExtension)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUICameraSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUICameraSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUICameraSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUICameraSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICameraSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUICameraSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUICameraSettingsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICameraSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICameraSettingsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICameraSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICameraSettingsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICameraSettingsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICameraSettingsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICameraSettingsActivatedEventArgs> for super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUICommandLineActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICommandLineActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::CommandLineActivationOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::CommandLineActivationOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUICommandLineActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUICommandLineActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUICommandLineActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUICommandLineActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUICommandLineActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUICommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUICommandLineActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICommandLineActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICommandLineActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICommandLineActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUICommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICommandLineActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUICommandLineActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICommandLineActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICommandLineActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICommandLineActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUICommandLineActivatedEventArgs> for super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUICommandLineActivatedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactCallActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactCallActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactCallActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIContactCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIContactCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactCallActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactCallActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactMapActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMapActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Address(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Address)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::ContactAddress>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactMapActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactMapActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactMapActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactMapActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactMapActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIContactMapActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIContactMapActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMapActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMapActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMapActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMapActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMapActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMapActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMapActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMapActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactMessageActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMessageActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactMessageActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactMessageActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactMessageActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactMessageActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactMessageActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIContactMessageActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIContactMessageActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMessageActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMessageActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMessageActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactMessageActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMessageActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMessageActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMessageActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactMessageActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactMessageActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactPanelActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPanelActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn ContactPanel(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContactPanel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::ContactPanel>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactPanelActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactPanelActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactPanelActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactPanelActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIContactPanelActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIContactPanelActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPanelActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPanelActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPanelActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPanelActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIContactPanelActivatedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactPickerActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPickerActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts_Provider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts_Provider"))]
    pub fn ContactPickerUI(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::Provider::ContactPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContactPickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::Provider::ContactPickerUI>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactPickerActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIContactPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIContactPickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPickerActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPickerActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactPostActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPostActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactPostActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactPostActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactPostActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactPostActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactPostActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIContactPostActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIContactPostActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPostActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPostActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPostActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactPostActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPostActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPostActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPostActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactPostActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIContactVideoCallActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactVideoCallActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContactActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ServiceUserId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Contacts\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Contacts"))]
    pub fn Contact(&self) -> ::windows::core::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Contact)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIContactVideoCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIContactVideoCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIContactVideoCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIContactVideoCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIContactVideoCallActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIContactVideoCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIContactVideoCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactVideoCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactVideoCallActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactVideoCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIContactVideoCallActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIContactVideoCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIDeviceActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDeviceActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn DeviceInformationId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformationId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIDeviceActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIDeviceActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIDeviceActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIDeviceActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIDeviceActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIDeviceActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIDeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIDeviceActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDeviceActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDeviceActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDeviceActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDeviceActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDeviceActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDeviceActivatedEventArgs> for super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIDevicePairingActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDevicePairingActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Enumeration\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Enumeration"))]
    pub fn DeviceInformation(&self) -> ::windows::core::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeviceInformation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIDevicePairingActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIDevicePairingActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIDevicePairingActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIDevicePairingActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIDevicePairingActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIDevicePairingActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIDevicePairingActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDevicePairingActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDevicePairingActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDevicePairingActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDevicePairingActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIDialReceiverActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDialReceiverActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn AppName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AppName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIDialReceiverActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIDialReceiverActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIDialReceiverActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIDialReceiverActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIDialReceiverActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIDialReceiverActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIDialReceiverActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDialReceiverActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDialReceiverActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIDialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDialReceiverActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIDialReceiverActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIFileActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "Storage"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Verb)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Search"))]
    pub fn NeighboringFilesQuery(&self) -> ::windows::core::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NeighboringFilesQuery)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIFileActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIFileActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIFileActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIFileActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIFileActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIFileActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IFileActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIFileActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFileActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIFileOpenPickerActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileOpenPickerActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Pickers_Provider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Pickers_Provider"))]
    pub fn FileOpenPickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileOpenPickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIFileOpenPickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIFileOpenPickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIFileOpenPickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIFileOpenPickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileOpenPickerActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIFileOpenPickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIFileOpenPickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileOpenPickerActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileOpenPickerActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
pub struct WebUIFileOpenPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFileOpenPickerContinuationEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections", feature = "Storage", feature = "deprecated"))]
    pub fn Files(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Files)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>>(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::clone::Clone for WebUIFileOpenPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for WebUIFileOpenPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::Eq for WebUIFileOpenPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::fmt::Debug for WebUIFileOpenPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileOpenPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows::core::RuntimeType for WebUIFileOpenPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for WebUIFileOpenPickerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for WebUIFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileOpenPickerContinuationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileOpenPickerContinuationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileOpenPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIFileSavePickerActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileSavePickerActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage_Pickers_Provider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Pickers_Provider"))]
    pub fn FileSavePickerUI(&self) -> ::windows::core::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileSavePickerUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn EnterpriseId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).EnterpriseId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIFileSavePickerActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIFileSavePickerActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIFileSavePickerActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIFileSavePickerActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileSavePickerActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIFileSavePickerActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIFileSavePickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileSavePickerActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileSavePickerActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIFileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileSavePickerActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIFileSavePickerActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
pub struct WebUIFileSavePickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFileSavePickerContinuationEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage", feature = "deprecated"))]
    pub fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).File)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::clone::Clone for WebUIFileSavePickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for WebUIFileSavePickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::Eq for WebUIFileSavePickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::fmt::Debug for WebUIFileSavePickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFileSavePickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows::core::RuntimeType for WebUIFileSavePickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for WebUIFileSavePickerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for WebUIFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileSavePickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileSavePickerContinuationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileSavePickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFileSavePickerContinuationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFileSavePickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`, `\"deprecated\"`*"]
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
#[repr(transparent)]
pub struct WebUIFolderPickerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl WebUIFolderPickerContinuationEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Storage\"`, `\"deprecated\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage", feature = "deprecated"))]
    pub fn Folder(&self) -> ::windows::core::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Folder)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::clone::Clone for WebUIFolderPickerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::PartialEq for WebUIFolderPickerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::cmp::Eq for WebUIFolderPickerContinuationEventArgs {}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::fmt::Debug for WebUIFolderPickerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIFolderPickerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows::core::RuntimeType for WebUIFolderPickerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
unsafe impl ::windows::core::Interface for WebUIFolderPickerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::windows::core::RuntimeName for WebUIFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs";
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFolderPickerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFolderPickerContinuationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<WebUIFolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFolderPickerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::From<&WebUIFolderPickerContinuationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<WebUIFolderPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(all(feature = "ApplicationModel_Activation", feature = "deprecated"))]
impl<'a> ::core::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIFolderPickerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUILaunchActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILaunchActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileActivatedInfo(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::TileActivatedInfo> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileActivatedInfo)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::TileActivatedInfo>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PrelaunchActivated(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PrelaunchActivated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUILaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUILaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUILaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUILaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUILaunchActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUILaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUILaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUILaunchActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILaunchActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILaunchActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUILockScreenActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Info(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Info)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUILockScreenActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUILockScreenActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUILockScreenActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUILockScreenActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUILockScreenActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUILockScreenActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUILockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUILockScreenActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenActivatedEventArgs> for super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUILockScreenCallActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenCallActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Arguments)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TileId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Calls\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Calls"))]
    pub fn CallUI(&self) -> ::windows::core::Result<super::super::ApplicationModel::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallUI)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Calls::LockScreenCallUI>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUILockScreenCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUILockScreenCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUILockScreenCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUILockScreenCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUILockScreenCallActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUILockScreenCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUILockScreenCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenCallActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenCallActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenCallActivatedEventArgs> for super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUILockScreenComponentActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenComponentActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUILockScreenComponentActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUILockScreenComponentActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUILockScreenComponentActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUILockScreenComponentActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUILockScreenComponentActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUILockScreenComponentActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUILockScreenComponentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUILockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenComponentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUILockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenComponentActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenComponentActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUILockScreenComponentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUILockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenComponentActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUILockScreenComponentActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenComponentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenComponentActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenComponentActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUILockScreenComponentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUILockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUILockScreenComponentActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUILockScreenComponentActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUILockScreenComponentActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct WebUINavigatedDeferral(::windows::core::IUnknown);
impl WebUINavigatedDeferral {
    pub fn Complete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Complete)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::clone::Clone for WebUINavigatedDeferral {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUINavigatedDeferral {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUINavigatedDeferral {}
impl ::core::fmt::Debug for WebUINavigatedDeferral {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUINavigatedDeferral").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUINavigatedDeferral {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUINavigatedDeferral;{d804204d-831f-46e2-b432-3afce211f962})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebUINavigatedDeferral {
    type Vtable = IWebUINavigatedDeferral_Vtbl;
    const IID: ::windows::core::GUID = <IWebUINavigatedDeferral as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUINavigatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedDeferral";
}
impl ::core::convert::From<WebUINavigatedDeferral> for ::windows::core::IUnknown {
    fn from(value: WebUINavigatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedDeferral> for ::windows::core::IUnknown {
    fn from(value: &WebUINavigatedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUINavigatedDeferral> for &::windows::core::IUnknown {
    fn from(value: &WebUINavigatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebUINavigatedDeferral> for ::windows::core::IInspectable {
    fn from(value: WebUINavigatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedDeferral> for ::windows::core::IInspectable {
    fn from(value: &WebUINavigatedDeferral) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUINavigatedDeferral> for &::windows::core::IInspectable {
    fn from(value: &WebUINavigatedDeferral) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct WebUINavigatedEventArgs(::windows::core::IUnknown);
impl WebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> ::windows::core::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NavigatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebUINavigatedOperation>(result__)
        }
    }
}
impl ::core::clone::Clone for WebUINavigatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUINavigatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUINavigatedEventArgs {}
impl ::core::fmt::Debug for WebUINavigatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUINavigatedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUINavigatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUINavigatedEventArgs;{a75841b8-2499-4030-a69d-15d2d9cfe524})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebUINavigatedEventArgs {
    type Vtable = IWebUINavigatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IWebUINavigatedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedEventArgs";
}
impl ::core::convert::From<WebUINavigatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUINavigatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebUINavigatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUINavigatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<WebUINavigatedEventArgs> for IWebUINavigatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUINavigatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&WebUINavigatedEventArgs> for IWebUINavigatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUINavigatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&WebUINavigatedEventArgs> for ::windows::core::InParam<'a, IWebUINavigatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUINavigatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct WebUINavigatedOperation(::windows::core::IUnknown);
impl WebUINavigatedOperation {
    pub fn GetDeferral(&self) -> ::windows::core::Result<WebUINavigatedDeferral> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetDeferral)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<WebUINavigatedDeferral>(result__)
        }
    }
}
impl ::core::clone::Clone for WebUINavigatedOperation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUINavigatedOperation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUINavigatedOperation {}
impl ::core::fmt::Debug for WebUINavigatedOperation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUINavigatedOperation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUINavigatedOperation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUINavigatedOperation;{7a965f08-8182-4a89-ab67-8492e8750d4b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebUINavigatedOperation {
    type Vtable = IWebUINavigatedOperation_Vtbl;
    const IID: ::windows::core::GUID = <IWebUINavigatedOperation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUINavigatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedOperation";
}
impl ::core::convert::From<WebUINavigatedOperation> for ::windows::core::IUnknown {
    fn from(value: WebUINavigatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedOperation> for ::windows::core::IUnknown {
    fn from(value: &WebUINavigatedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUINavigatedOperation> for &::windows::core::IUnknown {
    fn from(value: &WebUINavigatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebUINavigatedOperation> for ::windows::core::IInspectable {
    fn from(value: WebUINavigatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUINavigatedOperation> for ::windows::core::IInspectable {
    fn from(value: &WebUINavigatedOperation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUINavigatedOperation> for &::windows::core::IInspectable {
    fn from(value: &WebUINavigatedOperation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIPhoneCallActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPhoneCallActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn LineId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LineId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::GUID>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIPhoneCallActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIPhoneCallActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIPhoneCallActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIPhoneCallActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIPhoneCallActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIPhoneCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPhoneCallActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPhoneCallActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPhoneCallActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPhoneCallActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIPhoneCallActivatedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIPrint3DWorkflowActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrint3DWorkflowActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Printers_Extensions\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Printers_Extensions"))]
    pub fn Workflow(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Workflow)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIPrint3DWorkflowActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIPrint3DWorkflowActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIPrint3DWorkflowActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIPrint3DWorkflowActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIPrint3DWorkflowActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIPrint3DWorkflowActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrint3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrint3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrint3DWorkflowActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrint3DWorkflowActivatedEventArgs> for super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIPrintTaskSettingsActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintTaskSettingsActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Devices_Printers_Extensions\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Devices_Printers_Extensions"))]
    pub fn Configuration(&self) -> ::windows::core::Result<super::super::Devices::Printers::Extensions::PrintTaskConfiguration> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Configuration)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIPrintTaskSettingsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIPrintTaskSettingsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIPrintTaskSettingsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIPrintTaskSettingsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIPrintTaskSettingsActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIPrintTaskSettingsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintTaskSettingsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintTaskSettingsActivatedEventArgs> for super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIPrintWorkflowForegroundTaskActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIPrintWorkflowForegroundTaskActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIProtocolActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIProtocolActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIProtocolActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIProtocolActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIProtocolActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIProtocolActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIProtocolActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIProtocolActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIProtocolForResultsActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolForResultsActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Uri)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CallerPackageFamilyName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn Data(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Data)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn ProtocolForResultsOperation(&self) -> ::windows::core::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ProtocolForResultsOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIProtocolForResultsActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIProtocolForResultsActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIProtocolForResultsActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIProtocolForResultsActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIProtocolForResultsActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIProtocolForResultsActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIProtocolForResultsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolForResultsActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIProtocolForResultsActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIProtocolForResultsActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIRestrictedLaunchActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIRestrictedLaunchActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SharedContext(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SharedContext)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIRestrictedLaunchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIRestrictedLaunchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIRestrictedLaunchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIRestrictedLaunchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIRestrictedLaunchActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIRestrictedLaunchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIRestrictedLaunchActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIRestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIRestrictedLaunchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIRestrictedLaunchActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIRestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIRestrictedLaunchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIRestrictedLaunchActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUISearchActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUISearchActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CurrentlyShownApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn QueryText(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).QueryText)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Language)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Search\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Search"))]
    pub fn LinguisticDetails(&self) -> ::windows::core::Result<super::super::ApplicationModel::Search::SearchPaneQueryLinguisticDetails> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LinguisticDetails)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Search::SearchPaneQueryLinguisticDetails>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUISearchActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUISearchActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUISearchActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUISearchActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUISearchActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUISearchActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUISearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUISearchActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ISearchActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::ISearchActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUISearchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUISearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUISearchActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUISearchActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUISearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUISearchActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUISearchActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for super::super::ApplicationModel::Activation::ISearchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for super::super::ApplicationModel::Activation::ISearchActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ISearchActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUISearchActivatedEventArgs> for super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUISearchActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIShareTargetActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIShareTargetActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_DataTransfer_ShareTarget\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_DataTransfer_ShareTarget"))]
    pub fn ShareOperation(&self) -> ::windows::core::Result<super::super::ApplicationModel::DataTransfer::ShareTarget::ShareOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ShareOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::DataTransfer::ShareTarget::ShareOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIShareTargetActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIShareTargetActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIShareTargetActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIShareTargetActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIShareTargetActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIShareTargetActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIShareTargetActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIShareTargetActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIShareTargetActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIShareTargetActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIShareTargetActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIStartupTaskActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIStartupTaskActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TaskId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TaskId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIStartupTaskActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIStartupTaskActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIStartupTaskActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIStartupTaskActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIStartupTaskActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIStartupTaskActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIStartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIStartupTaskActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIStartupTaskActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIStartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIStartupTaskActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIStartupTaskActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Send for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::core::marker::Sync for WebUIStartupTaskActivatedEventArgs {}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIToastNotificationActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIToastNotificationActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Argument(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Argument)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn UserInput(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UserInput)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIToastNotificationActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIToastNotificationActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIToastNotificationActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIToastNotificationActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIToastNotificationActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIToastNotificationActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIToastNotificationActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIToastNotificationActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIToastNotificationActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIToastNotificationActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIToastNotificationActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIToastNotificationActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIToastNotificationActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIToastNotificationActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIToastNotificationActivatedEventArgs> for super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIToastNotificationActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIUserDataAccountProviderActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIUserDataAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_UserDataAccounts_Provider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_UserDataAccounts_Provider"))]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::UserDataAccounts::Provider::IUserDataAccountProviderOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIUserDataAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIUserDataAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIUserDataAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIUserDataAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIUserDataAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIUserDataAccountProviderActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIUserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIUserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIUserDataAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIUserDataAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`*"]
#[repr(transparent)]
pub struct WebUIView(::windows::core::IUnknown);
impl WebUIView {
    pub fn ApplicationViewId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ApplicationViewId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Closed<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<WebUIView, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Closed)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveClosed)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<WebUIView, super::super::ApplicationModel::Activation::IActivatedEventArgs>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Activated)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveActivated)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    pub fn IgnoreApplicationContentUriRulesNavigationRestrictions(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IgnoreApplicationContentUriRulesNavigationRestrictions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreApplicationContentUriRulesNavigationRestrictions(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIgnoreApplicationContentUriRulesNavigationRestrictions)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync() -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WebUIView>> {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<WebUIView>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWithUriAsync<'a, P0>(uri: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<WebUIView>>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Uri>>,
    {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithUriAsync)(::windows::core::Interface::as_raw(this), uri.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<WebUIView>>(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn Source(&self) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn SetSource<'a, P0>(&self, source: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::windows::core::Interface::as_raw(this), source.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn DocumentTitle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DocumentTitle)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn CanGoBack(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CanGoBack)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn CanGoForward(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CanGoForward)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn SetDefaultBackgroundColor(&self, value: super::Color) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetDefaultBackgroundColor)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows::core::Result<super::Color> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DefaultBackgroundColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn ContainsFullScreenElement(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContainsFullScreenElement)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn Settings(&self) -> ::windows::core::Result<super::super::Web::UI::WebViewControlSettings> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Settings)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Web::UI::WebViewControlSettings>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_UI"))]
    pub fn DeferredPermissionRequests(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::super::Web::UI::WebViewControlDeferredPermissionRequest>> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeferredPermissionRequests)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::IVectorView<super::super::Web::UI::WebViewControlDeferredPermissionRequest>>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn GoForward(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GoForward)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn GoBack(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GoBack)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn Refresh(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Refresh)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn Stop(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Stop)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn Navigate<'a, P0>(&self, source: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Uri>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Navigate)(::windows::core::Interface::as_raw(this), source.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn NavigateToString(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).NavigateToString)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(text)).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NavigateToLocalStreamUri<'a, P0, P1, E1>(&self, source: P0, streamresolver: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::Uri>>,
        P1: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Web::IUriToStreamResolver>, Error = E1>,
        E1: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).NavigateToLocalStreamUri)(::windows::core::Interface::as_raw(this), source.into().abi(), streamresolver.try_into().map_err(|e| e.into())?.abi()).ok() }
    }
    #[doc = "*Required features: `\"Web_Http\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Web_Http", feature = "Web_UI"))]
    pub fn NavigateWithHttpRequestMessage<'a, P0>(&self, requestmessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Web::Http::HttpRequestMessage>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).NavigateWithHttpRequestMessage)(::windows::core::Interface::as_raw(this), requestmessage.into().abi()).ok() }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_UI"))]
    pub fn InvokeScriptAsync<'a, P0, E0>(&self, scriptname: &::windows::core::HSTRING, arguments: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).InvokeScriptAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(scriptname), arguments.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<::windows::core::HSTRING>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams", feature = "Web_UI"))]
    pub fn CapturePreviewToStreamAsync<'a, P0, E0>(&self, stream: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::super::Storage::Streams::IRandomAccessStream>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CapturePreviewToStreamAsync)(::windows::core::Interface::as_raw(this), stream.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_DataTransfer\"`, `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "ApplicationModel_DataTransfer", feature = "Foundation", feature = "Web_UI"))]
    pub fn CaptureSelectedContentToDataPackageAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CaptureSelectedContentToDataPackageAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::ApplicationModel::DataTransfer::DataPackage>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn BuildLocalStreamUri(&self, contentidentifier: &::windows::core::HSTRING, relativepath: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::Uri> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BuildLocalStreamUri)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(contentidentifier), ::core::mem::transmute_copy(relativepath), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn GetDeferredPermissionRequestById(&self, id: u32, result: &mut ::core::option::Option<super::super::Web::UI::WebViewControlDeferredPermissionRequest>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetDeferredPermissionRequestById)(::windows::core::Interface::as_raw(this), id, result as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NavigationStarting<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationStartingEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NavigationStarting)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNavigationStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNavigationStarting)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ContentLoading<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlContentLoadingEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContentLoading)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveContentLoading(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContentLoading)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn DOMContentLoaded<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlDOMContentLoadedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DOMContentLoaded)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveDOMContentLoaded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveDOMContentLoaded)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NavigationCompleted<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationCompletedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NavigationCompleted)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNavigationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNavigationCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameNavigationStarting<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationStartingEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameNavigationStarting)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameNavigationStarting(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameNavigationStarting)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameContentLoading<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlContentLoadingEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameContentLoading)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameContentLoading(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameContentLoading)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameDOMContentLoaded<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlDOMContentLoadedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameDOMContentLoaded)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameDOMContentLoaded(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameDOMContentLoaded)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameNavigationCompleted<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNavigationCompletedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FrameNavigationCompleted)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameNavigationCompleted(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFrameNavigationCompleted)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ScriptNotify<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlScriptNotifyEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ScriptNotify)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveScriptNotify(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveScriptNotify)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn LongRunningScriptDetected<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlLongRunningScriptDetectedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).LongRunningScriptDetected)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveLongRunningScriptDetected(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveLongRunningScriptDetected)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnsafeContentWarningDisplaying<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, ::windows::core::IInspectable>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UnsafeContentWarningDisplaying)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnsafeContentWarningDisplaying(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnsafeContentWarningDisplaying)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnviewableContentIdentified<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlUnviewableContentIdentifiedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UnviewableContentIdentified)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnviewableContentIdentified(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnviewableContentIdentified)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn PermissionRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlPermissionRequestedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PermissionRequested)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemovePermissionRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemovePermissionRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnsupportedUriSchemeIdentified<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).UnsupportedUriSchemeIdentified)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnsupportedUriSchemeIdentified(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveUnsupportedUriSchemeIdentified)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NewWindowRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlNewWindowRequestedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).NewWindowRequested)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNewWindowRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveNewWindowRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ContainsFullScreenElementChanged<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, ::windows::core::IInspectable>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContainsFullScreenElementChanged)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveContainsFullScreenElementChanged(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveContainsFullScreenElementChanged)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn WebResourceRequested<'a, P0>(&self, handler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<super::super::Web::UI::IWebViewControl, super::super::Web::UI::WebViewControlWebResourceRequestedEventArgs>>>,
    {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WebResourceRequested)(::windows::core::Interface::as_raw(this), handler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Web_UI\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveWebResourceRequested(&self, token: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveWebResourceRequested)(::windows::core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Web_UI\"`*"]
    #[cfg(feature = "Web_UI")]
    pub fn AddInitializeScript(&self, script: &::windows::core::HSTRING) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::Web::UI::IWebViewControl2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddInitializeScript)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(script)).ok() }
    }
    #[doc(hidden)]
    pub fn IWebUIViewStatics<R, F: FnOnce(&IWebUIViewStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<WebUIView, IWebUIViewStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for WebUIView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for WebUIView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for WebUIView {}
impl ::core::fmt::Debug for WebUIView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for WebUIView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIView;{6783f64f-52da-4fd7-be69-8ef6284b423c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for WebUIView {
    type Vtable = IWebUIView_Vtbl;
    const IID: ::windows::core::GUID = <IWebUIView as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for WebUIView {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIView";
}
impl ::core::convert::From<WebUIView> for ::windows::core::IUnknown {
    fn from(value: WebUIView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUIView> for ::windows::core::IUnknown {
    fn from(value: &WebUIView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUIView> for &::windows::core::IUnknown {
    fn from(value: &WebUIView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<WebUIView> for ::windows::core::IInspectable {
    fn from(value: WebUIView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&WebUIView> for ::windows::core::IInspectable {
    fn from(value: &WebUIView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&WebUIView> for &::windows::core::IInspectable {
    fn from(value: &WebUIView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Web_UI")]
impl ::core::convert::TryFrom<WebUIView> for super::super::Web::UI::IWebViewControl {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_UI")]
impl ::core::convert::TryFrom<&WebUIView> for super::super::Web::UI::IWebViewControl {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::core::convert::TryFrom<&WebUIView> for ::windows::core::InParam<'a, super::super::Web::UI::IWebViewControl> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIView) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Web_UI")]
impl ::core::convert::TryFrom<WebUIView> for super::super::Web::UI::IWebViewControl2 {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_UI")]
impl ::core::convert::TryFrom<&WebUIView> for super::super::Web::UI::IWebViewControl2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::core::convert::TryFrom<&WebUIView> for ::windows::core::InParam<'a, super::super::Web::UI::IWebViewControl2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIView) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIVoiceCommandActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIVoiceCommandActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Media_SpeechRecognition\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Media_SpeechRecognition"))]
    pub fn Result(&self) -> ::windows::core::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Result)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIVoiceCommandActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIVoiceCommandActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIVoiceCommandActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIVoiceCommandActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIVoiceCommandActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIVoiceCommandActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIVoiceCommandActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIVoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIVoiceCommandActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIVoiceCommandActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIVoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIVoiceCommandActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIVoiceCommandActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIWalletActionActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWalletActionActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ItemId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ItemId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"ApplicationModel_Wallet\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "ApplicationModel_Wallet"))]
    pub fn ActionKind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActionKind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Wallet::WalletActionKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ActionId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActionId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIWalletActionActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIWalletActionActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIWalletActionActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIWalletActionActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIWalletActionActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIWalletActionActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIWalletActionActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWalletActionActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWalletActionActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWalletActionActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWalletActionActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWalletActionActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWalletActionActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWalletActionActivatedEventArgs> for super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIWebAccountProviderActivatedEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAccountProviderActivatedEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"System\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::core::Result<super::super::System::User> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).User)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::System::User>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Security_Authentication_Web_Provider\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Security_Authentication_Web_Provider"))]
    pub fn Operation(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Operation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIWebAccountProviderActivatedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIWebAccountProviderActivatedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIWebAccountProviderActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIWebAccountProviderActivatedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIWebAccountProviderActivatedEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIWebAccountProviderActivatedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIWebAccountProviderActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAccountProviderActivatedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAccountProviderActivatedEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAccountProviderActivatedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAccountProviderActivatedEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAccountProviderActivatedEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"UI_WebUI\"`, `\"ApplicationModel_Activation\"`*"]
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
pub struct WebUIWebAuthenticationBrokerContinuationEventArgs(::windows::core::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAuthenticationBrokerContinuationEventArgs {
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ActivationKind> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Kind)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::ApplicationExecutionState> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PreviousExecutionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`*"]
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(&self) -> ::windows::core::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).SplashScreen)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::core::Result<ActivatedOperation> {
        let this = &::windows::core::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ActivatedOperation)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ActivatedOperation>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Foundation_Collections\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation_Collections"))]
    pub fn ContinuationData(&self) -> ::windows::core::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::core::Interface::cast::<super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContinuationData)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[doc = "*Required features: `\"ApplicationModel_Activation\"`, `\"Security_Authentication_Web\"`*"]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Security_Authentication_Web"))]
    pub fn WebAuthenticationResult(&self) -> ::windows::core::Result<super::super::Security::Authentication::Web::WebAuthenticationResult> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).WebAuthenticationResult)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::clone::Clone for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::PartialEq for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::cmp::Eq for WebUIWebAuthenticationBrokerContinuationEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::fmt::Debug for WebUIWebAuthenticationBrokerContinuationEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WebUIWebAuthenticationBrokerContinuationEventArgs").field(&self.0).finish()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::RuntimeType for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::core::Interface for WebUIWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs as ::windows::core::Interface>::IID;
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IUnknown {
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs> for &::windows::core::IUnknown {
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::IInspectable {
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs> for &::windows::core::IInspectable {
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for super::super::ApplicationModel::Activation::IActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, IActivatedEventArgsDeferral> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs> for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs> for super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::core::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs> for ::windows::core::InParam<'a, super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs> {
    type Error = ::windows::core::Error;
    fn try_from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
