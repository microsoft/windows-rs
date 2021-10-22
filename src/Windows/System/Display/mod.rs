#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct DisplayRequest(::windows::runtime::IInspectable);
impl DisplayRequest {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            DisplayRequest,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn RequestActive(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn RequestRelease(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DisplayRequest {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.System.Display.DisplayRequest;{e5732044-f49f-4b60-8dd4-5e7e3a632ac0})",
    );
}
unsafe impl ::windows::runtime::Interface for DisplayRequest {
    type Vtable = IDisplayRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3849527364,
        62623,
        19296,
        [141, 212, 94, 126, 58, 99, 42, 192],
    );
}
impl ::windows::runtime::RuntimeName for DisplayRequest {
    const NAME: &'static str = "Windows.System.Display.DisplayRequest";
}
impl ::std::convert::From<DisplayRequest> for ::windows::runtime::IUnknown {
    fn from(value: DisplayRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&DisplayRequest> for ::windows::runtime::IUnknown {
    fn from(value: &DisplayRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for DisplayRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &DisplayRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<DisplayRequest> for ::windows::runtime::IInspectable {
    fn from(value: DisplayRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&DisplayRequest> for ::windows::runtime::IInspectable {
    fn from(value: &DisplayRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for DisplayRequest {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a DisplayRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IDisplayRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDisplayRequest {
    type Vtable = IDisplayRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3849527364,
        62623,
        19296,
        [141, 212, 94, 126, 58, 99, 42, 192],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayRequest_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut u32,
        values: *mut *mut ::windows::runtime::GUID,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
