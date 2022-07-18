#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteTextConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteTextConnection {
    type Vtable = IRemoteTextConnection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e7bb02a_183e_5e66_b5e4_3e6e5c570cf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteTextConnection_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub RegisterThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT,
    pub UnregisterThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT,
    pub ReportDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteTextConnectionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteTextConnectionFactory {
    type Vtable = IRemoteTextConnectionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88e075c2_0cae_596c_850f_78d345cd728b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteTextConnectionFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, pduforwarder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"System_RemoteDesktop_Input\"`*"]
#[repr(transparent)]
pub struct RemoteTextConnection(::windows::core::IUnknown);
impl RemoteTextConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsEnabled)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsEnabled)(::windows::core::Interface::as_raw(this), value).ok() }
    }
    pub fn RegisterThread(&self, threadid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RegisterThread)(::windows::core::Interface::as_raw(this), threadid).ok() }
    }
    pub fn UnregisterThread(&self, threadid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UnregisterThread)(::windows::core::Interface::as_raw(this), threadid).ok() }
    }
    pub fn ReportDataReceived(&self, pdudata: &[u8]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ReportDataReceived)(::windows::core::Interface::as_raw(this), pdudata.len() as u32, pdudata.as_ptr()).ok() }
    }
    pub fn CreateInstance<'a, P0>(connectionid: ::windows::core::GUID, pduforwarder: P0) -> ::windows::core::Result<RemoteTextConnection>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, RemoteTextConnectionDataHandler>>,
    {
        Self::IRemoteTextConnectionFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::windows::core::Interface::as_raw(this), connectionid, pduforwarder.into().abi(), result__.as_mut_ptr()).from_abi::<RemoteTextConnection>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteTextConnectionFactory<R, F: FnOnce(&IRemoteTextConnectionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<RemoteTextConnection, IRemoteTextConnectionFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for RemoteTextConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteTextConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteTextConnection {}
impl ::core::fmt::Debug for RemoteTextConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteTextConnection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RemoteTextConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteDesktop.Input.RemoteTextConnection;{4e7bb02a-183e-5e66-b5e4-3e6e5c570cf1})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RemoteTextConnection {
    type Vtable = IRemoteTextConnection_Vtbl;
    const IID: ::windows::core::GUID = <IRemoteTextConnection as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RemoteTextConnection {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Input.RemoteTextConnection";
}
impl ::core::convert::From<RemoteTextConnection> for ::windows::core::IUnknown {
    fn from(value: RemoteTextConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteTextConnection> for ::windows::core::IUnknown {
    fn from(value: &RemoteTextConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RemoteTextConnection> for &::windows::core::IUnknown {
    fn from(value: &RemoteTextConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<RemoteTextConnection> for ::windows::core::IInspectable {
    fn from(value: RemoteTextConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RemoteTextConnection> for ::windows::core::IInspectable {
    fn from(value: &RemoteTextConnection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&RemoteTextConnection> for &::windows::core::IInspectable {
    fn from(value: &RemoteTextConnection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<RemoteTextConnection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: RemoteTextConnection) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&RemoteTextConnection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteTextConnection) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::core::convert::TryFrom<&RemoteTextConnection> for ::windows::core::InParam<'a, super::super::super::Foundation::IClosable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &RemoteTextConnection) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
unsafe impl ::core::marker::Send for RemoteTextConnection {}
unsafe impl ::core::marker::Sync for RemoteTextConnection {}
#[doc = "*Required features: `\"System_RemoteDesktop_Input\"`*"]
#[repr(transparent)]
pub struct RemoteTextConnectionDataHandler(pub ::windows::core::IUnknown);
impl RemoteTextConnectionDataHandler {
    pub fn new<F: FnMut(&[u8]) -> ::windows::core::Result<bool> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RemoteTextConnectionDataHandlerBox::<F> { vtable: &RemoteTextConnectionDataHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, pdudata: &[u8]) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this), pdudata.len() as u32, pdudata.as_ptr(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
#[repr(C)]
struct RemoteTextConnectionDataHandlerBox<F: FnMut(&[u8]) -> ::windows::core::Result<bool> + ::core::marker::Send + 'static> {
    vtable: *const RemoteTextConnectionDataHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&[u8]) -> ::windows::core::Result<bool> + ::core::marker::Send + 'static> RemoteTextConnectionDataHandlerBox<F> {
    const VTABLE: RemoteTextConnectionDataHandler_Vtbl = RemoteTextConnectionDataHandler_Vtbl {
        base__: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<RemoteTextConnectionDataHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> ::windows::core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        match ((*this).invoke)(::core::slice::from_raw_parts(::core::mem::transmute_copy(&pdudata), pduData_array_size as _)) {
            ::core::result::Result::Ok(ok__) => {
                ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                ::core::mem::forget(ok__);
                ::windows::core::HRESULT(0)
            }
            ::core::result::Result::Err(err) => err.into(),
        }
    }
}
impl ::core::clone::Clone for RemoteTextConnectionDataHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RemoteTextConnectionDataHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RemoteTextConnectionDataHandler {}
impl ::core::fmt::Debug for RemoteTextConnectionDataHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RemoteTextConnectionDataHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for RemoteTextConnectionDataHandler {
    type Vtable = RemoteTextConnectionDataHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x099ffbc8_8bcb_41b5_b056_57e77021bf1b);
}
unsafe impl ::windows::core::RuntimeType for RemoteTextConnectionDataHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{099ffbc8-8bcb-41b5-b056-57e77021bf1b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct RemoteTextConnectionDataHandler_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
