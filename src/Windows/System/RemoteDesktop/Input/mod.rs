#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteTextConnection(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteTextConnection {
    type Vtable = IRemoteTextConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e7bb02a_183e_5e66_b5e4_3e6e5c570cf1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteTextConnectionVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, threadid: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8) -> ::windows::core::HRESULT,
);
#[doc(hidden)]
#[repr(transparent)]
pub struct IRemoteTextConnectionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRemoteTextConnectionFactory {
    type Vtable = IRemoteTextConnectionFactoryVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x88e075c2_0cae_596c_850f_78d345cd728b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteTextConnectionFactoryVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut u32, values: *mut *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, connectionid: ::windows::core::GUID, pduforwarder: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct RemoteTextConnection(::windows::core::IUnknown);
impl RemoteTextConnection {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    pub fn IsEnabled(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    pub fn RegisterThread(&self, threadid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).8)(::core::mem::transmute_copy(this), threadid).ok() }
    }
    pub fn UnregisterThread(&self, threadid: u32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).9)(::core::mem::transmute_copy(this), threadid).ok() }
    }
    pub fn ReportDataReceived(&self, pdudata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).10)(::core::mem::transmute_copy(this), pdudata.len() as u32, ::core::mem::transmute(pdudata.as_ptr())).ok() }
    }
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param1: ::windows::core::IntoParam<'a, RemoteTextConnectionDataHandler>>(connectionid: Param0, pduforwarder: Param1) -> ::windows::core::Result<RemoteTextConnection> {
        Self::IRemoteTextConnectionFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).6)(::core::mem::transmute_copy(this), connectionid.into_param().abi(), pduforwarder.into_param().abi(), &mut result__).from_abi::<RemoteTextConnection>(result__)
        })
    }
    pub fn IRemoteTextConnectionFactory<R, F: FnOnce(&IRemoteTextConnectionFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RemoteTextConnection, IRemoteTextConnectionFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
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
unsafe impl ::windows::core::RuntimeType for RemoteTextConnection {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.System.RemoteDesktop.Input.RemoteTextConnection;{4e7bb02a-183e-5e66-b5e4-3e6e5c570cf1})");
}
unsafe impl ::windows::core::Interface for RemoteTextConnection {
    type Vtable = IRemoteTextConnectionVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e7bb02a_183e_5e66_b5e4_3e6e5c570cf1);
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RemoteTextConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &RemoteTextConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RemoteTextConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &RemoteTextConnection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
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
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for RemoteTextConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &RemoteTextConnection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
unsafe impl ::core::marker::Send for RemoteTextConnection {}
unsafe impl ::core::marker::Sync for RemoteTextConnection {}
#[repr(transparent)]
pub struct RemoteTextConnectionDataHandler(pub ::windows::core::IUnknown);
impl RemoteTextConnectionDataHandler {
    pub fn new<F: FnMut(&[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<bool> + 'static>(invoke: F) -> Self {
        let com = RemoteTextConnectionDataHandlerBox::<F> { vtable: &RemoteTextConnectionDataHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    pub fn Invoke(&self, pdudata: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).3)(::core::mem::transmute_copy(this), pdudata.len() as u32, ::core::mem::transmute(pdudata.as_ptr()), &mut result__).from_abi::<bool>(result__)
        }
    }
}
#[repr(C)]
struct RemoteTextConnectionDataHandlerBox<F: FnMut(&[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<bool> + 'static> {
    vtable: *const RemoteTextConnectionDataHandlerVtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<bool> + 'static> RemoteTextConnectionDataHandlerBox<F> {
    const VTABLE: RemoteTextConnectionDataHandlerVtbl = RemoteTextConnectionDataHandlerVtbl(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<RemoteTextConnectionDataHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        match ((*this).invoke)(::core::slice::from_raw_parts(::core::mem::transmute_copy(&pdudata), pduData_array_size as _)) {
            ::core::result::Result::Ok(ok__) => {
                *result__ = ::core::mem::transmute_copy(&ok__);
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
unsafe impl ::windows::core::Interface for RemoteTextConnectionDataHandler {
    type Vtable = RemoteTextConnectionDataHandlerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x099ffbc8_8bcb_41b5_b056_57e77021bf1b);
}
unsafe impl ::windows::core::RuntimeType for RemoteTextConnectionDataHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{099ffbc8-8bcb-41b5-b056-57e77021bf1b}");
}
#[repr(C)]
#[doc(hidden)]
pub struct RemoteTextConnectionDataHandlerVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> ::windows::core::HRESULT);
