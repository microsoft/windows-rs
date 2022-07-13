#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct BaseValueSource(pub i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceUnknown: BaseValueSource = BaseValueSource(0i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceDefault: BaseValueSource = BaseValueSource(1i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceBuiltInStyle: BaseValueSource = BaseValueSource(2i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceStyle: BaseValueSource = BaseValueSource(3i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceLocal: BaseValueSource = BaseValueSource(4i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Inherited: BaseValueSource = BaseValueSource(5i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const DefaultStyleTrigger: BaseValueSource = BaseValueSource(6i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const TemplateTrigger: BaseValueSource = BaseValueSource(7i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const StyleTrigger: BaseValueSource = BaseValueSource(8i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ImplicitStyleReference: BaseValueSource = BaseValueSource(9i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ParentTemplate: BaseValueSource = BaseValueSource(10i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ParentTemplateTrigger: BaseValueSource = BaseValueSource(11i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Animation: BaseValueSource = BaseValueSource(12i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Coercion: BaseValueSource = BaseValueSource(13i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const BaseValueSourceVisualState: BaseValueSource = BaseValueSource(14i32);
impl ::core::marker::Copy for BaseValueSource {}
impl ::core::clone::Clone for BaseValueSource {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BaseValueSource {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BaseValueSource {
    type Abi = Self;
}
impl ::core::fmt::Debug for BaseValueSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BaseValueSource").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const E_UNKNOWNTYPE: ::windows::core::HRESULT = ::windows::core::HRESULT(-2144665560i32);
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IBitmapData(::windows::core::IUnknown);
impl IBitmapData {
    pub unsafe fn CopyBytesTo(&self, sourceoffsetinbytes: u32, pvbytes: &mut [u8], numberofbytescopied: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyBytesTo)(::windows::core::Interface::as_raw(self), sourceoffsetinbytes, pvbytes.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pvbytes)), ::core::mem::transmute(numberofbytescopied)).ok()
    }
    pub unsafe fn GetStride(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetStride)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetBitmapDescription(&self) -> ::windows::core::Result<BitmapDescription> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetBitmapDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BitmapDescription>(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetSourceBitmapDescription(&self) -> ::windows::core::Result<BitmapDescription> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceBitmapDescription)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<BitmapDescription>(result__)
    }
}
impl ::core::convert::From<IBitmapData> for ::windows::core::IUnknown {
    fn from(value: IBitmapData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IBitmapData> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IBitmapData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IBitmapData> for ::windows::core::IUnknown {
    fn from(value: &IBitmapData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    type Vtable = IBitmapData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1a34ef2_cad8_4635_a3d2_fcda8d3f3caf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapData_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub CopyBytesTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> ::windows::core::HRESULT,
    pub GetStride: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstride: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetBitmapDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetBitmapDescription: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetSourceBitmapDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetSourceBitmapDescription: usize,
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IVisualTreeService(::windows::core::IUnknown);
impl IVisualTreeService {
    pub unsafe fn AdviseVisualTreeChange<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IVisualTreeServiceCallback>>,
    {
        (::windows::core::Interface::vtable(self).AdviseVisualTreeChange)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
    pub unsafe fn UnadviseVisualTreeChange<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IVisualTreeServiceCallback>>,
    {
        (::windows::core::Interface::vtable(self).UnadviseVisualTreeChange)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetEnums)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstance<'a, P0, P1>(&self, typename: P0, value: P1) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateInstance)(::windows::core::Interface::as_raw(self), typename.into().abi(), value.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetPropertyValuesChain)(::windows::core::Interface::as_raw(self), instancehandle, ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetProperty)(::windows::core::Interface::as_raw(self), instancehandle, value, propertyindex).ok()
    }
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearProperty)(::windows::core::Interface::as_raw(self), instancehandle, propertyindex).ok()
    }
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetCollectionCount)(::windows::core::Interface::as_raw(self), instancehandle, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCollectionElements)(::windows::core::Interface::as_raw(self), instancehandle, startindex, ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddChild)(::windows::core::Interface::as_raw(self), parent, child, index).ok()
    }
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveChild)(::windows::core::Interface::as_raw(self), parent, index).ok()
    }
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ClearChildren)(::windows::core::Interface::as_raw(self), parent).ok()
    }
}
impl ::core::convert::From<IVisualTreeService> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeService> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVisualTreeService) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeService) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    type Vtable = IVisualTreeService_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa593b11a_d17f_48bb_8f66_83910731c8a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AdviseVisualTreeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UnadviseVisualTreeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub GetEnums: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    GetEnums: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, typename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinstancehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CreateInstance: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetPropertyValuesChain: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetPropertyValuesChain: usize,
    pub SetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::HRESULT,
    pub ClearProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, propertyindex: u32) -> ::windows::core::HRESULT,
    pub GetCollectionCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, pcollectionsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetCollectionElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetCollectionElements: usize,
    pub AddChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64, child: u64, index: u32) -> ::windows::core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64, index: u32) -> ::windows::core::HRESULT,
    pub ClearChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, parent: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IVisualTreeService2(::windows::core::IUnknown);
impl IVisualTreeService2 {
    pub unsafe fn AdviseVisualTreeChange<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IVisualTreeServiceCallback>>,
    {
        (::windows::core::Interface::vtable(self).base__.AdviseVisualTreeChange)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
    pub unsafe fn UnadviseVisualTreeChange<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IVisualTreeServiceCallback>>,
    {
        (::windows::core::Interface::vtable(self).base__.UnadviseVisualTreeChange)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetEnums)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstance<'a, P0, P1>(&self, typename: P0, value: P1) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.CreateInstance)(::windows::core::Interface::as_raw(self), typename.into().abi(), value.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetPropertyValuesChain)(::windows::core::Interface::as_raw(self), instancehandle, ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetProperty)(::windows::core::Interface::as_raw(self), instancehandle, value, propertyindex).ok()
    }
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ClearProperty)(::windows::core::Interface::as_raw(self), instancehandle, propertyindex).ok()
    }
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCollectionCount)(::windows::core::Interface::as_raw(self), instancehandle, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetCollectionElements)(::windows::core::Interface::as_raw(self), instancehandle, startindex, ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddChild)(::windows::core::Interface::as_raw(self), parent, child, index).ok()
    }
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.RemoveChild)(::windows::core::Interface::as_raw(self), parent, index).ok()
    }
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ClearChildren)(::windows::core::Interface::as_raw(self), parent).ok()
    }
    pub unsafe fn GetPropertyIndex<'a, P0>(&self, object: u64, propertyname: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPropertyIndex)(::windows::core::Interface::as_raw(self), object, propertyname.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProperty(&self, object: u64, propertyindex: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), object, propertyindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ReplaceResource)(::windows::core::Interface::as_raw(self), resourcedictionary, key, newvalue).ok()
    }
    pub unsafe fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> ::windows::core::Result<IBitmapData> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RenderTargetBitmap)(::windows::core::Interface::as_raw(self), handle, options, maxpixelwidth, maxpixelheight, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBitmapData>(result__)
    }
}
impl ::core::convert::From<IVisualTreeService2> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeService2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVisualTreeService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService2> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeService2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVisualTreeService2> for IVisualTreeService {
    fn from(value: IVisualTreeService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeService2> for &'a IVisualTreeService {
    fn from(value: &'a IVisualTreeService2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService2> for IVisualTreeService {
    fn from(value: &IVisualTreeService2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    type Vtable = IVisualTreeService2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x130f5136_ec43_4f61_89c7_9801a36d2e95);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService2_Vtbl {
    pub base__: IVisualTreeService_Vtbl,
    pub GetPropertyIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: u64, propertyname: ::windows::core::PCWSTR, ppropertyindex: *mut u32) -> ::windows::core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, object: u64, propertyindex: u32, pvalue: *mut u64) -> ::windows::core::HRESULT,
    pub ReplaceResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::HRESULT,
    pub RenderTargetBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IVisualTreeService3(::windows::core::IUnknown);
impl IVisualTreeService3 {
    pub unsafe fn AdviseVisualTreeChange<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IVisualTreeServiceCallback>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.AdviseVisualTreeChange)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
    pub unsafe fn UnadviseVisualTreeChange<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IVisualTreeServiceCallback>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.UnadviseVisualTreeChange)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetEnums)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcount), ::core::mem::transmute(ppenums)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateInstance<'a, P0, P1>(&self, typename: P0, value: P1) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, super::super::super::Foundation::BSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.CreateInstance)(::windows::core::Interface::as_raw(self), typename.into().abi(), value.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetPropertyValuesChain)(::windows::core::Interface::as_raw(self), instancehandle, ::core::mem::transmute(psourcecount), ::core::mem::transmute(pppropertysources), ::core::mem::transmute(ppropertycount), ::core::mem::transmute(pppropertyvalues)).ok()
    }
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetProperty)(::windows::core::Interface::as_raw(self), instancehandle, value, propertyindex).ok()
    }
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ClearProperty)(::windows::core::Interface::as_raw(self), instancehandle, propertyindex).ok()
    }
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetCollectionCount)(::windows::core::Interface::as_raw(self), instancehandle, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetCollectionElements)(::windows::core::Interface::as_raw(self), instancehandle, startindex, ::core::mem::transmute(pelementcount), ::core::mem::transmute(ppelementvalues)).ok()
    }
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.AddChild)(::windows::core::Interface::as_raw(self), parent, child, index).ok()
    }
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.RemoveChild)(::windows::core::Interface::as_raw(self), parent, index).ok()
    }
    pub unsafe fn ClearChildren(&self, parent: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.ClearChildren)(::windows::core::Interface::as_raw(self), parent).ok()
    }
    pub unsafe fn GetPropertyIndex<'a, P0>(&self, object: u64, propertyname: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPropertyIndex)(::windows::core::Interface::as_raw(self), object, propertyname.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn GetProperty(&self, object: u64, propertyindex: u32) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetProperty)(::windows::core::Interface::as_raw(self), object, propertyindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ReplaceResource)(::windows::core::Interface::as_raw(self), resourcedictionary, key, newvalue).ok()
    }
    pub unsafe fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> ::windows::core::Result<IBitmapData> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.RenderTargetBitmap)(::windows::core::Interface::as_raw(self), handle, options, maxpixelwidth, maxpixelheight, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IBitmapData>(result__)
    }
    pub unsafe fn ResolveResource<'a, P0>(&self, resourcecontext: u64, resourcename: P0, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ResolveResource)(::windows::core::Interface::as_raw(self), resourcecontext, resourcename.into(), resourcetype, propertyindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDictionaryItem<'a, P0, P1>(&self, dictionaryhandle: u64, resourcename: P0, resourceisimplicitstyle: P1) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionaryItem)(::windows::core::Interface::as_raw(self), dictionaryhandle, resourcename.into(), resourceisimplicitstyle.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    pub unsafe fn AddDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddDictionaryItem)(::windows::core::Interface::as_raw(self), dictionaryhandle, resourcekey, resourcehandle).ok()
    }
    pub unsafe fn RemoveDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveDictionaryItem)(::windows::core::Interface::as_raw(self), dictionaryhandle, resourcekey).ok()
    }
}
impl ::core::convert::From<IVisualTreeService3> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeService3> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService3> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeService3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVisualTreeService3> for IVisualTreeService {
    fn from(value: IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeService3> for &'a IVisualTreeService {
    fn from(value: &'a IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService3> for IVisualTreeService {
    fn from(value: &IVisualTreeService3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVisualTreeService3> for IVisualTreeService2 {
    fn from(value: IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeService3> for &'a IVisualTreeService2 {
    fn from(value: &'a IVisualTreeService3) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeService3> for IVisualTreeService2 {
    fn from(value: &IVisualTreeService3) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    type Vtable = IVisualTreeService3_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0e79c6e0_85a0_4be8_b41a_655cf1fd19bd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService3_Vtbl {
    pub base__: IVisualTreeService2_Vtbl,
    pub ResolveResource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourcecontext: u64, resourcename: ::windows::core::PCWSTR, resourcetype: ResourceType, propertyindex: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDictionaryItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcename: ::windows::core::PCWSTR, resourceisimplicitstyle: super::super::super::Foundation::BOOL, resourcehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDictionaryItem: usize,
    pub AddDictionaryItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> ::windows::core::HRESULT,
    pub RemoveDictionaryItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IVisualTreeServiceCallback(::windows::core::IUnknown);
impl IVisualTreeServiceCallback {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnVisualTreeChange<'a, P0>(&self, relation: ParentChildRelation, element: P0, mutationtype: VisualMutationType) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, VisualElement>>,
    {
        (::windows::core::Interface::vtable(self).OnVisualTreeChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(relation), element.into().abi(), mutationtype).ok()
    }
}
impl ::core::convert::From<IVisualTreeServiceCallback> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeServiceCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeServiceCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVisualTreeServiceCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeServiceCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    type Vtable = IVisualTreeServiceCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xaa7a8931_80e4_4fec_8f3b_553f87b4966e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnVisualTreeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, relation: ParentChildRelation, element: ::core::mem::ManuallyDrop<VisualElement>, mutationtype: VisualMutationType) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnVisualTreeChange: usize,
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IVisualTreeServiceCallback2(::windows::core::IUnknown);
impl IVisualTreeServiceCallback2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnVisualTreeChange<'a, P0>(&self, relation: ParentChildRelation, element: P0, mutationtype: VisualMutationType) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, VisualElement>>,
    {
        (::windows::core::Interface::vtable(self).base__.OnVisualTreeChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(relation), element.into().abi(), mutationtype).ok()
    }
    pub unsafe fn OnElementStateChanged<'a, P0>(&self, element: u64, elementstate: VisualElementState, context: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).OnElementStateChanged)(::windows::core::Interface::as_raw(self), element, elementstate, context.into()).ok()
    }
}
impl ::core::convert::From<IVisualTreeServiceCallback2> for ::windows::core::IUnknown {
    fn from(value: IVisualTreeServiceCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeServiceCallback2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IVisualTreeServiceCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback2> for ::windows::core::IUnknown {
    fn from(value: &IVisualTreeServiceCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IVisualTreeServiceCallback2> for IVisualTreeServiceCallback {
    fn from(value: IVisualTreeServiceCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IVisualTreeServiceCallback2> for &'a IVisualTreeServiceCallback {
    fn from(value: &'a IVisualTreeServiceCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVisualTreeServiceCallback2> for IVisualTreeServiceCallback {
    fn from(value: &IVisualTreeServiceCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    type Vtable = IVisualTreeServiceCallback2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbad9eb88_ae77_4397_b948_5fa2db0a19ea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback2_Vtbl {
    pub base__: IVisualTreeServiceCallback_Vtbl,
    pub OnElementStateChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, element: u64, elementstate: VisualElementState, context: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
pub struct IXamlDiagnostics(::windows::core::IUnknown);
impl IXamlDiagnostics {
    pub unsafe fn GetDispatcher(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDispatcher)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IInspectable>(result__)
    }
    pub unsafe fn GetUiLayer(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetUiLayer)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IInspectable>(result__)
    }
    pub unsafe fn GetApplication(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetApplication)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IInspectable>(result__)
    }
    pub unsafe fn GetIInspectableFromHandle(&self, instancehandle: u64) -> ::windows::core::Result<::windows::core::IInspectable> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetIInspectableFromHandle)(::windows::core::Interface::as_raw(self), instancehandle, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IInspectable>(result__)
    }
    pub unsafe fn GetHandleFromIInspectable<'a, P0>(&self, pinstance: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetHandleFromIInspectable)(::windows::core::Interface::as_raw(self), pinstance.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn HitTest(&self, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HitTest)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(rect), ::core::mem::transmute(pcount), ::core::mem::transmute(ppinstancehandles)).ok()
    }
    pub unsafe fn RegisterInstance<'a, P0>(&self, pinstance: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IInspectable>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).RegisterInstance)(::windows::core::Interface::as_raw(self), pinstance.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u64>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetInitializationData(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInitializationData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
}
impl ::core::convert::From<IXamlDiagnostics> for ::windows::core::IUnknown {
    fn from(value: IXamlDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IXamlDiagnostics> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IXamlDiagnostics) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IXamlDiagnostics> for ::windows::core::IUnknown {
    fn from(value: &IXamlDiagnostics) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
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
    type Vtable = IXamlDiagnostics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x18c9e2b6_3f43_4116_9f2b_ff935d7770d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDiagnostics_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetDispatcher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdispatcher: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUiLayer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplayer: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIInspectableFromHandle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, instancehandle: u64, ppinstance: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHandleFromIInspectable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, phandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub HitTest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    HitTest: usize,
    pub RegisterInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinstance: *mut ::core::ffi::c_void, pinstancehandle: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetInitializationData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinitializationdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetInitializationData: usize,
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[inline]
pub unsafe fn InitializeXamlDiagnostic<'a, P0, P1, P2>(endpointname: P0, pid: u32, wszdllxamldiagnostics: P1, wsztapdllname: P2, tapclsid: ::windows::core::GUID) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitializeXamlDiagnostic(endpointname: ::windows::core::PCWSTR, pid: u32, wszdllxamldiagnostics: ::windows::core::PCWSTR, wsztapdllname: ::windows::core::PCWSTR, tapclsid: ::windows::core::GUID) -> ::windows::core::HRESULT;
    }
    InitializeXamlDiagnostic(endpointname.into(), pid, wszdllxamldiagnostics.into(), wsztapdllname.into(), ::core::mem::transmute(tapclsid)).ok()
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[inline]
pub unsafe fn InitializeXamlDiagnosticsEx<'a, P0, P1, P2, P3>(endpointname: P0, pid: u32, wszdllxamldiagnostics: P1, wsztapdllname: P2, tapclsid: ::windows::core::GUID, wszinitializationdata: P3) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn InitializeXamlDiagnosticsEx(endpointname: ::windows::core::PCWSTR, pid: u32, wszdllxamldiagnostics: ::windows::core::PCWSTR, wsztapdllname: ::windows::core::PCWSTR, tapclsid: ::windows::core::GUID, wszinitializationdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    InitializeXamlDiagnosticsEx(endpointname.into(), pid, wszdllxamldiagnostics.into(), wsztapdllname.into(), ::core::mem::transmute(tapclsid), wszinitializationdata.into()).ok()
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
impl ::core::default::Default for MetadataBit {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MetadataBit {
    type Abi = Self;
}
impl ::core::fmt::Debug for MetadataBit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MetadataBit").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RenderTargetBitmapOptions(pub i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const RenderTarget: RenderTargetBitmapOptions = RenderTargetBitmapOptions(0i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = RenderTargetBitmapOptions(1i32);
impl ::core::marker::Copy for RenderTargetBitmapOptions {}
impl ::core::clone::Clone for RenderTargetBitmapOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RenderTargetBitmapOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RenderTargetBitmapOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for RenderTargetBitmapOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RenderTargetBitmapOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ResourceType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ResourceTypeStatic: ResourceType = ResourceType(0i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ResourceTypeTheme: ResourceType = ResourceType(1i32);
impl ::core::marker::Copy for ResourceType {}
impl ::core::clone::Clone for ResourceType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ResourceType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ResourceType {
    type Abi = Self;
}
impl ::core::fmt::Debug for ResourceType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ResourceType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VisualElementState(pub i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorResolved: VisualElementState = VisualElementState(0i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorResourceNotFound: VisualElementState = VisualElementState(1i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const ErrorInvalidResource: VisualElementState = VisualElementState(2i32);
impl ::core::marker::Copy for VisualElementState {}
impl ::core::clone::Clone for VisualElementState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisualElementState {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VisualElementState {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisualElementState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualElementState").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct VisualMutationType(pub i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Add: VisualMutationType = VisualMutationType(0i32);
#[doc = "*Required features: `\"Win32_UI_Xaml_Diagnostics\"`*"]
pub const Remove: VisualMutationType = VisualMutationType(1i32);
impl ::core::marker::Copy for VisualMutationType {}
impl ::core::clone::Clone for VisualMutationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VisualMutationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VisualMutationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for VisualMutationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VisualMutationType").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
