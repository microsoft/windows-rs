#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub type BaseValueSource = i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const BaseValueSourceUnknown: BaseValueSource = 0i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const BaseValueSourceDefault: BaseValueSource = 1i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const BaseValueSourceBuiltInStyle: BaseValueSource = 2i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const BaseValueSourceStyle: BaseValueSource = 3i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const BaseValueSourceLocal: BaseValueSource = 4i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const Inherited: BaseValueSource = 5i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const DefaultStyleTrigger: BaseValueSource = 6i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const TemplateTrigger: BaseValueSource = 7i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const StyleTrigger: BaseValueSource = 8i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const ImplicitStyleReference: BaseValueSource = 9i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const ParentTemplate: BaseValueSource = 10i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const ParentTemplateTrigger: BaseValueSource = 11i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const Animation: BaseValueSource = 12i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const Coercion: BaseValueSource = 13i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const BaseValueSourceVisualState: BaseValueSource = 14i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Graphics_Dxgi_Common'*"]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct BitmapDescription {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub AlphaMode: super::super::super::Graphics::Dxgi::Common::DXGI_ALPHA_MODE,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for BitmapDescription {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for BitmapDescription {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for BitmapDescription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BitmapDescription").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("AlphaMode", &self.AlphaMode).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
unsafe impl ::windows::core::Abi for BitmapDescription {
    type Abi = Self;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for BitmapDescription {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BitmapDescription>()) == 0 }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for BitmapDescription {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for BitmapDescription {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CollectionElementValue {
    pub Index: u32,
    pub ValueType: super::super::super::Foundation::BSTR,
    pub Value: super::super::super::Foundation::BSTR,
    pub MetadataBits: i64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CollectionElementValue {
    fn clone(&self) -> Self {
        Self { Index: self.Index, ValueType: self.ValueType.clone(), Value: self.Value.clone(), MetadataBits: self.MetadataBits }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CollectionElementValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CollectionElementValue").field("Index", &self.Index).field("ValueType", &self.ValueType).field("Value", &self.Value).field("MetadataBits", &self.MetadataBits).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CollectionElementValue {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CollectionElementValue {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.ValueType == other.ValueType && self.Value == other.Value && self.MetadataBits == other.MetadataBits
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CollectionElementValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CollectionElementValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const E_UNKNOWNTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144665560i32);
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation', 'Win32_System_Com'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct EnumType {
    pub Name: super::super::super::Foundation::BSTR,
    pub ValueInts: *mut super::super::super::System::Com::SAFEARRAY,
    pub ValueStrings: *mut super::super::super::System::Com::SAFEARRAY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for EnumType {
    fn clone(&self) -> Self {
        Self { Name: self.Name.clone(), ValueInts: self.ValueInts, ValueStrings: self.ValueStrings }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for EnumType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("EnumType").field("Name", &self.Name).field("ValueInts", &self.ValueInts).field("ValueStrings", &self.ValueStrings).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for EnumType {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for EnumType {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.ValueInts == other.ValueInts && self.ValueStrings == other.ValueStrings
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for EnumType {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for EnumType {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
#[repr(transparent)]
pub struct IBitmapData(::windows::core::IUnknown);
impl IBitmapData {
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn CopyBytesTo(&self, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(sourceoffsetinbytes), ::core::mem::transmute(maxbytestocopy), ::core::mem::transmute(pvbytes), ::core::mem::transmute(numberofbytescopied)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetStride(&self) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Graphics_Dxgi_Common'*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetBitmapDescription(&self) -> ::windows::core::Result<BitmapDescription> {
        let mut result__: BitmapDescription = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<BitmapDescription>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Graphics_Dxgi_Common'*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetSourceBitmapDescription(&self) -> ::windows::core::Result<BitmapDescription> {
        let mut result__: BitmapDescription = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<BitmapDescription>(result__)
    }
}
impl ::core::convert::From<IBitmapData> for ::windows::core::IUnknown {
    fn from(value: IBitmapData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitmapData> for ::windows::core::IUnknown {
    fn from(value: &IBitmapData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IBitmapData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IBitmapData {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IBitmapData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IBitmapData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBitmapData {}
impl ::core::fmt::Debug for IBitmapData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBitmapData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IBitmapData {
    type Vtable = IBitmapDataVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1a34ef2_cad8_4635_a3d2_fcda8d3f3caf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapDataVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstride: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
#[repr(transparent)]
pub struct IVisualTreeService(::windows::core::IUnknown);
impl IVisualTreeService {
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn AdviseVisualTreeChange<'a, Param0: ::windows::core::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn UnadviseVisualTreeChange<'a, Param0: ::windows::core::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, typename: Param0, value: Param1) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), typename.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(value), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(startindex), ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(child), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent)).ok()
    }
}
impl ::core::convert::From<IVisualTreeService> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVisualTreeService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVisualTreeService {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVisualTreeService {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVisualTreeService {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVisualTreeService {}
impl ::core::fmt::Debug for IVisualTreeService {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVisualTreeService").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVisualTreeService {
    type Vtable = IVisualTreeServiceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa593b11a_d17f_48bb_8f66_83910731c8a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, propertyindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64, child: u64, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
#[repr(transparent)]
pub struct IVisualTreeService2(::windows::core::IUnknown);
impl IVisualTreeService2 {
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn AdviseVisualTreeChange<'a, Param0: ::windows::core::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn UnadviseVisualTreeChange<'a, Param0: ::windows::core::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, typename: Param0, value: Param1) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), typename.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(value), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(startindex), ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(child), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyIndex<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, object: u64, propertyname: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), propertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetProperty(&self, object: u64, propertyindex: u32) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(propertyindex), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcedictionary), ::core::mem::transmute(key), ::core::mem::transmute(newvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> ::windows::core::Result<IBitmapData> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(options), ::core::mem::transmute(maxpixelwidth), ::core::mem::transmute(maxpixelheight), ::core::mem::transmute(&mut result__)).from_abi::<IBitmapData>(result__)
    }
}
impl ::core::convert::From<IVisualTreeService2> for IVisualTreeService {
    fn from(value: IVisualTreeService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService2> for IVisualTreeService {
    fn from(value: &IVisualTreeService2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVisualTreeService> for IVisualTreeService2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVisualTreeService> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVisualTreeService> for &IVisualTreeService2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVisualTreeService> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVisualTreeService2> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService2> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeService2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVisualTreeService2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVisualTreeService2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVisualTreeService2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVisualTreeService2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVisualTreeService2 {}
impl ::core::fmt::Debug for IVisualTreeService2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVisualTreeService2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVisualTreeService2 {
    type Vtable = IVisualTreeService2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x130f5136_ec43_4f61_89c7_9801a36d2e95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, propertyindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64, child: u64, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: u64, propertyname: super::super::super::Foundation::PWSTR, ppropertyindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
#[repr(transparent)]
pub struct IVisualTreeService3(::windows::core::IUnknown);
impl IVisualTreeService3 {
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn AdviseVisualTreeChange<'a, Param0: ::windows::core::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn UnadviseVisualTreeChange<'a, Param0: ::windows::core::IntoParam<'a, IVisualTreeServiceCallback>>(&self, pcallback: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation', 'Win32_System_Com'*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, typename: Param0, value: Param1) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), typename.into_param().abi(), value.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(value), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(startindex), ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(child), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent), ::core::mem::transmute(index)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(parent)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyIndex<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, object: u64, propertyname: Param1) -> ::windows::core::Result<u32> {
        let mut result__: u32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), propertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetProperty(&self, object: u64, propertyindex: u32) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(object), ::core::mem::transmute(propertyindex), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcedictionary), ::core::mem::transmute(key), ::core::mem::transmute(newvalue)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> ::windows::core::Result<IBitmapData> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(handle), ::core::mem::transmute(options), ::core::mem::transmute(maxpixelwidth), ::core::mem::transmute(maxpixelheight), ::core::mem::transmute(&mut result__)).from_abi::<IBitmapData>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ResolveResource<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, resourcecontext: u64, resourcename: Param1, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(resourcecontext), resourcename.into_param().abi(), ::core::mem::transmute(resourcetype), ::core::mem::transmute(propertyindex)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDictionaryItem<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, dictionaryhandle: u64, resourcename: Param1, resourceisimplicitstyle: Param2) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(dictionaryhandle), resourcename.into_param().abi(), resourceisimplicitstyle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn AddDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).21)(::core::mem::transmute_copy(self), ::core::mem::transmute(dictionaryhandle), ::core::mem::transmute(resourcekey), ::core::mem::transmute(resourcehandle)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn RemoveDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).22)(::core::mem::transmute_copy(self), ::core::mem::transmute(dictionaryhandle), ::core::mem::transmute(resourcekey)).ok()
    }
}
impl ::core::convert::From<IVisualTreeService3> for IVisualTreeService2 {
    fn from(value: IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService3> for IVisualTreeService2 {
    fn from(value: &IVisualTreeService3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVisualTreeService2> for IVisualTreeService3 {
    fn into_param(self) -> ::windows::core::Param<'a, IVisualTreeService2> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVisualTreeService2> for &IVisualTreeService3 {
    fn into_param(self) -> ::windows::core::Param<'a, IVisualTreeService2> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVisualTreeService3> for IVisualTreeService {
    fn from(value: IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService3> for IVisualTreeService {
    fn from(value: &IVisualTreeService3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVisualTreeService> for IVisualTreeService3 {
    fn into_param(self) -> ::windows::core::Param<'a, IVisualTreeService> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVisualTreeService> for &IVisualTreeService3 {
    fn into_param(self) -> ::windows::core::Param<'a, IVisualTreeService> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVisualTreeService3> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService3> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeService3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVisualTreeService3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVisualTreeService3 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVisualTreeService3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVisualTreeService3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVisualTreeService3 {}
impl ::core::fmt::Debug for IVisualTreeService3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVisualTreeService3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVisualTreeService3 {
    type Vtable = IVisualTreeService3Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e79c6e0_85a0_4be8_b41a_655cf1fd19bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService3Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, propertyindex: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64, child: u64, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64, index: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: u64, propertyname: super::super::super::Foundation::PWSTR, ppropertyindex: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcecontext: u64, resourcename: super::super::super::Foundation::PWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcename: super::super::super::Foundation::PWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL, resourcehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
#[repr(transparent)]
pub struct IVisualTreeServiceCallback(::windows::core::IUnknown);
impl IVisualTreeServiceCallback {
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnVisualTreeChange<'a, Param0: ::windows::core::IntoParam<'a, ParentChildRelation>, Param1: ::windows::core::IntoParam<'a, VisualElement>>(&self, relation: Param0, element: Param1, mutationtype: VisualMutationType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), relation.into_param().abi(), element.into_param().abi(), ::core::mem::transmute(mutationtype)).ok()
    }
}
impl ::core::convert::From<IVisualTreeServiceCallback> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeServiceCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeServiceCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVisualTreeServiceCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVisualTreeServiceCallback {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVisualTreeServiceCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVisualTreeServiceCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVisualTreeServiceCallback {}
impl ::core::fmt::Debug for IVisualTreeServiceCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVisualTreeServiceCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVisualTreeServiceCallback {
    type Vtable = IVisualTreeServiceCallbackVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7a8931_80e4_4fec_8f3b_553f87b4966e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallbackVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
#[repr(transparent)]
pub struct IVisualTreeServiceCallback2(::windows::core::IUnknown);
impl IVisualTreeServiceCallback2 {
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnVisualTreeChange<'a, Param0: ::windows::core::IntoParam<'a, ParentChildRelation>, Param1: ::windows::core::IntoParam<'a, VisualElement>>(&self, relation: Param0, element: Param1, mutationtype: VisualMutationType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), relation.into_param().abi(), element.into_param().abi(), ::core::mem::transmute(mutationtype)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnElementStateChanged<'a, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(&self, element: u64, elementstate: VisualElementState, context: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(element), ::core::mem::transmute(elementstate), context.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IVisualTreeServiceCallback2> for IVisualTreeServiceCallback {
    fn from(value: IVisualTreeServiceCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback2> for IVisualTreeServiceCallback {
    fn from(value: &IVisualTreeServiceCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVisualTreeServiceCallback> for IVisualTreeServiceCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVisualTreeServiceCallback> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IVisualTreeServiceCallback> for &IVisualTreeServiceCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, IVisualTreeServiceCallback> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVisualTreeServiceCallback2> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeServiceCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback2> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeServiceCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVisualTreeServiceCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IVisualTreeServiceCallback2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVisualTreeServiceCallback2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVisualTreeServiceCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVisualTreeServiceCallback2 {}
impl ::core::fmt::Debug for IVisualTreeServiceCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVisualTreeServiceCallback2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IVisualTreeServiceCallback2 {
    type Vtable = IVisualTreeServiceCallback2Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad9eb88_ae77_4397_b948_5fa2db0a19ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback2Vtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: u64, elementstate: VisualElementState, context: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
#[repr(transparent)]
pub struct IXamlDiagnostics(::windows::core::IUnknown);
impl IXamlDiagnostics {
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetDispatcher(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IInspectable>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetUiLayer(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IInspectable>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IInspectable>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetIInspectableFromHandle(&self, instancehandle: u64) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(instancehandle), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IInspectable>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn GetHandleFromIInspectable<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, pinstance: Param0) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pinstance.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTest<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::RECT>>(&self, rect: Param0, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), rect.into_param().abi(), ::core::mem::transmute(pcount), ::core::mem::transmute(ppinstancehandles)).ok()
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
    pub unsafe fn RegisterInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, pinstance: Param0) -> ::windows::core::Result<u64> {
        let mut result__: u64 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pinstance.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInitializationData(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IXamlDiagnostics> for ::windows::core::IUnknown {
    fn from(value: IXamlDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlDiagnostics> for ::windows::core::IUnknown {
    fn from(value: &IXamlDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IXamlDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IXamlDiagnostics {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IXamlDiagnostics {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IXamlDiagnostics {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXamlDiagnostics {}
impl ::core::fmt::Debug for IXamlDiagnostics {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXamlDiagnostics").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IXamlDiagnostics {
    type Vtable = IXamlDiagnosticsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18c9e2b6_3f43_4116_9f2b_ff935d7770d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDiagnosticsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, ppinstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, phandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, pinstancehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitializationdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeXamlDiagnostic<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(endpointname: Param0, pid: u32, wszdllxamldiagnostics: Param2, wsztapdllname: Param3, tapclsid: Param4) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeXamlDiagnostic(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        InitializeXamlDiagnostic(endpointname.into_param().abi(), ::core::mem::transmute(pid), wszdllxamldiagnostics.into_param().abi(), wsztapdllname.into_param().abi(), tapclsid.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeXamlDiagnosticsEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::GUID>, Param5: ::windows::core::IntoParam<'a, super::super::super::Foundation::PWSTR>>(endpointname: Param0, pid: u32, wszdllxamldiagnostics: Param2, wsztapdllname: Param3, tapclsid: Param4, wszinitializationdata: Param5) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeXamlDiagnosticsEx(endpointname: super::super::super::Foundation::PWSTR, pid: u32, wszdllxamldiagnostics: super::super::super::Foundation::PWSTR, wsztapdllname: super::super::super::Foundation::PWSTR, tapclsid: ::windows::core::GUID, wszinitializationdata: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        InitializeXamlDiagnosticsEx(endpointname.into_param().abi(), ::core::mem::transmute(pid), wszdllxamldiagnostics.into_param().abi(), wsztapdllname.into_param().abi(), tapclsid.into_param().abi(), wszinitializationdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
#[repr(transparent)]
pub struct MetadataBit(pub i32);
impl MetadataBit {
    pub const None: Self = Self(0i32);
    pub const IsValueHandle: Self = Self(1i32);
    pub const IsPropertyReadOnly: Self = Self(2i32);
    pub const IsValueCollection: Self = Self(4i32);
    pub const IsValueCollectionReadOnly: Self = Self(8i32);
    pub const IsValueBindingExpression: Self = Self(16i32);
    pub const IsValueNull: Self = Self(32i32);
    pub const IsValueHandleAndEvaluatedValue: Self = Self(64i32);
}
impl ::core::marker::Copy for MetadataBit {}
impl ::core::clone::Clone for MetadataBit {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for MetadataBit {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MetadataBit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for MetadataBit {}
impl ::core::fmt::Debug for MetadataBit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MetadataBit").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub struct ParentChildRelation {
    pub Parent: u64,
    pub Child: u64,
    pub ChildIndex: u32,
}
impl ::core::marker::Copy for ParentChildRelation {}
impl ::core::clone::Clone for ParentChildRelation {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ParentChildRelation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ParentChildRelation").field("Parent", &self.Parent).field("Child", &self.Child).field("ChildIndex", &self.ChildIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for ParentChildRelation {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ParentChildRelation {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ParentChildRelation>()) == 0 }
    }
}
impl ::core::cmp::Eq for ParentChildRelation {}
impl ::core::default::Default for ParentChildRelation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainSource {
    pub Handle: u64,
    pub TargetType: super::super::super::Foundation::BSTR,
    pub Name: super::super::super::Foundation::BSTR,
    pub Source: BaseValueSource,
    pub SrcInfo: SourceInfo,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PropertyChainSource {
    fn clone(&self) -> Self {
        Self { Handle: self.Handle, TargetType: self.TargetType.clone(), Name: self.Name.clone(), Source: self.Source, SrcInfo: self.SrcInfo.clone() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PropertyChainSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PropertyChainSource").field("Handle", &self.Handle).field("TargetType", &self.TargetType).field("Name", &self.Name).field("Source", &self.Source).field("SrcInfo", &self.SrcInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PropertyChainSource {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PropertyChainSource {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.TargetType == other.TargetType && self.Name == other.Name && self.Source == other.Source && self.SrcInfo == other.SrcInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PropertyChainSource {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PropertyChainSource {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PropertyChainValue {
    pub Index: u32,
    pub Type: super::super::super::Foundation::BSTR,
    pub DeclaringType: super::super::super::Foundation::BSTR,
    pub ValueType: super::super::super::Foundation::BSTR,
    pub ItemType: super::super::super::Foundation::BSTR,
    pub Value: super::super::super::Foundation::BSTR,
    pub Overridden: super::super::super::Foundation::BOOL,
    pub MetadataBits: i64,
    pub PropertyName: super::super::super::Foundation::BSTR,
    pub PropertyChainIndex: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PropertyChainValue {
    fn clone(&self) -> Self {
        Self {
            Index: self.Index,
            Type: self.Type.clone(),
            DeclaringType: self.DeclaringType.clone(),
            ValueType: self.ValueType.clone(),
            ItemType: self.ItemType.clone(),
            Value: self.Value.clone(),
            Overridden: self.Overridden,
            MetadataBits: self.MetadataBits,
            PropertyName: self.PropertyName.clone(),
            PropertyChainIndex: self.PropertyChainIndex,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PropertyChainValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PropertyChainValue").field("Index", &self.Index).field("Type", &self.Type).field("DeclaringType", &self.DeclaringType).field("ValueType", &self.ValueType).field("ItemType", &self.ItemType).field("Value", &self.Value).field("Overridden", &self.Overridden).field("MetadataBits", &self.MetadataBits).field("PropertyName", &self.PropertyName).field("PropertyChainIndex", &self.PropertyChainIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PropertyChainValue {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PropertyChainValue {
    fn eq(&self, other: &Self) -> bool {
        self.Index == other.Index && self.Type == other.Type && self.DeclaringType == other.DeclaringType && self.ValueType == other.ValueType && self.ItemType == other.ItemType && self.Value == other.Value && self.Overridden == other.Overridden && self.MetadataBits == other.MetadataBits && self.PropertyName == other.PropertyName && self.PropertyChainIndex == other.PropertyChainIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PropertyChainValue {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PropertyChainValue {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub type RenderTargetBitmapOptions = i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const RenderTarget: RenderTargetBitmapOptions = 0i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = 1i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub type ResourceType = i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const ResourceTypeStatic: ResourceType = 0i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const ResourceTypeTheme: ResourceType = 1i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SourceInfo {
    pub FileName: super::super::super::Foundation::BSTR,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: super::super::super::Foundation::BSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SourceInfo {
    fn clone(&self) -> Self {
        Self {
            FileName: self.FileName.clone(),
            LineNumber: self.LineNumber,
            ColumnNumber: self.ColumnNumber,
            CharPosition: self.CharPosition,
            Hash: self.Hash.clone(),
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SourceInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SourceInfo").field("FileName", &self.FileName).field("LineNumber", &self.LineNumber).field("ColumnNumber", &self.ColumnNumber).field("CharPosition", &self.CharPosition).field("Hash", &self.Hash).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SourceInfo {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SourceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.FileName == other.FileName && self.LineNumber == other.LineNumber && self.ColumnNumber == other.ColumnNumber && self.CharPosition == other.CharPosition && self.Hash == other.Hash
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SourceInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SourceInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct VisualElement {
    pub Handle: u64,
    pub SrcInfo: SourceInfo,
    pub Type: super::super::super::Foundation::BSTR,
    pub Name: super::super::super::Foundation::BSTR,
    pub NumChildren: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for VisualElement {
    fn clone(&self) -> Self {
        Self { Handle: self.Handle, SrcInfo: self.SrcInfo.clone(), Type: self.Type.clone(), Name: self.Name.clone(), NumChildren: self.NumChildren }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for VisualElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VisualElement").field("Handle", &self.Handle).field("SrcInfo", &self.SrcInfo).field("Type", &self.Type).field("Name", &self.Name).field("NumChildren", &self.NumChildren).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for VisualElement {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for VisualElement {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.SrcInfo == other.SrcInfo && self.Type == other.Type && self.Name == other.Name && self.NumChildren == other.NumChildren
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for VisualElement {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for VisualElement {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub type VisualElementState = i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const ErrorResolved: VisualElementState = 0i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const ErrorResourceNotFound: VisualElementState = 1i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const ErrorInvalidResource: VisualElementState = 2i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub type VisualMutationType = i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const Add: VisualMutationType = 0i32;
#[doc = "*Required features: 'Win32_UI_Xaml_Diagnostics'*"]
pub const Remove: VisualMutationType = 1i32;
