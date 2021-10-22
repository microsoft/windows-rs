#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub struct ClassicAppManager {}
impl ClassicAppManager {
    pub fn FindInstalledApp<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        appuninstallkey: Param0,
    ) -> ::windows::runtime::Result<InstalledClassicAppInfo> {
        Self::IClassicAppManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                appuninstallkey.into_param().abi(),
                &mut result__,
            )
            .from_abi::<InstalledClassicAppInfo>(result__)
        })
    }
    pub fn IClassicAppManagerStatics<
        R,
        F: FnOnce(&IClassicAppManagerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            ClassicAppManager,
            IClassicAppManagerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for ClassicAppManager {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.ClassicAppManager";
}
#[repr(C)]
#[derive(
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
)]
pub struct DeploymentPreviewContract(pub u8);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IClassicAppManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IClassicAppManagerStatics {
    type Vtable = IClassicAppManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3808089704,
        34860,
        20275,
        [176, 53, 13, 247, 185, 13, 103, 230],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IClassicAppManagerStatics_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appuninstallkey: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IInstalledClassicAppInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        175979939,
        26064,
        16518,
        [128, 214, 6, 16, 215, 96, 32, 125],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IInstalledClassicAppInfo_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct InstalledClassicAppInfo(::windows::runtime::IInspectable);
impl InstalledClassicAppInfo {
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn DisplayVersion(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InstalledClassicAppInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.Management.Deployment.Preview.InstalledClassicAppInfo;{0a7d3da3-65d0-4086-80d6-0610d760207d})" ) ;
}
unsafe impl ::windows::runtime::Interface for InstalledClassicAppInfo {
    type Vtable = IInstalledClassicAppInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        175979939,
        26064,
        16518,
        [128, 214, 6, 16, 215, 96, 32, 125],
    );
}
impl ::windows::runtime::RuntimeName for InstalledClassicAppInfo {
    const NAME: &'static str = "Windows.Management.Deployment.Preview.InstalledClassicAppInfo";
}
impl ::std::convert::From<InstalledClassicAppInfo> for ::windows::runtime::IUnknown {
    fn from(value: InstalledClassicAppInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InstalledClassicAppInfo> for ::windows::runtime::IUnknown {
    fn from(value: &InstalledClassicAppInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for InstalledClassicAppInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &InstalledClassicAppInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<InstalledClassicAppInfo> for ::windows::runtime::IInspectable {
    fn from(value: InstalledClassicAppInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InstalledClassicAppInfo> for ::windows::runtime::IInspectable {
    fn from(value: &InstalledClassicAppInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for InstalledClassicAppInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a InstalledClassicAppInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InstalledClassicAppInfo {}
unsafe impl ::std::marker::Sync for InstalledClassicAppInfo {}
