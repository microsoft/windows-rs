#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteTextConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteTextConnection {
    type Vtable = IRemoteTextConnection_Vtbl;
}
impl ::core::clone::Clone for IRemoteTextConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRemoteTextConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x4e7bb02a_183e_5e66_b5e4_3e6e5c570cf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteTextConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub SetIsEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows_core::HRESULT,
    pub RegisterThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT,
    pub UnregisterThread: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows_core::HRESULT,
    pub ReportDataReceived: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteTextConnectionFactory(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IRemoteTextConnectionFactory {
    type Vtable = IRemoteTextConnectionFactory_Vtbl;
}
impl ::core::clone::Clone for IRemoteTextConnectionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IRemoteTextConnectionFactory {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x88e075c2_0cae_596c_850f_78d345cd728b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteTextConnectionFactory_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID, pduforwarder: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc = "*Required features: `\"System_RemoteDesktop_Input\"`*"]
#[repr(transparent)]
pub struct RemoteTextConnection(::windows_core::IUnknown);
impl RemoteTextConnection {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows_core::Result<()> {
        let this = &::windows_core::ComInterface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows_core::Interface::vtable(this).Close)(::windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsEnabled)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).SetIsEnabled)(::windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RegisterThread(&self, threadid: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RegisterThread)(::windows_core::Interface::as_raw(this), threadid).ok() }
    }
    pub fn UnregisterThread(&self, threadid: u32) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).UnregisterThread)(::windows_core::Interface::as_raw(this), threadid).ok() }
    }
    pub fn ReportDataReceived(&self, pdudata: &[u8]) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).ReportDataReceived)(::windows_core::Interface::as_raw(this), pdudata.len() as u32, pdudata.as_ptr()).ok() }
    }
    pub fn CreateInstance<P0>(connectionid: ::windows_core::GUID, pduforwarder: P0) -> ::windows_core::Result<RemoteTextConnection>
    where
        P0: ::windows_core::IntoParam<RemoteTextConnectionDataHandler>,
    {
        Self::IRemoteTextConnectionFactory(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CreateInstance)(::windows_core::Interface::as_raw(this), connectionid, pduforwarder.into_param().abi(), &mut result__).from_abi(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRemoteTextConnectionFactory<R, F: FnOnce(&IRemoteTextConnectionFactory) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<RemoteTextConnection, IRemoteTextConnectionFactory> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
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
impl ::windows_core::RuntimeType for RemoteTextConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.System.RemoteDesktop.Input.RemoteTextConnection;{4e7bb02a-183e-5e66-b5e4-3e6e5c570cf1})");
}
impl ::core::clone::Clone for RemoteTextConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for RemoteTextConnection {
    type Vtable = IRemoteTextConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for RemoteTextConnection {
    const IID: ::windows_core::GUID = <IRemoteTextConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for RemoteTextConnection {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Input.RemoteTextConnection";
}
::windows_core::imp::interface_hierarchy!(RemoteTextConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
#[cfg(feature = "Foundation")]
impl ::windows_core::CanTryInto<super::super::super::Foundation::IClosable> for RemoteTextConnection {}
unsafe impl ::core::marker::Send for RemoteTextConnection {}
unsafe impl ::core::marker::Sync for RemoteTextConnection {}
#[doc = "*Required features: `\"System_RemoteDesktop_Input\"`*"]
#[repr(transparent)]
pub struct RemoteTextConnectionDataHandler(pub ::windows_core::IUnknown);
impl RemoteTextConnectionDataHandler {
    pub fn new<F: FnMut(&[u8]) -> ::windows_core::Result<bool> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = RemoteTextConnectionDataHandlerBox::<F> { vtable: &RemoteTextConnectionDataHandlerBox::<F>::VTABLE, count: ::windows_core::imp::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, pdudata: &[u8]) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Invoke)(::windows_core::Interface::as_raw(this), pdudata.len() as u32, pdudata.as_ptr(), &mut result__).from_abi(result__)
        }
    }
}
#[repr(C)]
struct RemoteTextConnectionDataHandlerBox<F: FnMut(&[u8]) -> ::windows_core::Result<bool> + ::core::marker::Send + 'static> {
    vtable: *const RemoteTextConnectionDataHandler_Vtbl,
    invoke: F,
    count: ::windows_core::imp::RefCount,
}
impl<F: FnMut(&[u8]) -> ::windows_core::Result<bool> + ::core::marker::Send + 'static> RemoteTextConnectionDataHandlerBox<F> {
    const VTABLE: RemoteTextConnectionDataHandler_Vtbl = RemoteTextConnectionDataHandler_Vtbl {
        base__: ::windows_core::IUnknown_Vtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release },
        Invoke: Self::Invoke,
    };
    unsafe extern "system" fn QueryInterface(this: *mut ::core::ffi::c_void, iid: &::windows_core::GUID, interface: *mut *const ::core::ffi::c_void) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        *interface = if iid == &<RemoteTextConnectionDataHandler as ::windows_core::ComInterface>::IID || iid == &<::windows_core::IUnknown as ::windows_core::ComInterface>::IID || iid == &<::windows_core::imp::IAgileObject as ::windows_core::ComInterface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows_core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows_core::HRESULT(0)
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> ::windows_core::HRESULT {
        let this = this as *mut *mut ::core::ffi::c_void as *mut Self;
        match ((*this).invoke)(::core::slice::from_raw_parts(::core::mem::transmute_copy(&pdudata), pduData_array_size as usize)) {
            ::core::result::Result::Ok(ok__) => {
                ::core::ptr::write(result__, ::core::mem::transmute_copy(&ok__));
                ::windows_core::HRESULT(0)
            }
            ::core::result::Result::Err(err) => err.into(),
        }
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
unsafe impl ::windows_core::Interface for RemoteTextConnectionDataHandler {
    type Vtable = RemoteTextConnectionDataHandler_Vtbl;
}
impl ::core::clone::Clone for RemoteTextConnectionDataHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for RemoteTextConnectionDataHandler {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x099ffbc8_8bcb_41b5_b056_57e77021bf1b);
}
impl ::windows_core::RuntimeType for RemoteTextConnectionDataHandler {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"{099ffbc8-8bcb-41b5-b056-57e77021bf1b}");
}
#[repr(C)]
#[doc(hidden)]
pub struct RemoteTextConnectionDataHandler_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> ::windows_core::HRESULT,
}
