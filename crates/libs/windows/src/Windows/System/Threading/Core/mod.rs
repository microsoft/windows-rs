#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IPreallocatedWorkItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPreallocatedWorkItem {
    type Vtable = IPreallocatedWorkItem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6daa9fc_bc5b_401a_a8b2_6e754d14daa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreallocatedWorkItem_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub RunAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPreallocatedWorkItemFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPreallocatedWorkItemFactory {
    type Vtable = IPreallocatedWorkItemFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3d32b45_dfea_469b_82c5_f6e3cefdeafb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreallocatedWorkItemFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItem: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItemWithPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, priority: super::WorkItemPriority, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItemWithPriority: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItemWithPriorityAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, priority: super::WorkItemPriority, options: super::WorkItemOptions, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItemWithPriorityAndOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignalNotifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISignalNotifier {
    type Vtable = ISignalNotifier_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14285e06_63a7_4713_b6d9_62f64b56fb8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignalNotifier_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignalNotifierStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ISignalNotifierStatics {
    type Vtable = ISignalNotifierStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c4e4566_8400_46d3_a115_7d0c0dfc9f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignalNotifierStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub AttachToEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachToEventWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, timeout: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachToEventWithTimeout: usize,
    pub AttachToSemaphore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachToSemaphoreWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, handler: ::windows::core::RawPtr, timeout: super::super::super::Foundation::TimeSpan, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachToSemaphoreWithTimeout: usize,
}
#[doc = "*Required features: `\"System_Threading_Core\"`*"]
#[repr(transparent)]
pub struct PreallocatedWorkItem(::windows::core::IUnknown);
impl PreallocatedWorkItem {
    #[doc = "*Required features: `\"System_Threading_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RunAsync)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItem<'a, Param0: ::windows::core::IntoParam<'a, super::WorkItemHandler>>(handler: Param0) -> ::windows::core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWorkItem)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<PreallocatedWorkItem>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItemWithPriority<'a, Param0: ::windows::core::IntoParam<'a, super::WorkItemHandler>>(handler: Param0, priority: super::WorkItemPriority) -> ::windows::core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWorkItemWithPriority)(::core::mem::transmute_copy(this), handler.into_param().abi(), priority, &mut result__).from_abi::<PreallocatedWorkItem>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItemWithPriorityAndOptions<'a, Param0: ::windows::core::IntoParam<'a, super::WorkItemHandler>>(handler: Param0, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> ::windows::core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWorkItemWithPriorityAndOptions)(::core::mem::transmute_copy(this), handler.into_param().abi(), priority, options, &mut result__).from_abi::<PreallocatedWorkItem>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPreallocatedWorkItemFactory<R, F: FnOnce(&IPreallocatedWorkItemFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PreallocatedWorkItem, IPreallocatedWorkItemFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PreallocatedWorkItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PreallocatedWorkItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PreallocatedWorkItem {}
impl ::core::fmt::Debug for PreallocatedWorkItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreallocatedWorkItem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PreallocatedWorkItem {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Threading.Core.PreallocatedWorkItem;{b6daa9fc-bc5b-401a-a8b2-6e754d14daa6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PreallocatedWorkItem {
    type Vtable = IPreallocatedWorkItem_Vtbl;
    const IID: ::windows::core::GUID = <IPreallocatedWorkItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PreallocatedWorkItem {
    const NAME: &'static str = "Windows.System.Threading.Core.PreallocatedWorkItem";
}
impl ::core::convert::From<PreallocatedWorkItem> for ::windows::core::IUnknown {
    fn from(value: PreallocatedWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PreallocatedWorkItem> for ::windows::core::IUnknown {
    fn from(value: &PreallocatedWorkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PreallocatedWorkItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PreallocatedWorkItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PreallocatedWorkItem> for ::windows::core::IInspectable {
    fn from(value: PreallocatedWorkItem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PreallocatedWorkItem> for ::windows::core::IInspectable {
    fn from(value: &PreallocatedWorkItem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PreallocatedWorkItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PreallocatedWorkItem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PreallocatedWorkItem {}
unsafe impl ::core::marker::Sync for PreallocatedWorkItem {}
#[doc = "*Required features: `\"System_Threading_Core\"`*"]
#[repr(transparent)]
pub struct SignalHandler(pub ::windows::core::IUnknown);
impl SignalHandler {
    pub fn new<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SignalHandlerBox::<F> { vtable: &SignalHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, SignalNotifier>>(&self, signalnotifier: Param0, timedout: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), signalnotifier.into_param().abi(), timedout).ok() }
    }
}
#[repr(C)]
struct SignalHandlerBox<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SignalHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SignalHandlerBox<F> {
    const VTABLE: SignalHandler_Vtbl = SignalHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<SignalHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, signalnotifier: ::windows::core::RawPtr, timedout: bool) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&signalnotifier), timedout).into()
    }
}
impl ::core::clone::Clone for SignalHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SignalHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SignalHandler {}
impl ::core::fmt::Debug for SignalHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignalHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for SignalHandler {
    type Vtable = SignalHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x923c402e_4721_440e_9dda_55b6f2e07710);
}
unsafe impl ::windows::core::RuntimeType for SignalHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{923c402e-4721-440e-9dda-55b6f2e07710}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct SignalHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalnotifier: ::windows::core::RawPtr, timedout: bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_Threading_Core\"`*"]
#[repr(transparent)]
pub struct SignalNotifier(::windows::core::IUnknown);
impl SignalNotifier {
    #[doc = "*Required features: `\"System_Threading_Core\"`*"]
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Enable)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`*"]
    pub fn Terminate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Terminate)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`*"]
    pub fn AttachToEvent<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SignalHandler>>(name: Param0, handler: Param1) -> ::windows::core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttachToEvent)(::core::mem::transmute_copy(this), name.into_param().abi(), handler.into_param().abi(), &mut result__).from_abi::<SignalNotifier>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachToEventWithTimeout<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SignalHandler>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(name: Param0, handler: Param1, timeout: Param2) -> ::windows::core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttachToEventWithTimeout)(::core::mem::transmute_copy(this), name.into_param().abi(), handler.into_param().abi(), timeout.into_param().abi(), &mut result__).from_abi::<SignalNotifier>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`*"]
    pub fn AttachToSemaphore<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SignalHandler>>(name: Param0, handler: Param1) -> ::windows::core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttachToSemaphore)(::core::mem::transmute_copy(this), name.into_param().abi(), handler.into_param().abi(), &mut result__).from_abi::<SignalNotifier>(result__)
        })
    }
    #[doc = "*Required features: `\"System_Threading_Core\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachToSemaphoreWithTimeout<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, SignalHandler>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::TimeSpan>>(name: Param0, handler: Param1, timeout: Param2) -> ::windows::core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AttachToSemaphoreWithTimeout)(::core::mem::transmute_copy(this), name.into_param().abi(), handler.into_param().abi(), timeout.into_param().abi(), &mut result__).from_abi::<SignalNotifier>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISignalNotifierStatics<R, F: FnOnce(&ISignalNotifierStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<SignalNotifier, ISignalNotifierStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for SignalNotifier {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for SignalNotifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for SignalNotifier {}
impl ::core::fmt::Debug for SignalNotifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SignalNotifier").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for SignalNotifier {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.Threading.Core.SignalNotifier;{14285e06-63a7-4713-b6d9-62f64b56fb8b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for SignalNotifier {
    type Vtable = ISignalNotifier_Vtbl;
    const IID: ::windows::core::GUID = <ISignalNotifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SignalNotifier {
    const NAME: &'static str = "Windows.System.Threading.Core.SignalNotifier";
}
impl ::core::convert::From<SignalNotifier> for ::windows::core::IUnknown {
    fn from(value: SignalNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignalNotifier> for ::windows::core::IUnknown {
    fn from(value: &SignalNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for SignalNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a SignalNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<SignalNotifier> for ::windows::core::IInspectable {
    fn from(value: SignalNotifier) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&SignalNotifier> for ::windows::core::IInspectable {
    fn from(value: &SignalNotifier) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for SignalNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a SignalNotifier {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for SignalNotifier {}
unsafe impl ::core::marker::Sync for SignalNotifier {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
