#[cfg(feature = "Graphics_Capture")]
pub mod Capture;
#[cfg(feature = "Graphics_DirectX")]
pub mod DirectX;
#[cfg(feature = "Graphics_Display")]
pub mod Display;
#[cfg(feature = "Graphics_Effects")]
pub mod Effects;
#[cfg(feature = "Graphics_Holographic")]
pub mod Holographic;
#[cfg(feature = "Graphics_Imaging")]
pub mod Imaging;
#[cfg(feature = "Graphics_Printing")]
pub mod Printing;
#[cfg(feature = "Graphics_Printing3D")]
pub mod Printing3D;
#[repr(C)]
#[doc = "*Required features: `\"Graphics\"`*"]
pub struct DisplayAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for DisplayAdapterId {}
impl ::core::clone::Clone for DisplayAdapterId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DisplayAdapterId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DisplayAdapterId").field("LowPart", &self.LowPart).field("HighPart", &self.HighPart).finish()
    }
}
unsafe impl ::windows::core::Abi for DisplayAdapterId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayAdapterId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.DisplayAdapterId;u4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for DisplayAdapterId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DisplayAdapterId>()) == 0 }
    }
}
impl ::core::cmp::Eq for DisplayAdapterId {}
impl ::core::default::Default for DisplayAdapterId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics\"`*"]
pub struct DisplayId {
    pub Value: u64,
}
impl ::core::marker::Copy for DisplayId {}
impl ::core::clone::Clone for DisplayId {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DisplayId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DisplayId").field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for DisplayId {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DisplayId {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.DisplayId;u8)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for DisplayId {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DisplayId>()) == 0 }
    }
}
impl ::core::cmp::Eq for DisplayId {}
impl ::core::default::Default for DisplayId {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Graphics\"`*"]
#[repr(transparent)]
pub struct IGeometrySource2D(::windows::core::IUnknown);
impl IGeometrySource2D {}
impl ::core::convert::From<IGeometrySource2D> for ::windows::core::IUnknown {
    fn from(value: IGeometrySource2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGeometrySource2D> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IGeometrySource2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGeometrySource2D> for ::windows::core::IUnknown {
    fn from(value: &IGeometrySource2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IGeometrySource2D> for ::windows::core::IInspectable {
    fn from(value: IGeometrySource2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IGeometrySource2D> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IGeometrySource2D) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGeometrySource2D> for ::windows::core::IInspectable {
    fn from(value: &IGeometrySource2D) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IGeometrySource2D {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGeometrySource2D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGeometrySource2D {}
impl ::core::fmt::Debug for IGeometrySource2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGeometrySource2D").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGeometrySource2D {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{caff7902-670c-4181-a624-da977203b845}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGeometrySource2D {
    type Vtable = IGeometrySource2D_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcaff7902_670c_4181_a624_da977203b845);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2D_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics\"`*"]
pub struct PointInt32 {
    pub X: i32,
    pub Y: i32,
}
impl ::core::marker::Copy for PointInt32 {}
impl ::core::clone::Clone for PointInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PointInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PointInt32").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
unsafe impl ::windows::core::Abi for PointInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for PointInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.PointInt32;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for PointInt32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PointInt32>()) == 0 }
    }
}
impl ::core::cmp::Eq for PointInt32 {}
impl ::core::default::Default for PointInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics\"`*"]
pub struct RectInt32 {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl ::core::marker::Copy for RectInt32 {}
impl ::core::clone::Clone for RectInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RectInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RectInt32").field("X", &self.X).field("Y", &self.Y).field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for RectInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for RectInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.RectInt32;i4;i4;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for RectInt32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RectInt32>()) == 0 }
    }
}
impl ::core::cmp::Eq for RectInt32 {}
impl ::core::default::Default for RectInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Graphics\"`*"]
pub struct SizeInt32 {
    pub Width: i32,
    pub Height: i32,
}
impl ::core::marker::Copy for SizeInt32 {}
impl ::core::clone::Clone for SizeInt32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SizeInt32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SizeInt32").field("Width", &self.Width).field("Height", &self.Height).finish()
    }
}
unsafe impl ::windows::core::Abi for SizeInt32 {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for SizeInt32 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.Graphics.SizeInt32;i4;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for SizeInt32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SizeInt32>()) == 0 }
    }
}
impl ::core::cmp::Eq for SizeInt32 {}
impl ::core::default::Default for SizeInt32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
