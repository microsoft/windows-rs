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
pub struct IWindowsMediaLibrarySharingDevice(::windows::runtime::IUnknown);
impl IWindowsMediaLibrarySharingDevice {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DeviceID(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    pub unsafe fn Authorization(
        &self,
    ) -> ::windows::runtime::Result<WindowsMediaLibrarySharingDeviceAuthorizationStatus> {
        let mut result__ : < WindowsMediaLibrarySharingDeviceAuthorizationStatus as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<WindowsMediaLibrarySharingDeviceAuthorizationStatus>(result__)
    }
    pub unsafe fn SetAuthorization(
        &self,
        authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(authorization),
        )
        .ok()
    }
    pub unsafe fn Properties(
        &self,
    ) -> ::windows::runtime::Result<IWindowsMediaLibrarySharingDeviceProperties> {
        let mut result__ : < IWindowsMediaLibrarySharingDeviceProperties as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IWindowsMediaLibrarySharingDeviceProperties>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsMediaLibrarySharingDevice {
    type Vtable = IWindowsMediaLibrarySharingDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1036829331,
        20441,
        16785,
        [162, 91, 142, 87, 197, 210, 123, 212],
    );
}
impl ::std::convert::From<IWindowsMediaLibrarySharingDevice> for ::windows::runtime::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWindowsMediaLibrarySharingDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWindowsMediaLibrarySharingDevice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWindowsMediaLibrarySharingDevice
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWindowsMediaLibrarySharingDevice>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: IWindowsMediaLibrarySharingDevice) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWindowsMediaLibrarySharingDevice>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &IWindowsMediaLibrarySharingDevice) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for IWindowsMediaLibrarySharingDevice
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &IWindowsMediaLibrarySharingDevice
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevice_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        authorization: *mut WindowsMediaLibrarySharingDeviceAuthorizationStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        authorization: WindowsMediaLibrarySharingDeviceAuthorizationStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceproperties: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWindowsMediaLibrarySharingDeviceProperties(::windows::runtime::IUnknown);
impl IWindowsMediaLibrarySharingDeviceProperties {
    pub unsafe fn Item(
        &self,
        index: i32,
    ) -> ::windows::runtime::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__ : < IWindowsMediaLibrarySharingDeviceProperty as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<IWindowsMediaLibrarySharingDeviceProperty>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProperty<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        name: Param0,
    ) -> ::windows::runtime::Result<IWindowsMediaLibrarySharingDeviceProperty> {
        let mut result__ : < IWindowsMediaLibrarySharingDeviceProperty as :: windows :: runtime :: Abi > :: Abi = :: std :: mem :: zeroed ( ) ;
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            name.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWindowsMediaLibrarySharingDeviceProperty>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsMediaLibrarySharingDeviceProperties {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3294769684,
        27398,
        16581,
        [166, 35, 178, 255, 76, 7, 107, 253],
    );
}
impl ::std::convert::From<IWindowsMediaLibrarySharingDeviceProperties>
    for ::windows::runtime::IUnknown
{
    fn from(value: IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWindowsMediaLibrarySharingDeviceProperties>
    for ::windows::runtime::IUnknown
{
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWindowsMediaLibrarySharingDeviceProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWindowsMediaLibrarySharingDeviceProperties
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWindowsMediaLibrarySharingDeviceProperties>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWindowsMediaLibrarySharingDeviceProperties>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperties) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for IWindowsMediaLibrarySharingDeviceProperties
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &IWindowsMediaLibrarySharingDeviceProperties
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperties_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        property: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        property: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWindowsMediaLibrarySharingDeviceProperty(::windows::runtime::IUnknown);
impl IWindowsMediaLibrarySharingDeviceProperty {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub unsafe fn Value(&self) -> ::windows::runtime::Result<super::super::System::Com::VARIANT> {
        let mut result__: <super::super::System::Com::VARIANT as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::System::Com::VARIANT>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsMediaLibrarySharingDeviceProperty {
    type Vtable = IWindowsMediaLibrarySharingDeviceProperty_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2179098919,
        31357,
        16551,
        [129, 212, 189, 220, 2, 150, 14, 62],
    );
}
impl ::std::convert::From<IWindowsMediaLibrarySharingDeviceProperty>
    for ::windows::runtime::IUnknown
{
    fn from(value: IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWindowsMediaLibrarySharingDeviceProperty>
    for ::windows::runtime::IUnknown
{
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWindowsMediaLibrarySharingDeviceProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWindowsMediaLibrarySharingDeviceProperty
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWindowsMediaLibrarySharingDeviceProperty>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWindowsMediaLibrarySharingDeviceProperty>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &IWindowsMediaLibrarySharingDeviceProperty) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for IWindowsMediaLibrarySharingDeviceProperty
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &IWindowsMediaLibrarySharingDeviceProperty
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDeviceProperty_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        name: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWindowsMediaLibrarySharingDevices(::windows::runtime::IUnknown);
impl IWindowsMediaLibrarySharingDevices {
    pub unsafe fn Item(
        &self,
        index: i32,
    ) -> ::windows::runtime::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__: <IWindowsMediaLibrarySharingDevice as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(index),
            &mut result__,
        )
        .from_abi::<IWindowsMediaLibrarySharingDevice>(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDevice<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        deviceid: Param0,
    ) -> ::windows::runtime::Result<IWindowsMediaLibrarySharingDevice> {
        let mut result__: <IWindowsMediaLibrarySharingDevice as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            deviceid.into_param().abi(),
            &mut result__,
        )
        .from_abi::<IWindowsMediaLibrarySharingDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsMediaLibrarySharingDevices {
    type Vtable = IWindowsMediaLibrarySharingDevices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        402913750,
        65133,
        17734,
        [191, 91, 153, 47, 232, 236, 18, 209],
    );
}
impl ::std::convert::From<IWindowsMediaLibrarySharingDevices> for ::windows::runtime::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingDevices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWindowsMediaLibrarySharingDevices> for ::windows::runtime::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingDevices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWindowsMediaLibrarySharingDevices
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWindowsMediaLibrarySharingDevices
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWindowsMediaLibrarySharingDevices>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: IWindowsMediaLibrarySharingDevices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWindowsMediaLibrarySharingDevices>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &IWindowsMediaLibrarySharingDevices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for IWindowsMediaLibrarySharingDevices
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &IWindowsMediaLibrarySharingDevices
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingDevices_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        index: i32,
        device: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        count: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        deviceid: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        device: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWindowsMediaLibrarySharingServices(::windows::runtime::IUnknown);
impl IWindowsMediaLibrarySharingServices {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn showShareMediaCPL<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        device: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            device.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn userHomeMediaSharingState(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn SetuserHomeMediaSharingState(
        &self,
        sharingenabled: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(sharingenabled),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn userHomeMediaSharingLibraryName(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetuserHomeMediaSharingLibraryName<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        libraryname: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(
            ::std::mem::transmute_copy(self),
            libraryname.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn computerHomeMediaSharingAllowedState(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn SetcomputerHomeMediaSharingAllowedState(
        &self,
        sharingallowed: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(sharingallowed),
        )
        .ok()
    }
    pub unsafe fn userInternetMediaSharingState(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn SetuserInternetMediaSharingState(
        &self,
        sharingenabled: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(sharingenabled),
        )
        .ok()
    }
    pub unsafe fn computerInternetMediaSharingAllowedState(
        &self,
    ) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn SetcomputerInternetMediaSharingAllowedState(
        &self,
        sharingallowed: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(sharingallowed),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn internetMediaSharingSecurityGroup(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetinternetMediaSharingSecurityGroup<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        securitygroup: Param0,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            securitygroup.into_param().abi(),
        )
        .ok()
    }
    pub unsafe fn allowSharingToAllDevices(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
    pub unsafe fn SetallowSharingToAllDevices(
        &self,
        sharingenabled: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(sharingenabled),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setDefaultAuthorization<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        macaddresses: Param0,
        friendlyname: Param1,
        authorization: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(
            ::std::mem::transmute_copy(self),
            macaddresses.into_param().abi(),
            friendlyname.into_param().abi(),
            ::std::mem::transmute(authorization),
        )
        .ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn setAuthorizationState<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BSTR>,
    >(
        &self,
        macaddress: Param0,
        authorizationstate: i16,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(
            ::std::mem::transmute_copy(self),
            macaddress.into_param().abi(),
            ::std::mem::transmute(authorizationstate),
        )
        .ok()
    }
    pub unsafe fn getAllDevices(
        &self,
    ) -> ::windows::runtime::Result<IWindowsMediaLibrarySharingDevices> {
        let mut result__: <IWindowsMediaLibrarySharingDevices as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<IWindowsMediaLibrarySharingDevices>(result__)
    }
    pub unsafe fn customSettingsApplied(&self) -> ::windows::runtime::Result<i16> {
        let mut result__: <i16 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).25)(
            ::std::mem::transmute_copy(self),
            &mut result__,
        )
        .from_abi::<i16>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsMediaLibrarySharingServices {
    type Vtable = IWindowsMediaLibrarySharingServices_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        32897118,
        2689,
        16602,
        [167, 200, 33, 239, 58, 248, 68, 12],
    );
}
impl ::std::convert::From<IWindowsMediaLibrarySharingServices> for ::windows::runtime::IUnknown {
    fn from(value: IWindowsMediaLibrarySharingServices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWindowsMediaLibrarySharingServices> for ::windows::runtime::IUnknown {
    fn from(value: &IWindowsMediaLibrarySharingServices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWindowsMediaLibrarySharingServices
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWindowsMediaLibrarySharingServices
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<IWindowsMediaLibrarySharingServices>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: IWindowsMediaLibrarySharingServices) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl ::std::convert::From<&IWindowsMediaLibrarySharingServices>
    for super::super::System::Ole::Automation::IDispatch
{
    fn from(value: &IWindowsMediaLibrarySharingServices) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for IWindowsMediaLibrarySharingServices
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(self))
    }
}
#[cfg(feature = "Win32_System_Ole_Automation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::System::Ole::Automation::IDispatch>
    for &IWindowsMediaLibrarySharingServices
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::System::Ole::Automation::IDispatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::System::Ole::Automation::IDispatch,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsMediaLibrarySharingServices_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pctinfo: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_Ole_Automation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        itinfo: u32,
        lcid: u32,
        pptinfo: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Ole_Automation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        riid: *const ::windows::runtime::GUID,
        rgsznames: *const super::super::Foundation::PWSTR,
        cnames: u32,
        lcid: u32,
        rgdispid: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        dispidmember: i32,
        riid: *const ::windows::runtime::GUID,
        lcid: u32,
        wflags: u16,
        pdispparams: *const super::super::System::Ole::Automation::DISPPARAMS,
        pvarresult: *mut ::std::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
        pexcepinfo: *mut ::std::mem::ManuallyDrop<super::super::System::Ole::Automation::EXCEPINFO>,
        puargerr: *mut u32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "Win32_Foundation",
        feature = "Win32_System_Com",
        feature = "Win32_System_Ole_Automation"
    )))]
    usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        device: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingenabled: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingenabled: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        libraryname: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        libraryname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingallowed: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingallowed: i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingenabled: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingenabled: i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingallowed: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingallowed: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        securitygroup: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        securitygroup: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingenabled: *mut i16,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sharingenabled: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        macaddresses: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        friendlyname: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        authorization: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        macaddress: ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>,
        authorizationstate: i16,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        devices: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        customsettingsapplied: *mut i16,
    ) -> ::windows::runtime::HRESULT,
);
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct WindowsMediaLibrarySharingDeviceAuthorizationStatus(pub i32);
pub const DEVICE_AUTHORIZATION_UNKNOWN: WindowsMediaLibrarySharingDeviceAuthorizationStatus =
    WindowsMediaLibrarySharingDeviceAuthorizationStatus(0i32);
pub const DEVICE_AUTHORIZATION_ALLOWED: WindowsMediaLibrarySharingDeviceAuthorizationStatus =
    WindowsMediaLibrarySharingDeviceAuthorizationStatus(1i32);
pub const DEVICE_AUTHORIZATION_DENIED: WindowsMediaLibrarySharingDeviceAuthorizationStatus =
    WindowsMediaLibrarySharingDeviceAuthorizationStatus(2i32);
impl ::std::convert::From<i32> for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for WindowsMediaLibrarySharingDeviceAuthorizationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
pub const WindowsMediaLibrarySharingServices: ::windows::runtime::GUID =
    ::windows::runtime::GUID::from_values(
        2908232448,
        31588,
        20057,
        [163, 141, 210, 197, 191, 81, 221, 179],
    );
