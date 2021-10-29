#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CLSID_VdsLoader: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2620976481, 54629, 18216, [174, 238, 200, 9, 82, 240, 236, 222]);
pub const CLSID_VdsService: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2098803659, 34550, 19096, [134, 40, 1, 190, 148, 201, 165, 117]);
pub const GPT_PARTITION_NAME_LENGTH: u32 = 36u32;
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumVdsObject(::windows::runtime::IUnknown);
impl IEnumVdsObject {
    pub unsafe fn Next(&self, celt: u32, ppobjectarray: *mut ::std::option::Option<::windows::runtime::IUnknown>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt), ::std::mem::transmute(ppobjectarray), ::std::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumVdsObject {
    type Vtable = IEnumVdsObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(293998775, 36244, 16432, [181, 184, 80, 8, 137, 120, 142, 78]);
}
impl ::std::convert::From<IEnumVdsObject> for ::windows::runtime::IUnknown {
    fn from(value: IEnumVdsObject) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IEnumVdsObject> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumVdsObject) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumVdsObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IEnumVdsObject {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumVdsObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32, ppobjectarray: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, celt: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsAdmin(::windows::runtime::IUnknown);
impl IVdsAdmin {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterProvider<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param6: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(
        &self,
        providerid: Param0,
        providerclsid: Param1,
        pwszname: Param2,
        r#type: VDS_PROVIDER_TYPE,
        pwszmachinename: Param4,
        pwszversion: Param5,
        guidversionid: Param6,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), providerid.into_param().abi(), providerclsid.into_param().abi(), pwszname.into_param().abi(), ::std::mem::transmute(r#type), pwszmachinename.into_param().abi(), pwszversion.into_param().abi(), guidversionid.into_param().abi()).ok()
    }
    pub unsafe fn UnregisterProvider<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, providerid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), providerid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsAdmin {
    type Vtable = IVdsAdmin_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3515410813, 34218, 19763, [171, 198, 38, 41, 154, 16, 255, 193]);
}
impl ::std::convert::From<IVdsAdmin> for ::windows::runtime::IUnknown {
    fn from(value: IVdsAdmin) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsAdmin> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsAdmin) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsAdmin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsAdmin {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdmin_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, providerid: ::windows::runtime::GUID, providerclsid: ::windows::runtime::GUID, pwszname: super::super::Foundation::PWSTR, r#type: VDS_PROVIDER_TYPE, pwszmachinename: super::super::Foundation::PWSTR, pwszversion: super::super::Foundation::PWSTR, guidversionid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, providerid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsAdviseSink(::windows::runtime::IUnknown);
impl IVdsAdviseSink {
    pub unsafe fn OnNotify(&self, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(lnumberofnotifications), ::std::mem::transmute(pnotificationarray)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsAdviseSink {
    type Vtable = IVdsAdviseSink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2200358173, 53081, 18742, [183, 134, 94, 252, 8, 121, 142, 37]);
}
impl ::std::convert::From<IVdsAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: IVdsAdviseSink) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsAdviseSink> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsAdviseSink) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsAdviseSink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAdviseSink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lnumberofnotifications: i32, pnotificationarray: *const VDS_NOTIFICATION) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsAsync(::windows::runtime::IUnknown);
impl IVdsAsync {
    pub unsafe fn Cancel(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Wait(&self, phrresult: *mut ::windows::runtime::HRESULT, pasyncout: *mut VDS_ASYNC_OUTPUT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(phrresult), ::std::mem::transmute(pasyncout)).ok()
    }
    pub unsafe fn QueryStatus(&self, phrresult: *mut ::windows::runtime::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(phrresult), ::std::mem::transmute(pulpercentcompleted)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsAsync {
    type Vtable = IVdsAsync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3587324781, 23125, 17554, [152, 137, 57, 122, 60, 45, 45, 188]);
}
impl ::std::convert::From<IVdsAsync> for ::windows::runtime::IUnknown {
    fn from(value: IVdsAsync) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsAsync> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsAsync) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsAsync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsAsync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrresult: *mut ::windows::runtime::HRESULT, pasyncout: *mut ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrresult: *mut ::windows::runtime::HRESULT, pulpercentcompleted: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsController(::windows::runtime::IUnknown);
impl IVdsController {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_CONTROLLER_PROP> {
        let mut result__: <VDS_CONTROLLER_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_CONTROLLER_PROP>(result__)
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::runtime::Result<IVdsSubSystem> {
        let mut result__: <IVdsSubSystem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsSubSystem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPortProperties(&self, sportnumber: i16) -> ::windows::runtime::Result<VDS_PORT_PROP> {
        let mut result__: <VDS_PORT_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(sportnumber), &mut result__).from_abi::<VDS_PORT_PROP>(result__)
    }
    pub unsafe fn FlushCache(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn InvalidateCache(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn SetStatus(&self, status: VDS_CONTROLLER_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsController {
    type Vtable = IVdsController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3411269998, 57339, 18250, [160, 120, 121, 13, 30, 43, 192, 130]);
}
impl ::std::convert::From<IVdsController> for ::windows::runtime::IUnknown {
    fn from(value: IVdsController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsController> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsController {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcontrollerprop: *mut VDS_CONTROLLER_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsubsystem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sportnumber: i16, pportprop: *mut VDS_PORT_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: VDS_CONTROLLER_STATUS) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsControllerControllerPort(::windows::runtime::IUnknown);
impl IVdsControllerControllerPort {
    pub unsafe fn QueryControllerPorts(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsControllerControllerPort {
    type Vtable = IVdsControllerControllerPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3395122015, 27566, 17088, [179, 14, 242, 102, 96, 69, 206, 113]);
}
impl ::std::convert::From<IVdsControllerControllerPort> for ::windows::runtime::IUnknown {
    fn from(value: IVdsControllerControllerPort) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsControllerControllerPort> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsControllerControllerPort) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsControllerControllerPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsControllerControllerPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsControllerControllerPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsControllerPort(::windows::runtime::IUnknown);
impl IVdsControllerPort {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_PORT_PROP> {
        let mut result__: <VDS_PORT_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_PORT_PROP>(result__)
    }
    pub unsafe fn GetController(&self) -> ::windows::runtime::Result<IVdsController> {
        let mut result__: <IVdsController as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsController>(result__)
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_PORT_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsControllerPort {
    type Vtable = IVdsControllerPort_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(409541901, 20095, 17384, [146, 228, 207, 68, 190, 238, 209, 28]);
}
impl ::std::convert::From<IVdsControllerPort> for ::windows::runtime::IUnknown {
    fn from(value: IVdsControllerPort) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsControllerPort> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsControllerPort) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsControllerPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsControllerPort {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsControllerPort_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportprop: *mut VDS_PORT_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontroller: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: VDS_PORT_STATUS) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsDrive(::windows::runtime::IUnknown);
impl IVdsDrive {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_DRIVE_PROP> {
        let mut result__: <VDS_DRIVE_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_DRIVE_PROP>(result__)
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::runtime::Result<IVdsSubSystem> {
        let mut result__: <IVdsSubSystem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsSubSystem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppextentarray), ::std::mem::transmute(plnumberofextents)).ok()
    }
    pub unsafe fn SetFlags(&self, ulflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn ClearFlags(&self, ulflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulflags)).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_DRIVE_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsDrive {
    type Vtable = IVdsDrive_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4280610724, 43742, 19307, [137, 139, 234, 166, 162, 8, 135, 199]);
}
impl ::std::convert::From<IVdsDrive> for ::windows::runtime::IUnknown {
    fn from(value: IVdsDrive) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsDrive> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsDrive) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsDrive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsDrive {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDrive_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdriveprop: *mut VDS_DRIVE_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsubsystem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulflags: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: VDS_DRIVE_STATUS) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsDrive2(::windows::runtime::IUnknown);
impl IVdsDrive2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties2(&self) -> ::windows::runtime::Result<VDS_DRIVE_PROP2> {
        let mut result__: <VDS_DRIVE_PROP2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_DRIVE_PROP2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsDrive2 {
    type Vtable = IVdsDrive2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1622517552, 44511, 17462, [140, 167, 87, 105, 226, 209, 255, 164]);
}
impl ::std::convert::From<IVdsDrive2> for ::windows::runtime::IUnknown {
    fn from(value: IVdsDrive2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsDrive2> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsDrive2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsDrive2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsDrive2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsDrive2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdriveprop2: *mut VDS_DRIVE_PROP2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsHwProvider(::windows::runtime::IUnknown);
impl IVdsHwProvider {
    pub unsafe fn QuerySubSystems(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn Reenumerate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Refresh(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsHwProvider {
    type Vtable = IVdsHwProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3650869934, 45370, 16760, [159, 219, 226, 127, 22, 180, 96, 62]);
}
impl ::std::convert::From<IVdsHwProvider> for ::windows::runtime::IUnknown {
    fn from(value: IVdsHwProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsHwProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsHwProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsHwProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsHwProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsHwProviderPrivate(::windows::runtime::IUnknown);
impl IVdsHwProviderPrivate {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryIfCreatedLun<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszdevicepath: Param0, pvdsluninformation: *const VDS_LUN_INFORMATION) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszdevicepath.into_param().abi(), ::std::mem::transmute(pvdsluninformation), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsHwProviderPrivate {
    type Vtable = IVdsHwProviderPrivate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2565962739, 40755, 20242, [135, 20, 139, 64, 117, 9, 44, 46]);
}
impl ::std::convert::From<IVdsHwProviderPrivate> for ::windows::runtime::IUnknown {
    fn from(value: IVdsHwProviderPrivate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsHwProviderPrivate> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsHwProviderPrivate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsHwProviderPrivate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsHwProviderPrivate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderPrivate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszdevicepath: super::super::Foundation::PWSTR, pvdsluninformation: *const VDS_LUN_INFORMATION, plunid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsHwProviderPrivateMpio(::windows::runtime::IUnknown);
impl IVdsHwProviderPrivateMpio {
    pub unsafe fn SetAllPathStatusesFromHbaPort<'a, Param0: ::windows::runtime::IntoParam<'a, VDS_HBAPORT_PROP>>(&self, hbaportprop: Param0, status: VDS_PATH_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hbaportprop.into_param().abi(), ::std::mem::transmute(status)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsHwProviderPrivateMpio {
    type Vtable = IVdsHwProviderPrivateMpio_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(822769429, 44075, 19567, [152, 39, 61, 116, 47, 53, 22, 118]);
}
impl ::std::convert::From<IVdsHwProviderPrivateMpio> for ::windows::runtime::IUnknown {
    fn from(value: IVdsHwProviderPrivateMpio) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsHwProviderPrivateMpio> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsHwProviderPrivateMpio) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsHwProviderPrivateMpio {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsHwProviderPrivateMpio {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderPrivateMpio_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hbaportprop: VDS_HBAPORT_PROP, status: VDS_PATH_STATUS) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsHwProviderStoragePools(::windows::runtime::IUnknown);
impl IVdsHwProviderStoragePools {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryStoragePools(&self, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulflags), ::std::mem::transmute(ullremainingfreespace), ::std::mem::transmute(ppoolattributes), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLunInStoragePool<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: Param2, pwszunmaskinglist: Param3, phints2: *const VDS_HINTS2) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), ::std::mem::transmute(ullsizeinbytes), storagepoolid.into_param().abi(), pwszunmaskinglist.into_param().abi(), ::std::mem::transmute(phints2), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSizeInStoragePool<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, r#type: VDS_LUN_TYPE, storagepoolid: Param1, phints2: *const VDS_HINTS2) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), storagepoolid.into_param().abi(), ::std::mem::transmute(phints2), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsHwProviderStoragePools {
    type Vtable = IVdsHwProviderStoragePools_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3585446778, 61832, 19577, [184, 108, 17, 201, 32, 173, 17, 184]);
}
impl ::std::convert::From<IVdsHwProviderStoragePools> for ::windows::runtime::IUnknown {
    fn from(value: IVdsHwProviderStoragePools) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsHwProviderStoragePools> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsHwProviderStoragePools) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsHwProviderStoragePools {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsHwProviderStoragePools {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderStoragePools_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulflags: u32, ullremainingfreespace: u64, ppoolattributes: *const VDS_POOL_ATTRIBUTES, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, storagepoolid: ::windows::runtime::GUID, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: VDS_LUN_TYPE, storagepoolid: ::windows::runtime::GUID, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsHwProviderType(::windows::runtime::IUnknown);
impl IVdsHwProviderType {
    pub unsafe fn GetProviderType(&self) -> ::windows::runtime::Result<VDS_HWPROVIDER_TYPE> {
        let mut result__: <VDS_HWPROVIDER_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_HWPROVIDER_TYPE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsHwProviderType {
    type Vtable = IVdsHwProviderType_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1041191270, 21549, 20422, [148, 122, 1, 33, 116, 36, 11, 126]);
}
impl ::std::convert::From<IVdsHwProviderType> for ::windows::runtime::IUnknown {
    fn from(value: IVdsHwProviderType) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsHwProviderType> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsHwProviderType) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsHwProviderType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsHwProviderType {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderType_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsHwProviderType2(::windows::runtime::IUnknown);
impl IVdsHwProviderType2 {
    pub unsafe fn GetProviderType2(&self) -> ::windows::runtime::Result<VDS_HWPROVIDER_TYPE> {
        let mut result__: <VDS_HWPROVIDER_TYPE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_HWPROVIDER_TYPE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsHwProviderType2 {
    type Vtable = IVdsHwProviderType2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2173707119, 50384, 20097, [128, 17, 214, 149, 18, 252, 201, 132]);
}
impl ::std::convert::From<IVdsHwProviderType2> for ::windows::runtime::IUnknown {
    fn from(value: IVdsHwProviderType2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsHwProviderType2> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsHwProviderType2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsHwProviderType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsHwProviderType2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsHwProviderType2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptype: *mut VDS_HWPROVIDER_TYPE) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsIscsiPortal(::windows::runtime::IUnknown);
impl IVdsIscsiPortal {
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_ISCSI_PORTAL_PROP> {
        let mut result__: <VDS_ISCSI_PORTAL_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_ISCSI_PORTAL_PROP>(result__)
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::runtime::Result<IVdsSubSystem> {
        let mut result__: <IVdsSubSystem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsSubSystem>(result__)
    }
    pub unsafe fn QueryAssociatedPortalGroups(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn SetStatus(&self, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(status)).ok()
    }
    pub unsafe fn SetIpsecTunnelAddress(&self, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptunneladdress), ::std::mem::transmute(pdestinationaddress)).ok()
    }
    pub unsafe fn GetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pinitiatorportaladdress), &mut result__).from_abi::<u64>(result__)
    }
    pub unsafe fn SetIpsecSecurity(&self, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pinitiatorportaladdress), ::std::mem::transmute(ullsecurityflags), ::std::mem::transmute(pipseckey)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsIscsiPortal {
    type Vtable = IVdsIscsiPortal_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2141276573, 60549, 19082, [164, 123, 255, 105, 32, 31, 205, 52]);
}
impl ::std::convert::From<IVdsIscsiPortal> for ::windows::runtime::IUnknown {
    fn from(value: IVdsIscsiPortal) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsIscsiPortal> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsIscsiPortal) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsIscsiPortal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsIscsiPortal {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiPortal_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportalprop: *mut VDS_ISCSI_PORTAL_PROP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsubsystem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: VDS_ISCSI_PORTAL_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptunneladdress: *const VDS_IPADDRESS, pdestinationaddress: *const VDS_IPADDRESS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinitiatorportaladdress: *const VDS_IPADDRESS, pullsecurityflags: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pinitiatorportaladdress: *const VDS_IPADDRESS, ullsecurityflags: u64, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsIscsiPortalGroup(::windows::runtime::IUnknown);
impl IVdsIscsiPortalGroup {
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_ISCSI_PORTALGROUP_PROP> {
        let mut result__: <VDS_ISCSI_PORTALGROUP_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_ISCSI_PORTALGROUP_PROP>(result__)
    }
    pub unsafe fn GetTarget(&self) -> ::windows::runtime::Result<IVdsIscsiTarget> {
        let mut result__: <IVdsIscsiTarget as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsIscsiTarget>(result__)
    }
    pub unsafe fn QueryAssociatedPortals(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn AddPortal<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, portalid: Param0) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), portalid.into_param().abi(), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn RemovePortal<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, portalid: Param0) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), portalid.into_param().abi(), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsAsync>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsIscsiPortalGroup {
    type Vtable = IVdsIscsiPortalGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4277532829, 41949, 19254, [191, 40, 231, 221, 224, 69, 197, 147]);
}
impl ::std::convert::From<IVdsIscsiPortalGroup> for ::windows::runtime::IUnknown {
    fn from(value: IVdsIscsiPortalGroup) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsIscsiPortalGroup> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsIscsiPortalGroup) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsIscsiPortalGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsIscsiPortalGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiPortalGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pportalgroupprop: *mut VDS_ISCSI_PORTALGROUP_PROP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pptarget: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portalid: ::windows::runtime::GUID, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, portalid: ::windows::runtime::GUID, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsIscsiTarget(::windows::runtime::IUnknown);
impl IVdsIscsiTarget {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_ISCSI_TARGET_PROP> {
        let mut result__: <VDS_ISCSI_TARGET_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_ISCSI_TARGET_PROP>(result__)
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::runtime::Result<IVdsSubSystem> {
        let mut result__: <IVdsSubSystem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsSubSystem>(result__)
    }
    pub unsafe fn QueryPortalGroups(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn QueryAssociatedLuns(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn CreatePortalGroup(&self) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pwszfriendlyname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSharedSecret<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptargetsharedsecret), pwszinitiatorname.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RememberInitiatorSharedSecret<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinitiatorname: Param0, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pwszinitiatorname.into_param().abi(), ::std::mem::transmute(pinitiatorsharedsecret)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetConnectedInitiators(&self, pppwszinitiatorlist: *mut *mut super::super::Foundation::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pppwszinitiatorlist), ::std::mem::transmute(plnumberofinitiators)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsIscsiTarget {
    type Vtable = IVdsIscsiTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2861518933, 33765, 19404, [170, 115, 25, 133, 26, 54, 168, 73]);
}
impl ::std::convert::From<IVdsIscsiTarget> for ::windows::runtime::IUnknown {
    fn from(value: IVdsIscsiTarget) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsIscsiTarget> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsIscsiTarget) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsIscsiTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsIscsiTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsIscsiTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptargetprop: *mut VDS_ISCSI_TARGET_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsubsystem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptargetsharedsecret: *const VDS_ISCSI_SHARED_SECRET, pwszinitiatorname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszinitiatorname: super::super::Foundation::PWSTR, pinitiatorsharedsecret: *const VDS_ISCSI_SHARED_SECRET) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppwszinitiatorlist: *mut *mut super::super::Foundation::PWSTR, plnumberofinitiators: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsLun(::windows::runtime::IUnknown);
impl IVdsLun {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_LUN_PROP> {
        let mut result__: <VDS_LUN_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_LUN_PROP>(result__)
    }
    pub unsafe fn GetSubSystem(&self) -> ::windows::runtime::Result<IVdsSubSystem> {
        let mut result__: <IVdsSubSystem as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsSubSystem>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdentificationData(&self) -> ::windows::runtime::Result<VDS_LUN_INFORMATION> {
        let mut result__: <VDS_LUN_INFORMATION as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_LUN_INFORMATION>(result__)
    }
    pub unsafe fn QueryActiveControllers(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn Extend(&self, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullnumberofbytestoadd), ::std::mem::transmute(pdriveidarray), ::std::mem::transmute(lnumberofdrives), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn Shrink(&self, ullnumberofbytestoremove: u64) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(ullnumberofbytestoremove), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn QueryPlexes(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn AddPlex<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, lunid: Param0) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), lunid.into_param().abi(), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn RemovePlex<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, plexid: Param0) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), plexid.into_param().abi(), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn Recover(&self) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMask<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszunmaskinglist: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pwszunmaskinglist.into_param().abi()).ok()
    }
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn AssociateControllers(&self, pactivecontrolleridarray: *const ::windows::runtime::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::runtime::GUID, lnumberofinactivecontrollers: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pactivecontrolleridarray), ::std::mem::transmute(lnumberofactivecontrollers), ::std::mem::transmute(pinactivecontrolleridarray), ::std::mem::transmute(lnumberofinactivecontrollers)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints(&self) -> ::windows::runtime::Result<VDS_HINTS> {
        let mut result__: <VDS_HINTS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_HINTS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(phints)).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_LUN_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(status)).ok()
    }
    pub unsafe fn QueryMaxLunExtendSize(&self, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdriveidarray), ::std::mem::transmute(lnumberofdrives), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsLun {
    type Vtable = IVdsLun_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(893430215, 58895, 16657, [168, 64, 139, 186, 108, 44, 131, 216]);
}
impl ::std::convert::From<IVdsLun> for ::windows::runtime::IUnknown {
    fn from(value: IVdsLun) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsLun> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsLun) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsLun {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsLun {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLun_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plunprop: *mut VDS_LUN_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsubsystem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pluninfo: *mut VDS_LUN_INFORMATION) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullnumberofbytestoadd: u64, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ullnumberofbytestoremove: u64, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, lunid: ::windows::runtime::GUID, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plexid: ::windows::runtime::GUID, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszunmaskinglist: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pactivecontrolleridarray: *const ::windows::runtime::GUID, lnumberofactivecontrollers: i32, pinactivecontrolleridarray: *const ::windows::runtime::GUID, lnumberofinactivecontrollers: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phints: *mut VDS_HINTS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phints: *const VDS_HINTS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: VDS_LUN_STATUS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, pullmaxbytestobeadded: *mut u64) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsLun2(::windows::runtime::IUnknown);
impl IVdsLun2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints2(&self) -> ::windows::runtime::Result<VDS_HINTS2> {
        let mut result__: <VDS_HINTS2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_HINTS2>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints2(&self, phints2: *const VDS_HINTS2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(phints2)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsLun2 {
    type Vtable = IVdsLun2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3853756213, 40699, 18842, [128, 113, 67, 148, 217, 238, 111, 203]);
}
impl ::std::convert::From<IVdsLun2> for ::windows::runtime::IUnknown {
    fn from(value: IVdsLun2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsLun2> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsLun2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsLun2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsLun2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLun2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phints2: *mut VDS_HINTS2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phints2: *const VDS_HINTS2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsLunControllerPorts(::windows::runtime::IUnknown);
impl IVdsLunControllerPorts {
    pub unsafe fn AssociateControllerPorts(&self, pactivecontrollerportidarray: *const ::windows::runtime::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::runtime::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pactivecontrollerportidarray), ::std::mem::transmute(lnumberofactivecontrollerports), ::std::mem::transmute(pinactivecontrollerportidarray), ::std::mem::transmute(lnumberofinactivecontrollerports)).ok()
    }
    pub unsafe fn QueryActiveControllerPorts(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsLunControllerPorts {
    type Vtable = IVdsLunControllerPorts_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1159717478, 55917, 16490, [187, 96, 130, 229, 52, 248, 90, 235]);
}
impl ::std::convert::From<IVdsLunControllerPorts> for ::windows::runtime::IUnknown {
    fn from(value: IVdsLunControllerPorts) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsLunControllerPorts> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsLunControllerPorts) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsLunControllerPorts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsLunControllerPorts {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunControllerPorts_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pactivecontrollerportidarray: *const ::windows::runtime::GUID, lnumberofactivecontrollerports: i32, pinactivecontrollerportidarray: *const ::windows::runtime::GUID, lnumberofinactivecontrollerports: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsLunIscsi(::windows::runtime::IUnknown);
impl IVdsLunIscsi {
    pub unsafe fn AssociateTargets(&self, ptargetidarray: *const ::windows::runtime::GUID, lnumberoftargets: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ptargetidarray), ::std::mem::transmute(lnumberoftargets)).ok()
    }
    pub unsafe fn QueryAssociatedTargets(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsLunIscsi {
    type Vtable = IVdsLunIscsi_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(226238052, 46491, 17838, [184, 106, 44, 44, 198, 164, 32, 103]);
}
impl ::std::convert::From<IVdsLunIscsi> for ::windows::runtime::IUnknown {
    fn from(value: IVdsLunIscsi) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsLunIscsi> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsLunIscsi) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsLunIscsi {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsLunIscsi {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunIscsi_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ptargetidarray: *const ::windows::runtime::GUID, lnumberoftargets: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsLunMpio(::windows::runtime::IUnknown);
impl IVdsLunMpio {
    pub unsafe fn GetPathInfo(&self, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pppaths), ::std::mem::transmute(plnumberofpaths)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetLoadBalancePolicy(&self, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppolicy), ::std::mem::transmute(pppaths), ::std::mem::transmute(plnumberofpaths)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetLoadBalancePolicy(&self, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(policy), ::std::mem::transmute(ppaths), ::std::mem::transmute(lnumberofpaths)).ok()
    }
    pub unsafe fn GetSupportedLbPolicies(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsLunMpio {
    type Vtable = IVdsLunMpio_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2086648547, 13114, 18593, [169, 130, 51, 193, 87, 136, 205, 227]);
}
impl ::std::convert::From<IVdsLunMpio> for ::windows::runtime::IUnknown {
    fn from(value: IVdsLunMpio) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsLunMpio> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsLunMpio) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsLunMpio {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsLunMpio {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunMpio_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppaths: *mut *mut VDS_PATH_INFO, plnumberofpaths: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppolicy: *mut VDS_LOADBALANCE_POLICY_ENUM, pppaths: *mut *mut VDS_PATH_POLICY, plnumberofpaths: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, policy: VDS_LOADBALANCE_POLICY_ENUM, ppaths: *const VDS_PATH_POLICY, lnumberofpaths: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pullbflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsLunNaming(::windows::runtime::IUnknown);
impl IVdsLunNaming {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszfriendlyname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsLunNaming {
    type Vtable = IVdsLunNaming_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2423588043, 27470, 19848, [163, 77, 23, 186, 102, 31, 187, 6]);
}
impl ::std::convert::From<IVdsLunNaming> for ::windows::runtime::IUnknown {
    fn from(value: IVdsLunNaming) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsLunNaming> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsLunNaming) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsLunNaming {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsLunNaming {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunNaming_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsLunNumber(::windows::runtime::IUnknown);
impl IVdsLunNumber {
    pub unsafe fn GetLunNumber(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsLunNumber {
    type Vtable = IVdsLunNumber_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3556335174, 21683, 16889, [182, 120, 15, 24, 113, 68, 58, 8]);
}
impl ::std::convert::From<IVdsLunNumber> for ::windows::runtime::IUnknown {
    fn from(value: IVdsLunNumber) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsLunNumber> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsLunNumber) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsLunNumber {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsLunNumber {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunNumber_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pullunnumber: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsLunPlex(::windows::runtime::IUnknown);
impl IVdsLunPlex {
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_LUN_PLEX_PROP> {
        let mut result__: <VDS_LUN_PLEX_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_LUN_PLEX_PROP>(result__)
    }
    pub unsafe fn GetLun(&self) -> ::windows::runtime::Result<IVdsLun> {
        let mut result__: <IVdsLun as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsLun>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryExtents(&self, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppextentarray), ::std::mem::transmute(plnumberofextents)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryHints(&self) -> ::windows::runtime::Result<VDS_HINTS> {
        let mut result__: <VDS_HINTS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_HINTS>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ApplyHints(&self, phints: *const VDS_HINTS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(phints)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsLunPlex {
    type Vtable = IVdsLunPlex_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(249669520, 23854, 19131, [140, 153, 196, 129, 232, 190, 33, 56]);
}
impl ::std::convert::From<IVdsLunPlex> for ::windows::runtime::IUnknown {
    fn from(value: IVdsLunPlex) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsLunPlex> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsLunPlex) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsLunPlex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsLunPlex {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsLunPlex_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplexprop: *mut VDS_LUN_PLEX_PROP) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplun: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppextentarray: *mut *mut VDS_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phints: *mut VDS_HINTS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phints: *const VDS_HINTS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsMaintenance(::windows::runtime::IUnknown);
impl IVdsMaintenance {
    pub unsafe fn StartMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(operation)).ok()
    }
    pub unsafe fn StopMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(operation)).ok()
    }
    pub unsafe fn PulseMaintenance(&self, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(operation), ::std::mem::transmute(ulcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsMaintenance {
    type Vtable = IVdsMaintenance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3672895219, 34083, 18413, [162, 185, 5, 206, 204, 226, 161, 174]);
}
impl ::std::convert::From<IVdsMaintenance> for ::windows::runtime::IUnknown {
    fn from(value: IVdsMaintenance) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsMaintenance> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsMaintenance) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsMaintenance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsMaintenance {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsMaintenance_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operation: VDS_MAINTENANCE_OPERATION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, operation: VDS_MAINTENANCE_OPERATION, ulcount: u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsProvider(::windows::runtime::IUnknown);
impl IVdsProvider {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_PROVIDER_PROP> {
        let mut result__: <VDS_PROVIDER_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_PROVIDER_PROP>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsProvider {
    type Vtable = IVdsProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(281404789, 31108, 20097, [165, 107, 67, 31, 95, 146, 174, 66]);
}
impl ::std::convert::From<IVdsProvider> for ::windows::runtime::IUnknown {
    fn from(value: IVdsProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproviderprop: *mut VDS_PROVIDER_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsProviderPrivate(::windows::runtime::IUnknown);
impl IVdsProviderPrivate {
    pub unsafe fn GetObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, objectid: Param0, r#type: VDS_OBJECT_TYPE) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), objectid.into_param().abi(), ::std::mem::transmute(r#type), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnLoad<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pwszmachinename: Param0, pcallbackobject: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pwszmachinename.into_param().abi(), pcallbackobject.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnUnload<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, bforceunload: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), bforceunload.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsProviderPrivate {
    type Vtable = IVdsProviderPrivate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(301190465, 47080, 18687, [148, 114, 157, 255, 1, 138, 162, 146]);
}
impl ::std::convert::From<IVdsProviderPrivate> for ::windows::runtime::IUnknown {
    fn from(value: IVdsProviderPrivate) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsProviderPrivate> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsProviderPrivate) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsProviderPrivate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsProviderPrivate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProviderPrivate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, objectid: ::windows::runtime::GUID, r#type: VDS_OBJECT_TYPE, ppobjectunk: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszmachinename: super::super::Foundation::PWSTR, pcallbackobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bforceunload: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsProviderSupport(::windows::runtime::IUnknown);
impl IVdsProviderSupport {
    pub unsafe fn GetVersionSupport(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsProviderSupport {
    type Vtable = IVdsProviderSupport_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(389201427, 59641, 18947, [191, 188, 95, 97, 106, 166, 108, 225]);
}
impl ::std::convert::From<IVdsProviderSupport> for ::windows::runtime::IUnknown {
    fn from(value: IVdsProviderSupport) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsProviderSupport> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsProviderSupport) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsProviderSupport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsProviderSupport {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsProviderSupport_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulversionsupport: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsStoragePool(::windows::runtime::IUnknown);
impl IVdsStoragePool {
    pub unsafe fn GetProvider(&self) -> ::windows::runtime::Result<IVdsProvider> {
        let mut result__: <IVdsProvider as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsProvider>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_STORAGE_POOL_PROP> {
        let mut result__: <VDS_STORAGE_POOL_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_STORAGE_POOL_PROP>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributes(&self) -> ::windows::runtime::Result<VDS_POOL_ATTRIBUTES> {
        let mut result__: <VDS_POOL_ATTRIBUTES as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_POOL_ATTRIBUTES>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryDriveExtents(&self, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppextentarray), ::std::mem::transmute(plnumberofextents)).ok()
    }
    pub unsafe fn QueryAllocatedLuns(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn QueryAllocatedStoragePools(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsStoragePool {
    type Vtable = IVdsStoragePool_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2469177551, 3763, 19368, [150, 32, 34, 102, 93, 127, 132, 80]);
}
impl ::std::convert::From<IVdsStoragePool> for ::windows::runtime::IUnknown {
    fn from(value: IVdsStoragePool) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsStoragePool> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsStoragePool) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsStoragePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsStoragePool {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsStoragePool_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstoragepoolprop: *mut VDS_STORAGE_POOL_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstoragepoolattributes: *mut VDS_POOL_ATTRIBUTES) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppextentarray: *mut *mut VDS_STORAGE_POOL_DRIVE_EXTENT, plnumberofextents: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsSubSystem(::windows::runtime::IUnknown);
impl IVdsSubSystem {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<VDS_SUB_SYSTEM_PROP> {
        let mut result__: <VDS_SUB_SYSTEM_PROP as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_SUB_SYSTEM_PROP>(result__)
    }
    pub unsafe fn GetProvider(&self) -> ::windows::runtime::Result<IVdsProvider> {
        let mut result__: <IVdsProvider as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IVdsProvider>(result__)
    }
    pub unsafe fn QueryControllers(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn QueryLuns(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn QueryDrives(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn GetDrive(&self, sbusnumber: i16, sslotnumber: i16) -> ::windows::runtime::Result<IVdsDrive> {
        let mut result__: <IVdsDrive as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(sbusnumber), ::std::mem::transmute(sslotnumber), &mut result__).from_abi::<IVdsDrive>(result__)
    }
    pub unsafe fn Reenumerate(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn SetControllerStatus(&self, ponlinecontrolleridarray: *const ::windows::runtime::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::runtime::GUID, lnumberofofflinecontrollers: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(ponlinecontrolleridarray), ::std::mem::transmute(lnumberofonlinecontrollers), ::std::mem::transmute(pofflinecontrolleridarray), ::std::mem::transmute(lnumberofofflinecontrollers)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLun<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, pwszunmaskinglist: Param4, phints: *const VDS_HINTS) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), ::std::mem::transmute(ullsizeinbytes), ::std::mem::transmute(pdriveidarray), ::std::mem::transmute(lnumberofdrives), pwszunmaskinglist.into_param().abi(), ::std::mem::transmute(phints), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn ReplaceDrive<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, drivetobereplaced: Param0, replacementdrive: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), drivetobereplaced.into_param().abi(), replacementdrive.into_param().abi()).ok()
    }
    pub unsafe fn SetStatus(&self, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(status)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSize(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), ::std::mem::transmute(pdriveidarray), ::std::mem::transmute(lnumberofdrives), ::std::mem::transmute(phints), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsSubSystem {
    type Vtable = IVdsSubSystem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1875829459, 28048, 20369, [128, 226, 165, 199, 202, 172, 169, 216]);
}
impl ::std::convert::From<IVdsSubSystem> for ::windows::runtime::IUnknown {
    fn from(value: IVdsSubSystem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsSubSystem> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsSubSystem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsSubSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsSubSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystem_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psubsystemprop: *mut VDS_SUB_SYSTEM_PROP) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sbusnumber: i16, sslotnumber: i16, ppdrive: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ponlinecontrolleridarray: *const ::windows::runtime::GUID, lnumberofonlinecontrollers: i32, pofflinecontrolleridarray: *const ::windows::runtime::GUID, lnumberofofflinecontrollers: i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints: *const VDS_HINTS, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, drivetobereplaced: ::windows::runtime::GUID, replacementdrive: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, status: VDS_SUB_SYSTEM_STATUS) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, phints: *const VDS_HINTS, pullmaxlunsize: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsSubSystem2(::windows::runtime::IUnknown);
impl IVdsSubSystem2 {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperties2(&self) -> ::windows::runtime::Result<VDS_SUB_SYSTEM_PROP2> {
        let mut result__: <VDS_SUB_SYSTEM_PROP2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<VDS_SUB_SYSTEM_PROP2>(result__)
    }
    pub unsafe fn GetDrive2(&self, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32) -> ::windows::runtime::Result<IVdsDrive> {
        let mut result__: <IVdsDrive as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(sbusnumber), ::std::mem::transmute(sslotnumber), ::std::mem::transmute(ulenclosurenumber), &mut result__).from_abi::<IVdsDrive>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateLun2<'a, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, pwszunmaskinglist: Param4, phints2: *const VDS_HINTS2) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), ::std::mem::transmute(ullsizeinbytes), ::std::mem::transmute(pdriveidarray), ::std::mem::transmute(lnumberofdrives), pwszunmaskinglist.into_param().abi(), ::std::mem::transmute(phints2), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryMaxLunCreateSize2(&self, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(r#type), ::std::mem::transmute(pdriveidarray), ::std::mem::transmute(lnumberofdrives), ::std::mem::transmute(phints2), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsSubSystem2 {
    type Vtable = IVdsSubSystem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3194382133, 30720, 19063, [157, 156, 64, 248, 91, 135, 226, 146]);
}
impl ::std::convert::From<IVdsSubSystem2> for ::windows::runtime::IUnknown {
    fn from(value: IVdsSubSystem2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsSubSystem2> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsSubSystem2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsSubSystem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsSubSystem2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystem2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psubsystemprop2: *mut VDS_SUB_SYSTEM_PROP2) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sbusnumber: i16, sslotnumber: i16, ulenclosurenumber: u32, ppdrive: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: VDS_LUN_TYPE, ullsizeinbytes: u64, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, pwszunmaskinglist: super::super::Foundation::PWSTR, phints2: *const VDS_HINTS2, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, r#type: VDS_LUN_TYPE, pdriveidarray: *const ::windows::runtime::GUID, lnumberofdrives: i32, phints2: *const VDS_HINTS2, pullmaxlunsize: *mut u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsSubSystemInterconnect(::windows::runtime::IUnknown);
impl IVdsSubSystemInterconnect {
    pub unsafe fn GetSupportedInterconnects(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVdsSubSystemInterconnect {
    type Vtable = IVdsSubSystemInterconnect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2658116960, 49473, 18299, [131, 186, 11, 108, 56, 247, 254, 191]);
}
impl ::std::convert::From<IVdsSubSystemInterconnect> for ::windows::runtime::IUnknown {
    fn from(value: IVdsSubSystemInterconnect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsSubSystemInterconnect> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsSubSystemInterconnect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsSubSystemInterconnect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsSubSystemInterconnect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemInterconnect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulsupportedinterconnectsflag: *mut u32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsSubSystemIscsi(::windows::runtime::IUnknown);
impl IVdsSubSystemIscsi {
    pub unsafe fn QueryTargets(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    pub unsafe fn QueryPortals(&self) -> ::windows::runtime::Result<IEnumVdsObject> {
        let mut result__: <IEnumVdsObject as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumVdsObject>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwsziscsiname: Param0, pwszfriendlyname: Param1) -> ::windows::runtime::Result<IVdsAsync> {
        let mut result__: <IVdsAsync as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pwsziscsiname.into_param().abi(), pwszfriendlyname.into_param().abi(), &mut result__).from_abi::<IVdsAsync>(result__)
    }
    pub unsafe fn SetIpsecGroupPresharedKey(&self, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pipseckey)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsSubSystemIscsi {
    type Vtable = IVdsSubSystemIscsi_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2569327, 16592, 19269, [140, 236, 89, 6, 220, 3, 128, 200]);
}
impl ::std::convert::From<IVdsSubSystemIscsi> for ::windows::runtime::IUnknown {
    fn from(value: IVdsSubSystemIscsi) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsSubSystemIscsi> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsSubSystemIscsi) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsSubSystemIscsi {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsSubSystemIscsi {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemIscsi_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwsziscsiname: super::super::Foundation::PWSTR, pwszfriendlyname: super::super::Foundation::PWSTR, ppasync: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pipseckey: *const VDS_ISCSI_IPSEC_KEY) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IVdsSubSystemNaming(::windows::runtime::IUnknown);
impl IVdsSubSystemNaming {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszfriendlyname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pwszfriendlyname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVdsSubSystemNaming {
    type Vtable = IVdsSubSystemNaming_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(225508003, 40148, 18688, [170, 32, 105, 129, 182, 170, 252, 117]);
}
impl ::std::convert::From<IVdsSubSystemNaming> for ::windows::runtime::IUnknown {
    fn from(value: IVdsSubSystemNaming) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVdsSubSystemNaming> for ::windows::runtime::IUnknown {
    fn from(value: &IVdsSubSystemNaming) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IVdsSubSystemNaming {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IVdsSubSystemNaming {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVdsSubSystemNaming_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pwszfriendlyname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
pub const MAX_FS_ALLOWED_CLUSTER_SIZES_SIZE: u32 = 32u32;
pub const MAX_FS_FORMAT_SUPPORT_NAME_SIZE: u32 = 32u32;
pub const MAX_FS_NAME_SIZE: u32 = 8u32;
impl ::std::clone::Clone for VDS_ASYNC_OUTPUT {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT {
    pub r#type: VDS_ASYNC_OUTPUT_TYPE,
    pub Anonymous: VDS_ASYNC_OUTPUT_0,
}
impl VDS_ASYNC_OUTPUT {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
impl ::std::clone::Clone for VDS_ASYNC_OUTPUT_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
pub union VDS_ASYNC_OUTPUT_0 {
    pub cp: VDS_ASYNC_OUTPUT_0_2,
    pub cv: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_5>,
    pub bvp: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_0>,
    pub sv: VDS_ASYNC_OUTPUT_0_7,
    pub cl: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_1>,
    pub ct: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_4>,
    pub cpg: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_3>,
    pub cvd: ::std::mem::ManuallyDrop<VDS_ASYNC_OUTPUT_0_6>,
}
impl VDS_ASYNC_OUTPUT_0 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_0 {
    pub pVolumeUnk: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl VDS_ASYNC_OUTPUT_0_0 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ASYNC_OUTPUT_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_bvp").field("pVolumeUnk", &self.pVolumeUnk).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.pVolumeUnk == other.pVolumeUnk
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0_0 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_1 {
    pub pLunUnk: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl VDS_ASYNC_OUTPUT_0_1 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ASYNC_OUTPUT_0_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_cl").field("pLunUnk", &self.pLunUnk).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pLunUnk == other.pLunUnk
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0_1 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_2 {
    pub ullOffset: u64,
    pub volumeId: ::windows::runtime::GUID,
}
impl VDS_ASYNC_OUTPUT_0_2 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ASYNC_OUTPUT_0_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_cp").field("ullOffset", &self.ullOffset).field("volumeId", &self.volumeId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_2 {
    fn eq(&self, other: &Self) -> bool {
        self.ullOffset == other.ullOffset && self.volumeId == other.volumeId
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0_2 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_3 {
    pub pPortalGroupUnk: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl VDS_ASYNC_OUTPUT_0_3 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ASYNC_OUTPUT_0_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_cpg").field("pPortalGroupUnk", &self.pPortalGroupUnk).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_3 {
    fn eq(&self, other: &Self) -> bool {
        self.pPortalGroupUnk == other.pPortalGroupUnk
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0_3 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0_3 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_4 {
    pub pTargetUnk: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl VDS_ASYNC_OUTPUT_0_4 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ASYNC_OUTPUT_0_4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_ct").field("pTargetUnk", &self.pTargetUnk).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_4 {
    fn eq(&self, other: &Self) -> bool {
        self.pTargetUnk == other.pTargetUnk
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0_4 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0_4 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_5 {
    pub pVolumeUnk: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl VDS_ASYNC_OUTPUT_0_5 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0_5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ASYNC_OUTPUT_0_5 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_cv").field("pVolumeUnk", &self.pVolumeUnk).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_5 {
    fn eq(&self, other: &Self) -> bool {
        self.pVolumeUnk == other.pVolumeUnk
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0_5 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0_5 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_6 {
    pub pVDiskUnk: ::std::option::Option<::windows::runtime::IUnknown>,
}
impl VDS_ASYNC_OUTPUT_0_6 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0_6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ASYNC_OUTPUT_0_6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_cvd").field("pVDiskUnk", &self.pVDiskUnk).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_6 {
    fn eq(&self, other: &Self) -> bool {
        self.pVDiskUnk == other.pVDiskUnk
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0_6 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0_6 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_ASYNC_OUTPUT_0_7 {
    pub ullReclaimedBytes: u64,
}
impl VDS_ASYNC_OUTPUT_0_7 {}
impl ::std::default::Default for VDS_ASYNC_OUTPUT_0_7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ASYNC_OUTPUT_0_7 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_sv").field("ullReclaimedBytes", &self.ullReclaimedBytes).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ASYNC_OUTPUT_0_7 {
    fn eq(&self, other: &Self) -> bool {
        self.ullReclaimedBytes == other.ullReclaimedBytes
    }
}
impl ::std::cmp::Eq for VDS_ASYNC_OUTPUT_0_7 {}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_0_7 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_ASYNC_OUTPUT_TYPE(pub i32);
pub const VDS_ASYNCOUT_UNKNOWN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(0i32);
pub const VDS_ASYNCOUT_CREATEVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(1i32);
pub const VDS_ASYNCOUT_EXTENDVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(2i32);
pub const VDS_ASYNCOUT_SHRINKVOLUME: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(3i32);
pub const VDS_ASYNCOUT_ADDVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(4i32);
pub const VDS_ASYNCOUT_BREAKVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(5i32);
pub const VDS_ASYNCOUT_REMOVEVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(6i32);
pub const VDS_ASYNCOUT_REPAIRVOLUMEPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(7i32);
pub const VDS_ASYNCOUT_RECOVERPACK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(8i32);
pub const VDS_ASYNCOUT_REPLACEDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(9i32);
pub const VDS_ASYNCOUT_CREATEPARTITION: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(10i32);
pub const VDS_ASYNCOUT_CLEAN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(11i32);
pub const VDS_ASYNCOUT_CREATELUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(50i32);
pub const VDS_ASYNCOUT_ADDLUNPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(52i32);
pub const VDS_ASYNCOUT_REMOVELUNPLEX: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(53i32);
pub const VDS_ASYNCOUT_EXTENDLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(54i32);
pub const VDS_ASYNCOUT_SHRINKLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(55i32);
pub const VDS_ASYNCOUT_RECOVERLUN: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(56i32);
pub const VDS_ASYNCOUT_LOGINTOTARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(60i32);
pub const VDS_ASYNCOUT_LOGOUTFROMTARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(61i32);
pub const VDS_ASYNCOUT_CREATETARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(62i32);
pub const VDS_ASYNCOUT_CREATEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(63i32);
pub const VDS_ASYNCOUT_DELETETARGET: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(64i32);
pub const VDS_ASYNCOUT_ADDPORTAL: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(65i32);
pub const VDS_ASYNCOUT_REMOVEPORTAL: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(66i32);
pub const VDS_ASYNCOUT_DELETEPORTALGROUP: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(67i32);
pub const VDS_ASYNCOUT_FORMAT: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(101i32);
pub const VDS_ASYNCOUT_CREATE_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(200i32);
pub const VDS_ASYNCOUT_ATTACH_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(201i32);
pub const VDS_ASYNCOUT_COMPACT_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(202i32);
pub const VDS_ASYNCOUT_MERGE_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(203i32);
pub const VDS_ASYNCOUT_EXPAND_VDISK: VDS_ASYNC_OUTPUT_TYPE = VDS_ASYNC_OUTPUT_TYPE(204i32);
impl ::std::convert::From<i32> for VDS_ASYNC_OUTPUT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_ASYNC_OUTPUT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VDS_ATTACH_VIRTUAL_DISK_FLAG_USE_FILE_ACL: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_CONTROLLER_NOTIFICATION {
    pub ulEvent: VDS_NF_CONTROLLER,
    pub controllerId: ::windows::runtime::GUID,
}
impl VDS_CONTROLLER_NOTIFICATION {}
impl ::std::default::Default for VDS_CONTROLLER_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_CONTROLLER_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_CONTROLLER_NOTIFICATION").field("ulEvent", &self.ulEvent).field("controllerId", &self.controllerId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_CONTROLLER_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.controllerId == other.controllerId
    }
}
impl ::std::cmp::Eq for VDS_CONTROLLER_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_CONTROLLER_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_CONTROLLER_PROP {
    pub id: ::windows::runtime::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub status: VDS_CONTROLLER_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfPorts: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_CONTROLLER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_CONTROLLER_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_CONTROLLER_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_CONTROLLER_PROP").field("id", &self.id).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("status", &self.status).field("health", &self.health).field("sNumberOfPorts", &self.sNumberOfPorts).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_CONTROLLER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.status == other.status && self.health == other.health && self.sNumberOfPorts == other.sNumberOfPorts
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_CONTROLLER_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_CONTROLLER_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_CONTROLLER_STATUS(pub i32);
pub const VDS_CS_UNKNOWN: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(0i32);
pub const VDS_CS_ONLINE: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(1i32);
pub const VDS_CS_NOT_READY: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(2i32);
pub const VDS_CS_OFFLINE: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(4i32);
pub const VDS_CS_FAILED: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(5i32);
pub const VDS_CS_REMOVED: VDS_CONTROLLER_STATUS = VDS_CONTROLLER_STATUS(8i32);
impl ::std::convert::From<i32> for VDS_CONTROLLER_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_CONTROLLER_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_DISK_NOTIFICATION {
    pub ulEvent: VDS_NF_DISK,
    pub diskId: ::windows::runtime::GUID,
}
impl VDS_DISK_NOTIFICATION {}
impl ::std::default::Default for VDS_DISK_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_DISK_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_DISK_NOTIFICATION").field("ulEvent", &self.ulEvent).field("diskId", &self.diskId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_DISK_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.diskId == other.diskId
    }
}
impl ::std::cmp::Eq for VDS_DISK_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_DISK_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_EXTENT {
    pub id: ::windows::runtime::GUID,
    pub LunId: ::windows::runtime::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_DRIVE_EXTENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_DRIVE_EXTENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_DRIVE_EXTENT").field("id", &self.id).field("LunId", &self.LunId).field("ullSize", &self.ullSize).field("bUsed", &self.bUsed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_DRIVE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.LunId == other.LunId && self.ullSize == other.ullSize && self.bUsed == other.bUsed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_DRIVE_EXTENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_DRIVE_FLAG(pub i32);
pub const VDS_DRF_HOTSPARE: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(1i32);
pub const VDS_DRF_ASSIGNED: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(2i32);
pub const VDS_DRF_UNASSIGNED: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(4i32);
pub const VDS_DRF_HOTSPARE_IN_USE: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(8i32);
pub const VDS_DRF_HOTSPARE_STANDBY: VDS_DRIVE_FLAG = VDS_DRIVE_FLAG(16i32);
impl ::std::convert::From<i32> for VDS_DRIVE_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_DRIVE_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_DRIVE_LETTER_NOTIFICATION {
    pub ulEvent: u32,
    pub wcLetter: u16,
    pub volumeId: ::windows::runtime::GUID,
}
impl VDS_DRIVE_LETTER_NOTIFICATION {}
impl ::std::default::Default for VDS_DRIVE_LETTER_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_DRIVE_LETTER_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_DRIVE_LETTER_NOTIFICATION").field("ulEvent", &self.ulEvent).field("wcLetter", &self.wcLetter).field("volumeId", &self.volumeId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_DRIVE_LETTER_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.wcLetter == other.wcLetter && self.volumeId == other.volumeId
    }
}
impl ::std::cmp::Eq for VDS_DRIVE_LETTER_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_DRIVE_LETTER_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_DRIVE_NOTIFICATION {
    pub ulEvent: VDS_NF_DRIVE,
    pub driveId: ::windows::runtime::GUID,
}
impl VDS_DRIVE_NOTIFICATION {}
impl ::std::default::Default for VDS_DRIVE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_DRIVE_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_DRIVE_NOTIFICATION").field("ulEvent", &self.ulEvent).field("driveId", &self.driveId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_DRIVE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.driveId == other.driveId
    }
}
impl ::std::cmp::Eq for VDS_DRIVE_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_DRIVE_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_PROP {
    pub id: ::windows::runtime::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_DRIVE_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_DRIVE_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_DRIVE_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_DRIVE_PROP")
            .field("id", &self.id)
            .field("ullSize", &self.ullSize)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sInternalBusNumber", &self.sInternalBusNumber)
            .field("sSlotNumber", &self.sSlotNumber)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_DRIVE_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.ulFlags == other.ulFlags && self.status == other.status && self.health == other.health && self.sInternalBusNumber == other.sInternalBusNumber && self.sSlotNumber == other.sSlotNumber
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_DRIVE_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_DRIVE_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_DRIVE_PROP2 {
    pub id: ::windows::runtime::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub status: VDS_DRIVE_STATUS,
    pub health: VDS_HEALTH,
    pub sInternalBusNumber: i16,
    pub sSlotNumber: i16,
    pub ulEnclosureNumber: u32,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub ulSpindleSpeed: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_DRIVE_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_DRIVE_PROP2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_DRIVE_PROP2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_DRIVE_PROP2")
            .field("id", &self.id)
            .field("ullSize", &self.ullSize)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sInternalBusNumber", &self.sInternalBusNumber)
            .field("sSlotNumber", &self.sSlotNumber)
            .field("ulEnclosureNumber", &self.ulEnclosureNumber)
            .field("busType", &self.busType)
            .field("ulSpindleSpeed", &self.ulSpindleSpeed)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_DRIVE_PROP2 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.ulFlags == other.ulFlags && self.status == other.status && self.health == other.health && self.sInternalBusNumber == other.sInternalBusNumber && self.sSlotNumber == other.sSlotNumber && self.ulEnclosureNumber == other.ulEnclosureNumber && self.busType == other.busType && self.ulSpindleSpeed == other.ulSpindleSpeed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_DRIVE_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_DRIVE_PROP2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_DRIVE_STATUS(pub i32);
pub const VDS_DRS_UNKNOWN: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(0i32);
pub const VDS_DRS_ONLINE: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(1i32);
pub const VDS_DRS_NOT_READY: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(2i32);
pub const VDS_DRS_OFFLINE: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(4i32);
pub const VDS_DRS_FAILED: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(5i32);
pub const VDS_DRS_REMOVED: VDS_DRIVE_STATUS = VDS_DRIVE_STATUS(8i32);
impl ::std::convert::From<i32> for VDS_DRIVE_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_DRIVE_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VDS_E_ACCESS_DENIED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212249i32 as _);
pub const VDS_E_ACTIVE_PARTITION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212232i32 as _);
pub const VDS_E_ADDRESSES_INCOMPLETELY_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211517i32 as _);
pub const VDS_E_ALIGN_BEYOND_FIRST_CYLINDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211949i32 as _);
pub const VDS_E_ALIGN_IS_ZERO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211888i32 as _);
pub const VDS_E_ALIGN_NOT_A_POWER_OF_TWO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211889i32 as _);
pub const VDS_E_ALIGN_NOT_SECTOR_SIZE_MULTIPLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211948i32 as _);
pub const VDS_E_ALIGN_NOT_ZERO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211947i32 as _);
pub const VDS_E_ALREADY_REGISTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212285i32 as _);
pub const VDS_E_ANOTHER_CALL_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212284i32 as _);
pub const VDS_E_ASSOCIATED_LUNS_EXIST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211509i32 as _);
pub const VDS_E_ASSOCIATED_PORTALS_EXIST: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211508i32 as _);
pub const VDS_E_ASYNC_OBJECT_FAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212210i32 as _);
pub const VDS_E_BAD_BOOT_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211898i32 as _);
pub const VDS_E_BAD_COOKIE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212271i32 as _);
pub const VDS_E_BAD_LABEL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212247i32 as _);
pub const VDS_E_BAD_PNP_MESSAGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212017i32 as _);
pub const VDS_E_BAD_PROVIDER_DATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212223i32 as _);
pub const VDS_E_BAD_REVISION_NUMBER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211880i32 as _);
pub const VDS_E_BLOCK_CLUSTERED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210749i32 as _);
pub const VDS_E_BOOT_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211257i32 as _);
pub const VDS_E_BOOT_PAGEFILE_DRIVE_LETTER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210994i32 as _);
pub const VDS_E_BOOT_PARTITION_NUMBER_CHANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212234i32 as _);
pub const VDS_E_CACHE_CORRUPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211946i32 as _);
pub const VDS_E_CANCEL_TOO_LATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212276i32 as _);
pub const VDS_E_CANNOT_CLEAR_VOLUME_FLAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211945i32 as _);
pub const VDS_E_CANNOT_EXTEND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212274i32 as _);
pub const VDS_E_CANNOT_SHRINK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212002i32 as _);
pub const VDS_E_CANT_INVALIDATE_FVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211886i32 as _);
pub const VDS_E_CANT_QUICK_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212246i32 as _);
pub const VDS_E_CLEAN_WITH_BOOTBACKING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210743i32 as _);
pub const VDS_E_CLEAN_WITH_CRITICAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210990i32 as _);
pub const VDS_E_CLEAN_WITH_DATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210992i32 as _);
pub const VDS_E_CLEAN_WITH_OEM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210991i32 as _);
pub const VDS_E_CLUSTER_COUNT_BEYOND_32BITS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212240i32 as _);
pub const VDS_E_CLUSTER_SIZE_TOO_BIG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212241i32 as _);
pub const VDS_E_CLUSTER_SIZE_TOO_SMALL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212242i32 as _);
pub const VDS_E_COMPRESSION_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210984i32 as _);
pub const VDS_E_CONFIG_LIMIT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211976i32 as _);
pub const VDS_E_CORRUPT_EXTENT_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212021i32 as _);
pub const VDS_E_CORRUPT_NOTIFICATION_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211990i32 as _);
pub const VDS_E_CORRUPT_PARTITION_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212023i32 as _);
pub const VDS_E_CORRUPT_VOLUME_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212029i32 as _);
pub const VDS_E_CRASHDUMP_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211250i32 as _);
pub const VDS_E_CRITICAL_PLEX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211906i32 as _);
pub const VDS_E_DELETE_WITH_BOOTBACKING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210745i32 as _);
pub const VDS_E_DELETE_WITH_CRITICAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210993i32 as _);
pub const VDS_E_DEVICE_IN_USE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212269i32 as _);
pub const VDS_E_DISK_BEING_CLEANED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211944i32 as _);
pub const VDS_E_DISK_CONFIGURATION_CORRUPTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211975i32 as _);
pub const VDS_E_DISK_CONFIGURATION_NOT_IN_SYNC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211974i32 as _);
pub const VDS_E_DISK_CONFIGURATION_UPDATE_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211973i32 as _);
pub const VDS_E_DISK_DYNAMIC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211972i32 as _);
pub const VDS_E_DISK_HAS_BANDS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210748i32 as _);
pub const VDS_E_DISK_IN_USE_BY_VOLUME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212212i32 as _);
pub const VDS_E_DISK_IO_FAILING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211968i32 as _);
pub const VDS_E_DISK_IS_OFFLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211254i32 as _);
pub const VDS_E_DISK_IS_READ_ONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211253i32 as _);
pub const VDS_E_DISK_LAYOUT_PARTITIONS_TOO_SMALL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211969i32 as _);
pub const VDS_E_DISK_NOT_CONVERTIBLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211943i32 as _);
pub const VDS_E_DISK_NOT_CONVERTIBLE_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210971i32 as _);
pub const VDS_E_DISK_NOT_EMPTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212268i32 as _);
pub const VDS_E_DISK_NOT_FOUND_IN_PACK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211987i32 as _);
pub const VDS_E_DISK_NOT_IMPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212206i32 as _);
pub const VDS_E_DISK_NOT_INITIALIZED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212265i32 as _);
pub const VDS_E_DISK_NOT_LOADED_TO_CACHE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212217i32 as _);
pub const VDS_E_DISK_NOT_MISSING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212031i32 as _);
pub const VDS_E_DISK_NOT_OFFLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211883i32 as _);
pub const VDS_E_DISK_NOT_ONLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212213i32 as _);
pub const VDS_E_DISK_PNP_REG_CORRUPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212203i32 as _);
pub const VDS_E_DISK_REMOVEABLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211942i32 as _);
pub const VDS_E_DISK_REMOVEABLE_NOT_EMPTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211941i32 as _);
pub const VDS_E_DISTINCT_VOLUME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211909i32 as _);
pub const VDS_E_DMADMIN_CORRUPT_NOTIFICATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212252i32 as _);
pub const VDS_E_DMADMIN_METHOD_CALL_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212256i32 as _);
pub const VDS_E_DMADMIN_SERVICE_CONNECTION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212261i32 as _);
pub const VDS_E_DRIVER_INTERNAL_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212027i32 as _);
pub const VDS_E_DRIVER_INVALID_PARAM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212004i32 as _);
pub const VDS_E_DRIVER_NO_PACK_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212019i32 as _);
pub const VDS_E_DRIVER_OBJECT_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211971i32 as _);
pub const VDS_E_DRIVE_LETTER_NOT_FREE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211940i32 as _);
pub const VDS_E_DUPLICATE_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211986i32 as _);
pub const VDS_E_DUP_EMPTY_PACK_GUID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212020i32 as _);
pub const VDS_E_DYNAMIC_DISKS_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211967i32 as _);
pub const VDS_E_EXTEND_FILE_SYSTEM_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212186i32 as _);
pub const VDS_E_EXTEND_MULTIPLE_DISKS_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211939i32 as _);
pub const VDS_E_EXTEND_TOO_MANY_CLUSTERS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210968i32 as _);
pub const VDS_E_EXTEND_UNKNOWN_FILESYSTEM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210967i32 as _);
pub const VDS_E_EXTENT_EXCEEDS_DISK_FREE_SPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212011i32 as _);
pub const VDS_E_EXTENT_SIZE_LESS_THAN_MIN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212237i32 as _);
pub const VDS_E_FAILED_TO_OFFLINE_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211881i32 as _);
pub const VDS_E_FAILED_TO_ONLINE_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211882i32 as _);
pub const VDS_E_FAT32_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210987i32 as _);
pub const VDS_E_FAT_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210986i32 as _);
pub const VDS_E_FAULT_TOLERANT_DISKS_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211966i32 as _);
pub const VDS_E_FLAG_ALREADY_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211911i32 as _);
pub const VDS_E_FORMAT_CRITICAL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210989i32 as _);
pub const VDS_E_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210985i32 as _);
pub const VDS_E_FORMAT_WITH_BOOTBACKING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210744i32 as _);
pub const VDS_E_FS_NOT_DETERMINED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211885i32 as _);
pub const VDS_E_GET_SAN_POLICY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211259i32 as _);
pub const VDS_E_GPT_ATTRIBUTES_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211965i32 as _);
pub const VDS_E_HIBERNATION_FILE_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211251i32 as _);
pub const VDS_E_IA64_BOOT_MIRRORED_TO_MBR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212198i32 as _);
pub const VDS_E_IMPORT_SET_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212207i32 as _);
pub const VDS_E_INCOMPATIBLE_FILE_SYSTEM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212251i32 as _);
pub const VDS_E_INCOMPATIBLE_MEDIA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212250i32 as _);
pub const VDS_E_INCORRECT_BOOT_VOLUME_EXTENT_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211260i32 as _);
pub const VDS_E_INCORRECT_SYSTEM_VOLUME_EXTENT_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211248i32 as _);
pub const VDS_E_INITIALIZED_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212287i32 as _);
pub const VDS_E_INITIALIZE_NOT_CALLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212286i32 as _);
pub const VDS_E_INITIATOR_ADAPTER_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211008i32 as _);
pub const VDS_E_INITIATOR_SPECIFIC_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211513i32 as _);
pub const VDS_E_INTERNAL_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212216i32 as _);
pub const VDS_E_INVALID_BLOCK_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211982i32 as _);
pub const VDS_E_INVALID_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212007i32 as _);
pub const VDS_E_INVALID_DISK_COUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211994i32 as _);
pub const VDS_E_INVALID_DRIVE_LETTER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211938i32 as _);
pub const VDS_E_INVALID_DRIVE_LETTER_COUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211937i32 as _);
pub const VDS_E_INVALID_ENUMERATOR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212028i32 as _);
pub const VDS_E_INVALID_EXTENT_COUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211993i32 as _);
pub const VDS_E_INVALID_FS_FLAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211936i32 as _);
pub const VDS_E_INVALID_FS_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211935i32 as _);
pub const VDS_E_INVALID_IP_ADDRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210997i32 as _);
pub const VDS_E_INVALID_ISCSI_PATH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210980i32 as _);
pub const VDS_E_INVALID_ISCSI_TARGET_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211005i32 as _);
pub const VDS_E_INVALID_MEMBER_COUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211998i32 as _);
pub const VDS_E_INVALID_MEMBER_ORDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211996i32 as _);
pub const VDS_E_INVALID_OBJECT_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211934i32 as _);
pub const VDS_E_INVALID_OPERATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212267i32 as _);
pub const VDS_E_INVALID_PACK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212006i32 as _);
pub const VDS_E_INVALID_PARTITION_LAYOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211933i32 as _);
pub const VDS_E_INVALID_PARTITION_STYLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211932i32 as _);
pub const VDS_E_INVALID_PARTITION_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211931i32 as _);
pub const VDS_E_INVALID_PATH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210981i32 as _);
pub const VDS_E_INVALID_PLEX_BLOCK_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211978i32 as _);
pub const VDS_E_INVALID_PLEX_COUNT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211999i32 as _);
pub const VDS_E_INVALID_PLEX_GUID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211988i32 as _);
pub const VDS_E_INVALID_PLEX_ORDER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211997i32 as _);
pub const VDS_E_INVALID_PLEX_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211979i32 as _);
pub const VDS_E_INVALID_PORT_PATH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211006i32 as _);
pub const VDS_E_INVALID_PROVIDER_CLSID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211930i32 as _);
pub const VDS_E_INVALID_PROVIDER_ID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211929i32 as _);
pub const VDS_E_INVALID_PROVIDER_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211928i32 as _);
pub const VDS_E_INVALID_PROVIDER_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211927i32 as _);
pub const VDS_E_INVALID_PROVIDER_VERSION_GUID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211926i32 as _);
pub const VDS_E_INVALID_PROVIDER_VERSION_STRING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211925i32 as _);
pub const VDS_E_INVALID_QUERY_PROVIDER_FLAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211924i32 as _);
pub const VDS_E_INVALID_SECTOR_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211984i32 as _);
pub const VDS_E_INVALID_SERVICE_FLAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211923i32 as _);
pub const VDS_E_INVALID_SHRINK_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211241i32 as _);
pub const VDS_E_INVALID_SPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212282i32 as _);
pub const VDS_E_INVALID_STATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210747i32 as _);
pub const VDS_E_INVALID_STRIPE_SIZE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211995i32 as _);
pub const VDS_E_INVALID_VOLUME_FLAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211922i32 as _);
pub const VDS_E_INVALID_VOLUME_LENGTH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211954i32 as _);
pub const VDS_E_INVALID_VOLUME_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211899i32 as _);
pub const VDS_E_IO_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212245i32 as _);
pub const VDS_E_ISCSI_CHAP_SECRET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210998i32 as _);
pub const VDS_E_ISCSI_GET_IKE_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211003i32 as _);
pub const VDS_E_ISCSI_GROUP_PRESHARE_KEY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210999i32 as _);
pub const VDS_E_ISCSI_INITIATOR_NODE_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211000i32 as _);
pub const VDS_E_ISCSI_LOGIN_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211512i32 as _);
pub const VDS_E_ISCSI_LOGOUT_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211511i32 as _);
pub const VDS_E_ISCSI_LOGOUT_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211504i32 as _);
pub const VDS_E_ISCSI_SESSION_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211510i32 as _);
pub const VDS_E_ISCSI_SET_IKE_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211002i32 as _);
pub const VDS_E_LAST_VALID_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211985i32 as _);
pub const VDS_E_LBN_REMAP_ENABLED_FLAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212202i32 as _);
pub const VDS_E_LDM_TIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212191i32 as _);
pub const VDS_E_LEGACY_VOLUME_FORMAT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212230i32 as _);
pub const VDS_E_LOG_UPDATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211897i32 as _);
pub const VDS_E_LUN_DISK_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211239i32 as _);
pub const VDS_E_LUN_DISK_MISSING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211240i32 as _);
pub const VDS_E_LUN_DISK_NOT_READY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211238i32 as _);
pub const VDS_E_LUN_DISK_NO_MEDIA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211237i32 as _);
pub const VDS_E_LUN_DISK_READ_ONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210978i32 as _);
pub const VDS_E_LUN_DYNAMIC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210976i32 as _);
pub const VDS_E_LUN_DYNAMIC_OFFLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210975i32 as _);
pub const VDS_E_LUN_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211234i32 as _);
pub const VDS_E_LUN_NOT_READY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211236i32 as _);
pub const VDS_E_LUN_OFFLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211235i32 as _);
pub const VDS_E_LUN_SHRINK_GPT_HEADER: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210974i32 as _);
pub const VDS_E_LUN_UPDATE_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210977i32 as _);
pub const VDS_E_MAX_USABLE_MBR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212184i32 as _);
pub const VDS_E_MEDIA_WRITE_PROTECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212248i32 as _);
pub const VDS_E_MEMBER_IS_HEALTHY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211964i32 as _);
pub const VDS_E_MEMBER_MISSING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211958i32 as _);
pub const VDS_E_MEMBER_REGENERATING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211963i32 as _);
pub const VDS_E_MEMBER_SIZE_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212010i32 as _);
pub const VDS_E_MIGRATE_OPEN_VOLUME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212228i32 as _);
pub const VDS_E_MIRROR_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210973i32 as _);
pub const VDS_E_MISSING_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212204i32 as _);
pub const VDS_E_MULTIPLE_DISCOVERY_DOMAINS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211506i32 as _);
pub const VDS_E_MULTIPLE_PACKS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212001i32 as _);
pub const VDS_E_NAME_NOT_UNIQUE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211519i32 as _);
pub const VDS_E_NON_CONTIGUOUS_DATA_PARTITIONS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212229i32 as _);
pub const VDS_E_NOT_AN_UNALLOCATED_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212264i32 as _);
pub const VDS_E_NOT_ENOUGH_DRIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212272i32 as _);
pub const VDS_E_NOT_ENOUGH_SPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212273i32 as _);
pub const VDS_E_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212288i32 as _);
pub const VDS_E_NO_DISCOVERY_DOMAIN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211507i32 as _);
pub const VDS_E_NO_DISKS_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212258i32 as _);
pub const VDS_E_NO_DISK_PATHNAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211505i32 as _);
pub const VDS_E_NO_DRIVELETTER_FLAG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212201i32 as _);
pub const VDS_E_NO_EXTENTS_FOR_PLEX: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211980i32 as _);
pub const VDS_E_NO_EXTENTS_FOR_VOLUME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212218i32 as _);
pub const VDS_E_NO_FREE_SPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212233i32 as _);
pub const VDS_E_NO_HEALTHY_DISKS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211977i32 as _);
pub const VDS_E_NO_IMPORT_TARGET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211501i32 as _);
pub const VDS_E_NO_MAINTENANCE_MODE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210750i32 as _);
pub const VDS_E_NO_MEDIA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212270i32 as _);
pub const VDS_E_NO_PNP_DISK_ARRIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212016i32 as _);
pub const VDS_E_NO_PNP_DISK_REMOVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212014i32 as _);
pub const VDS_E_NO_PNP_VOLUME_ARRIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212015i32 as _);
pub const VDS_E_NO_PNP_VOLUME_REMOVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212013i32 as _);
pub const VDS_E_NO_POOL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210752i32 as _);
pub const VDS_E_NO_POOL_CREATED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210751i32 as _);
pub const VDS_E_NO_SOFTWARE_PROVIDERS_LOADED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212032i32 as _);
pub const VDS_E_NO_VALID_LOG_COPIES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211894i32 as _);
pub const VDS_E_NO_VOLUME_LAYOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212030i32 as _);
pub const VDS_E_NO_VOLUME_PATHNAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211503i32 as _);
pub const VDS_E_NTFS_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210988i32 as _);
pub const VDS_E_OBJECT_DELETED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212277i32 as _);
pub const VDS_E_OBJECT_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212259i32 as _);
pub const VDS_E_OBJECT_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212283i32 as _);
pub const VDS_E_OBJECT_OUT_OF_SYNC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212205i32 as _);
pub const VDS_E_OBJECT_STATUS_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212239i32 as _);
pub const VDS_E_OFFLINE_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210970i32 as _);
pub const VDS_E_ONE_EXTENT_PER_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211983i32 as _);
pub const VDS_E_ONLINE_PACK_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212188i32 as _);
pub const VDS_E_OPERATION_CANCELED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212275i32 as _);
pub const VDS_E_OPERATION_DENIED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212278i32 as _);
pub const VDS_E_OPERATION_PENDING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212279i32 as _);
pub const VDS_E_PACK_NAME_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211962i32 as _);
pub const VDS_E_PACK_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212208i32 as _);
pub const VDS_E_PACK_OFFLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212220i32 as _);
pub const VDS_E_PACK_ONLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212000i32 as _);
pub const VDS_E_PAGEFILE_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211252i32 as _);
pub const VDS_E_PARTITION_LDM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211891i32 as _);
pub const VDS_E_PARTITION_LIMIT_REACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212281i32 as _);
pub const VDS_E_PARTITION_MSR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211892i32 as _);
pub const VDS_E_PARTITION_NON_DATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211907i32 as _);
pub const VDS_E_PARTITION_NOT_CYLINDER_ALIGNED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211970i32 as _);
pub const VDS_E_PARTITION_NOT_EMPTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212280i32 as _);
pub const VDS_E_PARTITION_NOT_OEM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211921i32 as _);
pub const VDS_E_PARTITION_OF_UNKNOWN_TYPE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212231i32 as _);
pub const VDS_E_PARTITION_PROTECTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211920i32 as _);
pub const VDS_E_PARTITION_STYLE_MISMATCH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211919i32 as _);
pub const VDS_E_PATH_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212266i32 as _);
pub const VDS_E_PLEX_IS_HEALTHY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211961i32 as _);
pub const VDS_E_PLEX_LAST_ACTIVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211960i32 as _);
pub const VDS_E_PLEX_MISSING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211959i32 as _);
pub const VDS_E_PLEX_NOT_LOADED_TO_CACHE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211893i32 as _);
pub const VDS_E_PLEX_REGENERATING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211957i32 as _);
pub const VDS_E_PLEX_SIZE_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211981i32 as _);
pub const VDS_E_PROVIDER_CACHE_CORRUPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212257i32 as _);
pub const VDS_E_PROVIDER_CACHE_OUTOFSYNC: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211502i32 as _);
pub const VDS_E_PROVIDER_EXITING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212012i32 as _);
pub const VDS_E_PROVIDER_FAILURE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212222i32 as _);
pub const VDS_E_PROVIDER_INITIALIZATION_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212260i32 as _);
pub const VDS_E_PROVIDER_INTERNAL_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211918i32 as _);
pub const VDS_E_PROVIDER_TYPE_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212214i32 as _);
pub const VDS_E_PROVIDER_VOL_DEVICE_NAME_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212254i32 as _);
pub const VDS_E_PROVIDER_VOL_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212253i32 as _);
pub const VDS_E_RAID5_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210972i32 as _);
pub const VDS_E_READONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211900i32 as _);
pub const VDS_E_REBOOT_REQUIRED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210996i32 as _);
pub const VDS_E_REFS_FORMAT_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210746i32 as _);
pub const VDS_E_REPAIR_VOLUMESTATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212192i32 as _);
pub const VDS_E_REQUIRES_CONTIGUOUS_DISK_SPACE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212224i32 as _);
pub const VDS_E_RETRY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212189i32 as _);
pub const VDS_E_REVERT_ON_CLOSE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212200i32 as _);
pub const VDS_E_REVERT_ON_CLOSE_MISMATCH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212190i32 as _);
pub const VDS_E_REVERT_ON_CLOSE_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212199i32 as _);
pub const VDS_E_SECTOR_SIZE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211229i32 as _);
pub const VDS_E_SECURITY_INCOMPLETELY_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211515i32 as _);
pub const VDS_E_SET_SAN_POLICY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211258i32 as _);
pub const VDS_E_SET_TUNNEL_MODE_OUTER_ADDRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211004i32 as _);
pub const VDS_E_SHRINK_DIRTY_VOLUME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211878i32 as _);
pub const VDS_E_SHRINK_EXTEND_UNALIGNED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210496i32 as _);
pub const VDS_E_SHRINK_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211887i32 as _);
pub const VDS_E_SHRINK_LUN_NOT_UNMASKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210979i32 as _);
pub const VDS_E_SHRINK_OVER_DATA: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211242i32 as _);
pub const VDS_E_SHRINK_SIZE_LESS_THAN_MIN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211917i32 as _);
pub const VDS_E_SHRINK_SIZE_TOO_BIG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211916i32 as _);
pub const VDS_E_SHRINK_UNKNOWN_FILESYSTEM: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210966i32 as _);
pub const VDS_E_SHRINK_USER_CANCELLED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211879i32 as _);
pub const VDS_E_SOURCE_IS_TARGET_PACK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211992i32 as _);
pub const VDS_E_SUBSYSTEM_ID_IS_NULL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211001i32 as _);
pub const VDS_E_SYSTEM_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211247i32 as _);
pub const VDS_E_TARGET_PACK_NOT_EMPTY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212003i32 as _);
pub const VDS_E_TARGET_PORTAL_NOT_FOUND: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211007i32 as _);
pub const VDS_E_TARGET_SPECIFIC_NOT_SUPPORTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211514i32 as _);
pub const VDS_E_TIMEOUT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212193i32 as _);
pub const VDS_E_UNABLE_TO_FIND_BOOT_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211261i32 as _);
pub const VDS_E_UNABLE_TO_FIND_SYSTEM_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211249i32 as _);
pub const VDS_E_UNEXPECTED_DISK_LAYOUT_CHANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211955i32 as _);
pub const VDS_E_UNRECOVERABLE_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212263i32 as _);
pub const VDS_E_UNRECOVERABLE_PROVIDER_ERROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211915i32 as _);
pub const VDS_E_VDISK_INVALID_OP_STATE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210982i32 as _);
pub const VDS_E_VDISK_NOT_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210983i32 as _);
pub const VDS_E_VDISK_PATHNAME_INVALID: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210969i32 as _);
pub const VDS_E_VD_ALREADY_ATTACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210956i32 as _);
pub const VDS_E_VD_ALREADY_COMPACTING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210958i32 as _);
pub const VDS_E_VD_ALREADY_DETACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210955i32 as _);
pub const VDS_E_VD_ALREADY_MERGING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210957i32 as _);
pub const VDS_E_VD_DISK_ALREADY_EXPANDING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210959i32 as _);
pub const VDS_E_VD_DISK_ALREADY_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210960i32 as _);
pub const VDS_E_VD_DISK_IS_COMPACTING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210963i32 as _);
pub const VDS_E_VD_DISK_IS_EXPANDING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210964i32 as _);
pub const VDS_E_VD_DISK_IS_MERGING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210962i32 as _);
pub const VDS_E_VD_DISK_NOT_OPEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210965i32 as _);
pub const VDS_E_VD_IS_ATTACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210961i32 as _);
pub const VDS_E_VD_IS_BEING_ATTACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210953i32 as _);
pub const VDS_E_VD_IS_BEING_DETACHED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210952i32 as _);
pub const VDS_E_VD_NOT_ATTACHED_READONLY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210954i32 as _);
pub const VDS_E_VOLUME_DISK_COUNT_MAX_EXCEEDED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211991i32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211230i32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_CORRUPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211232i32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211233i32 as _);
pub const VDS_E_VOLUME_EXTEND_FVE_RECOVERY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211231i32 as _);
pub const VDS_E_VOLUME_GUID_PATHNAME_NOT_ALLOWED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147210995i32 as _);
pub const VDS_E_VOLUME_HAS_PATH: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212194i32 as _);
pub const VDS_E_VOLUME_HIDDEN: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211914i32 as _);
pub const VDS_E_VOLUME_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212238i32 as _);
pub const VDS_E_VOLUME_INVALID_NAME: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212025i32 as _);
pub const VDS_E_VOLUME_LENGTH_NOT_SECTOR_SIZE_MULTIPLE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211953i32 as _);
pub const VDS_E_VOLUME_MIRRORED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211896i32 as _);
pub const VDS_E_VOLUME_NOT_A_MIRROR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212219i32 as _);
pub const VDS_E_VOLUME_NOT_FOUND_IN_PACK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211908i32 as _);
pub const VDS_E_VOLUME_NOT_HEALTHY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212226i32 as _);
pub const VDS_E_VOLUME_NOT_MOUNTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212209i32 as _);
pub const VDS_E_VOLUME_NOT_ONLINE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212227i32 as _);
pub const VDS_E_VOLUME_NOT_RETAINED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211952i32 as _);
pub const VDS_E_VOLUME_ON_DISK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212005i32 as _);
pub const VDS_E_VOLUME_PERMANENTLY_DISMOUNTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212195i32 as _);
pub const VDS_E_VOLUME_REGENERATING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211904i32 as _);
pub const VDS_E_VOLUME_RETAINED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211951i32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211243i32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_CORRUPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211245i32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_LOCKED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211246i32 as _);
pub const VDS_E_VOLUME_SHRINK_FVE_RECOVERY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211244i32 as _);
pub const VDS_E_VOLUME_SIMPLE_SPANNED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211895i32 as _);
pub const VDS_E_VOLUME_SPANS_DISKS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212225i32 as _);
pub const VDS_E_VOLUME_SYNCHRONIZING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147211905i32 as _);
pub const VDS_E_VOLUME_TEMPORARILY_DISMOUNTED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212196i32 as _);
pub const VDS_E_VOLUME_TOO_BIG: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212243i32 as _);
pub const VDS_E_VOLUME_TOO_SMALL: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212244i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_FILE_SYSTEM_NOTIFICATION {
    pub ulEvent: VDS_NF_FILE_SYSTEM,
    pub volumeId: ::windows::runtime::GUID,
    pub dwPercentCompleted: u32,
}
impl VDS_FILE_SYSTEM_NOTIFICATION {}
impl ::std::default::Default for VDS_FILE_SYSTEM_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_FILE_SYSTEM_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_FILE_SYSTEM_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).field("dwPercentCompleted", &self.dwPercentCompleted).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_FILE_SYSTEM_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.volumeId == other.volumeId && self.dwPercentCompleted == other.dwPercentCompleted
    }
}
impl ::std::cmp::Eq for VDS_FILE_SYSTEM_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_FILE_SYSTEM_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_FILE_SYSTEM_TYPE(pub i32);
pub const VDS_FST_UNKNOWN: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(0i32);
pub const VDS_FST_RAW: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(1i32);
pub const VDS_FST_FAT: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(2i32);
pub const VDS_FST_FAT32: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(3i32);
pub const VDS_FST_NTFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(4i32);
pub const VDS_FST_CDFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(5i32);
pub const VDS_FST_UDF: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(6i32);
pub const VDS_FST_EXFAT: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(7i32);
pub const VDS_FST_CSVFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(8i32);
pub const VDS_FST_REFS: VDS_FILE_SYSTEM_TYPE = VDS_FILE_SYSTEM_TYPE(9i32);
impl ::std::convert::From<i32> for VDS_FILE_SYSTEM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_FILE_SYSTEM_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_HBAPORT_PROP {
    pub id: ::windows::runtime::GUID,
    pub wwnNode: VDS_WWN,
    pub wwnPort: VDS_WWN,
    pub r#type: VDS_HBAPORT_TYPE,
    pub status: VDS_HBAPORT_STATUS,
    pub ulPortSpeed: u32,
    pub ulSupportedPortSpeed: u32,
}
impl VDS_HBAPORT_PROP {}
impl ::std::default::Default for VDS_HBAPORT_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_HBAPORT_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_HBAPORT_PROP").field("id", &self.id).field("wwnNode", &self.wwnNode).field("wwnPort", &self.wwnPort).field("r#type", &self.r#type).field("status", &self.status).field("ulPortSpeed", &self.ulPortSpeed).field("ulSupportedPortSpeed", &self.ulSupportedPortSpeed).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_HBAPORT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.wwnNode == other.wwnNode && self.wwnPort == other.wwnPort && self.r#type == other.r#type && self.status == other.status && self.ulPortSpeed == other.ulPortSpeed && self.ulSupportedPortSpeed == other.ulSupportedPortSpeed
    }
}
impl ::std::cmp::Eq for VDS_HBAPORT_PROP {}
unsafe impl ::windows::runtime::Abi for VDS_HBAPORT_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_HBAPORT_SPEED_FLAG(pub i32);
pub const VDS_HSF_UNKNOWN: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(0i32);
pub const VDS_HSF_1GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(1i32);
pub const VDS_HSF_2GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(2i32);
pub const VDS_HSF_10GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(4i32);
pub const VDS_HSF_4GBIT: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(8i32);
pub const VDS_HSF_NOT_NEGOTIATED: VDS_HBAPORT_SPEED_FLAG = VDS_HBAPORT_SPEED_FLAG(32768i32);
impl ::std::convert::From<i32> for VDS_HBAPORT_SPEED_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_HBAPORT_SPEED_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_HBAPORT_STATUS(pub i32);
pub const VDS_HPS_UNKNOWN: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(1i32);
pub const VDS_HPS_ONLINE: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(2i32);
pub const VDS_HPS_OFFLINE: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(3i32);
pub const VDS_HPS_BYPASSED: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(4i32);
pub const VDS_HPS_DIAGNOSTICS: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(5i32);
pub const VDS_HPS_LINKDOWN: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(6i32);
pub const VDS_HPS_ERROR: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(7i32);
pub const VDS_HPS_LOOPBACK: VDS_HBAPORT_STATUS = VDS_HBAPORT_STATUS(8i32);
impl ::std::convert::From<i32> for VDS_HBAPORT_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_HBAPORT_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_HBAPORT_TYPE(pub i32);
pub const VDS_HPT_UNKNOWN: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(1i32);
pub const VDS_HPT_OTHER: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(2i32);
pub const VDS_HPT_NOTPRESENT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(3i32);
pub const VDS_HPT_NPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(5i32);
pub const VDS_HPT_NLPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(6i32);
pub const VDS_HPT_FLPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(7i32);
pub const VDS_HPT_FPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(8i32);
pub const VDS_HPT_EPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(9i32);
pub const VDS_HPT_GPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(10i32);
pub const VDS_HPT_LPORT: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(20i32);
pub const VDS_HPT_PTP: VDS_HBAPORT_TYPE = VDS_HBAPORT_TYPE(21i32);
impl ::std::convert::From<i32> for VDS_HBAPORT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_HBAPORT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_HEALTH(pub i32);
pub const VDS_H_UNKNOWN: VDS_HEALTH = VDS_HEALTH(0i32);
pub const VDS_H_HEALTHY: VDS_HEALTH = VDS_HEALTH(1i32);
pub const VDS_H_REBUILDING: VDS_HEALTH = VDS_HEALTH(2i32);
pub const VDS_H_STALE: VDS_HEALTH = VDS_HEALTH(3i32);
pub const VDS_H_FAILING: VDS_HEALTH = VDS_HEALTH(4i32);
pub const VDS_H_FAILING_REDUNDANCY: VDS_HEALTH = VDS_HEALTH(5i32);
pub const VDS_H_FAILED_REDUNDANCY: VDS_HEALTH = VDS_HEALTH(6i32);
pub const VDS_H_FAILED_REDUNDANCY_FAILING: VDS_HEALTH = VDS_HEALTH(7i32);
pub const VDS_H_FAILED: VDS_HEALTH = VDS_HEALTH(8i32);
pub const VDS_H_REPLACED: VDS_HEALTH = VDS_HEALTH(9i32);
pub const VDS_H_PENDING_FAILURE: VDS_HEALTH = VDS_HEALTH(10i32);
pub const VDS_H_DEGRADED: VDS_HEALTH = VDS_HEALTH(11i32);
impl ::std::convert::From<i32> for VDS_HEALTH {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_HEALTH {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_HINTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_HINTS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_HINTS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_HINTS")
            .field("ullHintMask", &self.ullHintMask)
            .field("ullExpectedMaximumSize", &self.ullExpectedMaximumSize)
            .field("ulOptimalReadSize", &self.ulOptimalReadSize)
            .field("ulOptimalReadAlignment", &self.ulOptimalReadAlignment)
            .field("ulOptimalWriteSize", &self.ulOptimalWriteSize)
            .field("ulOptimalWriteAlignment", &self.ulOptimalWriteAlignment)
            .field("ulMaximumDriveCount", &self.ulMaximumDriveCount)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("bFastCrashRecoveryRequired", &self.bFastCrashRecoveryRequired)
            .field("bMostlyReads", &self.bMostlyReads)
            .field("bOptimizeForSequentialReads", &self.bOptimizeForSequentialReads)
            .field("bOptimizeForSequentialWrites", &self.bOptimizeForSequentialWrites)
            .field("bRemapEnabled", &self.bRemapEnabled)
            .field("bReadBackVerifyEnabled", &self.bReadBackVerifyEnabled)
            .field("bWriteThroughCachingEnabled", &self.bWriteThroughCachingEnabled)
            .field("bHardwareChecksumEnabled", &self.bHardwareChecksumEnabled)
            .field("bIsYankable", &self.bIsYankable)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_HINTS {
    fn eq(&self, other: &Self) -> bool {
        self.ullHintMask == other.ullHintMask
            && self.ullExpectedMaximumSize == other.ullExpectedMaximumSize
            && self.ulOptimalReadSize == other.ulOptimalReadSize
            && self.ulOptimalReadAlignment == other.ulOptimalReadAlignment
            && self.ulOptimalWriteSize == other.ulOptimalWriteSize
            && self.ulOptimalWriteAlignment == other.ulOptimalWriteAlignment
            && self.ulMaximumDriveCount == other.ulMaximumDriveCount
            && self.ulStripeSize == other.ulStripeSize
            && self.bFastCrashRecoveryRequired == other.bFastCrashRecoveryRequired
            && self.bMostlyReads == other.bMostlyReads
            && self.bOptimizeForSequentialReads == other.bOptimizeForSequentialReads
            && self.bOptimizeForSequentialWrites == other.bOptimizeForSequentialWrites
            && self.bRemapEnabled == other.bRemapEnabled
            && self.bReadBackVerifyEnabled == other.bReadBackVerifyEnabled
            && self.bWriteThroughCachingEnabled == other.bWriteThroughCachingEnabled
            && self.bHardwareChecksumEnabled == other.bHardwareChecksumEnabled
            && self.bIsYankable == other.bIsYankable
            && self.sRebuildPriority == other.sRebuildPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_HINTS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_HINTS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_HINTS2 {
    pub ullHintMask: u64,
    pub ullExpectedMaximumSize: u64,
    pub ulOptimalReadSize: u32,
    pub ulOptimalReadAlignment: u32,
    pub ulOptimalWriteSize: u32,
    pub ulOptimalWriteAlignment: u32,
    pub ulMaximumDriveCount: u32,
    pub ulStripeSize: u32,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ulReserved3: u32,
    pub bFastCrashRecoveryRequired: super::super::Foundation::BOOL,
    pub bMostlyReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialReads: super::super::Foundation::BOOL,
    pub bOptimizeForSequentialWrites: super::super::Foundation::BOOL,
    pub bRemapEnabled: super::super::Foundation::BOOL,
    pub bReadBackVerifyEnabled: super::super::Foundation::BOOL,
    pub bWriteThroughCachingEnabled: super::super::Foundation::BOOL,
    pub bHardwareChecksumEnabled: super::super::Foundation::BOOL,
    pub bIsYankable: super::super::Foundation::BOOL,
    pub bAllocateHotSpare: super::super::Foundation::BOOL,
    pub bUseMirroredCache: super::super::Foundation::BOOL,
    pub bReadCachingEnabled: super::super::Foundation::BOOL,
    pub bWriteCachingEnabled: super::super::Foundation::BOOL,
    pub bMediaScanEnabled: super::super::Foundation::BOOL,
    pub bConsistencyCheckEnabled: super::super::Foundation::BOOL,
    pub BusType: VDS_STORAGE_BUS_TYPE,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub bReserved3: super::super::Foundation::BOOL,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_HINTS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_HINTS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_HINTS2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_HINTS2")
            .field("ullHintMask", &self.ullHintMask)
            .field("ullExpectedMaximumSize", &self.ullExpectedMaximumSize)
            .field("ulOptimalReadSize", &self.ulOptimalReadSize)
            .field("ulOptimalReadAlignment", &self.ulOptimalReadAlignment)
            .field("ulOptimalWriteSize", &self.ulOptimalWriteSize)
            .field("ulOptimalWriteAlignment", &self.ulOptimalWriteAlignment)
            .field("ulMaximumDriveCount", &self.ulMaximumDriveCount)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("ulReserved1", &self.ulReserved1)
            .field("ulReserved2", &self.ulReserved2)
            .field("ulReserved3", &self.ulReserved3)
            .field("bFastCrashRecoveryRequired", &self.bFastCrashRecoveryRequired)
            .field("bMostlyReads", &self.bMostlyReads)
            .field("bOptimizeForSequentialReads", &self.bOptimizeForSequentialReads)
            .field("bOptimizeForSequentialWrites", &self.bOptimizeForSequentialWrites)
            .field("bRemapEnabled", &self.bRemapEnabled)
            .field("bReadBackVerifyEnabled", &self.bReadBackVerifyEnabled)
            .field("bWriteThroughCachingEnabled", &self.bWriteThroughCachingEnabled)
            .field("bHardwareChecksumEnabled", &self.bHardwareChecksumEnabled)
            .field("bIsYankable", &self.bIsYankable)
            .field("bAllocateHotSpare", &self.bAllocateHotSpare)
            .field("bUseMirroredCache", &self.bUseMirroredCache)
            .field("bReadCachingEnabled", &self.bReadCachingEnabled)
            .field("bWriteCachingEnabled", &self.bWriteCachingEnabled)
            .field("bMediaScanEnabled", &self.bMediaScanEnabled)
            .field("bConsistencyCheckEnabled", &self.bConsistencyCheckEnabled)
            .field("BusType", &self.BusType)
            .field("bReserved1", &self.bReserved1)
            .field("bReserved2", &self.bReserved2)
            .field("bReserved3", &self.bReserved3)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_HINTS2 {
    fn eq(&self, other: &Self) -> bool {
        self.ullHintMask == other.ullHintMask
            && self.ullExpectedMaximumSize == other.ullExpectedMaximumSize
            && self.ulOptimalReadSize == other.ulOptimalReadSize
            && self.ulOptimalReadAlignment == other.ulOptimalReadAlignment
            && self.ulOptimalWriteSize == other.ulOptimalWriteSize
            && self.ulOptimalWriteAlignment == other.ulOptimalWriteAlignment
            && self.ulMaximumDriveCount == other.ulMaximumDriveCount
            && self.ulStripeSize == other.ulStripeSize
            && self.ulReserved1 == other.ulReserved1
            && self.ulReserved2 == other.ulReserved2
            && self.ulReserved3 == other.ulReserved3
            && self.bFastCrashRecoveryRequired == other.bFastCrashRecoveryRequired
            && self.bMostlyReads == other.bMostlyReads
            && self.bOptimizeForSequentialReads == other.bOptimizeForSequentialReads
            && self.bOptimizeForSequentialWrites == other.bOptimizeForSequentialWrites
            && self.bRemapEnabled == other.bRemapEnabled
            && self.bReadBackVerifyEnabled == other.bReadBackVerifyEnabled
            && self.bWriteThroughCachingEnabled == other.bWriteThroughCachingEnabled
            && self.bHardwareChecksumEnabled == other.bHardwareChecksumEnabled
            && self.bIsYankable == other.bIsYankable
            && self.bAllocateHotSpare == other.bAllocateHotSpare
            && self.bUseMirroredCache == other.bUseMirroredCache
            && self.bReadCachingEnabled == other.bReadCachingEnabled
            && self.bWriteCachingEnabled == other.bWriteCachingEnabled
            && self.bMediaScanEnabled == other.bMediaScanEnabled
            && self.bConsistencyCheckEnabled == other.bConsistencyCheckEnabled
            && self.BusType == other.BusType
            && self.bReserved1 == other.bReserved1
            && self.bReserved2 == other.bReserved2
            && self.bReserved3 == other.bReserved3
            && self.sRebuildPriority == other.sRebuildPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_HINTS2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_HINTS2 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VDS_HINT_ALLOCATEHOTSPARE: i32 = 512i32;
pub const VDS_HINT_BUSTYPE: i32 = 1024i32;
pub const VDS_HINT_CONSISTENCYCHECKENABLED: i32 = 32768i32;
pub const VDS_HINT_FASTCRASHRECOVERYREQUIRED: i32 = 1i32;
pub const VDS_HINT_HARDWARECHECKSUMENABLED: i32 = 128i32;
pub const VDS_HINT_ISYANKABLE: i32 = 256i32;
pub const VDS_HINT_MEDIASCANENABLED: i32 = 16384i32;
pub const VDS_HINT_MOSTLYREADS: i32 = 2i32;
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALREADS: i32 = 4i32;
pub const VDS_HINT_OPTIMIZEFORSEQUENTIALWRITES: i32 = 8i32;
pub const VDS_HINT_READBACKVERIFYENABLED: i32 = 16i32;
pub const VDS_HINT_READCACHINGENABLED: i32 = 4096i32;
pub const VDS_HINT_REMAPENABLED: i32 = 32i32;
pub const VDS_HINT_USEMIRROREDCACHE: i32 = 2048i32;
pub const VDS_HINT_WRITECACHINGENABLED: i32 = 8192i32;
pub const VDS_HINT_WRITETHROUGHCACHINGENABLED: i32 = 64i32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_HWPROVIDER_TYPE(pub i32);
pub const VDS_HWT_UNKNOWN: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(0i32);
pub const VDS_HWT_PCI_RAID: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(1i32);
pub const VDS_HWT_FIBRE_CHANNEL: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(2i32);
pub const VDS_HWT_ISCSI: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(3i32);
pub const VDS_HWT_SAS: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(4i32);
pub const VDS_HWT_HYBRID: VDS_HWPROVIDER_TYPE = VDS_HWPROVIDER_TYPE(5i32);
impl ::std::convert::From<i32> for VDS_HWPROVIDER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_HWPROVIDER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_INTERCONNECT {
    pub m_addressType: VDS_INTERCONNECT_ADDRESS_TYPE,
    pub m_cbPort: u32,
    pub m_pbPort: *mut u8,
    pub m_cbAddress: u32,
    pub m_pbAddress: *mut u8,
}
impl VDS_INTERCONNECT {}
impl ::std::default::Default for VDS_INTERCONNECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_INTERCONNECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_INTERCONNECT").field("m_addressType", &self.m_addressType).field("m_cbPort", &self.m_cbPort).field("m_pbPort", &self.m_pbPort).field("m_cbAddress", &self.m_cbAddress).field("m_pbAddress", &self.m_pbAddress).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_INTERCONNECT {
    fn eq(&self, other: &Self) -> bool {
        self.m_addressType == other.m_addressType && self.m_cbPort == other.m_cbPort && self.m_pbPort == other.m_pbPort && self.m_cbAddress == other.m_cbAddress && self.m_pbAddress == other.m_pbAddress
    }
}
impl ::std::cmp::Eq for VDS_INTERCONNECT {}
unsafe impl ::windows::runtime::Abi for VDS_INTERCONNECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_INTERCONNECT_ADDRESS_TYPE(pub i32);
pub const VDS_IA_UNKNOWN: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(0i32);
pub const VDS_IA_FCFS: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(1i32);
pub const VDS_IA_FCPH: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(2i32);
pub const VDS_IA_FCPH3: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(3i32);
pub const VDS_IA_MAC: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(4i32);
pub const VDS_IA_SCSI: VDS_INTERCONNECT_ADDRESS_TYPE = VDS_INTERCONNECT_ADDRESS_TYPE(5i32);
impl ::std::convert::From<i32> for VDS_INTERCONNECT_ADDRESS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_INTERCONNECT_ADDRESS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_INTERCONNECT_FLAG(pub i32);
pub const VDS_ITF_PCI_RAID: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(1i32);
pub const VDS_ITF_FIBRE_CHANNEL: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(2i32);
pub const VDS_ITF_ISCSI: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(4i32);
pub const VDS_ITF_SAS: VDS_INTERCONNECT_FLAG = VDS_INTERCONNECT_FLAG(8i32);
impl ::std::convert::From<i32> for VDS_INTERCONNECT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_INTERCONNECT_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_IPADDRESS {
    pub r#type: VDS_IPADDRESS_TYPE,
    pub ipv4Address: u32,
    pub ipv6Address: [u8; 16],
    pub ulIpv6FlowInfo: u32,
    pub ulIpv6ScopeId: u32,
    pub wszTextAddress: [u16; 257],
    pub ulPort: u32,
}
impl VDS_IPADDRESS {}
impl ::std::default::Default for VDS_IPADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_IPADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_IPADDRESS")
            .field("r#type", &self.r#type)
            .field("ipv4Address", &self.ipv4Address)
            .field("ipv6Address", &self.ipv6Address)
            .field("ulIpv6FlowInfo", &self.ulIpv6FlowInfo)
            .field("ulIpv6ScopeId", &self.ulIpv6ScopeId)
            .field("wszTextAddress", &self.wszTextAddress)
            .field("ulPort", &self.ulPort)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VDS_IPADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.ipv4Address == other.ipv4Address && self.ipv6Address == other.ipv6Address && self.ulIpv6FlowInfo == other.ulIpv6FlowInfo && self.ulIpv6ScopeId == other.ulIpv6ScopeId && self.wszTextAddress == other.wszTextAddress && self.ulPort == other.ulPort
    }
}
impl ::std::cmp::Eq for VDS_IPADDRESS {}
unsafe impl ::windows::runtime::Abi for VDS_IPADDRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_IPADDRESS_TYPE(pub i32);
pub const VDS_IPT_TEXT: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(0i32);
pub const VDS_IPT_IPV4: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(1i32);
pub const VDS_IPT_IPV6: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(2i32);
pub const VDS_IPT_EMPTY: VDS_IPADDRESS_TYPE = VDS_IPADDRESS_TYPE(3i32);
impl ::std::convert::From<i32> for VDS_IPADDRESS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_IPADDRESS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_ISCSI_AUTH_TYPE(pub i32);
pub const VDS_IAT_NONE: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(0i32);
pub const VDS_IAT_CHAP: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(1i32);
pub const VDS_IAT_MUTUAL_CHAP: VDS_ISCSI_AUTH_TYPE = VDS_ISCSI_AUTH_TYPE(2i32);
impl ::std::convert::From<i32> for VDS_ISCSI_AUTH_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_AUTH_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    pub id: ::windows::runtime::GUID,
    pub pwszName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_ISCSI_INITIATOR_ADAPTER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_ISCSI_INITIATOR_ADAPTER_PROP").field("id", &self.id).field("pwszName", &self.pwszName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszName == other.pwszName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_ISCSI_INITIATOR_ADAPTER_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_INITIATOR_ADAPTER_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_ISCSI_INITIATOR_PORTAL_PROP {
    pub id: ::windows::runtime::GUID,
    pub address: VDS_IPADDRESS,
    pub ulPortIndex: u32,
}
impl VDS_ISCSI_INITIATOR_PORTAL_PROP {}
impl ::std::default::Default for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_ISCSI_INITIATOR_PORTAL_PROP").field("id", &self.id).field("address", &self.address).field("ulPortIndex", &self.ulPortIndex).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.address == other.address && self.ulPortIndex == other.ulPortIndex
    }
}
impl ::std::cmp::Eq for VDS_ISCSI_INITIATOR_PORTAL_PROP {}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_INITIATOR_PORTAL_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_ISCSI_IPSEC_FLAG(pub i32);
pub const VDS_IIF_VALID: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(1i32);
pub const VDS_IIF_IKE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(2i32);
pub const VDS_IIF_MAIN_MODE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(4i32);
pub const VDS_IIF_AGGRESSIVE_MODE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(8i32);
pub const VDS_IIF_PFS_ENABLE: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(16i32);
pub const VDS_IIF_TRANSPORT_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(32i32);
pub const VDS_IIF_TUNNEL_MODE_PREFERRED: VDS_ISCSI_IPSEC_FLAG = VDS_ISCSI_IPSEC_FLAG(64i32);
impl ::std::convert::From<i32> for VDS_ISCSI_IPSEC_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_IPSEC_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_ISCSI_IPSEC_KEY {
    pub pKey: *mut u8,
    pub ulKeySize: u32,
}
impl VDS_ISCSI_IPSEC_KEY {}
impl ::std::default::Default for VDS_ISCSI_IPSEC_KEY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ISCSI_IPSEC_KEY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_ISCSI_IPSEC_KEY").field("pKey", &self.pKey).field("ulKeySize", &self.ulKeySize).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ISCSI_IPSEC_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.pKey == other.pKey && self.ulKeySize == other.ulKeySize
    }
}
impl ::std::cmp::Eq for VDS_ISCSI_IPSEC_KEY {}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_IPSEC_KEY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_ISCSI_LOGIN_FLAG(pub i32);
pub const VDS_ILF_REQUIRE_IPSEC: VDS_ISCSI_LOGIN_FLAG = VDS_ISCSI_LOGIN_FLAG(1i32);
pub const VDS_ILF_MULTIPATH_ENABLED: VDS_ISCSI_LOGIN_FLAG = VDS_ISCSI_LOGIN_FLAG(2i32);
impl ::std::convert::From<i32> for VDS_ISCSI_LOGIN_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_LOGIN_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_ISCSI_LOGIN_TYPE(pub i32);
pub const VDS_ILT_MANUAL: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(0i32);
pub const VDS_ILT_PERSISTENT: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(1i32);
pub const VDS_ILT_BOOT: VDS_ISCSI_LOGIN_TYPE = VDS_ISCSI_LOGIN_TYPE(2i32);
impl ::std::convert::From<i32> for VDS_ISCSI_LOGIN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_LOGIN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_ISCSI_PORTALGROUP_PROP {
    pub id: ::windows::runtime::GUID,
    pub tag: u16,
}
impl VDS_ISCSI_PORTALGROUP_PROP {}
impl ::std::default::Default for VDS_ISCSI_PORTALGROUP_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ISCSI_PORTALGROUP_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_ISCSI_PORTALGROUP_PROP").field("id", &self.id).field("tag", &self.tag).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ISCSI_PORTALGROUP_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.tag == other.tag
    }
}
impl ::std::cmp::Eq for VDS_ISCSI_PORTALGROUP_PROP {}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_PORTALGROUP_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_ISCSI_PORTAL_PROP {
    pub id: ::windows::runtime::GUID,
    pub address: VDS_IPADDRESS,
    pub status: VDS_ISCSI_PORTAL_STATUS,
}
impl VDS_ISCSI_PORTAL_PROP {}
impl ::std::default::Default for VDS_ISCSI_PORTAL_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ISCSI_PORTAL_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_ISCSI_PORTAL_PROP").field("id", &self.id).field("address", &self.address).field("status", &self.status).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ISCSI_PORTAL_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.address == other.address && self.status == other.status
    }
}
impl ::std::cmp::Eq for VDS_ISCSI_PORTAL_PROP {}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_PORTAL_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_ISCSI_PORTAL_STATUS(pub i32);
pub const VDS_IPS_UNKNOWN: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(0i32);
pub const VDS_IPS_ONLINE: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(1i32);
pub const VDS_IPS_NOT_READY: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(2i32);
pub const VDS_IPS_OFFLINE: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(4i32);
pub const VDS_IPS_FAILED: VDS_ISCSI_PORTAL_STATUS = VDS_ISCSI_PORTAL_STATUS(5i32);
impl ::std::convert::From<i32> for VDS_ISCSI_PORTAL_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_PORTAL_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_ISCSI_SHARED_SECRET {
    pub pSharedSecret: *mut u8,
    pub ulSharedSecretSize: u32,
}
impl VDS_ISCSI_SHARED_SECRET {}
impl ::std::default::Default for VDS_ISCSI_SHARED_SECRET {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_ISCSI_SHARED_SECRET {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_ISCSI_SHARED_SECRET").field("pSharedSecret", &self.pSharedSecret).field("ulSharedSecretSize", &self.ulSharedSecretSize).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_ISCSI_SHARED_SECRET {
    fn eq(&self, other: &Self) -> bool {
        self.pSharedSecret == other.pSharedSecret && self.ulSharedSecretSize == other.ulSharedSecretSize
    }
}
impl ::std::cmp::Eq for VDS_ISCSI_SHARED_SECRET {}
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_SHARED_SECRET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_ISCSI_TARGET_PROP {
    pub id: ::windows::runtime::GUID,
    pub pwszIscsiName: super::super::Foundation::PWSTR,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub bChapEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_ISCSI_TARGET_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_ISCSI_TARGET_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_ISCSI_TARGET_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_ISCSI_TARGET_PROP").field("id", &self.id).field("pwszIscsiName", &self.pwszIscsiName).field("pwszFriendlyName", &self.pwszFriendlyName).field("bChapEnabled", &self.bChapEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_ISCSI_TARGET_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszIscsiName == other.pwszIscsiName && self.pwszFriendlyName == other.pwszFriendlyName && self.bChapEnabled == other.bChapEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_ISCSI_TARGET_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_ISCSI_TARGET_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_LOADBALANCE_POLICY_ENUM(pub i32);
pub const VDS_LBP_UNKNOWN: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(0i32);
pub const VDS_LBP_FAILOVER: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(1i32);
pub const VDS_LBP_ROUND_ROBIN: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(2i32);
pub const VDS_LBP_ROUND_ROBIN_WITH_SUBSET: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(3i32);
pub const VDS_LBP_DYN_LEAST_QUEUE_DEPTH: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(4i32);
pub const VDS_LBP_WEIGHTED_PATHS: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(5i32);
pub const VDS_LBP_LEAST_BLOCKS: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(6i32);
pub const VDS_LBP_VENDOR_SPECIFIC: VDS_LOADBALANCE_POLICY_ENUM = VDS_LOADBALANCE_POLICY_ENUM(7i32);
impl ::std::convert::From<i32> for VDS_LOADBALANCE_POLICY_ENUM {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_LOADBALANCE_POLICY_ENUM {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_LUN_FLAG(pub i32);
pub const VDS_LF_LBN_REMAP_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(1i32);
pub const VDS_LF_READ_BACK_VERIFY_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(2i32);
pub const VDS_LF_WRITE_THROUGH_CACHING_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(4i32);
pub const VDS_LF_HARDWARE_CHECKSUM_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(8i32);
pub const VDS_LF_READ_CACHE_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(16i32);
pub const VDS_LF_WRITE_CACHE_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(32i32);
pub const VDS_LF_MEDIA_SCAN_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(64i32);
pub const VDS_LF_CONSISTENCY_CHECK_ENABLED: VDS_LUN_FLAG = VDS_LUN_FLAG(128i32);
pub const VDS_LF_SNAPSHOT: VDS_LUN_FLAG = VDS_LUN_FLAG(256i32);
impl ::std::convert::From<i32> for VDS_LUN_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_LUN_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_LUN_INFORMATION {
    pub m_version: u32,
    pub m_DeviceType: u8,
    pub m_DeviceTypeModifier: u8,
    pub m_bCommandQueueing: super::super::Foundation::BOOL,
    pub m_BusType: VDS_STORAGE_BUS_TYPE,
    pub m_szVendorId: *mut u8,
    pub m_szProductId: *mut u8,
    pub m_szProductRevision: *mut u8,
    pub m_szSerialNumber: *mut u8,
    pub m_diskSignature: ::windows::runtime::GUID,
    pub m_deviceIdDescriptor: VDS_STORAGE_DEVICE_ID_DESCRIPTOR,
    pub m_cInterconnects: u32,
    pub m_rgInterconnects: *mut VDS_INTERCONNECT,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_LUN_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_LUN_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_LUN_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_LUN_INFORMATION")
            .field("m_version", &self.m_version)
            .field("m_DeviceType", &self.m_DeviceType)
            .field("m_DeviceTypeModifier", &self.m_DeviceTypeModifier)
            .field("m_bCommandQueueing", &self.m_bCommandQueueing)
            .field("m_BusType", &self.m_BusType)
            .field("m_szVendorId", &self.m_szVendorId)
            .field("m_szProductId", &self.m_szProductId)
            .field("m_szProductRevision", &self.m_szProductRevision)
            .field("m_szSerialNumber", &self.m_szSerialNumber)
            .field("m_diskSignature", &self.m_diskSignature)
            .field("m_deviceIdDescriptor", &self.m_deviceIdDescriptor)
            .field("m_cInterconnects", &self.m_cInterconnects)
            .field("m_rgInterconnects", &self.m_rgInterconnects)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_LUN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.m_version == other.m_version
            && self.m_DeviceType == other.m_DeviceType
            && self.m_DeviceTypeModifier == other.m_DeviceTypeModifier
            && self.m_bCommandQueueing == other.m_bCommandQueueing
            && self.m_BusType == other.m_BusType
            && self.m_szVendorId == other.m_szVendorId
            && self.m_szProductId == other.m_szProductId
            && self.m_szProductRevision == other.m_szProductRevision
            && self.m_szSerialNumber == other.m_szSerialNumber
            && self.m_diskSignature == other.m_diskSignature
            && self.m_deviceIdDescriptor == other.m_deviceIdDescriptor
            && self.m_cInterconnects == other.m_cInterconnects
            && self.m_rgInterconnects == other.m_rgInterconnects
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_LUN_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_LUN_INFORMATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_LUN_NOTIFICATION {
    pub ulEvent: VDS_NF_LUN,
    pub LunId: ::windows::runtime::GUID,
}
impl VDS_LUN_NOTIFICATION {}
impl ::std::default::Default for VDS_LUN_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_LUN_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_LUN_NOTIFICATION").field("ulEvent", &self.ulEvent).field("LunId", &self.LunId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_LUN_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.LunId == other.LunId
    }
}
impl ::std::cmp::Eq for VDS_LUN_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_LUN_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_LUN_PLEX_FLAG(pub i32);
pub const VDS_LPF_LBN_REMAP_ENABLED: VDS_LUN_PLEX_FLAG = VDS_LUN_PLEX_FLAG(1i32);
impl ::std::convert::From<i32> for VDS_LUN_PLEX_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_LUN_PLEX_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_LUN_PLEX_PROP {
    pub id: ::windows::runtime::GUID,
    pub ullSize: u64,
    pub r#type: VDS_LUN_PLEX_TYPE,
    pub status: VDS_LUN_PLEX_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub ulFlags: u32,
    pub ulStripeSize: u32,
    pub sRebuildPriority: i16,
}
impl VDS_LUN_PLEX_PROP {}
impl ::std::default::Default for VDS_LUN_PLEX_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_LUN_PLEX_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_LUN_PLEX_PROP")
            .field("id", &self.id)
            .field("ullSize", &self.ullSize)
            .field("r#type", &self.r#type)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("TransitionState", &self.TransitionState)
            .field("ulFlags", &self.ulFlags)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
impl ::std::cmp::PartialEq for VDS_LUN_PLEX_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.r#type == other.r#type && self.status == other.status && self.health == other.health && self.TransitionState == other.TransitionState && self.ulFlags == other.ulFlags && self.ulStripeSize == other.ulStripeSize && self.sRebuildPriority == other.sRebuildPriority
    }
}
impl ::std::cmp::Eq for VDS_LUN_PLEX_PROP {}
unsafe impl ::windows::runtime::Abi for VDS_LUN_PLEX_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_LUN_PLEX_STATUS(pub i32);
pub const VDS_LPS_UNKNOWN: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(0i32);
pub const VDS_LPS_ONLINE: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(1i32);
pub const VDS_LPS_NOT_READY: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(2i32);
pub const VDS_LPS_OFFLINE: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(4i32);
pub const VDS_LPS_FAILED: VDS_LUN_PLEX_STATUS = VDS_LUN_PLEX_STATUS(5i32);
impl ::std::convert::From<i32> for VDS_LUN_PLEX_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_LUN_PLEX_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_LUN_PLEX_TYPE(pub i32);
pub const VDS_LPT_UNKNOWN: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(0i32);
pub const VDS_LPT_SIMPLE: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(10i32);
pub const VDS_LPT_SPAN: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(11i32);
pub const VDS_LPT_STRIPE: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(12i32);
pub const VDS_LPT_PARITY: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(14i32);
pub const VDS_LPT_RAID2: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(15i32);
pub const VDS_LPT_RAID3: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(16i32);
pub const VDS_LPT_RAID4: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(17i32);
pub const VDS_LPT_RAID5: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(18i32);
pub const VDS_LPT_RAID6: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(19i32);
pub const VDS_LPT_RAID03: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(21i32);
pub const VDS_LPT_RAID05: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(22i32);
pub const VDS_LPT_RAID10: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(23i32);
pub const VDS_LPT_RAID15: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(24i32);
pub const VDS_LPT_RAID30: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(25i32);
pub const VDS_LPT_RAID50: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(26i32);
pub const VDS_LPT_RAID53: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(28i32);
pub const VDS_LPT_RAID60: VDS_LUN_PLEX_TYPE = VDS_LUN_PLEX_TYPE(29i32);
impl ::std::convert::From<i32> for VDS_LUN_PLEX_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_LUN_PLEX_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_LUN_PROP {
    pub id: ::windows::runtime::GUID,
    pub ullSize: u64,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub pwszUnmaskingList: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub r#type: VDS_LUN_TYPE,
    pub status: VDS_LUN_STATUS,
    pub health: VDS_HEALTH,
    pub TransitionState: VDS_TRANSITION_STATE,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_LUN_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_LUN_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_LUN_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_LUN_PROP")
            .field("id", &self.id)
            .field("ullSize", &self.ullSize)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("pwszUnmaskingList", &self.pwszUnmaskingList)
            .field("ulFlags", &self.ulFlags)
            .field("r#type", &self.r#type)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("TransitionState", &self.TransitionState)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_LUN_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.pwszUnmaskingList == other.pwszUnmaskingList && self.ulFlags == other.ulFlags && self.r#type == other.r#type && self.status == other.status && self.health == other.health && self.TransitionState == other.TransitionState && self.sRebuildPriority == other.sRebuildPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_LUN_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_LUN_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_LUN_STATUS(pub i32);
pub const VDS_LS_UNKNOWN: VDS_LUN_STATUS = VDS_LUN_STATUS(0i32);
pub const VDS_LS_ONLINE: VDS_LUN_STATUS = VDS_LUN_STATUS(1i32);
pub const VDS_LS_NOT_READY: VDS_LUN_STATUS = VDS_LUN_STATUS(2i32);
pub const VDS_LS_OFFLINE: VDS_LUN_STATUS = VDS_LUN_STATUS(4i32);
pub const VDS_LS_FAILED: VDS_LUN_STATUS = VDS_LUN_STATUS(5i32);
impl ::std::convert::From<i32> for VDS_LUN_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_LUN_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_LUN_TYPE(pub i32);
pub const VDS_LT_UNKNOWN: VDS_LUN_TYPE = VDS_LUN_TYPE(0i32);
pub const VDS_LT_DEFAULT: VDS_LUN_TYPE = VDS_LUN_TYPE(1i32);
pub const VDS_LT_FAULT_TOLERANT: VDS_LUN_TYPE = VDS_LUN_TYPE(2i32);
pub const VDS_LT_NON_FAULT_TOLERANT: VDS_LUN_TYPE = VDS_LUN_TYPE(3i32);
pub const VDS_LT_SIMPLE: VDS_LUN_TYPE = VDS_LUN_TYPE(10i32);
pub const VDS_LT_SPAN: VDS_LUN_TYPE = VDS_LUN_TYPE(11i32);
pub const VDS_LT_STRIPE: VDS_LUN_TYPE = VDS_LUN_TYPE(12i32);
pub const VDS_LT_MIRROR: VDS_LUN_TYPE = VDS_LUN_TYPE(13i32);
pub const VDS_LT_PARITY: VDS_LUN_TYPE = VDS_LUN_TYPE(14i32);
pub const VDS_LT_RAID2: VDS_LUN_TYPE = VDS_LUN_TYPE(15i32);
pub const VDS_LT_RAID3: VDS_LUN_TYPE = VDS_LUN_TYPE(16i32);
pub const VDS_LT_RAID4: VDS_LUN_TYPE = VDS_LUN_TYPE(17i32);
pub const VDS_LT_RAID5: VDS_LUN_TYPE = VDS_LUN_TYPE(18i32);
pub const VDS_LT_RAID6: VDS_LUN_TYPE = VDS_LUN_TYPE(19i32);
pub const VDS_LT_RAID01: VDS_LUN_TYPE = VDS_LUN_TYPE(20i32);
pub const VDS_LT_RAID03: VDS_LUN_TYPE = VDS_LUN_TYPE(21i32);
pub const VDS_LT_RAID05: VDS_LUN_TYPE = VDS_LUN_TYPE(22i32);
pub const VDS_LT_RAID10: VDS_LUN_TYPE = VDS_LUN_TYPE(23i32);
pub const VDS_LT_RAID15: VDS_LUN_TYPE = VDS_LUN_TYPE(24i32);
pub const VDS_LT_RAID30: VDS_LUN_TYPE = VDS_LUN_TYPE(25i32);
pub const VDS_LT_RAID50: VDS_LUN_TYPE = VDS_LUN_TYPE(26i32);
pub const VDS_LT_RAID51: VDS_LUN_TYPE = VDS_LUN_TYPE(27i32);
pub const VDS_LT_RAID53: VDS_LUN_TYPE = VDS_LUN_TYPE(28i32);
pub const VDS_LT_RAID60: VDS_LUN_TYPE = VDS_LUN_TYPE(29i32);
pub const VDS_LT_RAID61: VDS_LUN_TYPE = VDS_LUN_TYPE(30i32);
impl ::std::convert::From<i32> for VDS_LUN_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_LUN_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_MAINTENANCE_OPERATION(pub i32);
pub const BlinkLight: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(1i32);
pub const BeepAlarm: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(2i32);
pub const SpinDown: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(3i32);
pub const SpinUp: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(4i32);
pub const Ping: VDS_MAINTENANCE_OPERATION = VDS_MAINTENANCE_OPERATION(5i32);
impl ::std::convert::From<i32> for VDS_MAINTENANCE_OPERATION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_MAINTENANCE_OPERATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_MOUNT_POINT_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows::runtime::GUID,
}
impl VDS_MOUNT_POINT_NOTIFICATION {}
impl ::std::default::Default for VDS_MOUNT_POINT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_MOUNT_POINT_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_MOUNT_POINT_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_MOUNT_POINT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.volumeId == other.volumeId
    }
}
impl ::std::cmp::Eq for VDS_MOUNT_POINT_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_MOUNT_POINT_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_NF_CONTROLLER(pub u32);
pub const VDS_NF_CONTROLLER_ARRIVE: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(103u32);
pub const VDS_NF_CONTROLLER_DEPART: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(104u32);
pub const VDS_NF_CONTROLLER_MODIFY: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(350u32);
pub const VDS_NF_CONTROLLER_REMOVED: VDS_NF_CONTROLLER = VDS_NF_CONTROLLER(351u32);
impl ::std::convert::From<u32> for VDS_NF_CONTROLLER {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_NF_CONTROLLER {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VDS_NF_CONTROLLER {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VDS_NF_CONTROLLER {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VDS_NF_CONTROLLER {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VDS_NF_CONTROLLER {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VDS_NF_CONTROLLER {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_NF_DISK(pub u32);
pub const VDS_NF_DISK_ARRIVE: VDS_NF_DISK = VDS_NF_DISK(8u32);
pub const VDS_NF_DISK_DEPART: VDS_NF_DISK = VDS_NF_DISK(9u32);
pub const VDS_NF_DISK_MODIFY: VDS_NF_DISK = VDS_NF_DISK(10u32);
impl ::std::convert::From<u32> for VDS_NF_DISK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_NF_DISK {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VDS_NF_DISK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VDS_NF_DISK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VDS_NF_DISK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VDS_NF_DISK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VDS_NF_DISK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_NF_DRIVE(pub u32);
pub const VDS_NF_DRIVE_ARRIVE: VDS_NF_DRIVE = VDS_NF_DRIVE(105u32);
pub const VDS_NF_DRIVE_DEPART: VDS_NF_DRIVE = VDS_NF_DRIVE(106u32);
pub const VDS_NF_DRIVE_MODIFY: VDS_NF_DRIVE = VDS_NF_DRIVE(107u32);
pub const VDS_NF_DRIVE_REMOVED: VDS_NF_DRIVE = VDS_NF_DRIVE(354u32);
impl ::std::convert::From<u32> for VDS_NF_DRIVE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_NF_DRIVE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VDS_NF_DRIVE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VDS_NF_DRIVE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VDS_NF_DRIVE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VDS_NF_DRIVE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VDS_NF_DRIVE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const VDS_NF_DRIVE_LETTER_ASSIGN: u32 = 202u32;
pub const VDS_NF_DRIVE_LETTER_FREE: u32 = 201u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_NF_FILE_SYSTEM(pub u32);
pub const VDS_NF_FILE_SYSTEM_MODIFY: VDS_NF_FILE_SYSTEM = VDS_NF_FILE_SYSTEM(203u32);
pub const VDS_NF_FILE_SYSTEM_FORMAT_PROGRESS: VDS_NF_FILE_SYSTEM = VDS_NF_FILE_SYSTEM(204u32);
impl ::std::convert::From<u32> for VDS_NF_FILE_SYSTEM {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_NF_FILE_SYSTEM {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VDS_NF_FILE_SYSTEM {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VDS_NF_FILE_SYSTEM {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VDS_NF_FILE_SYSTEM {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VDS_NF_FILE_SYSTEM {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VDS_NF_FILE_SYSTEM {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const VDS_NF_FILE_SYSTEM_SHRINKING_PROGRESS: u32 = 206u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_NF_LUN(pub u32);
pub const VDS_NF_LUN_ARRIVE: VDS_NF_LUN = VDS_NF_LUN(108u32);
pub const VDS_NF_LUN_DEPART: VDS_NF_LUN = VDS_NF_LUN(109u32);
pub const VDS_NF_LUN_MODIFY: VDS_NF_LUN = VDS_NF_LUN(110u32);
impl ::std::convert::From<u32> for VDS_NF_LUN {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_NF_LUN {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VDS_NF_LUN {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VDS_NF_LUN {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VDS_NF_LUN {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VDS_NF_LUN {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VDS_NF_LUN {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const VDS_NF_MOUNT_POINTS_CHANGE: u32 = 205u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_NF_PACK(pub u32);
pub const VDS_NF_PACK_ARRIVE: VDS_NF_PACK = VDS_NF_PACK(1u32);
pub const VDS_NF_PACK_DEPART: VDS_NF_PACK = VDS_NF_PACK(2u32);
pub const VDS_NF_PACK_MODIFY: VDS_NF_PACK = VDS_NF_PACK(3u32);
impl ::std::convert::From<u32> for VDS_NF_PACK {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_NF_PACK {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VDS_NF_PACK {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VDS_NF_PACK {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VDS_NF_PACK {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VDS_NF_PACK {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VDS_NF_PACK {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const VDS_NF_PARTITION_ARRIVE: u32 = 11u32;
pub const VDS_NF_PARTITION_DEPART: u32 = 12u32;
pub const VDS_NF_PARTITION_MODIFY: u32 = 13u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_NF_PORT(pub u32);
pub const VDS_NF_PORT_ARRIVE: VDS_NF_PORT = VDS_NF_PORT(121u32);
pub const VDS_NF_PORT_DEPART: VDS_NF_PORT = VDS_NF_PORT(122u32);
pub const VDS_NF_PORT_MODIFY: VDS_NF_PORT = VDS_NF_PORT(352u32);
pub const VDS_NF_PORT_REMOVED: VDS_NF_PORT = VDS_NF_PORT(353u32);
impl ::std::convert::From<u32> for VDS_NF_PORT {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_NF_PORT {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for VDS_NF_PORT {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for VDS_NF_PORT {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for VDS_NF_PORT {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for VDS_NF_PORT {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for VDS_NF_PORT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const VDS_NF_PORTAL_ARRIVE: u32 = 123u32;
pub const VDS_NF_PORTAL_DEPART: u32 = 124u32;
pub const VDS_NF_PORTAL_GROUP_ARRIVE: u32 = 129u32;
pub const VDS_NF_PORTAL_GROUP_DEPART: u32 = 130u32;
pub const VDS_NF_PORTAL_GROUP_MODIFY: u32 = 131u32;
pub const VDS_NF_PORTAL_MODIFY: u32 = 125u32;
pub const VDS_NF_SERVICE_OUT_OF_SYNC: u32 = 301u32;
pub const VDS_NF_SUB_SYSTEM_ARRIVE: u32 = 101u32;
pub const VDS_NF_SUB_SYSTEM_DEPART: u32 = 102u32;
pub const VDS_NF_SUB_SYSTEM_MODIFY: u32 = 151u32;
pub const VDS_NF_TARGET_ARRIVE: u32 = 126u32;
pub const VDS_NF_TARGET_DEPART: u32 = 127u32;
pub const VDS_NF_TARGET_MODIFY: u32 = 128u32;
pub const VDS_NF_VOLUME_ARRIVE: u32 = 4u32;
pub const VDS_NF_VOLUME_DEPART: u32 = 5u32;
pub const VDS_NF_VOLUME_MODIFY: u32 = 6u32;
pub const VDS_NF_VOLUME_REBUILDING_PROGRESS: u32 = 7u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_NOTIFICATION {
    pub objectType: VDS_NOTIFICATION_TARGET_TYPE,
    pub Anonymous: VDS_NOTIFICATION_0,
}
impl VDS_NOTIFICATION {}
impl ::std::default::Default for VDS_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VDS_NOTIFICATION {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VDS_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VDS_NOTIFICATION_0 {
    pub Pack: VDS_PACK_NOTIFICATION,
    pub Disk: VDS_DISK_NOTIFICATION,
    pub Volume: VDS_VOLUME_NOTIFICATION,
    pub Partition: VDS_PARTITION_NOTIFICATION,
    pub Letter: VDS_DRIVE_LETTER_NOTIFICATION,
    pub FileSystem: VDS_FILE_SYSTEM_NOTIFICATION,
    pub MountPoint: VDS_MOUNT_POINT_NOTIFICATION,
    pub SubSystem: VDS_SUB_SYSTEM_NOTIFICATION,
    pub Controller: VDS_CONTROLLER_NOTIFICATION,
    pub Drive: VDS_DRIVE_NOTIFICATION,
    pub Lun: VDS_LUN_NOTIFICATION,
    pub Port: VDS_PORT_NOTIFICATION,
    pub Portal: VDS_PORTAL_NOTIFICATION,
    pub Target: VDS_TARGET_NOTIFICATION,
    pub PortalGroup: VDS_PORTAL_GROUP_NOTIFICATION,
    pub Service: VDS_SERVICE_NOTIFICATION,
}
impl VDS_NOTIFICATION_0 {}
impl ::std::default::Default for VDS_NOTIFICATION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VDS_NOTIFICATION_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VDS_NOTIFICATION_0 {}
unsafe impl ::windows::runtime::Abi for VDS_NOTIFICATION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_NOTIFICATION_TARGET_TYPE(pub i32);
pub const VDS_NTT_UNKNOWN: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(0i32);
pub const VDS_NTT_PACK: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(10i32);
pub const VDS_NTT_VOLUME: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(11i32);
pub const VDS_NTT_DISK: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(13i32);
pub const VDS_NTT_PARTITION: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(60i32);
pub const VDS_NTT_DRIVE_LETTER: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(61i32);
pub const VDS_NTT_FILE_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(62i32);
pub const VDS_NTT_MOUNT_POINT: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(63i32);
pub const VDS_NTT_SUB_SYSTEM: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(30i32);
pub const VDS_NTT_CONTROLLER: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(31i32);
pub const VDS_NTT_DRIVE: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(32i32);
pub const VDS_NTT_LUN: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(33i32);
pub const VDS_NTT_PORT: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(35i32);
pub const VDS_NTT_PORTAL: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(36i32);
pub const VDS_NTT_TARGET: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(37i32);
pub const VDS_NTT_PORTAL_GROUP: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(38i32);
pub const VDS_NTT_SERVICE: VDS_NOTIFICATION_TARGET_TYPE = VDS_NOTIFICATION_TARGET_TYPE(200i32);
impl ::std::convert::From<i32> for VDS_NOTIFICATION_TARGET_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_NOTIFICATION_TARGET_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_OBJECT_TYPE(pub i32);
pub const VDS_OT_UNKNOWN: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(0i32);
pub const VDS_OT_PROVIDER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(1i32);
pub const VDS_OT_PACK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(10i32);
pub const VDS_OT_VOLUME: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(11i32);
pub const VDS_OT_VOLUME_PLEX: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(12i32);
pub const VDS_OT_DISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(13i32);
pub const VDS_OT_SUB_SYSTEM: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(30i32);
pub const VDS_OT_CONTROLLER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(31i32);
pub const VDS_OT_DRIVE: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(32i32);
pub const VDS_OT_LUN: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(33i32);
pub const VDS_OT_LUN_PLEX: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(34i32);
pub const VDS_OT_PORT: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(35i32);
pub const VDS_OT_PORTAL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(36i32);
pub const VDS_OT_TARGET: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(37i32);
pub const VDS_OT_PORTAL_GROUP: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(38i32);
pub const VDS_OT_STORAGE_POOL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(39i32);
pub const VDS_OT_HBAPORT: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(90i32);
pub const VDS_OT_INIT_ADAPTER: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(91i32);
pub const VDS_OT_INIT_PORTAL: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(92i32);
pub const VDS_OT_ASYNC: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(100i32);
pub const VDS_OT_ENUM: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(101i32);
pub const VDS_OT_VDISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(200i32);
pub const VDS_OT_OPEN_VDISK: VDS_OBJECT_TYPE = VDS_OBJECT_TYPE(201i32);
impl ::std::convert::From<i32> for VDS_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_OBJECT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_PACK_NOTIFICATION {
    pub ulEvent: VDS_NF_PACK,
    pub packId: ::windows::runtime::GUID,
}
impl VDS_PACK_NOTIFICATION {}
impl ::std::default::Default for VDS_PACK_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_PACK_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PACK_NOTIFICATION").field("ulEvent", &self.ulEvent).field("packId", &self.packId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_PACK_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.packId == other.packId
    }
}
impl ::std::cmp::Eq for VDS_PACK_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_PACK_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_PARTITION_NOTIFICATION {
    pub ulEvent: u32,
    pub diskId: ::windows::runtime::GUID,
    pub ullOffset: u64,
}
impl VDS_PARTITION_NOTIFICATION {}
impl ::std::default::Default for VDS_PARTITION_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_PARTITION_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PARTITION_NOTIFICATION").field("ulEvent", &self.ulEvent).field("diskId", &self.diskId).field("ullOffset", &self.ullOffset).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_PARTITION_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.diskId == other.diskId && self.ullOffset == other.ullOffset
    }
}
impl ::std::cmp::Eq for VDS_PARTITION_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_PARTITION_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_PATH_ID {
    pub ullSourceId: u64,
    pub ullPathId: u64,
}
impl VDS_PATH_ID {}
impl ::std::default::Default for VDS_PATH_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_PATH_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PATH_ID").field("ullSourceId", &self.ullSourceId).field("ullPathId", &self.ullPathId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_PATH_ID {
    fn eq(&self, other: &Self) -> bool {
        self.ullSourceId == other.ullSourceId && self.ullPathId == other.ullPathId
    }
}
impl ::std::cmp::Eq for VDS_PATH_ID {}
unsafe impl ::windows::runtime::Abi for VDS_PATH_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_PATH_INFO {
    pub pathId: VDS_PATH_ID,
    pub r#type: VDS_HWPROVIDER_TYPE,
    pub status: VDS_PATH_STATUS,
    pub Anonymous1: VDS_PATH_INFO_0,
    pub Anonymous2: VDS_PATH_INFO_1,
    pub Anonymous3: VDS_PATH_INFO_2,
}
impl VDS_PATH_INFO {}
impl ::std::default::Default for VDS_PATH_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VDS_PATH_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VDS_PATH_INFO {}
unsafe impl ::windows::runtime::Abi for VDS_PATH_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VDS_PATH_INFO_0 {
    pub controllerPortId: ::windows::runtime::GUID,
    pub targetPortalId: ::windows::runtime::GUID,
}
impl VDS_PATH_INFO_0 {}
impl ::std::default::Default for VDS_PATH_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VDS_PATH_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VDS_PATH_INFO_0 {}
unsafe impl ::windows::runtime::Abi for VDS_PATH_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VDS_PATH_INFO_1 {
    pub hbaPortId: ::windows::runtime::GUID,
    pub initiatorAdapterId: ::windows::runtime::GUID,
}
impl VDS_PATH_INFO_1 {}
impl ::std::default::Default for VDS_PATH_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VDS_PATH_INFO_1 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VDS_PATH_INFO_1 {}
unsafe impl ::windows::runtime::Abi for VDS_PATH_INFO_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union VDS_PATH_INFO_2 {
    pub pHbaPortProp: *mut VDS_HBAPORT_PROP,
    pub pInitiatorPortalIpAddr: *mut VDS_IPADDRESS,
}
impl VDS_PATH_INFO_2 {}
impl ::std::default::Default for VDS_PATH_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for VDS_PATH_INFO_2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for VDS_PATH_INFO_2 {}
unsafe impl ::windows::runtime::Abi for VDS_PATH_INFO_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PATH_POLICY {
    pub pathId: VDS_PATH_ID,
    pub bPrimaryPath: super::super::Foundation::BOOL,
    pub ulWeight: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_PATH_POLICY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_PATH_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_PATH_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PATH_POLICY").field("pathId", &self.pathId).field("bPrimaryPath", &self.bPrimaryPath).field("ulWeight", &self.ulWeight).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_PATH_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.pathId == other.pathId && self.bPrimaryPath == other.bPrimaryPath && self.ulWeight == other.ulWeight
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_PATH_POLICY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_PATH_POLICY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_PATH_STATUS(pub i32);
pub const VDS_MPS_UNKNOWN: VDS_PATH_STATUS = VDS_PATH_STATUS(0i32);
pub const VDS_MPS_ONLINE: VDS_PATH_STATUS = VDS_PATH_STATUS(1i32);
pub const VDS_MPS_FAILED: VDS_PATH_STATUS = VDS_PATH_STATUS(5i32);
pub const VDS_MPS_STANDBY: VDS_PATH_STATUS = VDS_PATH_STATUS(7i32);
impl ::std::convert::From<i32> for VDS_PATH_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_PATH_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_POOL_ATTRIBUTES {
    pub ullAttributeMask: u64,
    pub raidType: VDS_RAID_TYPE,
    pub busType: VDS_STORAGE_BUS_TYPE,
    pub pwszIntendedUsage: super::super::Foundation::PWSTR,
    pub bSpinDown: super::super::Foundation::BOOL,
    pub bIsThinProvisioned: super::super::Foundation::BOOL,
    pub ullProvisionedSpace: u64,
    pub bNoSinglePointOfFailure: super::super::Foundation::BOOL,
    pub ulDataRedundancyMax: u32,
    pub ulDataRedundancyMin: u32,
    pub ulDataRedundancyDefault: u32,
    pub ulPackageRedundancyMax: u32,
    pub ulPackageRedundancyMin: u32,
    pub ulPackageRedundancyDefault: u32,
    pub ulStripeSize: u32,
    pub ulStripeSizeMax: u32,
    pub ulStripeSizeMin: u32,
    pub ulDefaultStripeSize: u32,
    pub ulNumberOfColumns: u32,
    pub ulNumberOfColumnsMax: u32,
    pub ulNumberOfColumnsMin: u32,
    pub ulDefaultNumberofColumns: u32,
    pub ulDataAvailabilityHint: u32,
    pub ulAccessRandomnessHint: u32,
    pub ulAccessDirectionHint: u32,
    pub ulAccessSizeHint: u32,
    pub ulAccessLatencyHint: u32,
    pub ulAccessBandwidthWeightHint: u32,
    pub ulStorageCostHint: u32,
    pub ulStorageEfficiencyHint: u32,
    pub ulNumOfCustomAttributes: u32,
    pub pPoolCustomAttributes: *mut VDS_POOL_CUSTOM_ATTRIBUTES,
    pub bReserved1: super::super::Foundation::BOOL,
    pub bReserved2: super::super::Foundation::BOOL,
    pub ulReserved1: u32,
    pub ulReserved2: u32,
    pub ullReserved1: u64,
    pub ullReserved2: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_POOL_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_POOL_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_POOL_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_POOL_ATTRIBUTES")
            .field("ullAttributeMask", &self.ullAttributeMask)
            .field("raidType", &self.raidType)
            .field("busType", &self.busType)
            .field("pwszIntendedUsage", &self.pwszIntendedUsage)
            .field("bSpinDown", &self.bSpinDown)
            .field("bIsThinProvisioned", &self.bIsThinProvisioned)
            .field("ullProvisionedSpace", &self.ullProvisionedSpace)
            .field("bNoSinglePointOfFailure", &self.bNoSinglePointOfFailure)
            .field("ulDataRedundancyMax", &self.ulDataRedundancyMax)
            .field("ulDataRedundancyMin", &self.ulDataRedundancyMin)
            .field("ulDataRedundancyDefault", &self.ulDataRedundancyDefault)
            .field("ulPackageRedundancyMax", &self.ulPackageRedundancyMax)
            .field("ulPackageRedundancyMin", &self.ulPackageRedundancyMin)
            .field("ulPackageRedundancyDefault", &self.ulPackageRedundancyDefault)
            .field("ulStripeSize", &self.ulStripeSize)
            .field("ulStripeSizeMax", &self.ulStripeSizeMax)
            .field("ulStripeSizeMin", &self.ulStripeSizeMin)
            .field("ulDefaultStripeSize", &self.ulDefaultStripeSize)
            .field("ulNumberOfColumns", &self.ulNumberOfColumns)
            .field("ulNumberOfColumnsMax", &self.ulNumberOfColumnsMax)
            .field("ulNumberOfColumnsMin", &self.ulNumberOfColumnsMin)
            .field("ulDefaultNumberofColumns", &self.ulDefaultNumberofColumns)
            .field("ulDataAvailabilityHint", &self.ulDataAvailabilityHint)
            .field("ulAccessRandomnessHint", &self.ulAccessRandomnessHint)
            .field("ulAccessDirectionHint", &self.ulAccessDirectionHint)
            .field("ulAccessSizeHint", &self.ulAccessSizeHint)
            .field("ulAccessLatencyHint", &self.ulAccessLatencyHint)
            .field("ulAccessBandwidthWeightHint", &self.ulAccessBandwidthWeightHint)
            .field("ulStorageCostHint", &self.ulStorageCostHint)
            .field("ulStorageEfficiencyHint", &self.ulStorageEfficiencyHint)
            .field("ulNumOfCustomAttributes", &self.ulNumOfCustomAttributes)
            .field("pPoolCustomAttributes", &self.pPoolCustomAttributes)
            .field("bReserved1", &self.bReserved1)
            .field("bReserved2", &self.bReserved2)
            .field("ulReserved1", &self.ulReserved1)
            .field("ulReserved2", &self.ulReserved2)
            .field("ullReserved1", &self.ullReserved1)
            .field("ullReserved2", &self.ullReserved2)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_POOL_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.ullAttributeMask == other.ullAttributeMask
            && self.raidType == other.raidType
            && self.busType == other.busType
            && self.pwszIntendedUsage == other.pwszIntendedUsage
            && self.bSpinDown == other.bSpinDown
            && self.bIsThinProvisioned == other.bIsThinProvisioned
            && self.ullProvisionedSpace == other.ullProvisionedSpace
            && self.bNoSinglePointOfFailure == other.bNoSinglePointOfFailure
            && self.ulDataRedundancyMax == other.ulDataRedundancyMax
            && self.ulDataRedundancyMin == other.ulDataRedundancyMin
            && self.ulDataRedundancyDefault == other.ulDataRedundancyDefault
            && self.ulPackageRedundancyMax == other.ulPackageRedundancyMax
            && self.ulPackageRedundancyMin == other.ulPackageRedundancyMin
            && self.ulPackageRedundancyDefault == other.ulPackageRedundancyDefault
            && self.ulStripeSize == other.ulStripeSize
            && self.ulStripeSizeMax == other.ulStripeSizeMax
            && self.ulStripeSizeMin == other.ulStripeSizeMin
            && self.ulDefaultStripeSize == other.ulDefaultStripeSize
            && self.ulNumberOfColumns == other.ulNumberOfColumns
            && self.ulNumberOfColumnsMax == other.ulNumberOfColumnsMax
            && self.ulNumberOfColumnsMin == other.ulNumberOfColumnsMin
            && self.ulDefaultNumberofColumns == other.ulDefaultNumberofColumns
            && self.ulDataAvailabilityHint == other.ulDataAvailabilityHint
            && self.ulAccessRandomnessHint == other.ulAccessRandomnessHint
            && self.ulAccessDirectionHint == other.ulAccessDirectionHint
            && self.ulAccessSizeHint == other.ulAccessSizeHint
            && self.ulAccessLatencyHint == other.ulAccessLatencyHint
            && self.ulAccessBandwidthWeightHint == other.ulAccessBandwidthWeightHint
            && self.ulStorageCostHint == other.ulStorageCostHint
            && self.ulStorageEfficiencyHint == other.ulStorageEfficiencyHint
            && self.ulNumOfCustomAttributes == other.ulNumOfCustomAttributes
            && self.pPoolCustomAttributes == other.pPoolCustomAttributes
            && self.bReserved1 == other.bReserved1
            && self.bReserved2 == other.bReserved2
            && self.ulReserved1 == other.ulReserved1
            && self.ulReserved2 == other.ulReserved2
            && self.ullReserved1 == other.ullReserved1
            && self.ullReserved2 == other.ullReserved2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_POOL_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_POOL_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VDS_POOL_ATTRIB_ACCS_BDW_WT_HINT: i32 = 16777216i32;
pub const VDS_POOL_ATTRIB_ACCS_DIR_HINT: i32 = 2097152i32;
pub const VDS_POOL_ATTRIB_ACCS_LTNCY_HINT: i32 = 8388608i32;
pub const VDS_POOL_ATTRIB_ACCS_RNDM_HINT: i32 = 1048576i32;
pub const VDS_POOL_ATTRIB_ACCS_SIZE_HINT: i32 = 4194304i32;
pub const VDS_POOL_ATTRIB_ALLOW_SPINDOWN: i32 = 4i32;
pub const VDS_POOL_ATTRIB_BUSTYPE: i32 = 2i32;
pub const VDS_POOL_ATTRIB_CUSTOM_ATTRIB: i32 = 134217728i32;
pub const VDS_POOL_ATTRIB_DATA_AVL_HINT: i32 = 524288i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_DEF: i32 = 128i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MAX: i32 = 32i32;
pub const VDS_POOL_ATTRIB_DATA_RDNCY_MIN: i32 = 64i32;
pub const VDS_POOL_ATTRIB_NO_SINGLE_POF: i32 = 16i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS: i32 = 32768i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_DEF: i32 = 262144i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MAX: i32 = 65536i32;
pub const VDS_POOL_ATTRIB_NUM_CLMNS_MIN: i32 = 131072i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_DEF: i32 = 1024i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MAX: i32 = 256i32;
pub const VDS_POOL_ATTRIB_PKG_RDNCY_MIN: i32 = 512i32;
pub const VDS_POOL_ATTRIB_RAIDTYPE: i32 = 1i32;
pub const VDS_POOL_ATTRIB_STOR_COST_HINT: i32 = 33554432i32;
pub const VDS_POOL_ATTRIB_STOR_EFFCY_HINT: i32 = 67108864i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE: i32 = 2048i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_DEF: i32 = 16384i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MAX: i32 = 4096i32;
pub const VDS_POOL_ATTRIB_STRIPE_SIZE_MIN: i32 = 8192i32;
pub const VDS_POOL_ATTRIB_THIN_PROVISION: i32 = 8i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_POOL_CUSTOM_ATTRIBUTES {
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszValue: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_POOL_CUSTOM_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_POOL_CUSTOM_ATTRIBUTES").field("pwszName", &self.pwszName).field("pwszValue", &self.pwszValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_POOL_CUSTOM_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.pwszValue == other.pwszValue
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_POOL_CUSTOM_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_POOL_CUSTOM_ATTRIBUTES {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_PORTAL_GROUP_NOTIFICATION {
    pub ulEvent: u32,
    pub portalGroupId: ::windows::runtime::GUID,
}
impl VDS_PORTAL_GROUP_NOTIFICATION {}
impl ::std::default::Default for VDS_PORTAL_GROUP_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_PORTAL_GROUP_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PORTAL_GROUP_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portalGroupId", &self.portalGroupId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_PORTAL_GROUP_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.portalGroupId == other.portalGroupId
    }
}
impl ::std::cmp::Eq for VDS_PORTAL_GROUP_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_PORTAL_GROUP_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_PORTAL_NOTIFICATION {
    pub ulEvent: u32,
    pub portalId: ::windows::runtime::GUID,
}
impl VDS_PORTAL_NOTIFICATION {}
impl ::std::default::Default for VDS_PORTAL_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_PORTAL_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PORTAL_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portalId", &self.portalId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_PORTAL_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.portalId == other.portalId
    }
}
impl ::std::cmp::Eq for VDS_PORTAL_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_PORTAL_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_PORT_NOTIFICATION {
    pub ulEvent: VDS_NF_PORT,
    pub portId: ::windows::runtime::GUID,
}
impl VDS_PORT_NOTIFICATION {}
impl ::std::default::Default for VDS_PORT_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_PORT_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PORT_NOTIFICATION").field("ulEvent", &self.ulEvent).field("portId", &self.portId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_PORT_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.portId == other.portId
    }
}
impl ::std::cmp::Eq for VDS_PORT_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_PORT_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PORT_PROP {
    pub id: ::windows::runtime::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub status: VDS_PORT_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_PORT_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_PORT_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_PORT_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PORT_PROP").field("id", &self.id).field("pwszFriendlyName", &self.pwszFriendlyName).field("pwszIdentification", &self.pwszIdentification).field("status", &self.status).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_PORT_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszFriendlyName == other.pwszFriendlyName && self.pwszIdentification == other.pwszIdentification && self.status == other.status
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_PORT_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_PORT_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_PORT_STATUS(pub i32);
pub const VDS_PRS_UNKNOWN: VDS_PORT_STATUS = VDS_PORT_STATUS(0i32);
pub const VDS_PRS_ONLINE: VDS_PORT_STATUS = VDS_PORT_STATUS(1i32);
pub const VDS_PRS_NOT_READY: VDS_PORT_STATUS = VDS_PORT_STATUS(2i32);
pub const VDS_PRS_OFFLINE: VDS_PORT_STATUS = VDS_PORT_STATUS(4i32);
pub const VDS_PRS_FAILED: VDS_PORT_STATUS = VDS_PORT_STATUS(5i32);
pub const VDS_PRS_REMOVED: VDS_PORT_STATUS = VDS_PORT_STATUS(8i32);
impl ::std::convert::From<i32> for VDS_PORT_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_PORT_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_PROVIDER_FLAG(pub i32);
pub const VDS_PF_DYNAMIC: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(1i32);
pub const VDS_PF_INTERNAL_HARDWARE_PROVIDER: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(2i32);
pub const VDS_PF_ONE_DISK_ONLY_PER_PACK: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(4i32);
pub const VDS_PF_ONE_PACK_ONLINE_ONLY: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(8i32);
pub const VDS_PF_VOLUME_SPACE_MUST_BE_CONTIGUOUS: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(16i32);
pub const VDS_PF_SUPPORT_DYNAMIC: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(-2147483648i32);
pub const VDS_PF_SUPPORT_FAULT_TOLERANT: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(1073741824i32);
pub const VDS_PF_SUPPORT_DYNAMIC_1394: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(536870912i32);
pub const VDS_PF_SUPPORT_MIRROR: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(32i32);
pub const VDS_PF_SUPPORT_RAID5: VDS_PROVIDER_FLAG = VDS_PROVIDER_FLAG(64i32);
impl ::std::convert::From<i32> for VDS_PROVIDER_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_PROVIDER_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_PROVIDER_LBSUPPORT_FLAG(pub i32);
pub const VDS_LBF_FAILOVER: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(1i32);
pub const VDS_LBF_ROUND_ROBIN: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(2i32);
pub const VDS_LBF_ROUND_ROBIN_WITH_SUBSET: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(4i32);
pub const VDS_LBF_DYN_LEAST_QUEUE_DEPTH: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(8i32);
pub const VDS_LBF_WEIGHTED_PATHS: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(16i32);
pub const VDS_LBF_LEAST_BLOCKS: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(32i32);
pub const VDS_LBF_VENDOR_SPECIFIC: VDS_PROVIDER_LBSUPPORT_FLAG = VDS_PROVIDER_LBSUPPORT_FLAG(64i32);
impl ::std::convert::From<i32> for VDS_PROVIDER_LBSUPPORT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_PROVIDER_LBSUPPORT_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_PROVIDER_PROP {
    pub id: ::windows::runtime::GUID,
    pub pwszName: super::super::Foundation::PWSTR,
    pub guidVersionId: ::windows::runtime::GUID,
    pub pwszVersion: super::super::Foundation::PWSTR,
    pub r#type: VDS_PROVIDER_TYPE,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_PROVIDER_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_PROVIDER_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_PROVIDER_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_PROVIDER_PROP")
            .field("id", &self.id)
            .field("pwszName", &self.pwszName)
            .field("guidVersionId", &self.guidVersionId)
            .field("pwszVersion", &self.pwszVersion)
            .field("r#type", &self.r#type)
            .field("ulFlags", &self.ulFlags)
            .field("ulStripeSizeFlags", &self.ulStripeSizeFlags)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_PROVIDER_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pwszName == other.pwszName && self.guidVersionId == other.guidVersionId && self.pwszVersion == other.pwszVersion && self.r#type == other.r#type && self.ulFlags == other.ulFlags && self.ulStripeSizeFlags == other.ulStripeSizeFlags && self.sRebuildPriority == other.sRebuildPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_PROVIDER_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_PROVIDER_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_PROVIDER_TYPE(pub i32);
pub const VDS_PT_UNKNOWN: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(0i32);
pub const VDS_PT_SOFTWARE: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(1i32);
pub const VDS_PT_HARDWARE: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(2i32);
pub const VDS_PT_VIRTUALDISK: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(3i32);
pub const VDS_PT_MAX: VDS_PROVIDER_TYPE = VDS_PROVIDER_TYPE(4i32);
impl ::std::convert::From<i32> for VDS_PROVIDER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_PROVIDER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_RAID_TYPE(pub i32);
pub const VDS_RT_UNKNOWN: VDS_RAID_TYPE = VDS_RAID_TYPE(0i32);
pub const VDS_RT_RAID0: VDS_RAID_TYPE = VDS_RAID_TYPE(10i32);
pub const VDS_RT_RAID1: VDS_RAID_TYPE = VDS_RAID_TYPE(11i32);
pub const VDS_RT_RAID2: VDS_RAID_TYPE = VDS_RAID_TYPE(12i32);
pub const VDS_RT_RAID3: VDS_RAID_TYPE = VDS_RAID_TYPE(13i32);
pub const VDS_RT_RAID4: VDS_RAID_TYPE = VDS_RAID_TYPE(14i32);
pub const VDS_RT_RAID5: VDS_RAID_TYPE = VDS_RAID_TYPE(15i32);
pub const VDS_RT_RAID6: VDS_RAID_TYPE = VDS_RAID_TYPE(16i32);
pub const VDS_RT_RAID01: VDS_RAID_TYPE = VDS_RAID_TYPE(17i32);
pub const VDS_RT_RAID03: VDS_RAID_TYPE = VDS_RAID_TYPE(18i32);
pub const VDS_RT_RAID05: VDS_RAID_TYPE = VDS_RAID_TYPE(19i32);
pub const VDS_RT_RAID10: VDS_RAID_TYPE = VDS_RAID_TYPE(20i32);
pub const VDS_RT_RAID15: VDS_RAID_TYPE = VDS_RAID_TYPE(21i32);
pub const VDS_RT_RAID30: VDS_RAID_TYPE = VDS_RAID_TYPE(22i32);
pub const VDS_RT_RAID50: VDS_RAID_TYPE = VDS_RAID_TYPE(23i32);
pub const VDS_RT_RAID51: VDS_RAID_TYPE = VDS_RAID_TYPE(24i32);
pub const VDS_RT_RAID53: VDS_RAID_TYPE = VDS_RAID_TYPE(25i32);
pub const VDS_RT_RAID60: VDS_RAID_TYPE = VDS_RAID_TYPE(26i32);
pub const VDS_RT_RAID61: VDS_RAID_TYPE = VDS_RAID_TYPE(27i32);
impl ::std::convert::From<i32> for VDS_RAID_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_RAID_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VDS_REBUILD_PRIORITY_MAX: u32 = 16u32;
pub const VDS_REBUILD_PRIORITY_MIN: u32 = 0u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_RECOVER_ACTION(pub i32);
pub const VDS_RA_UNKNOWN: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(0i32);
pub const VDS_RA_REFRESH: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(1i32);
pub const VDS_RA_RESTART: VDS_RECOVER_ACTION = VDS_RECOVER_ACTION(2i32);
impl ::std::convert::From<i32> for VDS_RECOVER_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_RECOVER_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_SERVICE_NOTIFICATION {
    pub ulEvent: u32,
    pub action: VDS_RECOVER_ACTION,
}
impl VDS_SERVICE_NOTIFICATION {}
impl ::std::default::Default for VDS_SERVICE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_SERVICE_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_SERVICE_NOTIFICATION").field("ulEvent", &self.ulEvent).field("action", &self.action).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_SERVICE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.action == other.action
    }
}
impl ::std::cmp::Eq for VDS_SERVICE_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_SERVICE_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_STORAGE_BUS_TYPE(pub i32);
pub const VDSBusTypeUnknown: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(0i32);
pub const VDSBusTypeScsi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(1i32);
pub const VDSBusTypeAtapi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(2i32);
pub const VDSBusTypeAta: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(3i32);
pub const VDSBusType1394: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(4i32);
pub const VDSBusTypeSsa: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(5i32);
pub const VDSBusTypeFibre: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(6i32);
pub const VDSBusTypeUsb: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(7i32);
pub const VDSBusTypeRAID: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(8i32);
pub const VDSBusTypeiScsi: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(9i32);
pub const VDSBusTypeSas: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(10i32);
pub const VDSBusTypeSata: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(11i32);
pub const VDSBusTypeSd: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(12i32);
pub const VDSBusTypeMmc: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(13i32);
pub const VDSBusTypeMax: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(14i32);
pub const VDSBusTypeVirtual: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(14i32);
pub const VDSBusTypeFileBackedVirtual: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(15i32);
pub const VDSBusTypeSpaces: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(16i32);
pub const VDSBusTypeNVMe: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(17i32);
pub const VDSBusTypeScm: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(18i32);
pub const VDSBusTypeUfs: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(19i32);
pub const VDSBusTypeMaxReserved: VDS_STORAGE_BUS_TYPE = VDS_STORAGE_BUS_TYPE(127i32);
impl ::std::convert::From<i32> for VDS_STORAGE_BUS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_BUS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    pub m_version: u32,
    pub m_cIdentifiers: u32,
    pub m_rgIdentifiers: *mut VDS_STORAGE_IDENTIFIER,
}
impl VDS_STORAGE_DEVICE_ID_DESCRIPTOR {}
impl ::std::default::Default for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_STORAGE_DEVICE_ID_DESCRIPTOR").field("m_version", &self.m_version).field("m_cIdentifiers", &self.m_cIdentifiers).field("m_rgIdentifiers", &self.m_rgIdentifiers).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.m_version == other.m_version && self.m_cIdentifiers == other.m_cIdentifiers && self.m_rgIdentifiers == other.m_rgIdentifiers
    }
}
impl ::std::cmp::Eq for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {}
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_DEVICE_ID_DESCRIPTOR {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_STORAGE_IDENTIFIER {
    pub m_CodeSet: VDS_STORAGE_IDENTIFIER_CODE_SET,
    pub m_Type: VDS_STORAGE_IDENTIFIER_TYPE,
    pub m_cbIdentifier: u32,
    pub m_rgbIdentifier: *mut u8,
}
impl VDS_STORAGE_IDENTIFIER {}
impl ::std::default::Default for VDS_STORAGE_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_STORAGE_IDENTIFIER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_STORAGE_IDENTIFIER").field("m_CodeSet", &self.m_CodeSet).field("m_Type", &self.m_Type).field("m_cbIdentifier", &self.m_cbIdentifier).field("m_rgbIdentifier", &self.m_rgbIdentifier).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_STORAGE_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.m_CodeSet == other.m_CodeSet && self.m_Type == other.m_Type && self.m_cbIdentifier == other.m_cbIdentifier && self.m_rgbIdentifier == other.m_rgbIdentifier
    }
}
impl ::std::cmp::Eq for VDS_STORAGE_IDENTIFIER {}
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_IDENTIFIER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_STORAGE_IDENTIFIER_CODE_SET(pub i32);
pub const VDSStorageIdCodeSetReserved: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(0i32);
pub const VDSStorageIdCodeSetBinary: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(1i32);
pub const VDSStorageIdCodeSetAscii: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(2i32);
pub const VDSStorageIdCodeSetUtf8: VDS_STORAGE_IDENTIFIER_CODE_SET = VDS_STORAGE_IDENTIFIER_CODE_SET(3i32);
impl ::std::convert::From<i32> for VDS_STORAGE_IDENTIFIER_CODE_SET {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_IDENTIFIER_CODE_SET {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_STORAGE_IDENTIFIER_TYPE(pub i32);
pub const VDSStorageIdTypeVendorSpecific: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(0i32);
pub const VDSStorageIdTypeVendorId: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(1i32);
pub const VDSStorageIdTypeEUI64: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(2i32);
pub const VDSStorageIdTypeFCPHName: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(3i32);
pub const VDSStorageIdTypePortRelative: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(4i32);
pub const VDSStorageIdTypeTargetPortGroup: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(5i32);
pub const VDSStorageIdTypeLogicalUnitGroup: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(6i32);
pub const VDSStorageIdTypeMD5LogicalUnitIdentifier: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(7i32);
pub const VDSStorageIdTypeScsiNameString: VDS_STORAGE_IDENTIFIER_TYPE = VDS_STORAGE_IDENTIFIER_TYPE(8i32);
impl ::std::convert::From<i32> for VDS_STORAGE_IDENTIFIER_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_IDENTIFIER_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_STORAGE_POOL_DRIVE_EXTENT {
    pub id: ::windows::runtime::GUID,
    pub ullSize: u64,
    pub bUsed: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_STORAGE_POOL_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_STORAGE_POOL_DRIVE_EXTENT").field("id", &self.id).field("ullSize", &self.ullSize).field("bUsed", &self.bUsed).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_STORAGE_POOL_DRIVE_EXTENT {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.ullSize == other.ullSize && self.bUsed == other.bUsed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_STORAGE_POOL_DRIVE_EXTENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_POOL_DRIVE_EXTENT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_STORAGE_POOL_PROP {
    pub id: ::windows::runtime::GUID,
    pub status: VDS_STORAGE_POOL_STATUS,
    pub health: VDS_HEALTH,
    pub r#type: VDS_STORAGE_POOL_TYPE,
    pub pwszName: super::super::Foundation::PWSTR,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub ullTotalConsumedSpace: u64,
    pub ullTotalManagedSpace: u64,
    pub ullRemainingFreeSpace: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_STORAGE_POOL_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_STORAGE_POOL_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_STORAGE_POOL_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_STORAGE_POOL_PROP")
            .field("id", &self.id)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("r#type", &self.r#type)
            .field("pwszName", &self.pwszName)
            .field("pwszDescription", &self.pwszDescription)
            .field("ullTotalConsumedSpace", &self.ullTotalConsumedSpace)
            .field("ullTotalManagedSpace", &self.ullTotalManagedSpace)
            .field("ullRemainingFreeSpace", &self.ullRemainingFreeSpace)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_STORAGE_POOL_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.status == other.status && self.health == other.health && self.r#type == other.r#type && self.pwszName == other.pwszName && self.pwszDescription == other.pwszDescription && self.ullTotalConsumedSpace == other.ullTotalConsumedSpace && self.ullTotalManagedSpace == other.ullTotalManagedSpace && self.ullRemainingFreeSpace == other.ullRemainingFreeSpace
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_STORAGE_POOL_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_POOL_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_STORAGE_POOL_STATUS(pub i32);
pub const VDS_SPS_UNKNOWN: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(0i32);
pub const VDS_SPS_ONLINE: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(1i32);
pub const VDS_SPS_NOT_READY: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(2i32);
pub const VDS_SPS_OFFLINE: VDS_STORAGE_POOL_STATUS = VDS_STORAGE_POOL_STATUS(4i32);
impl ::std::convert::From<i32> for VDS_STORAGE_POOL_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_POOL_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_STORAGE_POOL_TYPE(pub i32);
pub const VDS_SPT_UNKNOWN: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(0i32);
pub const VDS_SPT_PRIMORDIAL: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(1i32);
pub const VDS_SPT_CONCRETE: VDS_STORAGE_POOL_TYPE = VDS_STORAGE_POOL_TYPE(2i32);
impl ::std::convert::From<i32> for VDS_STORAGE_POOL_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_STORAGE_POOL_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_SUB_SYSTEM_FLAG(pub i32);
pub const VDS_SF_LUN_MASKING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1i32);
pub const VDS_SF_LUN_PLEXING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2i32);
pub const VDS_SF_LUN_REMAPPING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4i32);
pub const VDS_SF_DRIVE_EXTENT_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8i32);
pub const VDS_SF_HARDWARE_CHECKSUM_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16i32);
pub const VDS_SF_RADIUS_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(32i32);
pub const VDS_SF_READ_BACK_VERIFY_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(64i32);
pub const VDS_SF_WRITE_THROUGH_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(128i32);
pub const VDS_SF_SUPPORTS_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(512i32);
pub const VDS_SF_SUPPORTS_NON_FAULT_TOLERANT_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1024i32);
pub const VDS_SF_SUPPORTS_SIMPLE_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2048i32);
pub const VDS_SF_SUPPORTS_SPAN_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4096i32);
pub const VDS_SF_SUPPORTS_STRIPE_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8192i32);
pub const VDS_SF_SUPPORTS_MIRROR_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16384i32);
pub const VDS_SF_SUPPORTS_PARITY_LUNS: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(32768i32);
pub const VDS_SF_SUPPORTS_AUTH_CHAP: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(65536i32);
pub const VDS_SF_SUPPORTS_AUTH_MUTUAL_CHAP: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(131072i32);
pub const VDS_SF_SUPPORTS_SIMPLE_TARGET_CONFIG: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(262144i32);
pub const VDS_SF_SUPPORTS_LUN_NUMBER: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(524288i32);
pub const VDS_SF_SUPPORTS_MIRRORED_CACHE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(1048576i32);
pub const VDS_SF_READ_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(2097152i32);
pub const VDS_SF_WRITE_CACHING_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(4194304i32);
pub const VDS_SF_MEDIA_SCAN_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(8388608i32);
pub const VDS_SF_CONSISTENCY_CHECK_CAPABLE: VDS_SUB_SYSTEM_FLAG = VDS_SUB_SYSTEM_FLAG(16777216i32);
impl ::std::convert::From<i32> for VDS_SUB_SYSTEM_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_SUB_SYSTEM_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_SUB_SYSTEM_NOTIFICATION {
    pub ulEvent: u32,
    pub subSystemId: ::windows::runtime::GUID,
}
impl VDS_SUB_SYSTEM_NOTIFICATION {}
impl ::std::default::Default for VDS_SUB_SYSTEM_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_SUB_SYSTEM_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_SUB_SYSTEM_NOTIFICATION").field("ulEvent", &self.ulEvent).field("subSystemId", &self.subSystemId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_SUB_SYSTEM_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.subSystemId == other.subSystemId
    }
}
impl ::std::cmp::Eq for VDS_SUB_SYSTEM_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_SUB_SYSTEM_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_SUB_SYSTEM_PROP {
    pub id: ::windows::runtime::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_SUB_SYSTEM_PROP {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_SUB_SYSTEM_PROP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_SUB_SYSTEM_PROP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_SUB_SYSTEM_PROP")
            .field("id", &self.id)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("ulStripeSizeFlags", &self.ulStripeSizeFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sNumberOfInternalBuses", &self.sNumberOfInternalBuses)
            .field("sMaxNumberOfSlotsEachBus", &self.sMaxNumberOfSlotsEachBus)
            .field("sMaxNumberOfControllers", &self.sMaxNumberOfControllers)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_SUB_SYSTEM_PROP {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.pwszFriendlyName == other.pwszFriendlyName
            && self.pwszIdentification == other.pwszIdentification
            && self.ulFlags == other.ulFlags
            && self.ulStripeSizeFlags == other.ulStripeSizeFlags
            && self.status == other.status
            && self.health == other.health
            && self.sNumberOfInternalBuses == other.sNumberOfInternalBuses
            && self.sMaxNumberOfSlotsEachBus == other.sMaxNumberOfSlotsEachBus
            && self.sMaxNumberOfControllers == other.sMaxNumberOfControllers
            && self.sRebuildPriority == other.sRebuildPriority
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_SUB_SYSTEM_PROP {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_SUB_SYSTEM_PROP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct VDS_SUB_SYSTEM_PROP2 {
    pub id: ::windows::runtime::GUID,
    pub pwszFriendlyName: super::super::Foundation::PWSTR,
    pub pwszIdentification: super::super::Foundation::PWSTR,
    pub ulFlags: u32,
    pub ulStripeSizeFlags: u32,
    pub ulSupportedRaidTypeFlags: u32,
    pub status: VDS_SUB_SYSTEM_STATUS,
    pub health: VDS_HEALTH,
    pub sNumberOfInternalBuses: i16,
    pub sMaxNumberOfSlotsEachBus: i16,
    pub sMaxNumberOfControllers: i16,
    pub sRebuildPriority: i16,
    pub ulNumberOfEnclosures: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl VDS_SUB_SYSTEM_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for VDS_SUB_SYSTEM_PROP2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for VDS_SUB_SYSTEM_PROP2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_SUB_SYSTEM_PROP2")
            .field("id", &self.id)
            .field("pwszFriendlyName", &self.pwszFriendlyName)
            .field("pwszIdentification", &self.pwszIdentification)
            .field("ulFlags", &self.ulFlags)
            .field("ulStripeSizeFlags", &self.ulStripeSizeFlags)
            .field("ulSupportedRaidTypeFlags", &self.ulSupportedRaidTypeFlags)
            .field("status", &self.status)
            .field("health", &self.health)
            .field("sNumberOfInternalBuses", &self.sNumberOfInternalBuses)
            .field("sMaxNumberOfSlotsEachBus", &self.sMaxNumberOfSlotsEachBus)
            .field("sMaxNumberOfControllers", &self.sMaxNumberOfControllers)
            .field("sRebuildPriority", &self.sRebuildPriority)
            .field("ulNumberOfEnclosures", &self.ulNumberOfEnclosures)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for VDS_SUB_SYSTEM_PROP2 {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.pwszFriendlyName == other.pwszFriendlyName
            && self.pwszIdentification == other.pwszIdentification
            && self.ulFlags == other.ulFlags
            && self.ulStripeSizeFlags == other.ulStripeSizeFlags
            && self.ulSupportedRaidTypeFlags == other.ulSupportedRaidTypeFlags
            && self.status == other.status
            && self.health == other.health
            && self.sNumberOfInternalBuses == other.sNumberOfInternalBuses
            && self.sMaxNumberOfSlotsEachBus == other.sMaxNumberOfSlotsEachBus
            && self.sMaxNumberOfControllers == other.sMaxNumberOfControllers
            && self.sRebuildPriority == other.sRebuildPriority
            && self.ulNumberOfEnclosures == other.ulNumberOfEnclosures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for VDS_SUB_SYSTEM_PROP2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for VDS_SUB_SYSTEM_PROP2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_SUB_SYSTEM_STATUS(pub i32);
pub const VDS_SSS_UNKNOWN: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(0i32);
pub const VDS_SSS_ONLINE: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(1i32);
pub const VDS_SSS_NOT_READY: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(2i32);
pub const VDS_SSS_OFFLINE: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(4i32);
pub const VDS_SSS_FAILED: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(5i32);
pub const VDS_SSS_PARTIALLY_MANAGED: VDS_SUB_SYSTEM_STATUS = VDS_SUB_SYSTEM_STATUS(9i32);
impl ::std::convert::From<i32> for VDS_SUB_SYSTEM_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_SUB_SYSTEM_STATUS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(pub i32);
pub const VDS_SF_SUPPORTS_RAID2_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(1i32);
pub const VDS_SF_SUPPORTS_RAID3_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(2i32);
pub const VDS_SF_SUPPORTS_RAID4_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(4i32);
pub const VDS_SF_SUPPORTS_RAID5_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(8i32);
pub const VDS_SF_SUPPORTS_RAID6_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(16i32);
pub const VDS_SF_SUPPORTS_RAID01_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(32i32);
pub const VDS_SF_SUPPORTS_RAID03_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(64i32);
pub const VDS_SF_SUPPORTS_RAID05_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(128i32);
pub const VDS_SF_SUPPORTS_RAID10_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(256i32);
pub const VDS_SF_SUPPORTS_RAID15_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(512i32);
pub const VDS_SF_SUPPORTS_RAID30_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(1024i32);
pub const VDS_SF_SUPPORTS_RAID50_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(2048i32);
pub const VDS_SF_SUPPORTS_RAID51_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(4096i32);
pub const VDS_SF_SUPPORTS_RAID53_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(8192i32);
pub const VDS_SF_SUPPORTS_RAID60_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(16384i32);
pub const VDS_SF_SUPPORTS_RAID61_LUNS: VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG = VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG(32768i32);
impl ::std::convert::From<i32> for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_SUB_SYSTEM_SUPPORTED_RAID_TYPE_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VDS_S_ACCESS_PATH_NOT_DELETED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(279108i32 as _);
pub const VDS_S_ALREADY_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272148i32 as _);
pub const VDS_S_BOOT_PARTITION_NUMBER_CHANGE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271414i32 as _);
pub const VDS_S_DEFAULT_PLEX_MEMBER_IDS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271640i32 as _);
pub const VDS_S_DISK_DISMOUNT_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272393i32 as _);
pub const VDS_S_DISK_IS_MISSING: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271624i32 as _);
pub const VDS_S_DISK_MOUNT_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272392i32 as _);
pub const VDS_S_DISK_PARTIALLY_CLEANED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271386i32 as _);
pub const VDS_S_DISMOUNT_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271735i32 as _);
pub const VDS_S_EXTEND_FILE_SYSTEM_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271461i32 as _);
pub const VDS_S_FS_LOCK: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271747i32 as _);
pub const VDS_S_GPT_BOOT_MIRRORED_TO_MBR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(-2147212183i32 as _);
pub const VDS_S_IA64_BOOT_MIRRORED_TO_MBR: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271450i32 as _);
pub const VDS_S_IN_PROGRESS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271437i32 as _);
pub const VDS_S_ISCSI_LOGIN_ALREAD_EXISTS: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272386i32 as _);
pub const VDS_S_ISCSI_PERSISTENT_LOGIN_MAY_NOT_BE_REMOVED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272385i32 as _);
pub const VDS_S_ISCSI_SESSION_NOT_FOUND_PERSISTENT_LOGIN_REMOVED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272384i32 as _);
pub const VDS_S_MBR_BOOT_MIRRORED_TO_GPT: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271463i32 as _);
pub const VDS_S_NAME_TRUNCATED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272128i32 as _);
pub const VDS_S_NONCONFORMANT_PARTITION_INFO: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271626i32 as _);
pub const VDS_S_NO_NOTIFICATION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271639i32 as _);
pub const VDS_S_PLEX_NOT_LOADED_TO_CACHE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271755i32 as _);
pub const VDS_S_PROPERTIES_INCOMPLETE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272149i32 as _);
pub const VDS_S_PROVIDER_ERROR_LOADING_CACHE: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271393i32 as _);
pub const VDS_S_REMOUNT_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271736i32 as _);
pub const VDS_S_RESYNC_NOTIFICATION_TASK_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271738i32 as _);
pub const VDS_S_STATUSES_INCOMPLETELY_SET: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(272130i32 as _);
pub const VDS_S_SYSTEM_PARTITION: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271630i32 as _);
pub const VDS_S_UNABLE_TO_GET_GPT_ATTRIBUTES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271451i32 as _);
pub const VDS_S_UPDATE_BOOTFILE_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271412i32 as _);
pub const VDS_S_VOLUME_COMPRESS_FAILED: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271427i32 as _);
pub const VDS_S_VSS_FLUSH_AND_HOLD_WRITES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271745i32 as _);
pub const VDS_S_VSS_RELEASE_WRITES: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271746i32 as _);
pub const VDS_S_WINPE_BOOTENTRY: ::windows::runtime::HRESULT = ::windows::runtime::HRESULT(271758i32 as _);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_TARGET_NOTIFICATION {
    pub ulEvent: u32,
    pub targetId: ::windows::runtime::GUID,
}
impl VDS_TARGET_NOTIFICATION {}
impl ::std::default::Default for VDS_TARGET_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_TARGET_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_TARGET_NOTIFICATION").field("ulEvent", &self.ulEvent).field("targetId", &self.targetId).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_TARGET_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.targetId == other.targetId
    }
}
impl ::std::cmp::Eq for VDS_TARGET_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_TARGET_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_TRANSITION_STATE(pub i32);
pub const VDS_TS_UNKNOWN: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(0i32);
pub const VDS_TS_STABLE: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(1i32);
pub const VDS_TS_EXTENDING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(2i32);
pub const VDS_TS_SHRINKING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(3i32);
pub const VDS_TS_RECONFIGING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(4i32);
pub const VDS_TS_RESTRIPING: VDS_TRANSITION_STATE = VDS_TRANSITION_STATE(5i32);
impl ::std::convert::From<i32> for VDS_TRANSITION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_TRANSITION_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct VDS_VERSION_SUPPORT_FLAG(pub i32);
pub const VDS_VSF_1_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(1i32);
pub const VDS_VSF_1_1: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(2i32);
pub const VDS_VSF_2_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(4i32);
pub const VDS_VSF_2_1: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(8i32);
pub const VDS_VSF_3_0: VDS_VERSION_SUPPORT_FLAG = VDS_VERSION_SUPPORT_FLAG(16i32);
impl ::std::convert::From<i32> for VDS_VERSION_SUPPORT_FLAG {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for VDS_VERSION_SUPPORT_FLAG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_VOLUME_NOTIFICATION {
    pub ulEvent: u32,
    pub volumeId: ::windows::runtime::GUID,
    pub plexId: ::windows::runtime::GUID,
    pub ulPercentCompleted: u32,
}
impl VDS_VOLUME_NOTIFICATION {}
impl ::std::default::Default for VDS_VOLUME_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_VOLUME_NOTIFICATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_VOLUME_NOTIFICATION").field("ulEvent", &self.ulEvent).field("volumeId", &self.volumeId).field("plexId", &self.plexId).field("ulPercentCompleted", &self.ulPercentCompleted).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_VOLUME_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.ulEvent == other.ulEvent && self.volumeId == other.volumeId && self.plexId == other.plexId && self.ulPercentCompleted == other.ulPercentCompleted
    }
}
impl ::std::cmp::Eq for VDS_VOLUME_NOTIFICATION {}
unsafe impl ::windows::runtime::Abi for VDS_VOLUME_NOTIFICATION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct VDS_WWN {
    pub rguchWwn: [u8; 8],
}
impl VDS_WWN {}
impl ::std::default::Default for VDS_WWN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for VDS_WWN {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("VDS_WWN").field("rguchWwn", &self.rguchWwn).finish()
    }
}
impl ::std::cmp::PartialEq for VDS_WWN {
    fn eq(&self, other: &Self) -> bool {
        self.rguchWwn == other.rguchWwn
    }
}
impl ::std::cmp::Eq for VDS_WWN {}
unsafe impl ::windows::runtime::Abi for VDS_WWN {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VER_VDS_LUN_INFORMATION: u32 = 1u32;
