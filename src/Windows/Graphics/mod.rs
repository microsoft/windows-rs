#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
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
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DisplayAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl DisplayAdapterId {}
impl ::std::default::Default for DisplayAdapterId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DisplayAdapterId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DisplayAdapterId")
            .field("LowPart", &self.LowPart)
            .field("HighPart", &self.HighPart)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DisplayAdapterId {
    fn eq(&self, other: &Self) -> bool {
        self.LowPart == other.LowPart && self.HighPart == other.HighPart
    }
}
impl ::std::cmp::Eq for DisplayAdapterId {}
unsafe impl ::windows::runtime::Abi for DisplayAdapterId {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayAdapterId {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Windows.Graphics.DisplayAdapterId;u4;i4)",
    );
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct DisplayId {
    pub Value: u64,
}
impl DisplayId {}
impl ::std::default::Default for DisplayId {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for DisplayId {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DisplayId")
            .field("Value", &self.Value)
            .finish()
    }
}
impl ::std::cmp::PartialEq for DisplayId {
    fn eq(&self, other: &Self) -> bool {
        self.Value == other.Value
    }
}
impl ::std::cmp::Eq for DisplayId {}
unsafe impl ::windows::runtime::Abi for DisplayId {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DisplayId {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.DisplayId;u8)");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IGeometrySource2D(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IGeometrySource2D {
    type Vtable = IGeometrySource2D_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3405740290,
        26380,
        16769,
        [166, 36, 218, 151, 114, 3, 184, 69],
    );
}
impl IGeometrySource2D {}
unsafe impl ::windows::runtime::RuntimeType for IGeometrySource2D {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{caff7902-670c-4181-a624-da977203b845}");
}
impl ::std::convert::From<IGeometrySource2D> for ::windows::runtime::IUnknown {
    fn from(value: IGeometrySource2D) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IGeometrySource2D> for ::windows::runtime::IUnknown {
    fn from(value: &IGeometrySource2D) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IGeometrySource2D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IGeometrySource2D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IGeometrySource2D> for ::windows::runtime::IInspectable {
    fn from(value: IGeometrySource2D) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IGeometrySource2D> for ::windows::runtime::IInspectable {
    fn from(value: &IGeometrySource2D) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IGeometrySource2D {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IGeometrySource2D
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGeometrySource2D_abi(
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
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PointInt32 {
    pub X: i32,
    pub Y: i32,
}
impl PointInt32 {}
impl ::std::default::Default for PointInt32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PointInt32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PointInt32")
            .field("X", &self.X)
            .field("Y", &self.Y)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PointInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::std::cmp::Eq for PointInt32 {}
unsafe impl ::windows::runtime::Abi for PointInt32 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PointInt32 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.PointInt32;i4;i4)");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RectInt32 {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl RectInt32 {}
impl ::std::default::Default for RectInt32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RectInt32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RectInt32")
            .field("X", &self.X)
            .field("Y", &self.Y)
            .field("Width", &self.Width)
            .field("Height", &self.Height)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RectInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X
            && self.Y == other.Y
            && self.Width == other.Width
            && self.Height == other.Height
    }
}
impl ::std::cmp::Eq for RectInt32 {}
unsafe impl ::windows::runtime::Abi for RectInt32 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RectInt32 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"struct(Windows.Graphics.RectInt32;i4;i4;i4;i4)",
    );
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SizeInt32 {
    pub Width: i32,
    pub Height: i32,
}
impl SizeInt32 {}
impl ::std::default::Default for SizeInt32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SizeInt32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SizeInt32")
            .field("Width", &self.Width)
            .field("Height", &self.Height)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SizeInt32 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height
    }
}
impl ::std::cmp::Eq for SizeInt32 {}
unsafe impl ::windows::runtime::Abi for SizeInt32 {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for SizeInt32 {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"struct(Windows.Graphics.SizeInt32;i4;i4)");
}
