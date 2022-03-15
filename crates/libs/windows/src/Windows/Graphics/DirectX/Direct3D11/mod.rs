#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Direct3DBindings(pub u32);
impl Direct3DBindings {
    pub const VertexBuffer: Self = Self(1u32);
    pub const IndexBuffer: Self = Self(2u32);
    pub const ConstantBuffer: Self = Self(4u32);
    pub const ShaderResource: Self = Self(8u32);
    pub const StreamOutput: Self = Self(16u32);
    pub const RenderTarget: Self = Self(32u32);
    pub const DepthStencil: Self = Self(64u32);
    pub const UnorderedAccess: Self = Self(128u32);
    pub const Decoder: Self = Self(512u32);
    pub const VideoEncoder: Self = Self(1024u32);
}
impl ::core::marker::Copy for Direct3DBindings {}
impl ::core::clone::Clone for Direct3DBindings {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Direct3DBindings {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Direct3DBindings {
    type Abi = Self;
}
impl ::core::fmt::Debug for Direct3DBindings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direct3DBindings").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for Direct3DBindings {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for Direct3DBindings {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for Direct3DBindings {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for Direct3DBindings {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for Direct3DBindings {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
unsafe impl ::windows::core::RuntimeType for Direct3DBindings {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.Direct3D11.Direct3DBindings;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
pub struct Direct3DMultisampleDescription {
    pub Count: i32,
    pub Quality: i32,
}
impl ::core::marker::Copy for Direct3DMultisampleDescription {}
impl ::core::clone::Clone for Direct3DMultisampleDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Direct3DMultisampleDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Direct3DMultisampleDescription").field("Count", &self.Count).field("Quality", &self.Quality).finish()
    }
}
unsafe impl ::windows::core::Abi for Direct3DMultisampleDescription {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Direct3DMultisampleDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Direct3DMultisampleDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Direct3DMultisampleDescription>()) == 0 }
    }
}
impl ::core::cmp::Eq for Direct3DMultisampleDescription {}
impl ::core::default::Default for Direct3DMultisampleDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
pub struct Direct3DSurfaceDescription {
    pub Width: i32,
    pub Height: i32,
    pub Format: super::DirectXPixelFormat,
    pub MultisampleDescription: Direct3DMultisampleDescription,
}
impl ::core::marker::Copy for Direct3DSurfaceDescription {}
impl ::core::clone::Clone for Direct3DSurfaceDescription {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for Direct3DSurfaceDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Direct3DSurfaceDescription").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("MultisampleDescription", &self.MultisampleDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for Direct3DSurfaceDescription {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for Direct3DSurfaceDescription {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.DirectX.Direct3D11.Direct3DSurfaceDescription;i4;i4;enum(Windows.Graphics.DirectX.DirectXPixelFormat;i4);struct(Windows.Graphics.DirectX.Direct3D11.Direct3DMultisampleDescription;i4;i4))");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for Direct3DSurfaceDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<Direct3DSurfaceDescription>()) == 0 }
    }
}
impl ::core::cmp::Eq for Direct3DSurfaceDescription {}
impl ::core::default::Default for Direct3DSurfaceDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct Direct3DUsage(pub i32);
impl Direct3DUsage {
    pub const Default: Self = Self(0i32);
    pub const Immutable: Self = Self(1i32);
    pub const Dynamic: Self = Self(2i32);
    pub const Staging: Self = Self(3i32);
}
impl ::core::marker::Copy for Direct3DUsage {}
impl ::core::clone::Clone for Direct3DUsage {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for Direct3DUsage {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for Direct3DUsage {
    type Abi = Self;
}
impl ::core::fmt::Debug for Direct3DUsage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Direct3DUsage").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Direct3DUsage {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.Graphics.DirectX.Direct3D11.Direct3DUsage;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
#[repr(transparent)]
pub struct IDirect3DDevice(::windows::core::IUnknown);
impl IDirect3DDevice {
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    pub fn Trim(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Trim)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IDirect3DDevice> for ::windows::core::IUnknown {
    fn from(value: IDirect3DDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDevice> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DDevice> for ::windows::core::IInspectable {
    fn from(value: IDirect3DDevice) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DDevice> for ::windows::core::IInspectable {
    fn from(value: &IDirect3DDevice) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDirect3DDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDirect3DDevice {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IDirect3DDevice> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IDirect3DDevice) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IDirect3DDevice> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDirect3DDevice) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for IDirect3DDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &IDirect3DDevice {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IDirect3DDevice {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DDevice {}
impl ::core::fmt::Debug for IDirect3DDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DDevice").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDirect3DDevice {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a37624ab-8d5f-4650-9d3e-9eae3d9bc670}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDirect3DDevice {
    type Vtable = IDirect3DDevice_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa37624ab_8d5f_4650_9d3e_9eae3d9bc670);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DDevice_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Trim: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
#[repr(transparent)]
pub struct IDirect3DSurface(::windows::core::IUnknown);
impl IDirect3DSurface {
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`*"]
    pub fn Description(&self) -> ::windows::core::Result<Direct3DSurfaceDescription> {
        let this = self;
        unsafe {
            let mut result__: Direct3DSurfaceDescription = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Description)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Direct3DSurfaceDescription>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_DirectX_Direct3D11\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IDirect3DSurface> for ::windows::core::IUnknown {
    fn from(value: IDirect3DSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DSurface> for ::windows::core::IUnknown {
    fn from(value: &IDirect3DSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDirect3DSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDirect3DSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDirect3DSurface> for ::windows::core::IInspectable {
    fn from(value: IDirect3DSurface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDirect3DSurface> for ::windows::core::IInspectable {
    fn from(value: &IDirect3DSurface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDirect3DSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDirect3DSurface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IDirect3DSurface> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IDirect3DSurface) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IDirect3DSurface> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IDirect3DSurface) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for IDirect3DSurface {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &IDirect3DSurface {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IDirect3DSurface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDirect3DSurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDirect3DSurface {}
impl ::core::fmt::Debug for IDirect3DSurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDirect3DSurface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDirect3DSurface {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0bf4a146-13c1-4694-bee3-7abf15eaf586}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDirect3DSurface {
    type Vtable = IDirect3DSurface_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0bf4a146_13c1_4694_bee3_7abf15eaf586);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDirect3DSurface_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut Direct3DSurfaceDescription) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
