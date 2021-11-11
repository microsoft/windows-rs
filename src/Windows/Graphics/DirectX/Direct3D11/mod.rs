#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Graphics_DirectX_Direct3D11`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Direct3DBindings(pub u32);
impl Direct3DBindings {
    pub const VertexBuffer: Direct3DBindings = Direct3DBindings(1u32);
    pub const IndexBuffer: Direct3DBindings = Direct3DBindings(2u32);
    pub const ConstantBuffer: Direct3DBindings = Direct3DBindings(4u32);
    pub const ShaderResource: Direct3DBindings = Direct3DBindings(8u32);
    pub const StreamOutput: Direct3DBindings = Direct3DBindings(16u32);
    pub const RenderTarget: Direct3DBindings = Direct3DBindings(32u32);
    pub const DepthStencil: Direct3DBindings = Direct3DBindings(64u32);
    pub const UnorderedAccess: Direct3DBindings = Direct3DBindings(128u32);
    pub const Decoder: Direct3DBindings = Direct3DBindings(512u32);
    pub const VideoEncoder: Direct3DBindings = Direct3DBindings(1024u32);
}
impl ::core::convert::From<u32> for Direct3DBindings {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Direct3DBindings {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Direct3DBindings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.Direct3D11.Direct3DBindings;u4)");
}
impl ::windows::runtime::DefaultType for Direct3DBindings {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for Direct3DBindings {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for Direct3DBindings {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for Direct3DBindings {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for Direct3DBindings {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for Direct3DBindings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Graphics_DirectX_Direct3D11`*"]
pub struct Direct3DMultisampleDescription {
    pub Count: i32,
    pub Quality: i32,
}
impl Direct3DMultisampleDescription {}
impl ::core::default::Default for Direct3DMultisampleDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Direct3DMultisampleDescription {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Direct3DMultisampleDescription").field("Count", &self.Count).field("Quality", &self.Quality).finish()
    }
}
impl ::core::cmp::PartialEq for Direct3DMultisampleDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Quality == other.Quality
    }
}
impl ::core::cmp::Eq for Direct3DMultisampleDescription {}
unsafe impl ::windows::runtime::Abi for Direct3DMultisampleDescription {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Direct3DMultisampleDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription;i4;i4)");
}
impl ::windows::runtime::DefaultType for Direct3DMultisampleDescription {
    type DefaultType = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Graphics_DirectX_Direct3D11`*"]
pub struct Direct3DSurfaceDescription {
    pub Width: i32,
    pub Height: i32,
    pub Format: super::DirectXPixelFormat,
    pub MultisampleDescription: Direct3DMultisampleDescription,
}
impl Direct3DSurfaceDescription {}
impl ::core::default::Default for Direct3DSurfaceDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for Direct3DSurfaceDescription {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("Direct3DSurfaceDescription").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("MultisampleDescription", &self.MultisampleDescription).finish()
    }
}
impl ::core::cmp::PartialEq for Direct3DSurfaceDescription {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.MultisampleDescription == other.MultisampleDescription
    }
}
impl ::core::cmp::Eq for Direct3DSurfaceDescription {}
unsafe impl ::windows::runtime::Abi for Direct3DSurfaceDescription {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Direct3DSurfaceDescription {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.DirectX.Direct3D11.Direct3DSurfaceDescription;i4;i4;enum(Windows.Graphics.DirectX.DirectXPixelFormat;i4);struct(Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription;i4;i4))");
}
impl ::windows::runtime::DefaultType for Direct3DSurfaceDescription {
    type DefaultType = Self;
}
#[doc = "*Required features: `Graphics_DirectX_Direct3D11`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct Direct3DUsage(pub i32);
impl Direct3DUsage {
    pub const Default: Direct3DUsage = Direct3DUsage(0i32);
    pub const Immutable: Direct3DUsage = Direct3DUsage(1i32);
    pub const Dynamic: Direct3DUsage = Direct3DUsage(2i32);
    pub const Staging: Direct3DUsage = Direct3DUsage(3i32);
}
impl ::core::convert::From<i32> for Direct3DUsage {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for Direct3DUsage {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for Direct3DUsage {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.Direct3D11.Direct3DUsage;i4)");
}
impl ::windows::runtime::DefaultType for Direct3DUsage {
    type DefaultType = Self;
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_DirectX_Direct3D11`*"]
pub struct IDirect3DDevice(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDirect3DDevice {
    type Vtable = IDirect3DDevice_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa37624ab_8d5f_4650_9d3e_9eae3d9bc670);
}
impl IDirect3DDevice {
    #[doc = "*Required features: `Graphics_DirectX_Direct3D11`*"]
    pub fn Trim(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_DirectX_Direct3D11`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDirect3DDevice {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{a37624ab-8d5f-4650-9d3e-9eae3d9bc670}");
}
impl ::core::convert::From<IDirect3DDevice> for ::windows::runtime::IUnknown {
    fn from(value: IDirect3DDevice) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDirect3DDevice> for ::windows::runtime::IUnknown {
    fn from(value: &IDirect3DDevice) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirect3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirect3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDirect3DDevice> for ::windows::runtime::IInspectable {
    fn from(value: IDirect3DDevice) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirect3DDevice> for ::windows::runtime::IInspectable {
    fn from(value: &IDirect3DDevice) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDirect3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDirect3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IDirect3DDevice> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IDirect3DDevice) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IDirect3DDevice> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IDirect3DDevice) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for IDirect3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &IDirect3DDevice {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Graphics_DirectX_Direct3D11`*"]
pub struct IDirect3DSurface(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDirect3DSurface {
    type Vtable = IDirect3DSurface_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x0bf4a146_13c1_4694_bee3_7abf15eaf586);
}
impl IDirect3DSurface {
    #[doc = "*Required features: `Graphics_DirectX_Direct3D11`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<Direct3DSurfaceDescription> {
        let this = self;
        unsafe {
            let mut result__: Direct3DSurfaceDescription = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Direct3DSurfaceDescription>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Graphics_DirectX_Direct3D11`, `Foundation`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IDirect3DSurface {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{0bf4a146-13c1-4694-bee3-7abf15eaf586}");
}
impl ::core::convert::From<IDirect3DSurface> for ::windows::runtime::IUnknown {
    fn from(value: IDirect3DSurface) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IDirect3DSurface> for ::windows::runtime::IUnknown {
    fn from(value: &IDirect3DSurface) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDirect3DSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDirect3DSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IDirect3DSurface> for ::windows::runtime::IInspectable {
    fn from(value: IDirect3DSurface) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IDirect3DSurface> for ::windows::runtime::IInspectable {
    fn from(value: &IDirect3DSurface) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IDirect3DSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IDirect3DSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IDirect3DSurface> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IDirect3DSurface) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IDirect3DSurface> for super::super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IDirect3DSurface) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for IDirect3DSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::super::Foundation::IClosable> for &IDirect3DSurface {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSurface_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut Direct3DSurfaceDescription) -> ::windows::runtime::HRESULT,
);
