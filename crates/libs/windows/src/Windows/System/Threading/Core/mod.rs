#[doc(hidden)]
#[repr(transparent)]
pub struct IPreallocatedWorkItem(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPreallocatedWorkItem {
    type Vtable = IPreallocatedWorkItem_Vtbl;
}
unsafe impl ::windows::core::Interface for IPreallocatedWorkItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb6daa9fc_bc5b_401a_a8b2_6e754d14daa6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreallocatedWorkItem_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub RunAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RunAsync: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPreallocatedWorkItemFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IPreallocatedWorkItemFactory {
    type Vtable = IPreallocatedWorkItemFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IPreallocatedWorkItemFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe3d32b45_dfea_469b_82c5_f6e3cefdeafb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPreallocatedWorkItemFactory_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItem: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItemWithPriority: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, priority: super::WorkItemPriority, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItemWithPriority: usize,
    #[cfg(feature = "Foundation")]
    pub CreateWorkItemWithPriorityAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, priority: super::WorkItemPriority, options: super::WorkItemOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CreateWorkItemWithPriorityAndOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignalNotifier(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISignalNotifier {
    type Vtable = ISignalNotifier_Vtbl;
}
unsafe impl ::windows::core::Interface for ISignalNotifier {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14285e06_63a7_4713_b6d9_62f64b56fb8b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignalNotifier_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Terminate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ISignalNotifierStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for ISignalNotifierStatics {
    type Vtable = ISignalNotifierStatics_Vtbl;
}
unsafe impl ::windows::core::Interface for ISignalNotifierStatics {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1c4e4566_8400_46d3_a115_7d0c0dfc9f62);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISignalNotifierStatics_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub AttachToEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachToEventWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, timeout: super::super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachToEventWithTimeout: usize,
    pub AttachToSemaphore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub AttachToSemaphoreWithTimeout: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, timeout: super::super::super::Foundation::TimeSpan, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AttachToSemaphoreWithTimeout: usize,
}
#[doc = "*Required features: `\"System_Threading_Core\"`*"]
#[repr(transparent)]
pub struct PreallocatedWorkItem(::windows::core::IUnknown);
impl PreallocatedWorkItem {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RunAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).RunAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItem(handler: &super::WorkItemHandler) -> ::windows::core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWorkItem)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItemWithPriority(handler: &super::WorkItemHandler, priority: super::WorkItemPriority) -> ::windows::core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWorkItemWithPriority)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), priority, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateWorkItemWithPriorityAndOptions(handler: &super::WorkItemHandler, priority: super::WorkItemPriority, options: super::WorkItemOptions) -> ::windows::core::Result<PreallocatedWorkItem> {
        Self::IPreallocatedWorkItemFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CreateWorkItemWithPriorityAndOptions)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), priority, options, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPreallocatedWorkItemFactory<R, F: FnOnce(&IPreallocatedWorkItemFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<PreallocatedWorkItem, IPreallocatedWorkItemFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for PreallocatedWorkItem {
    type Vtable = IPreallocatedWorkItem_Vtbl;
}
unsafe impl ::windows::core::Interface for PreallocatedWorkItem {
    const IID: ::windows::core::GUID = <IPreallocatedWorkItem as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PreallocatedWorkItem {
    const NAME: &'static str = "Windows.System.Threading.Core.PreallocatedWorkItem";
}
::windows::core::interface_hierarchy!(PreallocatedWorkItem, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for PreallocatedWorkItem {}
unsafe impl ::core::marker::Sync for PreallocatedWorkItem {}
#[doc = "*Required features: `\"System_Threading_Core\"`*"]
#[repr(transparent)]
pub struct SignalNotifier(::windows::core::IUnknown);
impl SignalNotifier {
    pub fn Enable(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Enable)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn Terminate(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Terminate)(::windows::core::Vtable::as_raw(this)).ok() }
    }
    pub fn AttachToEvent(name: &::windows::core::HSTRING, handler: &SignalHandler) -> ::windows::core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttachToEvent)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachToEventWithTimeout(name: &::windows::core::HSTRING, handler: &SignalHandler, timeout: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttachToEventWithTimeout)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(handler), timeout, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    pub fn AttachToSemaphore(name: &::windows::core::HSTRING, handler: &SignalHandler) -> ::windows::core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttachToSemaphore)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AttachToSemaphoreWithTimeout(name: &::windows::core::HSTRING, handler: &SignalHandler, timeout: super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<SignalNotifier> {
        Self::ISignalNotifierStatics(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).AttachToSemaphoreWithTimeout)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(name), ::core::mem::transmute_copy(handler), timeout, result__.as_mut_ptr()).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn ISignalNotifierStatics<R, F: FnOnce(&ISignalNotifierStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<SignalNotifier, ISignalNotifierStatics> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
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
unsafe impl ::windows::core::Vtable for SignalNotifier {
    type Vtable = ISignalNotifier_Vtbl;
}
unsafe impl ::windows::core::Interface for SignalNotifier {
    const IID: ::windows::core::GUID = <ISignalNotifier as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for SignalNotifier {
    const NAME: &'static str = "Windows.System.Threading.Core.SignalNotifier";
}
::windows::core::interface_hierarchy!(SignalNotifier, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for SignalNotifier {}
unsafe impl ::core::marker::Sync for SignalNotifier {}
#[doc = "*Required features: `\"System_Threading_Core\"`*"]
#[repr(transparent)]
pub struct SignalHandler(pub ::windows::core::IUnknown);
impl SignalHandler {
    pub fn new<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = SignalHandlerBox::<F> { vtable: &SignalHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, signalnotifier: &SignalNotifier, timedout: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).Invoke)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(signalnotifier), timedout).ok() }
    }
}
#[repr(C)]
struct SignalHandlerBox<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const SignalHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<SignalNotifier>, bool) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> SignalHandlerBox<F> {
    const VTABLE: SignalHandler_Vtbl = SignalHandler_Vtbl {
        base__: ::windows::core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<SignalHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, signalnotifier: *mut ::core::ffi::c_void, timedout: bool) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
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
unsafe impl ::windows::core::Vtable for SignalHandler {
    type Vtable = SignalHandler_Vtbl;
}
unsafe impl ::windows::core::Interface for SignalHandler {
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
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signalnotifier: *mut ::core::ffi::c_void, timedout: bool) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
