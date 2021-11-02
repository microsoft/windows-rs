#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRemoteTextConnection(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteTextConnection {
    type Vtable = IRemoteTextConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1316728874, 6206, 24166, [181, 228, 62, 110, 92, 87, 12, 241]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteTextConnection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threadid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threadid: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pduData_array_size: u32, pdudata: *const u8) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IRemoteTextConnectionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteTextConnectionFactory {
    type Vtable = IRemoteTextConnectionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2296411586, 3246, 22892, [133, 15, 120, 211, 69, 205, 114, 139]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteTextConnectionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, connectionid: ::windows::runtime::GUID, pduforwarder: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `System_RemoteDesktop_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RemoteTextConnection(::windows::runtime::IInspectable);
impl RemoteTextConnection {
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `System_RemoteDesktop_Input`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `System_RemoteDesktop_Input`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `System_RemoteDesktop_Input`*"]
    pub fn SetIsEnabled(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `System_RemoteDesktop_Input`*"]
    pub fn RegisterThread(&self, threadid: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), threadid).ok() }
    }
    #[doc = "*Required features: `System_RemoteDesktop_Input`*"]
    pub fn UnregisterThread(&self, threadid: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), threadid).ok() }
    }
    #[doc = "*Required features: `System_RemoteDesktop_Input`*"]
    pub fn ReportDataReceived(&self, pdudata: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), pdudata.len() as u32, ::std::mem::transmute(pdudata.as_ptr())).ok() }
    }
    #[doc = "*Required features: `System_RemoteDesktop_Input`*"]
    pub fn CreateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, RemoteTextConnectionDataHandler>>(connectionid: Param0, pduforwarder: Param1) -> ::windows::runtime::Result<RemoteTextConnection> {
        Self::IRemoteTextConnectionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), connectionid.into_param().abi(), pduforwarder.into_param().abi(), &mut result__).from_abi::<RemoteTextConnection>(result__)
        })
    }
    pub fn IRemoteTextConnectionFactory<R, F: FnOnce(&IRemoteTextConnectionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteTextConnection, IRemoteTextConnectionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteTextConnection {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.System.RemoteDesktop.Input.RemoteTextConnection;{4e7bb02a-183e-5e66-b5e4-3e6e5c570cf1})");
}
unsafe impl ::windows::runtime::Interface for RemoteTextConnection {
    type Vtable = IRemoteTextConnection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1316728874, 6206, 24166, [181, 228, 62, 110, 92, 87, 12, 241]);
}
impl ::windows::runtime::RuntimeName for RemoteTextConnection {
    const NAME: &'static str = "Windows.System.RemoteDesktop.Input.RemoteTextConnection";
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<RemoteTextConnection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteTextConnection) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&RemoteTextConnection> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteTextConnection) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for RemoteTextConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &RemoteTextConnection {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RemoteTextConnection {}
unsafe impl ::std::marker::Sync for RemoteTextConnection {}
#[doc = "*Required features: `System_RemoteDesktop_Input`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct RemoteTextConnectionDataHandler(::windows::runtime::IUnknown);
impl RemoteTextConnectionDataHandler {
    pub fn new<F: FnMut(&[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<bool> + 'static>(invoke: F) -> Self {
        let com = RemoteTextConnectionDataHandler_box::<F> {
            vtable: &RemoteTextConnectionDataHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `System_RemoteDesktop_Input`*"]
    pub fn Invoke(&self, pdudata: &[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).3)(::std::mem::transmute_copy(this), pdudata.len() as u32, ::std::mem::transmute(pdudata.as_ptr()), &mut result__).from_abi::<bool>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteTextConnectionDataHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"delegate({099ffbc8-8bcb-41b5-b056-57e77021bf1b})");
}
unsafe impl ::windows::runtime::Interface for RemoteTextConnectionDataHandler {
    type Vtable = RemoteTextConnectionDataHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(161479624, 35787, 16821, [176, 86, 87, 231, 112, 33, 191, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct RemoteTextConnectionDataHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct RemoteTextConnectionDataHandler_box<F: FnMut(&[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<bool> + 'static> {
    vtable: *const RemoteTextConnectionDataHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<F: FnMut(&[<u8 as ::windows::runtime::Abi>::DefaultType]) -> ::windows::runtime::Result<bool> + 'static> RemoteTextConnectionDataHandler_box<F> {
    const VTABLE: RemoteTextConnectionDataHandler_abi = RemoteTextConnectionDataHandler_abi(Self::QueryInterface, Self::AddRef, Self::Release, Self::Invoke);
    unsafe extern "system" fn QueryInterface(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<RemoteTextConnectionDataHandler as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID {
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
    unsafe extern "system" fn Invoke(this: ::windows::runtime::RawPtr, pduData_array_size: u32, pdudata: *const u8, result__: *mut bool) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        match ((*this).invoke)(::std::slice::from_raw_parts(::std::mem::transmute_copy(&pdudata), pduData_array_size as _)) {
            ::std::result::Result::Ok(ok__) => {
                *result__ = ::std::mem::transmute_copy(&ok__);
                ::std::mem::forget(ok__);
                ::windows::runtime::HRESULT(0)
            }
            ::std::result::Result::Err(err) => err.into(),
        }
    }
}
