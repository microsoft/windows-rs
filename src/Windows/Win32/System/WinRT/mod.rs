#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ACTIVATIONTYPE(pub i32);
pub const ACTIVATIONTYPE_UNCATEGORIZED: ACTIVATIONTYPE = ACTIVATIONTYPE(0i32);
pub const ACTIVATIONTYPE_FROM_MONIKER: ACTIVATIONTYPE = ACTIVATIONTYPE(1i32);
pub const ACTIVATIONTYPE_FROM_DATA: ACTIVATIONTYPE = ACTIVATIONTYPE(2i32);
pub const ACTIVATIONTYPE_FROM_STORAGE: ACTIVATIONTYPE = ACTIVATIONTYPE(4i32);
pub const ACTIVATIONTYPE_FROM_STREAM: ACTIVATIONTYPE = ACTIVATIONTYPE(8i32);
pub const ACTIVATIONTYPE_FROM_FILE: ACTIVATIONTYPE = ACTIVATIONTYPE(16i32);
impl ::std::convert::From<i32> for ACTIVATIONTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ACTIVATIONTYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(pub isize);
impl ::std::default::Default for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {}
unsafe impl ::windows::runtime::Abi for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct AgileReferenceOptions(pub i32);
pub const AGILEREFERENCE_DEFAULT: AgileReferenceOptions = AgileReferenceOptions(0i32);
pub const AGILEREFERENCE_DELAYEDMARSHAL: AgileReferenceOptions = AgileReferenceOptions(1i32);
impl ::std::convert::From<i32> for AgileReferenceOptions {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for AgileReferenceOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct BSOS_OPTIONS(pub i32);
pub const BSOS_DEFAULT: BSOS_OPTIONS = BSOS_OPTIONS(0i32);
pub const BSOS_PREFERDESTINATIONSTREAM: BSOS_OPTIONS = BSOS_OPTIONS(1i32);
impl ::std::convert::From<i32> for BSOS_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for BSOS_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CASTING_CONNECTION_ERROR_STATUS(pub i32);
pub const CASTING_CONNECTION_ERROR_STATUS_SUCCEEDED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(0i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_DID_NOT_RESPOND: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(1i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_ERROR: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(2i32);
pub const CASTING_CONNECTION_ERROR_STATUS_DEVICE_LOCKED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(3i32);
pub const CASTING_CONNECTION_ERROR_STATUS_PROTECTED_PLAYBACK_FAILED: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(4i32);
pub const CASTING_CONNECTION_ERROR_STATUS_INVALID_CASTING_SOURCE: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(5i32);
pub const CASTING_CONNECTION_ERROR_STATUS_UNKNOWN: CASTING_CONNECTION_ERROR_STATUS = CASTING_CONNECTION_ERROR_STATUS(6i32);
impl ::std::convert::From<i32> for CASTING_CONNECTION_ERROR_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CASTING_CONNECTION_ERROR_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CASTING_CONNECTION_STATE(pub i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(0i32);
pub const CASTING_CONNECTION_STATE_CONNECTED: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(1i32);
pub const CASTING_CONNECTION_STATE_RENDERING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(2i32);
pub const CASTING_CONNECTION_STATE_DISCONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(3i32);
pub const CASTING_CONNECTION_STATE_CONNECTING: CASTING_CONNECTION_STATE = CASTING_CONNECTION_STATE(4i32);
impl ::std::convert::From<i32> for CASTING_CONNECTION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CASTING_CONNECTION_STATE {
    type Abi = Self;
}
pub const CLSID_AudioFrameNativeFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(379626425, 40805, 16642, [147, 103, 44, 218, 58, 79, 55, 42]);
pub const CLSID_SoftwareBitmapNativeFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2229687953, 34306, 19076, [190, 70, 112, 139, 233, 205, 75, 116]);
pub const CLSID_VideoFrameNativeFactory: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3516151914, 1251, 18452, [129, 0, 178, 176, 174, 109, 120, 199]);
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const CastingSourceInfo_Property_CastingTypes: &'static str = "CastingTypes";
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const CastingSourceInfo_Property_PreferredSourceUriScheme: &'static str = "PreferredSourceUriScheme";
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const CastingSourceInfo_Property_ProtectedMedia: &'static str = "ProtectedMedia";
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64) -> ::windows::runtime::Result<ServerInformation> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CoDecodeProxy(dwclientpid: u32, ui64proxyaddress: u64, pserverinformation: *mut ServerInformation) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <ServerInformation as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CoDecodeProxy(::std::mem::transmute(dwclientpid), ::std::mem::transmute(ui64proxyaddress), &mut result__).from_abi::<ServerInformation>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Dxgi`*"]
#[inline]
pub unsafe fn CreateDirect3D11DeviceFromDXGIDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGIDevice>>(dxgidevice: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11DeviceFromDXGIDevice(dxgidevice: ::windows::runtime::RawPtr, graphicsdevice: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateDirect3D11DeviceFromDXGIDevice(dxgidevice.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Dxgi`*"]
#[inline]
pub unsafe fn CreateDirect3D11SurfaceFromDXGISurface<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGISurface>>(dgxisurface: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDirect3D11SurfaceFromDXGISurface(dgxisurface: ::windows::runtime::RawPtr, graphicssurface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateDirect3D11SurfaceFromDXGISurface(dgxisurface.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "System")]
#[doc = "*Required features: `Win32_System_WinRT`, `System`*"]
#[inline]
pub unsafe fn CreateDispatcherQueueController<'a, Param0: ::windows::runtime::IntoParam<'a, DispatcherQueueOptions>>(options: Param0) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueueController> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateDispatcherQueueController(options: DispatcherQueueOptions, dispatcherqueuecontroller: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::super::System::DispatcherQueueController as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        CreateDispatcherQueueController(options.into_param().abi(), &mut result__).from_abi::<super::super::super::System::DispatcherQueueController>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CreateProcessMethod(pub i32);
pub const CpCreateProcess: CreateProcessMethod = CreateProcessMethod(0i32);
pub const CpCreateProcessAsUser: CreateProcessMethod = CreateProcessMethod(1i32);
pub const CpAicLaunchAdminProcess: CreateProcessMethod = CreateProcessMethod(2i32);
impl ::std::convert::From<i32> for CreateProcessMethod {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CreateProcessMethod {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn CreateRandomAccessStreamOnFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, T: ::windows::runtime::Interface>(filepath: Param0, accessmode: u32) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRandomAccessStreamOnFile(filepath: super::super::Foundation::PWSTR, accessmode: u32, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CreateRandomAccessStreamOnFile(filepath.into_param().abi(), ::std::mem::transmute(accessmode), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com`*"]
#[inline]
pub unsafe fn CreateRandomAccessStreamOverStream<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>, T: ::windows::runtime::Interface>(stream: Param0, options: BSOS_OPTIONS) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRandomAccessStreamOverStream(stream: ::windows::runtime::RawPtr, options: BSOS_OPTIONS, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CreateRandomAccessStreamOverStream(stream.into_param().abi(), ::std::mem::transmute(options), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn CreateStreamOverRandomAccessStream<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, T: ::windows::runtime::Interface>(randomaccessstream: Param0) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateStreamOverRandomAccessStream(randomaccessstream: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        CreateStreamOverRandomAccessStream(randomaccessstream.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPATCHERQUEUE_THREAD_APARTMENTTYPE(pub i32);
pub const DQTAT_COM_NONE: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(0i32);
pub const DQTAT_COM_ASTA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(1i32);
pub const DQTAT_COM_STA: DISPATCHERQUEUE_THREAD_APARTMENTTYPE = DISPATCHERQUEUE_THREAD_APARTMENTTYPE(2i32);
impl ::std::convert::From<i32> for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPATCHERQUEUE_THREAD_APARTMENTTYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct DISPATCHERQUEUE_THREAD_TYPE(pub i32);
pub const DQTYPE_THREAD_DEDICATED: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(1i32);
pub const DQTYPE_THREAD_CURRENT: DISPATCHERQUEUE_THREAD_TYPE = DISPATCHERQUEUE_THREAD_TYPE(2i32);
impl ::std::convert::From<i32> for DISPATCHERQUEUE_THREAD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DISPATCHERQUEUE_THREAD_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub struct DispatcherQueueOptions {
    pub dwSize: u32,
    pub threadType: DISPATCHERQUEUE_THREAD_TYPE,
    pub apartmentType: DISPATCHERQUEUE_THREAD_APARTMENTTYPE,
}
impl DispatcherQueueOptions {}
impl ::std::default::Default for DispatcherQueueOptions {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DispatcherQueueOptions {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DispatcherQueueOptions").field("dwSize", &self.dwSize).field("threadType", &self.threadType).field("apartmentType", &self.apartmentType).finish()
    }
}
impl ::std::cmp::PartialEq for DispatcherQueueOptions {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.threadType == other.threadType && self.apartmentType == other.apartmentType
    }
}
impl ::std::cmp::Eq for DispatcherQueueOptions {}
unsafe impl ::windows::runtime::Abi for DispatcherQueueOptions {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub struct EventRegistrationToken {
    pub value: i64,
}
impl EventRegistrationToken {}
impl ::std::default::Default for EventRegistrationToken {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for EventRegistrationToken {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("EventRegistrationToken").field("value", &self.value).finish()
    }
}
impl ::std::cmp::PartialEq for EventRegistrationToken {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl ::std::cmp::Eq for EventRegistrationToken {}
unsafe impl ::windows::runtime::Abi for EventRegistrationToken {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GRAPHICS_EFFECT_PROPERTY_MAPPING(pub i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_UNKNOWN: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(0i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_DIRECT: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(1i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORX: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(2i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORY: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(3i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORZ: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(4i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_VECTORW: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(5i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RECT_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(6i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_RADIANS_TO_DEGREES: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(7i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLORMATRIX_ALPHA_MODE: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(8i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR3: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(9i32);
pub const GRAPHICS_EFFECT_PROPERTY_MAPPING_COLOR_TO_VECTOR4: GRAPHICS_EFFECT_PROPERTY_MAPPING = GRAPHICS_EFFECT_PROPERTY_MAPPING(10i32);
impl ::std::convert::From<i32> for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GRAPHICS_EFFECT_PROPERTY_MAPPING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn GetRestrictedErrorInfo() -> ::windows::runtime::Result<IRestrictedErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetRestrictedErrorInfo(pprestrictederrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IRestrictedErrorInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        GetRestrictedErrorInfo(&mut result__).from_abi::<IRestrictedErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HANDLE_ACCESS_OPTIONS(pub u32);
pub const HAO_NONE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(0u32);
pub const HAO_READ_ATTRIBUTES: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(128u32);
pub const HAO_READ: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(1179785u32);
pub const HAO_WRITE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(1179926u32);
pub const HAO_DELETE: HANDLE_ACCESS_OPTIONS = HANDLE_ACCESS_OPTIONS(65536u32);
impl ::std::convert::From<u32> for HANDLE_ACCESS_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HANDLE_ACCESS_OPTIONS {
    type Abi = Self;
}
impl ::std::ops::BitOr for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HANDLE_ACCESS_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HANDLE_ACCESS_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HANDLE_ACCESS_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HANDLE_CREATION_OPTIONS(pub i32);
pub const HCO_CREATE_NEW: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(1i32);
pub const HCO_CREATE_ALWAYS: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(2i32);
pub const HCO_OPEN_EXISTING: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(3i32);
pub const HCO_OPEN_ALWAYS: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(4i32);
pub const HCO_TRUNCATE_EXISTING: HANDLE_CREATION_OPTIONS = HANDLE_CREATION_OPTIONS(5i32);
impl ::std::convert::From<i32> for HANDLE_CREATION_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HANDLE_CREATION_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HANDLE_OPTIONS(pub u32);
pub const HO_NONE: HANDLE_OPTIONS = HANDLE_OPTIONS(0u32);
pub const HO_OPEN_REQUIRING_OPLOCK: HANDLE_OPTIONS = HANDLE_OPTIONS(262144u32);
pub const HO_DELETE_ON_CLOSE: HANDLE_OPTIONS = HANDLE_OPTIONS(67108864u32);
pub const HO_SEQUENTIAL_SCAN: HANDLE_OPTIONS = HANDLE_OPTIONS(134217728u32);
pub const HO_RANDOM_ACCESS: HANDLE_OPTIONS = HANDLE_OPTIONS(268435456u32);
pub const HO_NO_BUFFERING: HANDLE_OPTIONS = HANDLE_OPTIONS(536870912u32);
pub const HO_OVERLAPPED: HANDLE_OPTIONS = HANDLE_OPTIONS(1073741824u32);
pub const HO_WRITE_THROUGH: HANDLE_OPTIONS = HANDLE_OPTIONS(2147483648u32);
impl ::std::convert::From<u32> for HANDLE_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HANDLE_OPTIONS {
    type Abi = Self;
}
impl ::std::ops::BitOr for HANDLE_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HANDLE_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HANDLE_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HANDLE_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HANDLE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HANDLE_SHARING_OPTIONS(pub u32);
pub const HSO_SHARE_NONE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(0u32);
pub const HSO_SHARE_READ: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(1u32);
pub const HSO_SHARE_WRITE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(2u32);
pub const HSO_SHARE_DELETE: HANDLE_SHARING_OPTIONS = HANDLE_SHARING_OPTIONS(4u32);
impl ::std::convert::From<u32> for HANDLE_SHARING_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HANDLE_SHARING_OPTIONS {
    type Abi = Self;
}
impl ::std::ops::BitOr for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HANDLE_SHARING_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HANDLE_SHARING_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HANDLE_SHARING_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HSTRING_BUFFER(pub isize);
impl ::std::default::Default for HSTRING_BUFFER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HSTRING_BUFFER {}
unsafe impl ::windows::runtime::Abi for HSTRING_BUFFER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
pub struct HSTRING_HEADER {
    pub Reserved: HSTRING_HEADER_0,
}
#[cfg(feature = "Win32_Foundation")]
impl HSTRING_HEADER {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSTRING_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSTRING_HEADER {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSTRING_HEADER {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSTRING_HEADER {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
pub union HSTRING_HEADER_0 {
    pub Reserved1: *mut ::std::ffi::c_void,
    pub Reserved2: [super::super::Foundation::CHAR; 24],
}
#[cfg(feature = "Win32_Foundation")]
impl HSTRING_HEADER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HSTRING_HEADER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HSTRING_HEADER_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HSTRING_HEADER_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HSTRING_HEADER_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn HSTRING_UserFree(param0: *const u32, param1: *const ::windows::runtime::HSTRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserFree(param0: *const u32, param1: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>);
        }
        ::std::mem::transmute(HSTRING_UserFree(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn HSTRING_UserFree64(param0: *const u32, param1: *const ::windows::runtime::HSTRING) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserFree64(param0: *const u32, param1: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>);
        }
        ::std::mem::transmute(HSTRING_UserFree64(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::windows::runtime::HSTRING) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserMarshal(param0: *const u32, param1: *mut u8, param2: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> *mut u8;
        }
        ::std::mem::transmute(HSTRING_UserMarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::windows::runtime::HSTRING) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserMarshal64(param0: *const u32, param1: *mut u8, param2: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> *mut u8;
        }
        ::std::mem::transmute(HSTRING_UserMarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::windows::runtime::HSTRING) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserSize(param0: *const u32, param1: u32, param2: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> u32;
        }
        ::std::mem::transmute(HSTRING_UserSize(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::windows::runtime::HSTRING) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserSize64(param0: *const u32, param1: u32, param2: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> u32;
        }
        ::std::mem::transmute(HSTRING_UserSize64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::windows::runtime::HSTRING) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserUnmarshal(param0: *const u32, param1: *const u8, param2: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> *mut u8;
        }
        ::std::mem::transmute(HSTRING_UserUnmarshal(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::windows::runtime::HSTRING) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HSTRING_UserUnmarshal64(param0: *const u32, param1: *const u8, param2: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> *mut u8;
        }
        ::std::mem::transmute(HSTRING_UserUnmarshal64(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IAccountsSettingsPaneInterop(::windows::runtime::IUnknown);
impl IAccountsSettingsPaneInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn ShowManageAccountsForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn ShowAddAccountForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAccountsSettingsPaneInterop {
    type Vtable = IAccountsSettingsPaneInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3555594925, 14437, 17250, [151, 70, 183, 90, 104, 45, 240, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccountsSettingsPaneInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, accountssettingspane: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, asyncaction: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, asyncaction: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IActivationFactory(::windows::runtime::IUnknown);
impl IActivationFactory {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ActivateInstance(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IActivationFactory {
    type Vtable = IActivationFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(53, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivationFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, instance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IAgileReference(::windows::runtime::IUnknown);
impl IAgileReference {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Resolve(&self, riid: *const ::windows::runtime::GUID, ppvobjectreference: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobjectreference)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAgileReference {
    type Vtable = IAgileReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3225381443, 26020, 38936, [152, 126, 224, 184, 16, 210, 166, 242]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAgileReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobjectreference: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IApartmentShutdown(::windows::runtime::IUnknown);
impl IApartmentShutdown {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn OnUninitialize(&self, ui64apartmentidentifier: u64) {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(ui64apartmentidentifier)))
    }
}
unsafe impl ::windows::runtime::Interface for IApartmentShutdown {
    type Vtable = IApartmentShutdown_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2733660681, 10146, 17077, [188, 14, 172, 22, 62, 244, 157, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IApartmentShutdown_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ui64apartmentidentifier: u64),
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IAppServiceConnectionExtendedExecution(::windows::runtime::IUnknown);
impl IAppServiceConnectionExtendedExecution {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn OpenForExtendedExecutionAsync<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAppServiceConnectionExtendedExecution {
    type Vtable = IAppServiceConnectionExtendedExecution_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1696699780, 63947, 19171, [129, 249, 162, 138, 108, 164, 80, 217]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppServiceConnectionExtendedExecution_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, operation: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IAudioFrameNative(::windows::runtime::IUnknown);
impl IAudioFrameNative {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetData<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioFrameNative {
    type Vtable = IAudioFrameNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(549330478, 37647, 18246, [147, 53, 60, 51, 47, 37, 80, 147]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IAudioFrameNativeFactory(::windows::runtime::IUnknown);
impl IAudioFrameNativeFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn CreateFromMFSample<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Media::MediaFoundation::IMFSample>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, T: ::windows::runtime::Interface>(&self, data: Param0, forcereadonly: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IAudioFrameNativeFactory {
    type Vtable = IAudioFrameNativeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2077654264, 49021, 17382, [175, 141, 177, 112, 238, 12, 1, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAudioFrameNativeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, forcereadonly: super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IBufferByteAccess(::windows::runtime::IUnknown);
impl IBufferByteAccess {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Buffer(&self) -> ::windows::runtime::Result<*mut u8> {
        let mut result__: <*mut u8 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut u8>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IBufferByteAccess {
    type Vtable = IBufferByteAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2421821423, 48211, 4575, [140, 73, 0, 30, 79, 198, 134, 218]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICastingController(::windows::runtime::IUnknown);
impl ICastingController {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, castingengine: Param0, castingsource: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), castingengine.into_param().abi(), castingsource.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Connect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Disconnect(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Advise<'a, Param0: ::windows::runtime::IntoParam<'a, ICastingEventHandler>>(&self, eventhandler: Param0) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), eventhandler.into_param().abi(), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn UnAdvise(&self, cookie: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(cookie)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICastingController {
    type Vtable = ICastingController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4037370915, 42596, 20413, [139, 67, 64, 154, 69, 232, 217, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingController_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, castingengine: ::windows::runtime::RawPtr, castingsource: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventhandler: ::windows::runtime::RawPtr, cookie: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICastingEventHandler(::windows::runtime::IUnknown);
impl ICastingEventHandler {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn OnStateChanged(&self, newstate: CASTING_CONNECTION_STATE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(newstate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn OnError<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(errorstatus), errormessage.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICastingEventHandler {
    type Vtable = ICastingEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3348786359, 48829, 18342, [162, 173, 77, 69, 173, 121, 199, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingEventHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newstate: CASTING_CONNECTION_STATE) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errorstatus: CASTING_CONNECTION_ERROR_STATUS, errormessage: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICastingSourceInfo(::windows::runtime::IUnknown);
impl ICastingSourceInfo {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetController(&self) -> ::windows::runtime::Result<ICastingController> {
        let mut result__: <ICastingController as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ICastingController>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetProperties(&self) -> ::windows::runtime::Result<super::PropertiesSystem::INamedPropertyStore> {
        let mut result__: <super::PropertiesSystem::INamedPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::PropertiesSystem::INamedPropertyStore>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICastingSourceInfo {
    type Vtable = ICastingSourceInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1158683319, 31802, 19406, [149, 0, 18, 192, 144, 36, 178, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICastingSourceInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, controller: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, props: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICompositionCapabilitiesInteropFactory(::windows::runtime::IUnknown);
impl ICompositionCapabilitiesInteropFactory {
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `UI_Composition`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, hwnd: Param0) -> ::windows::runtime::Result<super::super::super::UI::Composition::CompositionCapabilities> {
        let mut result__: <super::super::super::UI::Composition::CompositionCapabilities as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), &mut result__).from_abi::<super::super::super::UI::Composition::CompositionCapabilities>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionCapabilitiesInteropFactory {
    type Vtable = ICompositionCapabilitiesInteropFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(748532566, 59149, 17986, [130, 152, 188, 74, 165, 180, 134, 92]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionCapabilitiesInteropFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICompositionDrawingSurfaceInterop(::windows::runtime::IUnknown);
impl ICompositionDrawingSurfaceInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::std::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(updaterect), ::std::mem::transmute(iid), ::std::mem::transmute(updateobject), ::std::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Resize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(&self, sizepixels: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), sizepixels.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(scrollrect), ::std::mem::transmute(cliprect), ::std::mem::transmute(offsetx), ::std::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionDrawingSurfaceInterop {
    type Vtable = ICompositionDrawingSurfaceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4244956899, 65036, 19516, [171, 25, 160, 118, 1, 165, 118, 238]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::std::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sizepixels: super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICompositionDrawingSurfaceInterop2(::windows::runtime::IUnknown);
impl ICompositionDrawingSurfaceInterop2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::std::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(updaterect), ::std::mem::transmute(iid), ::std::mem::transmute(updateobject), ::std::mem::transmute(updateoffset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Resize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::SIZE>>(&self, sizepixels: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), sizepixels.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Scroll(&self, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(scrollrect), ::std::mem::transmute(cliprect), ::std::mem::transmute(offsetx), ::std::mem::transmute(offsety)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn CopySurface<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, destinationresource: Param0, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::Foundation::RECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), destinationresource.into_param().abi(), ::std::mem::transmute(destinationoffsetx), ::std::mem::transmute(destinationoffsety), ::std::mem::transmute(sourcerectangle)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionDrawingSurfaceInterop2 {
    type Vtable = ICompositionDrawingSurfaceInterop2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1105611438, 39104, 16953, [142, 149, 163, 48, 221, 106, 161, 139]);
}
impl ::std::convert::From<ICompositionDrawingSurfaceInterop2> for ICompositionDrawingSurfaceInterop {
    fn from(value: ICompositionDrawingSurfaceInterop2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICompositionDrawingSurfaceInterop2> for ICompositionDrawingSurfaceInterop {
    fn from(value: &ICompositionDrawingSurfaceInterop2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionDrawingSurfaceInterop> for ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionDrawingSurfaceInterop> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICompositionDrawingSurfaceInterop>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ICompositionDrawingSurfaceInterop> for &ICompositionDrawingSurfaceInterop2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ICompositionDrawingSurfaceInterop> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ICompositionDrawingSurfaceInterop>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionDrawingSurfaceInterop2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::std::ffi::c_void, updateoffset: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, sizepixels: super::super::Foundation::SIZE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, scrollrect: *const super::super::Foundation::RECT, cliprect: *const super::super::Foundation::RECT, offsetx: i32, offsety: i32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, destinationresource: ::windows::runtime::RawPtr, destinationoffsetx: i32, destinationoffsety: i32, sourcerectangle: *const super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICompositionGraphicsDeviceInterop(::windows::runtime::IUnknown);
impl ICompositionGraphicsDeviceInterop {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetRenderingDevice(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetRenderingDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICompositionGraphicsDeviceInterop {
    type Vtable = ICompositionGraphicsDeviceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2702638961, 63679, 19594, [156, 152, 112, 119, 154, 50, 169, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositionGraphicsDeviceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICompositorDesktopInterop(::windows::runtime::IUnknown);
impl ICompositorDesktopInterop {
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `UI_Composition_Desktop`, `Win32_Foundation`*"]
    pub unsafe fn CreateDesktopWindowTarget<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, hwndtarget: Param0, istopmost: Param1) -> ::windows::runtime::Result<super::super::super::UI::Composition::Desktop::DesktopWindowTarget> {
        let mut result__: <super::super::super::UI::Composition::Desktop::DesktopWindowTarget as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), hwndtarget.into_param().abi(), istopmost.into_param().abi(), &mut result__).from_abi::<super::super::super::UI::Composition::Desktop::DesktopWindowTarget>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn EnsureOnThread(&self, threadid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(threadid)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICompositorDesktopInterop {
    type Vtable = ICompositorDesktopInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(702976506, 17767, 19914, [179, 25, 208, 242, 7, 235, 104, 7]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorDesktopInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndtarget: super::super::Foundation::HWND, istopmost: super::super::Foundation::BOOL, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Composition_Desktop", feature = "Win32_Foundation")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, threadid: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICompositorInterop(::windows::runtime::IUnknown);
impl ICompositorInterop {
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `UI_Composition`, `Win32_Foundation`*"]
    pub unsafe fn CreateCompositionSurfaceForHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, swapchain: Param0) -> ::windows::runtime::Result<super::super::super::UI::Composition::ICompositionSurface> {
        let mut result__: <super::super::super::UI::Composition::ICompositionSurface as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), swapchain.into_param().abi(), &mut result__).from_abi::<super::super::super::UI::Composition::ICompositionSurface>(result__)
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `Win32_System_WinRT`, `UI_Composition`*"]
    pub unsafe fn CreateCompositionSurfaceForSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, swapchain: Param0) -> ::windows::runtime::Result<super::super::super::UI::Composition::ICompositionSurface> {
        let mut result__: <super::super::super::UI::Composition::ICompositionSurface as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), swapchain.into_param().abi(), &mut result__).from_abi::<super::super::super::UI::Composition::ICompositionSurface>(result__)
    }
    #[cfg(feature = "UI_Composition")]
    #[doc = "*Required features: `Win32_System_WinRT`, `UI_Composition`*"]
    pub unsafe fn CreateGraphicsDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, renderingdevice: Param0) -> ::windows::runtime::Result<super::super::super::UI::Composition::CompositionGraphicsDevice> {
        let mut result__: <super::super::super::UI::Composition::CompositionGraphicsDevice as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), renderingdevice.into_param().abi(), &mut result__).from_abi::<super::super::super::UI::Composition::CompositionGraphicsDevice>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICompositorInterop {
    type Vtable = ICompositorInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(623476060, 15060, 19612, [181, 207, 227, 106, 56, 81, 35, 48]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICompositorInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "UI_Composition", feature = "Win32_Foundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: super::super::Foundation::HANDLE, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "UI_Composition", feature = "Win32_Foundation")))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
    #[cfg(feature = "UI_Composition")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, renderingdevice: ::windows::runtime::RawPtr, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Composition"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICoreFrameworkInputViewInterop(::windows::runtime::IUnknown);
impl ICoreFrameworkInputViewInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICoreFrameworkInputViewInterop {
    type Vtable = ICoreFrameworkInputViewInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(238920514, 45340, 18507, [156, 28, 190, 13, 97, 194, 246, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, coreframeworkinputview: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICorrelationVectorInformation(::windows::runtime::IUnknown);
impl ICorrelationVectorInformation {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn LastCorrelationVectorForThread(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn NextCorrelationVectorForThread(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetNextCorrelationVectorForThread<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, cv: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), cv.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICorrelationVectorInformation {
    type Vtable = ICorrelationVectorInformation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2210892604, 55435, 18768, [170, 110, 34, 184, 210, 42, 171, 211]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorInformation_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cv: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cv: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cv: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ICorrelationVectorSource(::windows::runtime::IUnknown);
impl ICorrelationVectorSource {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn CorrelationVector(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICorrelationVectorSource {
    type Vtable = ICorrelationVectorSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(355174971, 47545, 18053, [181, 110, 151, 72, 71, 188, 117, 69]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICorrelationVectorSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cv: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDDEInitializer(::windows::runtime::IUnknown);
impl IDDEInitializer {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    pub unsafe fn Initialize<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param3: ::windows::runtime::IntoParam<'a, super::super::UI::Shell::IShellItem>,
        Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>,
        Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        fileextensionorprotocol: Param0,
        method: CreateProcessMethod,
        currentdirectory: Param2,
        exectarget: Param3,
        site: Param4,
        application: Param5,
        targetfile: Param6,
        arguments: Param7,
        verb: Param8,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            fileextensionorprotocol.into_param().abi(),
            ::std::mem::transmute(method),
            currentdirectory.into_param().abi(),
            exectarget.into_param().abi(),
            site.into_param().abi(),
            application.into_param().abi(),
            targetfile.into_param().abi(),
            arguments.into_param().abi(),
            verb.into_param().abi(),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDDEInitializer {
    type Vtable = IDDEInitializer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(819761951, 13308, 20477, [161, 104, 148, 34, 88, 207, 60, 164]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDDEInitializer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fileextensionorprotocol: super::super::Foundation::PWSTR, method: CreateProcessMethod, currentdirectory: super::super::Foundation::PWSTR, exectarget: ::windows::runtime::RawPtr, site: ::windows::runtime::RawPtr, application: super::super::Foundation::PWSTR, targetfile: super::super::Foundation::PWSTR, arguments: super::super::Foundation::PWSTR, verb: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDesktopWindowTargetInterop(::windows::runtime::IUnknown);
impl IDesktopWindowTargetInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Hwnd(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDesktopWindowTargetInterop {
    type Vtable = IDesktopWindowTargetInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(903607710, 58361, 17840, [129, 231, 254, 117, 244, 20, 93, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowTargetInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDesktopWindowXamlSourceNative(::windows::runtime::IUnknown);
impl IDesktopWindowXamlSourceNative {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn AttachToWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, parentwnd: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), parentwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn WindowHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDesktopWindowXamlSourceNative {
    type Vtable = IDesktopWindowXamlSourceNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1019015615, 12150, 20124, [150, 171, 232, 75, 55, 151, 37, 84]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDesktopWindowXamlSourceNative2(::windows::runtime::IUnknown);
impl IDesktopWindowXamlSourceNative2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn AttachToWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, parentwnd: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), parentwnd.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn WindowHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn PreTranslateMessage(&self, message: *const super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(message), ::std::mem::transmute(result)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDesktopWindowXamlSourceNative2 {
    type Vtable = IDesktopWindowXamlSourceNative2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3822901447, 12375, 18066, [153, 195, 123, 119, 32, 175, 218, 49]);
}
impl ::std::convert::From<IDesktopWindowXamlSourceNative2> for IDesktopWindowXamlSourceNative {
    fn from(value: IDesktopWindowXamlSourceNative2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IDesktopWindowXamlSourceNative2> for IDesktopWindowXamlSourceNative {
    fn from(value: &IDesktopWindowXamlSourceNative2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDesktopWindowXamlSourceNative> for IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDesktopWindowXamlSourceNative> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDesktopWindowXamlSourceNative>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IDesktopWindowXamlSourceNative> for &IDesktopWindowXamlSourceNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IDesktopWindowXamlSourceNative> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IDesktopWindowXamlSourceNative>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDesktopWindowXamlSourceNative2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, parentwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, message: *const super::super::UI::WindowsAndMessaging::MSG, result: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDirect3DDxgiInterfaceAccess(::windows::runtime::IUnknown);
impl IDirect3DDxgiInterfaceAccess {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetInterface<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDirect3DDxgiInterfaceAccess {
    type Vtable = IDirect3DDxgiInterfaceAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2847133714, 15858, 20195, [184, 209, 134, 149, 244, 87, 211, 193]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDxgiInterfaceAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, p: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDisplayDeviceInterop(::windows::runtime::IUnknown);
impl IDisplayDeviceInterop {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Security`*"]
    pub unsafe fn CreateSharedHandle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param3: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, pobject: Param0, psecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: Param3) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pobject.into_param().abi(), ::std::mem::transmute(psecurityattributes), ::std::mem::transmute(access), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn OpenSharedHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, nthandle: Param0, riid: Param1, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), nthandle.into_param().abi(), riid.into_param().abi(), ::std::mem::transmute(ppvobj)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDisplayDeviceInterop {
    type Vtable = IDisplayDeviceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1681097560, 13930, 18203, [189, 86, 221, 142, 244, 142, 67, 155]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayDeviceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobject: ::windows::runtime::RawPtr, psecurityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, access: u32, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, phandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Security")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nthandle: super::super::Foundation::HANDLE, riid: ::windows::runtime::GUID, ppvobj: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDisplayPathInterop(::windows::runtime::IUnknown);
impl IDisplayPathInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn CreateSourcePresentationHandle(&self) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetSourceId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDisplayPathInterop {
    type Vtable = IDisplayPathInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2797224453, 58782, 20081, [178, 91, 78, 67, 109, 33, 238, 61]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayPathInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pvalue: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psourceid: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IDragDropManagerInterop(::windows::runtime::IUnknown);
impl IDragDropManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, hwnd: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IDragDropManagerInterop {
    type Vtable = IDragDropManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1524157351, 19457, 19884, [144, 116, 130, 120, 148, 41, 45, 99]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragDropManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IFindReferenceTargetsCallback(::windows::runtime::IUnknown);
impl IFindReferenceTargetsCallback {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn FoundTrackerTarget<'a, Param0: ::windows::runtime::IntoParam<'a, IReferenceTrackerTarget>>(&self, target: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), target.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFindReferenceTargetsCallback {
    type Vtable = IFindReferenceTargetsCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(78858348, 18055, 16937, [141, 20, 80, 90, 181, 132, 221, 136]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFindReferenceTargetsCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, target: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IGeometrySource2DInterop(::windows::runtime::IUnknown);
impl IGeometrySource2DInterop {
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct2D`*"]
    pub unsafe fn GetGeometry(&self) -> ::windows::runtime::Result<super::super::Graphics::Direct2D::ID2D1Geometry> {
        let mut result__: <super::super::Graphics::Direct2D::ID2D1Geometry as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Direct2D::ID2D1Geometry>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct2D")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct2D`*"]
    pub unsafe fn TryGetGeometryUsingFactory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct2D::ID2D1Factory>>(&self, factory: Param0) -> ::windows::runtime::Result<super::super::Graphics::Direct2D::ID2D1Geometry> {
        let mut result__: <super::super::Graphics::Direct2D::ID2D1Geometry as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), factory.into_param().abi(), &mut result__).from_abi::<super::super::Graphics::Direct2D::ID2D1Geometry>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGeometrySource2DInterop {
    type Vtable = IGeometrySource2DInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(106409843, 21501, 18383, [132, 255, 200, 73, 45, 42, 128, 163]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2DInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct2D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct2D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, factory: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct2D"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IGraphicsCaptureItemInterop(::windows::runtime::IUnknown);
impl IGraphicsCaptureItemInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn CreateForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, window: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Graphics_Gdi")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Gdi`*"]
    pub unsafe fn CreateForMonitor<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Gdi::HMONITOR>, T: ::windows::runtime::Interface>(&self, monitor: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), monitor.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGraphicsCaptureItemInterop {
    type Vtable = IGraphicsCaptureItemInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(908650523, 15532, 19552, [183, 244, 35, 206, 14, 12, 51, 86]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsCaptureItemInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, result: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Graphics_Gdi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, monitor: super::super::Graphics::Gdi::HMONITOR, riid: *const ::windows::runtime::GUID, result: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Gdi"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IGraphicsEffectD2D1Interop(::windows::runtime::IUnknown);
impl IGraphicsEffectD2D1Interop {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetEffectId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetNamedPropertyMapping<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(index), ::std::mem::transmute(mapping)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetPropertyCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Foundation`*"]
    pub unsafe fn GetProperty(&self, index: u32) -> ::windows::runtime::Result<super::super::super::Foundation::IPropertyValue> {
        let mut result__: <super::super::super::Foundation::IPropertyValue as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<super::super::super::Foundation::IPropertyValue>(result__)
    }
    #[cfg(feature = "Graphics_Effects")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Graphics_Effects`*"]
    pub unsafe fn GetSource(&self, index: u32) -> ::windows::runtime::Result<super::super::super::Graphics::Effects::IGraphicsEffectSource> {
        let mut result__: <super::super::super::Graphics::Effects::IGraphicsEffectSource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(index), &mut result__).from_abi::<super::super::super::Graphics::Effects::IGraphicsEffectSource>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetSourceCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IGraphicsEffectD2D1Interop {
    type Vtable = IGraphicsEffectD2D1Interop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(801469316, 41064, 17623, [163, 49, 48, 152, 47, 207, 113, 119]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectD2D1Interop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, id: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, index: *mut u32, mapping: *mut GRAPHICS_EFFECT_PROPERTY_MAPPING) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Graphics_Effects")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, index: u32, source: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Graphics_Effects"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IHolographicCameraInterop(::windows::runtime::IUnknown);
impl IHolographicCameraInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn CreateDirect3D12BackBufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Device>>(&self, pdevice: Param0, ptexture2ddesc: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::runtime::Result<super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdevice.into_param().abi(), ::std::mem::transmute(ptexture2ddesc), &mut result__).from_abi::<super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn CreateDirect3D12HardwareProtectedBackBufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Device>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>(&self, pdevice: Param0, ptexture2ddesc: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: Param2) -> ::windows::runtime::Result<super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pdevice.into_param().abi(), ::std::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into_param().abi(), &mut result__).from_abi::<super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1, duration: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), ::std::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, presourcetounacquire: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), presourcetounacquire.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHolographicCameraInterop {
    type Vtable = IHolographicCameraInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2093087173, 27906, 16890, [149, 0, 225, 128, 158, 180, 142, 236]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, ptexture2ddesc: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, ppcreatedtexture2dresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, ptexture2ddesc: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::runtime::RawPtr, ppcreatedtexture2dresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi")))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetoacquire: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetoacquire: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr, duration: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetounacquire: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IHolographicCameraRenderingParametersInterop(::windows::runtime::IUnknown);
impl IHolographicCameraRenderingParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12Resource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::std::mem::transmute(colorresourcefencesignalvalue)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12ResourceWithDepthData<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Fence>, Param3: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>, Param4: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Fence>>(
        &self,
        pcolorresourcetocommit: Param0,
        pcolorresourcefence: Param1,
        colorresourcefencesignalvalue: u64,
        pdepthresourcetocommit: Param3,
        pdepthresourcefence: Param4,
        depthresourcefencesignalvalue: u64,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            pcolorresourcetocommit.into_param().abi(),
            pcolorresourcefence.into_param().abi(),
            ::std::mem::transmute(colorresourcefencesignalvalue),
            pdepthresourcetocommit.into_param().abi(),
            pdepthresourcefence.into_param().abi(),
            ::std::mem::transmute(depthresourcefencesignalvalue),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHolographicCameraRenderingParametersInterop {
    type Vtable = IHolographicCameraRenderingParametersInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4149962966, 53757, 18183, [170, 253, 250, 111, 76, 14, 59, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicCameraRenderingParametersInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcolorresourcetocommit: ::windows::runtime::RawPtr, pcolorresourcefence: ::windows::runtime::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcolorresourcetocommit: ::windows::runtime::RawPtr, pcolorresourcefence: ::windows::runtime::RawPtr, colorresourcefencesignalvalue: u64, pdepthresourcetocommit: ::windows::runtime::RawPtr, pdepthresourcefence: ::windows::runtime::RawPtr, depthresourcefencesignalvalue: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IHolographicQuadLayerInterop(::windows::runtime::IUnknown);
impl IHolographicQuadLayerInterop {
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn CreateDirect3D12ContentBufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Device>>(&self, pdevice: Param0, ptexture2ddesc: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC) -> ::windows::runtime::Result<super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdevice.into_param().abi(), ::std::mem::transmute(ptexture2ddesc), &mut result__).from_abi::<super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn CreateDirect3D12HardwareProtectedContentBufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Device>, Param2: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12ProtectedResourceSession>>(
        &self,
        pdevice: Param0,
        ptexture2ddesc: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC,
        pprotectedresourcesession: Param2,
    ) -> ::windows::runtime::Result<super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pdevice.into_param().abi(), ::std::mem::transmute(ptexture2ddesc), pprotectedresourcesession.into_param().abi(), &mut result__).from_abi::<super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn AcquireDirect3D12BufferResourceWithTimeout<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, presourcetoacquire: Param0, pcommandqueue: Param1, duration: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), presourcetoacquire.into_param().abi(), pcommandqueue.into_param().abi(), ::std::mem::transmute(duration)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn UnacquireDirect3D12BufferResource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, presourcetounacquire: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), presourcetounacquire.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHolographicQuadLayerInterop {
    type Vtable = IHolographicQuadLayerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3483797744, 25502, 19015, [131, 215, 107, 127, 94, 191, 127, 237]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, ptexture2ddesc: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pptexture2dresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi")))] usize,
    #[cfg(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdevice: ::windows::runtime::RawPtr, ptexture2ddesc: *const super::super::Graphics::Direct3D12::D3D12_RESOURCE_DESC, pprotectedresourcesession: ::windows::runtime::RawPtr, ppcreatedtexture2dresource: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Graphics_Direct3D12", feature = "Win32_Graphics_Dxgi")))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetoacquire: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetoacquire: ::windows::runtime::RawPtr, pcommandqueue: ::windows::runtime::RawPtr, duration: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presourcetounacquire: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IHolographicQuadLayerUpdateParametersInterop(::windows::runtime::IUnknown);
impl IHolographicQuadLayerUpdateParametersInterop {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CommitDirect3D12Resource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Fence>>(&self, pcolorresourcetocommit: Param0, pcolorresourcefence: Param1, colorresourcefencesignalvalue: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pcolorresourcetocommit.into_param().abi(), pcolorresourcefence.into_param().abi(), ::std::mem::transmute(colorresourcefencesignalvalue)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IHolographicQuadLayerUpdateParametersInterop {
    type Vtable = IHolographicQuadLayerUpdateParametersInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3858057677, 51465, 17487, [136, 9, 124, 193, 138, 156, 137, 32]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicQuadLayerUpdateParametersInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcolorresourcetocommit: ::windows::runtime::RawPtr, pcolorresourcefence: ::windows::runtime::RawPtr, colorresourcefencesignalvalue: u64) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IHolographicSpaceInterop(::windows::runtime::IUnknown);
impl IHolographicSpaceInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn CreateForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, window: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IHolographicSpaceInterop {
    type Vtable = IHolographicSpaceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1548674358, 27288, 19334, [161, 112, 88, 112, 19, 214, 253, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHolographicSpaceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, holographicspace: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IInputPaneInterop(::windows::runtime::IUnknown);
impl IInputPaneInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IInputPaneInterop {
    type Vtable = IInputPaneInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1976511575, 37269, 18737, [131, 50, 240, 180, 9, 233, 22, 175]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInputPaneInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, inputpane: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IIsolatedEnvironmentInterop(::windows::runtime::IUnknown);
impl IIsolatedEnvironmentInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetHostHwndInterop<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, containerhwnd: Param0) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), containerhwnd.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IIsolatedEnvironmentInterop {
    type Vtable = IIsolatedEnvironmentInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2238790702, 36450, 18117, [141, 226, 198, 71, 225, 213, 70, 54]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedEnvironmentInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, containerhwnd: super::super::Foundation::HWND, hosthwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ILanguageExceptionErrorInfo(::windows::runtime::IUnknown);
impl ILanguageExceptionErrorInfo {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetLanguageException(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILanguageExceptionErrorInfo {
    type Vtable = ILanguageExceptionErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(77782003, 57219, 4460, [9, 70, 8, 18, 171, 246, 224, 125]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languageexception: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ILanguageExceptionErrorInfo2(::windows::runtime::IUnknown);
impl ILanguageExceptionErrorInfo2 {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetLanguageException(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetPreviousLanguageExceptionErrorInfo(&self) -> ::windows::runtime::Result<ILanguageExceptionErrorInfo2> {
        let mut result__: <ILanguageExceptionErrorInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ILanguageExceptionErrorInfo2>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn CapturePropagationContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, languageexception: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), languageexception.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetPropagationContextHead(&self) -> ::windows::runtime::Result<ILanguageExceptionErrorInfo2> {
        let mut result__: <ILanguageExceptionErrorInfo2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ILanguageExceptionErrorInfo2>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILanguageExceptionErrorInfo2 {
    type Vtable = ILanguageExceptionErrorInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1464264132, 23447, 16972, [182, 32, 40, 34, 145, 87, 52, 221]);
}
impl ::std::convert::From<ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
    fn from(value: ILanguageExceptionErrorInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ILanguageExceptionErrorInfo2> for ILanguageExceptionErrorInfo {
    fn from(value: &ILanguageExceptionErrorInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILanguageExceptionErrorInfo> for ILanguageExceptionErrorInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILanguageExceptionErrorInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILanguageExceptionErrorInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILanguageExceptionErrorInfo> for &ILanguageExceptionErrorInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILanguageExceptionErrorInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILanguageExceptionErrorInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionErrorInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languageexception: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, previouslanguageexceptionerrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, languageexception: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, propagatedlanguageexceptionerrorinfohead: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ILanguageExceptionStackBackTrace(::windows::runtime::IUnknown);
impl ILanguageExceptionStackBackTrace {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetStackBackTrace(&self, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(maxframestocapture), ::std::mem::transmute(stackbacktrace), ::std::mem::transmute(framescaptured)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILanguageExceptionStackBackTrace {
    type Vtable = ILanguageExceptionStackBackTrace_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3420798901, 63847, 16984, [141, 52, 66, 245, 226, 88, 51, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionStackBackTrace_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, maxframestocapture: u32, stackbacktrace: *mut usize, framescaptured: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ILanguageExceptionTransform(::windows::runtime::IUnknown);
impl ILanguageExceptionTransform {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetTransformedRestrictedErrorInfo(&self) -> ::windows::runtime::Result<IRestrictedErrorInfo> {
        let mut result__: <IRestrictedErrorInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IRestrictedErrorInfo>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILanguageExceptionTransform {
    type Vtable = ILanguageExceptionTransform_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4273316465, 42701, 17870, [136, 10, 105, 103, 6, 186, 220, 101]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILanguageExceptionTransform_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, restrictederrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ILearningModelDeviceFactoryNative(::windows::runtime::IUnknown);
impl ILearningModelDeviceFactoryNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateFromD3D12CommandQueue<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12CommandQueue>>(&self, value: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), value.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILearningModelDeviceFactoryNative {
    type Vtable = ILearningModelDeviceFactoryNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(513487265, 26158, 19168, [175, 103, 246, 59, 179, 55, 230, 52]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelDeviceFactoryNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ILearningModelOperatorProviderNative(::windows::runtime::IUnknown);
impl ILearningModelOperatorProviderNative {
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_AI_MachineLearning_WinML`*"]
    pub unsafe fn GetRegistry(&self) -> ::windows::runtime::Result<super::super::AI::MachineLearning::WinML::IMLOperatorRegistry> {
        let mut result__: <super::super::AI::MachineLearning::WinML::IMLOperatorRegistry as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::AI::MachineLearning::WinML::IMLOperatorRegistry>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ILearningModelOperatorProviderNative {
    type Vtable = ILearningModelOperatorProviderNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(450535994, 60263, 16883, [170, 216, 93, 152, 78, 155, 172, 212]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelOperatorProviderNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_AI_MachineLearning_WinML")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppoperatorregistry: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_AI_MachineLearning_WinML"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ILearningModelSessionOptionsNative(::windows::runtime::IUnknown);
impl ILearningModelSessionOptionsNative {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetIntraOpNumThreadsOverride(&self, intraopnumthreads: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(intraopnumthreads)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILearningModelSessionOptionsNative {
    type Vtable = ILearningModelSessionOptionsNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3340670271, 14260, 17764, [134, 88, 216, 57, 104, 102, 219, 13]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILearningModelSessionOptionsNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, intraopnumthreads: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IMemoryBufferByteAccess(::windows::runtime::IUnknown);
impl IMemoryBufferByteAccess {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(value), ::std::mem::transmute(capacity)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMemoryBufferByteAccess {
    type Vtable = IMemoryBufferByteAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1527591477, 19898, 19780, [134, 94, 143, 29, 14, 79, 208, 77]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMemoryBufferByteAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut u8, capacity: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IMessageDispatcher(::windows::runtime::IUnknown);
impl IMessageDispatcher {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn PumpMessages(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IMessageDispatcher {
    type Vtable = IMessageDispatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4126690447, 53200, 19670, [182, 107, 197, 210, 111, 241, 104, 157]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageDispatcher_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IOplockBreakingHandler(::windows::runtime::IUnknown);
impl IOplockBreakingHandler {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn OplockBreaking(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IOplockBreakingHandler {
    type Vtable = IOplockBreakingHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2188033597, 15053, 18387, [132, 242, 136, 170, 237, 207, 99, 4]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IOplockBreakingHandler_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPdfRendererNative(::windows::runtime::IUnknown);
impl IPdfRendererNative {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Graphics_Direct2D`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn RenderPageToSurface<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGISurface>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::POINT>>(&self, pdfpage: Param0, psurface: Param1, offset: Param2, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pdfpage.into_param().abi(), psurface.into_param().abi(), offset.into_param().abi(), ::std::mem::transmute(prenderparams)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Graphics_Direct2D`*"]
    pub unsafe fn RenderPageToDeviceContext<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct2D::ID2D1DeviceContext>>(&self, pdfpage: Param0, pd2ddevicecontext: Param1, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pdfpage.into_param().abi(), pd2ddevicecontext.into_param().abi(), ::std::mem::transmute(prenderparams)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPdfRendererNative {
    type Vtable = IPdfRendererNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2107493777, 53879, 18759, [133, 39, 7, 160, 218, 237, 169, 74]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPdfRendererNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdfpage: ::windows::runtime::RawPtr, psurface: ::windows::runtime::RawPtr, offset: super::super::Foundation::POINT, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D", feature = "Win32_Graphics_Dxgi")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdfpage: ::windows::runtime::RawPtr, pd2ddevicecontext: ::windows::runtime::RawPtr, prenderparams: *const PDF_RENDER_PARAMS) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPlayToManagerInterop(::windows::runtime::IUnknown);
impl IPlayToManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn ShowPlayToUIForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(&self, appwindow: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), appwindow.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPlayToManagerInterop {
    type Vtable = IPlayToManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(607733401, 7980, 20147, [140, 215, 14, 193, 218, 66, 165, 64]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPlayToManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, playtomanager: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrintManagerInterop(::windows::runtime::IUnknown);
impl IPrintManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintManagerInterop {
    type Vtable = IPrintManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3309525570, 36163, 20091, [166, 138, 239, 49, 30, 57, 32, 135]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, printmanager: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, asyncoperation: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrintWorkflowConfigurationNative(::windows::runtime::IUnknown);
impl IPrintWorkflowConfigurationNative {
    #[cfg(feature = "Win32_Graphics_Printing")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Printing`*"]
    pub unsafe fn PrinterQueue(&self) -> ::windows::runtime::Result<super::super::Graphics::Printing::IPrinterQueue> {
        let mut result__: <super::super::Graphics::Printing::IPrinterQueue as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Printing::IPrinterQueue>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Printing")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Printing`*"]
    pub unsafe fn DriverProperties(&self) -> ::windows::runtime::Result<super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__: <super::super::Graphics::Printing::IPrinterPropertyBag as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
    #[cfg(feature = "Win32_Graphics_Printing")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Printing`*"]
    pub unsafe fn UserProperties(&self) -> ::windows::runtime::Result<super::super::Graphics::Printing::IPrinterPropertyBag> {
        let mut result__: <super::super::Graphics::Printing::IPrinterPropertyBag as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Printing::IPrinterPropertyBag>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowConfigurationNative {
    type Vtable = IPrintWorkflowConfigurationNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3226910218, 40674, 17674, [152, 35, 150, 79, 0, 6, 242, 187]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowConfigurationNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Printing"))] usize,
    #[cfg(feature = "Win32_Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Printing"))] usize,
    #[cfg(feature = "Win32_Graphics_Printing")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Printing"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative(::windows::runtime::IUnknown);
impl IPrintWorkflowObjectModelSourceFileContentNative {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn StartXpsOMGeneration<'a, Param0: ::windows::runtime::IntoParam<'a, IPrintWorkflowXpsReceiver>>(&self, receiver: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), receiver.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Storage_Xps")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Storage_Xps`*"]
    pub unsafe fn ObjectFactory(&self) -> ::windows::runtime::Result<super::super::Storage::Xps::IXpsOMObjectFactory1> {
        let mut result__: <super::super::Storage::Xps::IXpsOMObjectFactory1 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Storage::Xps::IXpsOMObjectFactory1>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowObjectModelSourceFileContentNative {
    type Vtable = IPrintWorkflowObjectModelSourceFileContentNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1758061687, 39230, 16466, [138, 198, 69, 78, 255, 88, 219, 157]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, receiver: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Storage_Xps")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative(::windows::runtime::IUnknown);
impl IPrintWorkflowXpsObjectModelTargetPackageNative {
    #[cfg(feature = "Win32_Storage_Xps")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Storage_Xps`*"]
    pub unsafe fn DocumentPackageTarget(&self) -> ::windows::runtime::Result<super::super::Storage::Xps::IXpsDocumentPackageTarget> {
        let mut result__: <super::super::Storage::Xps::IXpsDocumentPackageTarget as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Storage::Xps::IXpsDocumentPackageTarget>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowXpsObjectModelTargetPackageNative {
    type Vtable = IPrintWorkflowXpsObjectModelTargetPackageNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2107030644, 39764, 19617, [173, 58, 151, 156, 61, 68, 221, 172]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Storage_Xps")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Storage_Xps"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrintWorkflowXpsReceiver(::windows::runtime::IUnknown);
impl IPrintWorkflowXpsReceiver {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com`*"]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(&self, documentsequenceprintticket: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetDocumentSequenceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, documentsequenceuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddDocumentData<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, documentid: u32, documentprintticket: Param1, documenturi: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(documentid), documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    pub unsafe fn AddPage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Xps::IXpsOMPageReference>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, documentid: u32, pageid: u32, pagereference: Param2, pageuri: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(documentid), ::std::mem::transmute(pageid), pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowXpsReceiver {
    type Vtable = IPrintWorkflowXpsReceiver_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(67728244, 30648, 18422, [129, 103, 170, 226, 157, 76, 248, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequenceprintticket: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequenceuri: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentid: u32, documentprintticket: ::windows::runtime::RawPtr, documenturi: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentid: u32, pageid: u32, pagereference: ::windows::runtime::RawPtr, pageuri: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrintWorkflowXpsReceiver2(::windows::runtime::IUnknown);
impl IPrintWorkflowXpsReceiver2 {
    #[cfg(feature = "Win32_System_Com")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com`*"]
    pub unsafe fn SetDocumentSequencePrintTicket<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(&self, documentsequenceprintticket: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), documentsequenceprintticket.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetDocumentSequenceUri<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, documentsequenceuri: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), documentsequenceuri.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn AddDocumentData<'a, Param1: ::windows::runtime::IntoParam<'a, super::Com::IStream>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, documentid: u32, documentprintticket: Param1, documenturi: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(documentid), documentprintticket.into_param().abi(), documenturi.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Storage_Xps`*"]
    pub unsafe fn AddPage<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Storage::Xps::IXpsOMPageReference>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, documentid: u32, pageid: u32, pagereference: Param2, pageuri: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(documentid), ::std::mem::transmute(pageid), pagereference.into_param().abi(), pageuri.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Close(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Failed(&self, xpserror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(xpserror)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IPrintWorkflowXpsReceiver2 {
    type Vtable = IPrintWorkflowXpsReceiver2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(37473292, 57259, 19041, [176, 116, 73, 12, 105, 149, 88, 13]);
}
impl ::std::convert::From<IPrintWorkflowXpsReceiver2> for IPrintWorkflowXpsReceiver {
    fn from(value: IPrintWorkflowXpsReceiver2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IPrintWorkflowXpsReceiver2> for IPrintWorkflowXpsReceiver {
    fn from(value: &IPrintWorkflowXpsReceiver2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintWorkflowXpsReceiver> for IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintWorkflowXpsReceiver> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPrintWorkflowXpsReceiver>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IPrintWorkflowXpsReceiver> for &IPrintWorkflowXpsReceiver2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, IPrintWorkflowXpsReceiver> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IPrintWorkflowXpsReceiver>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintWorkflowXpsReceiver2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_Com")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequenceprintticket: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentsequenceuri: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentid: u32, documentprintticket: ::windows::runtime::RawPtr, documenturi: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, documentid: u32, pageid: u32, pagereference: ::windows::runtime::RawPtr, pageuri: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Storage_Xps")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xpserror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IPrinting3DManagerInterop(::windows::runtime::IUnknown);
impl IPrinting3DManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn ShowPrintUIForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IPrinting3DManagerInterop {
    type Vtable = IPrinting3DManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2627932176, 5252, 17799, [178, 107, 221, 223, 159, 156, 174, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrinting3DManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, printmanager: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, asyncoperation: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IRandomAccessStreamFileAccessMode(::windows::runtime::IUnknown);
impl IRandomAccessStreamFileAccessMode {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetMode(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRandomAccessStreamFileAccessMode {
    type Vtable = IRandomAccessStreamFileAccessMode_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(858675272, 11797, 17806, [133, 196, 201, 17, 192, 195, 214, 244]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRandomAccessStreamFileAccessMode_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fileaccessmode: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IReferenceTracker(::windows::runtime::IUnknown);
impl IReferenceTracker {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ConnectFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn DisconnectFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn FindTrackerTargets<'a, Param0: ::windows::runtime::IntoParam<'a, IFindReferenceTargetsCallback>>(&self, callback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetReferenceTrackerManager(&self) -> ::windows::runtime::Result<IReferenceTrackerManager> {
        let mut result__: <IReferenceTrackerManager as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IReferenceTrackerManager>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn AddRefFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ReleaseFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn PegFromTrackerSource(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTracker {
    type Vtable = IReferenceTracker_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(299086138, 6158, 18313, [168, 190, 119, 18, 136, 40, 147, 230]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTracker_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IReferenceTrackerExtension(::windows::runtime::IUnknown);
impl IReferenceTrackerExtension {}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerExtension {
    type Vtable = IReferenceTrackerExtension_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1317633194, 22997, 17939, [143, 140, 247, 235, 209, 243, 153, 176]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerExtension_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IReferenceTrackerHost(::windows::runtime::IUnknown);
impl IReferenceTrackerHost {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn DisconnectUnusedReferenceSources(&self, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(options)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ReleaseDisconnectedReferenceSources(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn NotifyEndOfReferenceTrackingOnThread(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetTrackerTarget<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, unknown: Param0) -> ::windows::runtime::Result<IReferenceTrackerTarget> {
        let mut result__: <IReferenceTrackerTarget as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), unknown.into_param().abi(), &mut result__).from_abi::<IReferenceTrackerTarget>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn AddMemoryPressure(&self, bytesallocated: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(bytesallocated)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn RemoveMemoryPressure(&self, bytesallocated: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(bytesallocated)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerHost {
    type Vtable = IReferenceTrackerHost_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(698817642, 15426, 17430, [163, 157, 226, 130, 90, 7, 167, 115]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerHost_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: XAML_REFERENCETRACKER_DISCONNECT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, unknown: ::windows::runtime::RawPtr, newreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bytesallocated: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bytesallocated: u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IReferenceTrackerManager(::windows::runtime::IUnknown);
impl IReferenceTrackerManager {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ReferenceTrackingStarted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn FindTrackerTargetsCompleted(&self, findfailed: u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(findfailed)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ReferenceTrackingCompleted(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetReferenceTrackerHost<'a, Param0: ::windows::runtime::IntoParam<'a, IReferenceTrackerHost>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), value.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerManager {
    type Vtable = IReferenceTrackerManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1022461108, 31947, 19930, [132, 85, 126, 108, 233, 154, 50, 152]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, findfailed: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IReferenceTrackerTarget(::windows::runtime::IUnknown);
impl IReferenceTrackerTarget {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn AddRefFromReferenceTracker(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ReleaseFromReferenceTracker(&self) -> u32 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)))
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Peg(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Unpeg(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReferenceTrackerTarget {
    type Vtable = IReferenceTrackerTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1690125304, 49134, 20164, [183, 235, 41, 53, 21, 141, 174, 33]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReferenceTrackerTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IRestrictedErrorInfo(::windows::runtime::IUnknown);
impl IRestrictedErrorInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetErrorDetails(&self, description: *mut super::super::Foundation::BSTR, error: *mut ::windows::runtime::HRESULT, restricteddescription: *mut super::super::Foundation::BSTR, capabilitysid: *mut super::super::Foundation::BSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(description), ::std::mem::transmute(error), ::std::mem::transmute(restricteddescription), ::std::mem::transmute(capabilitysid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetReference(&self) -> ::windows::runtime::Result<super::super::Foundation::BSTR> {
        let mut result__: <super::super::Foundation::BSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IRestrictedErrorInfo {
    type Vtable = IRestrictedErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2193256594, 19592, 17021, [167, 188, 22, 221, 147, 254, 182, 126]);
}
unsafe impl ::std::marker::Send for IRestrictedErrorInfo {}
unsafe impl ::std::marker::Sync for IRestrictedErrorInfo {}
#[repr(C)]
#[doc(hidden)]
pub struct IRestrictedErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, description: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, error: *mut ::windows::runtime::HRESULT, restricteddescription: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>, capabilitysid: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, reference: *mut ::std::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IRoMetaDataLocator(::windows::runtime::IUnknown);
impl IRoMetaDataLocator {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Locate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, IRoSimpleMetaDataBuilder>>(&self, nameelement: Param0, metadatadestination: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), nameelement.into_param().abi(), metadatadestination.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRoMetaDataLocator {
    type Vtable = IRoMetaDataLocator_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoMetaDataLocator_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, nameelement: super::super::Foundation::PWSTR, metadatadestination: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IRoSimpleMetaDataBuilder(::windows::runtime::IUnknown);
impl IRoSimpleMetaDataBuilder {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetWinRtInterface<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, iid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), iid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetDelegate<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, iid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), iid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetInterfaceGroupSimpleDefault<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0, defaultinterfacename: Param1, defaultinterfaceiid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), name.into_param().abi(), defaultinterfacename.into_param().abi(), ::std::mem::transmute(defaultinterfaceiid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetInterfaceGroupParameterizedDefault<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementcount), ::std::mem::transmute(defaultinterfacenameelements)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetRuntimeClassSimpleDefault<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0, defaultinterfacename: Param1, defaultinterfaceiid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), name.into_param().abi(), defaultinterfacename.into_param().abi(), ::std::mem::transmute(defaultinterfaceiid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetRuntimeClassParameterizedDefault<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(elementcount), ::std::mem::transmute(defaultinterfacenameelements)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetStruct<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0, numfields: u32, fieldtypenames: *const super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), name.into_param().abi(), ::std::mem::transmute(numfields), ::std::mem::transmute(fieldtypenames)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetEnum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, name: Param0, basetype: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), name.into_param().abi(), basetype.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetParameterizedInterface<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, piid: Param0, numargs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), piid.into_param().abi(), ::std::mem::transmute(numargs)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetParameterizedDelegate<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, piid: Param0, numargs: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), piid.into_param().abi(), ::std::mem::transmute(numargs)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRoSimpleMetaDataBuilder {
    type Vtable = IRoSimpleMetaDataBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::zeroed();
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoSimpleMetaDataBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, defaultinterfacename: super::super::Foundation::PWSTR, defaultinterfaceiid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, defaultinterfacename: super::super::Foundation::PWSTR, defaultinterfaceiid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, elementcount: u32, defaultinterfacenameelements: *const super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, numfields: u32, fieldtypenames: *const super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: super::super::Foundation::PWSTR, basetype: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piid: ::windows::runtime::GUID, numargs: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, piid: ::windows::runtime::GUID, numargs: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IShareWindowCommandEventArgsInterop(::windows::runtime::IUnknown);
impl IShareWindowCommandEventArgsInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetWindow(&self) -> ::windows::runtime::Result<super::super::Foundation::HWND> {
        let mut result__: <super::super::Foundation::HWND as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::HWND>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IShareWindowCommandEventArgsInterop {
    type Vtable = IShareWindowCommandEventArgsInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1701947169, 25661, 17364, [172, 164, 107, 111, 95, 48, 241, 173]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandEventArgsInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IShareWindowCommandSourceInterop(::windows::runtime::IUnknown);
impl IShareWindowCommandSourceInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IShareWindowCommandSourceInterop {
    type Vtable = IShareWindowCommandSourceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1176115487, 33828, 17318, [160, 250, 52, 81, 162, 47, 86, 171]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IShareWindowCommandSourceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, sharewindowcommandsource: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISoftwareBitmapNative(::windows::runtime::IUnknown);
impl ISoftwareBitmapNative {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetData<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISoftwareBitmapNative {
    type Vtable = ISoftwareBitmapNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2495382549, 1258, 19246, [175, 19, 77, 233, 90, 168, 152, 235]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISoftwareBitmapNativeFactory(::windows::runtime::IUnknown);
impl ISoftwareBitmapNativeFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Graphics_Imaging`*"]
    pub unsafe fn CreateFromWICBitmap<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Imaging::IWICBitmap>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, T: ::windows::runtime::Interface>(&self, data: Param0, forcereadonly: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), data.into_param().abi(), forcereadonly.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn CreateFromMF2DBuffer2<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Media::MediaFoundation::IMF2DBuffer2>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, T: ::windows::runtime::Interface>(&self, data: Param0, subtype: *const ::windows::runtime::GUID, width: u32, height: u32, forcereadonly: Param4, mindisplayaperture: *const super::super::Media::MediaFoundation::MFVideoArea) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(
            ::std::mem::transmute_copy(self),
            data.into_param().abi(),
            ::std::mem::transmute(subtype),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            forcereadonly.into_param().abi(),
            ::std::mem::transmute(mindisplayaperture),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISoftwareBitmapNativeFactory {
    type Vtable = ISoftwareBitmapNativeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3284238828, 10516, 18321, [175, 2, 2, 210, 36, 161, 11, 67]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoftwareBitmapNativeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, forcereadonly: super::super::Foundation::BOOL, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Imaging")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, subtype: *const ::windows::runtime::GUID, width: u32, height: u32, forcereadonly: super::super::Foundation::BOOL, mindisplayaperture: *const super::super::Media::MediaFoundation::MFVideoArea, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISpatialInteractionManagerInterop(::windows::runtime::IUnknown);
impl ISpatialInteractionManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, window: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISpatialInteractionManagerInterop {
    type Vtable = ISpatialInteractionManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1548674358, 27288, 19334, [161, 112, 88, 112, 19, 214, 253, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpatialInteractionManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, spatialinteractionmanager: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IStorageFolderHandleAccess(::windows::runtime::IUnknown);
impl IStorageFolderHandleAccess {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, IOplockBreakingHandler>>(&self, filename: Param0, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: Param5) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), filename.into_param().abi(), ::std::mem::transmute(creationoptions), ::std::mem::transmute(accessoptions), ::std::mem::transmute(sharingoptions), ::std::mem::transmute(options), oplockbreakinghandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStorageFolderHandleAccess {
    type Vtable = IStorageFolderHandleAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3742995343, 21602, 18592, [190, 101, 210, 163, 39, 26, 8, 214]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageFolderHandleAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, filename: super::super::Foundation::PWSTR, creationoptions: HANDLE_CREATION_OPTIONS, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::runtime::RawPtr, interophandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IStorageItemHandleAccess(::windows::runtime::IUnknown);
impl IStorageItemHandleAccess {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Create<'a, Param3: ::windows::runtime::IntoParam<'a, IOplockBreakingHandler>>(&self, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: Param3) -> ::windows::runtime::Result<super::super::Foundation::HANDLE> {
        let mut result__: <super::super::Foundation::HANDLE as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(accessoptions), ::std::mem::transmute(sharingoptions), ::std::mem::transmute(options), oplockbreakinghandler.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IStorageItemHandleAccess {
    type Vtable = IStorageItemHandleAccess_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1554159282, 11301, 19746, [183, 133, 184, 133, 200, 32, 30, 106]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemHandleAccess_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, accessoptions: HANDLE_ACCESS_OPTIONS, sharingoptions: HANDLE_SHARING_OPTIONS, options: HANDLE_OPTIONS, oplockbreakinghandler: ::windows::runtime::RawPtr, interophandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISurfaceImageSourceManagerNative(::windows::runtime::IUnknown);
impl ISurfaceImageSourceManagerNative {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn FlushAllSurfacesWithDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, device: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceManagerNative {
    type Vtable = ISurfaceImageSourceManagerNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1283954871, 7560, 18959, [181, 155, 185, 63, 96, 13, 232, 200]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceManagerNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISurfaceImageSourceNative(::windows::runtime::IUnknown);
impl ISurfaceImageSourceNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn BeginDraw<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::std::option::Option<super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), updaterect.into_param().abi(), ::std::mem::transmute(surface), ::std::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceNative {
    type Vtable = ISurfaceImageSourceNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4075417025, 54023, 17701, [152, 134, 15, 175, 170, 68, 22, 60]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: super::super::Foundation::RECT, surface: *mut ::windows::runtime::RawPtr, offset: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISurfaceImageSourceNativeWithD2D(::windows::runtime::IUnknown);
impl ISurfaceImageSourceNativeWithD2D {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetDevice<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, device: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn BeginDraw(&self, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::std::ffi::c_void, offset: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(updaterect), ::std::mem::transmute(iid), ::std::mem::transmute(updateobject), ::std::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SuspendDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn ResumeDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISurfaceImageSourceNativeWithD2D {
    type Vtable = ISurfaceImageSourceNativeWithD2D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1412006435, 16865, 19009, [156, 8, 2, 232, 37, 104, 100, 161]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISurfaceImageSourceNativeWithD2D_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: *const super::super::Foundation::RECT, iid: *const ::windows::runtime::GUID, updateobject: *mut *mut ::std::ffi::c_void, offset: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISwapChainBackgroundPanelNative(::windows::runtime::IUnknown);
impl ISwapChainBackgroundPanelNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISwapChainBackgroundPanelNative {
    type Vtable = ISwapChainBackgroundPanelNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1136573774, 44501, 16437, [143, 133, 86, 8, 208, 142, 157, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainBackgroundPanelNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISwapChainInterop(::windows::runtime::IUnknown);
impl ISwapChainInterop {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, swapchain: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISwapChainInterop {
    type Vtable = ISwapChainInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(653563552, 32568, 17915, [136, 247, 250, 170, 190, 103, 221, 89]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISwapChainPanelNative(::windows::runtime::IUnknown);
impl ISwapChainPanelNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISwapChainPanelNative {
    type Vtable = ISwapChainPanelNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4180613586, 15070, 17830, [162, 12, 246, 241, 234, 144, 85, 75]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISwapChainPanelNative2(::windows::runtime::IUnknown);
impl ISwapChainPanelNative2 {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetSwapChain<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGISwapChain>>(&self, swapchain: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), swapchain.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn SetSwapChainHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, swapchainhandle: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), swapchainhandle.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISwapChainPanelNative2 {
    type Vtable = ISwapChainPanelNative2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3584226828, 14258, 17570, [147, 123, 141, 142, 185, 114, 104, 33]);
}
impl ::std::convert::From<ISwapChainPanelNative2> for ISwapChainPanelNative {
    fn from(value: ISwapChainPanelNative2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISwapChainPanelNative2> for ISwapChainPanelNative {
    fn from(value: &ISwapChainPanelNative2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISwapChainPanelNative> for ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISwapChainPanelNative> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISwapChainPanelNative>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISwapChainPanelNative> for &ISwapChainPanelNative2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISwapChainPanelNative> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISwapChainPanelNative>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISwapChainPanelNative2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchain: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, swapchainhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ISystemMediaTransportControlsInterop(::windows::runtime::IUnknown);
impl ISystemMediaTransportControlsInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, appwindow: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISystemMediaTransportControlsInterop {
    type Vtable = ISystemMediaTransportControlsInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3719317293, 51473, 18975, [134, 217, 220, 61, 113, 169, 95, 90]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISystemMediaTransportControlsInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, mediatransportcontrol: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ITensorNative(::windows::runtime::IUnknown);
impl ITensorNative {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetBuffer(&self, value: *mut *mut u8, capacity: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(value), ::std::mem::transmute(capacity)).ok()
    }
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn GetD3D12Resource(&self) -> ::windows::runtime::Result<super::super::Graphics::Direct3D12::ID3D12Resource> {
        let mut result__: <super::super::Graphics::Direct3D12::ID3D12Resource as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Graphics::Direct3D12::ID3D12Resource>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ITensorNative {
    type Vtable = ITensorNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1391806447, 23299, 18869, [130, 214, 86, 95, 30, 224, 221, 73]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut *mut u8, capacity: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ITensorStaticsNative(::windows::runtime::IUnknown);
impl ITensorStaticsNative {
    #[cfg(feature = "Win32_Graphics_Direct3D12")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Direct3D12`*"]
    pub unsafe fn CreateFromD3D12Resource<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Direct3D12::ID3D12Resource>>(&self, value: Param0, shape: *mut i64, shapecount: i32, result: *mut ::std::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), value.into_param().abi(), ::std::mem::transmute(shape), ::std::mem::transmute(shapecount), ::std::mem::transmute(result)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ITensorStaticsNative {
    type Vtable = ITensorStaticsNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(969954724, 26358, 20156, [149, 217, 122, 41, 235, 231, 105, 10]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITensorStaticsNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Direct3D12")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, shape: *mut i64, shapecount: i32, result: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Direct3D12"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ITrackerOwner(::windows::runtime::IUnknown);
impl ITrackerOwner {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn CreateTrackerHandle(&self) -> ::windows::runtime::Result<*mut TrackerHandle__> {
        let mut result__: <*mut TrackerHandle__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<*mut TrackerHandle__>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn DeleteTrackerHandle(&self, handle: *const TrackerHandle__) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(handle)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetTrackerValue<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, handle: *const TrackerHandle__, value: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(handle), value.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn TryGetSafeTrackerValue(&self, handle: *const TrackerHandle__, returnvalue: *mut ::std::option::Option<::windows::runtime::IUnknown>) -> u8 {
        ::std::mem::transmute((::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(handle), ::std::mem::transmute(returnvalue)))
    }
}
unsafe impl ::windows::runtime::Interface for ITrackerOwner {
    type Vtable = ITrackerOwner_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3945054731, 38934, 19143, [140, 255, 54, 246, 122, 17, 143, 78]);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITrackerOwner_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, returnvalue: *mut *mut TrackerHandle__) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: *const TrackerHandle__) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: *const TrackerHandle__, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handle: *const TrackerHandle__, returnvalue: *mut ::windows::runtime::RawPtr) -> u8,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUIViewSettingsInterop(::windows::runtime::IUnknown);
impl IUIViewSettingsInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, hwnd: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwnd.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUIViewSettingsInterop {
    type Vtable = IUIViewSettingsInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(915725305, 36712, 17598, [143, 245, 25, 92, 152, 237, 232, 166]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIViewSettingsInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwnd: super::super::Foundation::HWND, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUnbufferedFileHandleOplockCallback(::windows::runtime::IUnknown);
impl IUnbufferedFileHandleOplockCallback {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn OnBrokenCallback(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUnbufferedFileHandleOplockCallback {
    type Vtable = IUnbufferedFileHandleOplockCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3506543118, 25155, 17193, [132, 151, 46, 117, 137, 77, 119, 16]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleOplockCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUnbufferedFileHandleProvider(::windows::runtime::IUnknown);
impl IUnbufferedFileHandleProvider {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn OpenUnbufferedFileHandle<'a, Param0: ::windows::runtime::IntoParam<'a, IUnbufferedFileHandleOplockCallback>>(&self, oplockbreakcallback: Param0) -> ::windows::runtime::Result<usize> {
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), oplockbreakcallback.into_param().abi(), &mut result__).from_abi::<usize>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn CloseUnbufferedFileHandle(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUnbufferedFileHandleProvider {
    type Vtable = IUnbufferedFileHandleProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2791084297, 17067, 19348, [167, 177, 221, 46, 78, 104, 81, 94]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUnbufferedFileHandleProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, oplockbreakcallback: ::windows::runtime::RawPtr, filehandle: *mut usize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUserActivityInterop(::windows::runtime::IUnknown);
impl IUserActivityInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn CreateSessionForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, window: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUserActivityInterop {
    type Vtable = IUserActivityInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(450769229, 3594, 16601, [130, 76, 154, 8, 138, 80, 5, 159]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, iid: *const ::windows::runtime::GUID, value: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUserActivityRequestManagerInterop(::windows::runtime::IUnknown);
impl IUserActivityRequestManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetForWindow<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, T: ::windows::runtime::Interface>(&self, window: Param0) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), window.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUserActivityRequestManagerInterop {
    type Vtable = IUserActivityRequestManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3714709622, 38553, 18197, [144, 149, 227, 126, 163, 13, 250, 27]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivityRequestManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, window: super::super::Foundation::HWND, iid: *const ::windows::runtime::GUID, value: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUserActivitySourceHostInterop(::windows::runtime::IUnknown);
impl IUserActivitySourceHostInterop {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn SetActivitySourceHost<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, activitysourcehost: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), activitysourcehost.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IUserActivitySourceHostInterop {
    type Vtable = IUserActivitySourceHostInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3244161212, 34884, 18554, [184, 91, 117, 120, 224, 246, 20, 25]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserActivitySourceHostInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activitysourcehost: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IUserConsentVerifierInterop(::windows::runtime::IUnknown);
impl IUserConsentVerifierInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn RequestVerificationForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, T: ::windows::runtime::Interface>(&self, appwindow: Param0, message: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), message.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IUserConsentVerifierInterop {
    type Vtable = IUserConsentVerifierInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(971002051, 20084, 17434, [141, 192, 184, 17, 4, 223, 148, 156]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IUserConsentVerifierInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, message: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, riid: *const ::windows::runtime::GUID, asyncoperation: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IVideoFrameNative(::windows::runtime::IUnknown);
impl IVideoFrameNative {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetData<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetDevice<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVideoFrameNative {
    type Vtable = IVideoFrameNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(649752619, 12618, 17952, [170, 246, 122, 81, 170, 88, 250, 24]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IVideoFrameNativeFactory(::windows::runtime::IUnknown);
impl IVideoFrameNativeFactory {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Media_MediaFoundation`*"]
    pub unsafe fn CreateFromMFSample<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Media::MediaFoundation::IMFSample>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param6: ::windows::runtime::IntoParam<'a, super::super::Media::MediaFoundation::IMFDXGIDeviceManager>, T: ::windows::runtime::Interface>(
        &self,
        data: Param0,
        subtype: *const ::windows::runtime::GUID,
        width: u32,
        height: u32,
        forcereadonly: Param4,
        mindisplayaperture: *const super::super::Media::MediaFoundation::MFVideoArea,
        device: Param6,
    ) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(
            ::std::mem::transmute_copy(self),
            data.into_param().abi(),
            ::std::mem::transmute(subtype),
            ::std::mem::transmute(width),
            ::std::mem::transmute(height),
            forcereadonly.into_param().abi(),
            ::std::mem::transmute(mindisplayaperture),
            device.into_param().abi(),
            &<T as ::windows::runtime::Interface>::IID,
            &mut result__ as *mut _ as *mut _,
        )
        .and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IVideoFrameNativeFactory {
    type Vtable = IVideoFrameNativeFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1776511294, 36382, 20067, [172, 76, 127, 220, 33, 217, 115, 29]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVideoFrameNativeFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation"))]
    pub  unsafe extern "system" fn(this: ::windows::runtime::RawPtr, data: ::windows::runtime::RawPtr, subtype: *const ::windows::runtime::GUID, width: u32, height: u32, forcereadonly: super::super::Foundation::BOOL, mindisplayaperture: *const super::super::Media::MediaFoundation::MFVideoArea, device: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Media_MediaFoundation")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IVirtualSurfaceImageSourceNative(::windows::runtime::IUnknown);
impl IVirtualSurfaceImageSourceNative {
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn SetDevice<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGIDevice>>(&self, device: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), device.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Graphics_Dxgi`*"]
    pub unsafe fn BeginDraw<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, updaterect: Param0, surface: *mut ::std::option::Option<super::super::Graphics::Dxgi::IDXGISurface>, offset: *mut super::super::Foundation::POINT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), updaterect.into_param().abi(), ::std::mem::transmute(surface), ::std::mem::transmute(offset)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn EndDraw(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn Invalidate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::RECT>>(&self, updaterect: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), updaterect.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetUpdateRectCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetUpdateRects(&self, updates: *mut super::super::Foundation::RECT, count: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(updates), ::std::mem::transmute(count)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn GetVisibleBounds(&self) -> ::windows::runtime::Result<super::super::Foundation::RECT> {
        let mut result__: <super::super::Foundation::RECT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::RECT>(result__)
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn RegisterForUpdatesNeeded<'a, Param0: ::windows::runtime::IntoParam<'a, IVirtualSurfaceUpdatesCallbackNative>>(&self, callback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), callback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Resize(&self, newwidth: i32, newheight: i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(newwidth), ::std::mem::transmute(newheight)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceImageSourceNative {
    type Vtable = IVirtualSurfaceImageSourceNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3914664323, 13835, 20307, [179, 145, 175, 214, 149, 7, 134, 145]);
}
impl ::std::convert::From<IVirtualSurfaceImageSourceNative> for ISurfaceImageSourceNative {
    fn from(value: IVirtualSurfaceImageSourceNative) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IVirtualSurfaceImageSourceNative> for ISurfaceImageSourceNative {
    fn from(value: &IVirtualSurfaceImageSourceNative) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISurfaceImageSourceNative> for IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISurfaceImageSourceNative> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISurfaceImageSourceNative>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISurfaceImageSourceNative> for &IVirtualSurfaceImageSourceNative {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISurfaceImageSourceNative> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISurfaceImageSourceNative>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceImageSourceNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Graphics_Dxgi")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, device: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: super::super::Foundation::RECT, surface: *mut ::windows::runtime::RawPtr, offset: *mut super::super::Foundation::POINT) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updaterect: super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, updates: *mut super::super::Foundation::RECT, count: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bounds: *mut super::super::Foundation::RECT) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, newwidth: i32, newheight: i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IVirtualSurfaceUpdatesCallbackNative(::windows::runtime::IUnknown);
impl IVirtualSurfaceUpdatesCallbackNative {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn UpdatesNeeded(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVirtualSurfaceUpdatesCallbackNative {
    type Vtable = IVirtualSurfaceUpdatesCallbackNative_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3690129735, 36460, 16980, [158, 238, 119, 56, 247, 19, 134, 201]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualSurfaceUpdatesCallbackNative_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IVisualInteractionSourceInterop(::windows::runtime::IUnknown);
impl IVisualInteractionSourceInterop {
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_UI_Input_Pointer`, `Win32_UI_WindowsAndMessaging`*"]
    pub unsafe fn TryRedirectForManipulation(&self, pointerinfo: *const super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pointerinfo)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IVisualInteractionSourceInterop {
    type Vtable = IVisualInteractionSourceInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(301346001, 12189, 17107, [176, 95, 214, 121, 13, 158, 159, 142]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualInteractionSourceInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pointerinfo: *const super::super::UI::Input::Pointer::POINTER_INFO) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Input_Pointer", feature = "Win32_UI_WindowsAndMessaging")))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWeakReference(::windows::runtime::IUnknown);
impl IWeakReference {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Resolve<T: ::windows::runtime::Interface>(&self) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWeakReference {
    type Vtable = IWeakReference_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(55, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReference_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, objectreference: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWeakReferenceSource(::windows::runtime::IUnknown);
impl IWeakReferenceSource {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn GetWeakReference(&self) -> ::windows::runtime::Result<IWeakReference> {
        let mut result__: <IWeakReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IWeakReference>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWeakReferenceSource {
    type Vtable = IWeakReferenceSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(56, 0, 0, [192, 0, 0, 0, 0, 0, 0, 70]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWeakReferenceSource_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, weakreference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWebAuthenticationCoreManagerInterop(::windows::runtime::IUnknown);
impl IWebAuthenticationCoreManagerInterop {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn RequestTokenForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, T: ::windows::runtime::Interface>(&self, appwindow: Param0, request: Param1) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), request.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
    pub unsafe fn RequestTokenWithWebAccountForWindowAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>, T: ::windows::runtime::Interface>(&self, appwindow: Param0, request: Param1, webaccount: Param2) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), appwindow.into_param().abi(), request.into_param().abi(), webaccount.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWebAuthenticationCoreManagerInterop {
    type Vtable = IWebAuthenticationCoreManagerInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4105758724, 33054, 17462, [182, 156, 68, 203, 103, 183, 32, 132]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebAuthenticationCoreManagerInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, request: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, asyncinfo: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, appwindow: super::super::Foundation::HWND, request: ::windows::runtime::RawPtr, webaccount: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, asyncinfo: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop(::windows::runtime::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn CreateFromWin32Handle<T: ::windows::runtime::Interface>(&self, win32handle: u64, enableaboutdata: u8) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(win32handle), ::std::mem::transmute(enableaboutdata), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsDevicesAllJoynBusAttachmentFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentFactoryInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1267692805, 45625, 20091, [136, 175, 246, 104, 37, 117, 216, 97]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentFactoryInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, win32handle: u64, enableaboutdata: u8, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop(::windows::runtime::IUnknown);
impl IWindowsDevicesAllJoynBusAttachmentInterop {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Win32Handle(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsDevicesAllJoynBusAttachmentInterop {
    type Vtable = IWindowsDevicesAllJoynBusAttachmentInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4253664859, 46350, 18969, [157, 12, 180, 43, 120, 50, 129, 205]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusAttachmentInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop(::windows::runtime::IUnknown);
impl IWindowsDevicesAllJoynBusObjectFactoryInterop {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn CreateFromWin32Handle<T: ::windows::runtime::Interface>(&self, win32handle: u64) -> ::windows::runtime::Result<T> {
        let mut result__ = ::std::option::Option::None;
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(win32handle), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsDevicesAllJoynBusObjectFactoryInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectFactoryInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1635050758, 35733, 20022, [149, 192, 184, 143, 237, 52, 147, 140]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectFactoryInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, win32handle: u64, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct IWindowsDevicesAllJoynBusObjectInterop(::windows::runtime::IUnknown);
impl IWindowsDevicesAllJoynBusObjectInterop {
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn AddPropertyGetHandler<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, context: *const ::std::ffi::c_void, interfacename: Param1, callback: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(context), interfacename.into_param().abi(), ::std::mem::transmute(callback)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn AddPropertySetHandler<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, context: *const ::std::ffi::c_void, interfacename: Param1, callback: isize) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(context), interfacename.into_param().abi(), ::std::mem::transmute(callback)).ok()
    }
    #[doc = "*Required features: `Win32_System_WinRT`*"]
    pub unsafe fn Win32Handle(&self) -> ::windows::runtime::Result<u64> {
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u64>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IWindowsDevicesAllJoynBusObjectInterop {
    type Vtable = IWindowsDevicesAllJoynBusObjectInterop_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3616187349, 20564, 17039, [153, 242, 236, 58, 93, 227, 195, 188]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowsDevicesAllJoynBusObjectInterop_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void, interfacename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, callback: isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void, interfacename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, callback: isize) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut u64) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsErrorPropagationEnabled() -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(IsErrorPropagationEnabled())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512u32;
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MetaDataGetDispenser(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        MetaDataGetDispenser(::std::mem::transmute(rclsid), ::std::mem::transmute(riid), ::std::mem::transmute(ppv)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`, `Win32_Graphics_Direct2D`*"]
pub struct PDF_RENDER_PARAMS {
    pub SourceRect: super::super::Graphics::Direct2D::D2D_RECT_F,
    pub DestinationWidth: u32,
    pub DestinationHeight: u32,
    pub BackgroundColor: super::super::Graphics::Direct2D::D2D_COLOR_F,
    pub IgnoreHighContrast: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
impl PDF_RENDER_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
impl ::std::default::Default for PDF_RENDER_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
impl ::std::fmt::Debug for PDF_RENDER_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PDF_RENDER_PARAMS").field("SourceRect", &self.SourceRect).field("DestinationWidth", &self.DestinationWidth).field("DestinationHeight", &self.DestinationHeight).field("BackgroundColor", &self.BackgroundColor).field("IgnoreHighContrast", &self.IgnoreHighContrast).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
impl ::std::cmp::PartialEq for PDF_RENDER_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SourceRect == other.SourceRect && self.DestinationWidth == other.DestinationWidth && self.DestinationHeight == other.DestinationHeight && self.BackgroundColor == other.BackgroundColor && self.IgnoreHighContrast == other.IgnoreHighContrast
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
impl ::std::cmp::Eq for PDF_RENDER_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct2D"))]
unsafe impl ::windows::runtime::Abi for PDF_RENDER_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi")]
pub type PFN_PDF_CREATE_RENDERER = unsafe extern "system" fn(param0: ::windows::runtime::RawPtr, param1: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
pub type PINSPECT_HSTRING_CALLBACK = unsafe extern "system" fn(context: *const ::std::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows::runtime::HRESULT;
pub type PINSPECT_HSTRING_CALLBACK2 = unsafe extern "system" fn(context: *const ::std::ffi::c_void, readaddress: u64, length: u32, buffer: *mut u8) -> ::windows::runtime::HRESULT;
pub type PINSPECT_MEMORY_CALLBACK = unsafe extern "system" fn(context: *const ::std::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> ::windows::runtime::HRESULT;
#[cfg(feature = "Win32_Graphics_Dxgi")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Graphics_Dxgi`*"]
#[inline]
pub unsafe fn PdfCreateRenderer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Graphics::Dxgi::IDXGIDevice>>(pdevice: Param0) -> ::windows::runtime::Result<IPdfRendererNative> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PdfCreateRenderer(pdevice: ::windows::runtime::RawPtr, pprenderer: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IPdfRendererNative as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        PdfCreateRenderer(pdevice.into_param().abi(), &mut result__).from_abi::<IPdfRendererNative>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct ROPARAMIIDHANDLE(pub isize);
impl ::std::default::Default for ROPARAMIIDHANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for ROPARAMIIDHANDLE {}
unsafe impl ::windows::runtime::Abi for ROPARAMIIDHANDLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RO_ERROR_REPORTING_FLAGS(pub u32);
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(0u32);
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(1u32);
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(2u32);
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(4u32);
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = RO_ERROR_REPORTING_FLAGS(8u32);
impl ::std::convert::From<u32> for RO_ERROR_REPORTING_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RO_ERROR_REPORTING_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RO_ERROR_REPORTING_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RO_ERROR_REPORTING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RO_INIT_TYPE(pub i32);
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = RO_INIT_TYPE(0i32);
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = RO_INIT_TYPE(1i32);
impl ::std::convert::From<i32> for RO_INIT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RO_INIT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoActivateInstance<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(activatableclassid: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoActivateInstance(activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, instance: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::IInspectable as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoActivateInstance(activatableclassid.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoCaptureErrorContext(hr: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoCaptureErrorContext(hr: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
        }
        RoCaptureErrorContext(::std::mem::transmute(hr)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoClearError() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoClearError();
        }
        ::std::mem::transmute(RoClearError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoFailFastWithErrorContext(hrerror: ::windows::runtime::HRESULT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoFailFastWithErrorContext(hrerror: ::windows::runtime::HRESULT);
        }
        ::std::mem::transmute(RoFailFastWithErrorContext(::std::mem::transmute(hrerror)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoFreeParameterizedTypeExtra<'a, Param0: ::windows::runtime::IntoParam<'a, ROPARAMIIDHANDLE>>(extra: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE);
        }
        ::std::mem::transmute(RoFreeParameterizedTypeExtra(extra.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoGetActivationFactory<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, T: ::windows::runtime::Interface>(activatableclassid: Param0) -> ::windows::runtime::Result<T> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetActivationFactory(activatableclassid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, iid: *const ::windows::runtime::GUID, factory: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        let mut result__ = ::std::option::Option::None;
        RoGetActivationFactory(activatableclassid.into_param().abi(), &<T as ::windows::runtime::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoGetAgileReference<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(options: AgileReferenceOptions, riid: *const ::windows::runtime::GUID, punk: Param2) -> ::windows::runtime::Result<IAgileReference> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetAgileReference(options: AgileReferenceOptions, riid: *const ::windows::runtime::GUID, punk: ::windows::runtime::RawPtr, ppagilereference: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IAgileReference as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoGetAgileReference(::std::mem::transmute(options), ::std::mem::transmute(riid), punk.into_param().abi(), &mut result__).from_abi::<IAgileReference>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoGetApartmentIdentifier() -> ::windows::runtime::Result<u64> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetApartmentIdentifier(apartmentidentifier: *mut u64) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u64 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoGetApartmentIdentifier(&mut result__).from_abi::<u64>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Com_Marshal")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_System_Com_Marshal`*"]
#[inline]
pub unsafe fn RoGetBufferMarshaler() -> ::windows::runtime::Result<super::Com::Marshal::IMarshal> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetBufferMarshaler(buffermarshaler: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::Com::Marshal::IMarshal as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoGetBufferMarshaler(&mut result__).from_abi::<super::Com::Marshal::IMarshal>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoGetErrorReportingFlags() -> ::windows::runtime::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetErrorReportingFlags(pflags: *mut u32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoGetErrorReportingFlags(&mut result__).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<IRestrictedErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetMatchingRestrictedErrorInfo(hrin: ::windows::runtime::HRESULT, pprestrictederrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IRestrictedErrorInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoGetMatchingRestrictedErrorInfo(::std::mem::transmute(hrin), &mut result__).from_abi::<IRestrictedErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RoGetParameterizedTypeInstanceIID<'a, Param2: ::windows::runtime::IntoParam<'a, IRoMetaDataLocator>>(nameelementcount: u32, nameelements: *const super::super::Foundation::PWSTR, metadatalocator: Param2, iid: *mut ::windows::runtime::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetParameterizedTypeInstanceIID(nameelementcount: u32, nameelements: *const super::super::Foundation::PWSTR, metadatalocator: ::windows::runtime::RawPtr, iid: *mut ::windows::runtime::GUID, pextra: *mut ROPARAMIIDHANDLE) -> ::windows::runtime::HRESULT;
        }
        RoGetParameterizedTypeInstanceIID(::std::mem::transmute(nameelementcount), ::std::mem::transmute(nameelements), metadatalocator.into_param().abi(), ::std::mem::transmute(iid), ::std::mem::transmute(pextra)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoGetServerActivatableClasses<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(servername: Param0, activatableclassids: *mut *mut ::windows::runtime::HSTRING, count: *mut u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoGetServerActivatableClasses(servername: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, activatableclassids: *mut *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, count: *mut u32) -> ::windows::runtime::HRESULT;
        }
        RoGetServerActivatableClasses(servername.into_param().abi(), ::std::mem::transmute(activatableclassids), ::std::mem::transmute(count)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoInitialize(inittype: RO_INIT_TYPE) -> ::windows::runtime::HRESULT;
        }
        RoInitialize(::std::mem::transmute(inittype)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: ::std::option::Option<PINSPECT_MEMORY_CALLBACK>, context: *const ::std::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> ::windows::runtime::HRESULT;
        }
        RoInspectCapturedStackBackTrace(::std::mem::transmute(targeterrorinfoaddress), ::std::mem::transmute(machine), ::std::mem::transmute(readmemorycallback), ::std::mem::transmute(context), ::std::mem::transmute(framecount), ::std::mem::transmute(targetbacktraceaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: ::std::option::Option<PINSPECT_MEMORY_CALLBACK>, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<usize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void, targeterrorinfoaddress: *mut usize) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <usize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoInspectThreadErrorInfo(::std::mem::transmute(targettebaddress), ::std::mem::transmute(machine), ::std::mem::transmute(readmemorycallback), ::std::mem::transmute(context), &mut result__).from_abi::<usize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RoOriginateError<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(error: ::windows::runtime::HRESULT, message: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoOriginateError(error: ::windows::runtime::HRESULT, message: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RoOriginateError(::std::mem::transmute(error), message.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RoOriginateErrorW<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(error: ::windows::runtime::HRESULT, cchmax: u32, message: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoOriginateErrorW(error: ::windows::runtime::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RoOriginateErrorW(::std::mem::transmute(error), ::std::mem::transmute(cchmax), message.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RoOriginateLanguageException<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(error: ::windows::runtime::HRESULT, message: Param1, languageexception: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoOriginateLanguageException(error: ::windows::runtime::HRESULT, message: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, languageexception: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RoOriginateLanguageException(::std::mem::transmute(error), message.into_param().abi(), languageexception.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RoParameterizedTypeExtraGetTypeSignature<'a, Param0: ::windows::runtime::IntoParam<'a, ROPARAMIIDHANDLE>>(extra: Param0) -> super::super::Foundation::PSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> super::super::Foundation::PSTR;
        }
        ::std::mem::transmute(RoParameterizedTypeExtraGetTypeSignature(extra.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const ::windows::runtime::HSTRING, activationfactorycallbacks: *const isize, count: u32) -> ::windows::runtime::Result<isize> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoRegisterActivationFactories(activatableclassids: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, activationfactorycallbacks: *const isize, count: u32, cookie: *mut isize) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <isize as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoRegisterActivationFactories(::std::mem::transmute(activatableclassids), ::std::mem::transmute(activationfactorycallbacks), ::std::mem::transmute(count), &mut result__).from_abi::<isize>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<'a, Param0: ::windows::runtime::IntoParam<'a, IApartmentShutdown>>(callbackobject: Param0, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoRegisterForApartmentShutdown(callbackobject: ::windows::runtime::RawPtr, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::runtime::HRESULT;
        }
        RoRegisterForApartmentShutdown(callbackobject.into_param().abi(), ::std::mem::transmute(apartmentidentifier), ::std::mem::transmute(regcookie)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoReportFailedDelegate<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, IRestrictedErrorInfo>>(punkdelegate: Param0, prestrictederrorinfo: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoReportFailedDelegate(punkdelegate: ::windows::runtime::RawPtr, prestrictederrorinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        RoReportFailedDelegate(punkdelegate.into_param().abi(), prestrictederrorinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoReportUnhandledError<'a, Param0: ::windows::runtime::IntoParam<'a, IRestrictedErrorInfo>>(prestrictederrorinfo: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoReportUnhandledError(prestrictederrorinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        RoReportUnhandledError(prestrictederrorinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(reference: Param0) -> ::windows::runtime::Result<IRestrictedErrorInfo> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoResolveRestrictedErrorInfoReference(reference: super::super::Foundation::PWSTR, pprestrictederrorinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <IRestrictedErrorInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        RoResolveRestrictedErrorInfoReference(reference.into_param().abi(), &mut result__).from_abi::<IRestrictedErrorInfo>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoRevokeActivationFactories(cookie: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoRevokeActivationFactories(cookie: isize);
        }
        ::std::mem::transmute(RoRevokeActivationFactories(::std::mem::transmute(cookie)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: u32) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoSetErrorReportingFlags(flags: u32) -> ::windows::runtime::HRESULT;
        }
        RoSetErrorReportingFlags(::std::mem::transmute(flags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RoTransformError<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(olderror: ::windows::runtime::HRESULT, newerror: ::windows::runtime::HRESULT, message: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoTransformError(olderror: ::windows::runtime::HRESULT, newerror: ::windows::runtime::HRESULT, message: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RoTransformError(::std::mem::transmute(olderror), ::std::mem::transmute(newerror), message.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn RoTransformErrorW<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(olderror: ::windows::runtime::HRESULT, newerror: ::windows::runtime::HRESULT, cchmax: u32, message: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoTransformErrorW(olderror: ::windows::runtime::HRESULT, newerror: ::windows::runtime::HRESULT, cchmax: u32, message: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RoTransformErrorW(::std::mem::transmute(olderror), ::std::mem::transmute(newerror), ::std::mem::transmute(cchmax), message.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoUninitialize() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoUninitialize();
        }
        ::std::mem::transmute(RoUninitialize())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn RoUnregisterForApartmentShutdown<'a, Param0: ::windows::runtime::IntoParam<'a, APARTMENT_SHUTDOWN_REGISTRATION_COOKIE>>(regcookie: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> ::windows::runtime::HRESULT;
        }
        RoUnregisterForApartmentShutdown(regcookie.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub struct ServerInformation {
    pub dwServerPid: u32,
    pub dwServerTid: u32,
    pub ui64ServerAddress: u64,
}
impl ServerInformation {}
impl ::std::default::Default for ServerInformation {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ServerInformation {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ServerInformation").field("dwServerPid", &self.dwServerPid).field("dwServerTid", &self.dwServerTid).field("ui64ServerAddress", &self.ui64ServerAddress).finish()
    }
}
impl ::std::cmp::PartialEq for ServerInformation {
    fn eq(&self, other: &Self) -> bool {
        self.dwServerPid == other.dwServerPid && self.dwServerTid == other.dwServerTid && self.ui64ServerAddress == other.ui64ServerAddress
    }
}
impl ::std::cmp::Eq for ServerInformation {}
unsafe impl ::windows::runtime::Abi for ServerInformation {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn SetRestrictedErrorInfo<'a, Param0: ::windows::runtime::IntoParam<'a, IRestrictedErrorInfo>>(prestrictederrorinfo: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetRestrictedErrorInfo(prestrictederrorinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        SetRestrictedErrorInfo(prestrictederrorinfo.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WinRT`*"]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl TrackerHandle__ {}
impl ::std::default::Default for TrackerHandle__ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for TrackerHandle__ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("TrackerHandle__").field("unused", &self.unused).finish()
    }
}
impl ::std::cmp::PartialEq for TrackerHandle__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::std::cmp::Eq for TrackerHandle__ {}
unsafe impl ::windows::runtime::Abi for TrackerHandle__ {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct TrustLevel(pub i32);
pub const BaseTrust: TrustLevel = TrustLevel(0i32);
pub const PartialTrust: TrustLevel = TrustLevel(1i32);
pub const FullTrust: TrustLevel = TrustLevel(2i32);
impl ::std::convert::From<i32> for TrustLevel {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for TrustLevel {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsCompareStringOrdinal<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string1: Param0, string2: Param1) -> ::windows::runtime::Result<i32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsCompareStringOrdinal(string1: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, string2: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result: *mut i32) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsCompareStringOrdinal(string1.into_param().abi(), string2.into_param().abi(), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsConcatString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string1: Param0, string2: Param1) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsConcatString(string1: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, string2: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsConcatString(string1.into_param().abi(), string2.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WindowsCreateString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(sourcestring: Param0, length: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsCreateString(sourcestring: super::super::Foundation::PWSTR, length: u32, string: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsCreateString(sourcestring.into_param().abi(), ::std::mem::transmute(length), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WindowsCreateStringReference<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(sourcestring: Param0, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::windows::runtime::HSTRING) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsCreateStringReference(sourcestring: super::super::Foundation::PWSTR, length: u32, hstringheader: *mut HSTRING_HEADER, string: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        WindowsCreateStringReference(sourcestring.into_param().abi(), ::std::mem::transmute(length), ::std::mem::transmute(hstringheader), ::std::mem::transmute(string)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsDeleteString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsDeleteString(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        WindowsDeleteString(string.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsDeleteStringBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, HSTRING_BUFFER>>(bufferhandle: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsDeleteStringBuffer(bufferhandle: HSTRING_BUFFER) -> ::windows::runtime::HRESULT;
        }
        WindowsDeleteStringBuffer(bufferhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsDuplicateString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsDuplicateString(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsDuplicateString(string.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsGetStringLen<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsGetStringLen(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> u32;
        }
        ::std::mem::transmute(WindowsGetStringLen(string.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WindowsGetStringRawBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0, length: *mut u32) -> super::super::Foundation::PWSTR {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsGetStringRawBuffer(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, length: *mut u32) -> super::super::Foundation::PWSTR;
        }
        ::std::mem::transmute(WindowsGetStringRawBuffer(string.into_param().abi(), ::std::mem::transmute(length)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsInspectString(targethstring: usize, machine: u16, callback: ::std::option::Option<PINSPECT_HSTRING_CALLBACK>, context: *const ::std::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsInspectString(targethstring: usize, machine: u16, callback: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void, length: *mut u32, targetstringaddress: *mut usize) -> ::windows::runtime::HRESULT;
        }
        WindowsInspectString(::std::mem::transmute(targethstring), ::std::mem::transmute(machine), ::std::mem::transmute(callback), ::std::mem::transmute(context), ::std::mem::transmute(length), ::std::mem::transmute(targetstringaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsInspectString2(targethstring: u64, machine: u16, callback: ::std::option::Option<PINSPECT_HSTRING_CALLBACK2>, context: *const ::std::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsInspectString2(targethstring: u64, machine: u16, callback: ::windows::runtime::RawPtr, context: *const ::std::ffi::c_void, length: *mut u32, targetstringaddress: *mut u64) -> ::windows::runtime::HRESULT;
        }
        WindowsInspectString2(::std::mem::transmute(targethstring), ::std::mem::transmute(machine), ::std::mem::transmute(callback), ::std::mem::transmute(context), ::std::mem::transmute(length), ::std::mem::transmute(targetstringaddress)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WindowsIsStringEmpty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsIsStringEmpty(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(WindowsIsStringEmpty(string.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsPreallocateStringBuffer(length: u32, charbuffer: *mut *mut u16, bufferhandle: *mut HSTRING_BUFFER) -> ::windows::runtime::HRESULT;
        }
        WindowsPreallocateStringBuffer(::std::mem::transmute(length), ::std::mem::transmute(charbuffer), ::std::mem::transmute(bufferhandle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsPromoteStringBuffer<'a, Param0: ::windows::runtime::IntoParam<'a, HSTRING_BUFFER>>(bufferhandle: Param0) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsPromoteStringBuffer(bufferhandle: HSTRING_BUFFER, string: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsPromoteStringBuffer(bufferhandle.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsReplaceString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0, stringreplaced: Param1, stringreplacewith: Param2) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsReplaceString(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, stringreplaced: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, stringreplacewith: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsReplaceString(string.into_param().abi(), stringreplaced.into_param().abi(), stringreplacewith.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WinRT`, `Win32_Foundation`*"]
#[inline]
pub unsafe fn WindowsStringHasEmbeddedNull<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsStringHasEmbeddedNull(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, hasembednull: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsStringHasEmbeddedNull(string.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsSubstring<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0, startindex: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsSubstring(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, startindex: u32, newstring: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsSubstring(string.into_param().abi(), ::std::mem::transmute(startindex), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsSubstringWithSpecifiedLength<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0, startindex: u32, length: u32) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsSubstringWithSpecifiedLength(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, startindex: u32, length: u32, newstring: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsSubstringWithSpecifiedLength(string.into_param().abi(), ::std::mem::transmute(startindex), ::std::mem::transmute(length), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsTrimStringEnd<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0, trimstring: Param1) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsTrimStringEnd(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, trimstring: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsTrimStringEnd(string.into_param().abi(), trimstring.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[inline]
pub unsafe fn WindowsTrimStringStart<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(string: Param0, trimstring: Param1) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WindowsTrimStringStart(string: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, trimstring: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, newstring: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <::windows::runtime::HSTRING as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        WindowsTrimStringStart(string.into_param().abi(), trimstring.into_param().abi(), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_WinRT`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XAML_REFERENCETRACKER_DISCONNECT(pub i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(0i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(1i32);
impl ::std::convert::From<i32> for XAML_REFERENCETRACKER_DISCONNECT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XAML_REFERENCETRACKER_DISCONNECT {
    type Abi = Self;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
