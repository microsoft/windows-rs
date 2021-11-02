#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlDirect(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlDirect {
    type Vtable = IXamlDirect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1610224277, 44498, 22799, [160, 81, 112, 152, 155, 134, 106, 222]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDirect_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, object: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, typeindex: XamlTypeIndex, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::CornerRadius) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Duration) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::GridLength) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Thickness) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Media::Matrix) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media_Media3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: super::super::Media::Media3D::Matrix3D) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::DateTime) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Point) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Rect) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::Size) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::super::Foundation::TimeSpan) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::super::Color) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::CornerRadius) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Duration) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::GridLength) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Thickness) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Xaml_Media")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Media::Matrix) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media"))] usize,
    #[cfg(feature = "UI_Xaml_Media_Media3D")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut super::super::Media::Media3D::Matrix3D) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Media_Media3D"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, propertyindex: XamlPropertyIndex) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, result__: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, index: u32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, index: u32, value: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, value: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, index: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, eventindex: XamlEventIndex, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, eventindex: XamlEventIndex, handler: ::windows::runtime::RawPtr, handledeventstoo: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, xamldirectobject: ::windows::runtime::RawPtr, eventindex: XamlEventIndex, handler: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
pub struct IXamlDirectObject(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlDirectObject {
    type Vtable = IXamlDirectObject_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(274811522, 52964, 17989, [186, 37, 208, 113, 206, 119, 131, 85]);
}
impl IXamlDirectObject {}
unsafe impl ::windows::runtime::RuntimeType for IXamlDirectObject {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{10614a82-cee4-4645-ba25-d071ce778355}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDirectObject_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc(hidden)]
pub struct IXamlDirectStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IXamlDirectStatics {
    type Vtable = IXamlDirectStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(840468428, 5348, 23663, [135, 141, 251, 182, 4, 173, 125, 23]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDirectStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct XamlDirect(::windows::runtime::IInspectable);
impl XamlDirect {
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetObject<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetXamlDirectObject<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, object: Param0) -> ::windows::runtime::Result<IXamlDirectObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), object.into_param().abi(), &mut result__).from_abi::<IXamlDirectObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn CreateInstance(&self, typeindex: XamlTypeIndex) -> ::windows::runtime::Result<IXamlDirectObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), typeindex, &mut result__).from_abi::<IXamlDirectObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetObjectProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetXamlDirectObjectProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetBooleanProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetDoubleProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetInt32Property<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetStringProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn SetDateTimeProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::DateTime>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn SetPointProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn SetRectProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Rect>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn SetSizeProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::Size>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn SetTimeSpanProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::super::Foundation::TimeSpan>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetColorProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::super::Color>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetCornerRadiusProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::CornerRadius>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn SetDurationProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::Duration>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetGridLengthProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::GridLength>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetThicknessProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::Thickness>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `UI_Xaml_Media`*"]
    pub fn SetMatrixProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::Media::Matrix>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `UI_Xaml_Media_Media3D`*"]
    pub fn SetMatrix3DProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, super::super::Media::Media3D::Matrix3D>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn SetEnumProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, value).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetObjectProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetXamlDirectObjectProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<IXamlDirectObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<IXamlDirectObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetBooleanProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetDoubleProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<f64> {
        let this = self;
        unsafe {
            let mut result__: f64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetInt32Property<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetStringProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn GetDateTimeProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn GetPointProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::super::super::Foundation::Point> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Point = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::Point>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn GetRectProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Rect = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::Rect>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn GetSizeProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::super::super::Foundation::Size> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::Size = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::Size>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn GetTimeSpanProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetColorProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetCornerRadiusProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::CornerRadius> {
        let this = self;
        unsafe {
            let mut result__: super::super::CornerRadius = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::CornerRadius>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `Foundation`*"]
    pub fn GetDurationProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::Duration> {
        let this = self;
        unsafe {
            let mut result__: super::super::Duration = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::Duration>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetGridLengthProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::GridLength> {
        let this = self;
        unsafe {
            let mut result__: super::super::GridLength = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::GridLength>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetThicknessProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::Thickness> {
        let this = self;
        unsafe {
            let mut result__: super::super::Thickness = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::Thickness>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `UI_Xaml_Media`*"]
    pub fn GetMatrixProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::Media::Matrix> {
        let this = self;
        unsafe {
            let mut result__: super::super::Media::Matrix = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::Media::Matrix>(result__)
        }
    }
    #[cfg(feature = "UI_Xaml_Media_Media3D")]
    #[doc = "*Required features: `UI_Xaml_Core_Direct`, `UI_Xaml_Media_Media3D`*"]
    pub fn GetMatrix3DProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<super::super::Media::Media3D::Matrix3D> {
        let this = self;
        unsafe {
            let mut result__: super::super::Media::Media3D::Matrix3D = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<super::super::Media::Media3D::Matrix3D>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetEnumProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex, &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn ClearProperty<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, propertyindex: XamlPropertyIndex) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), propertyindex).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetCollectionCount<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0) -> ::windows::runtime::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetXamlDirectObjectFromCollectionAt<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, index: u32) -> ::windows::runtime::Result<IXamlDirectObject> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), index, &mut result__).from_abi::<IXamlDirectObject>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn AddToCollection<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param1: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn InsertIntoCollectionAt<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, index: u32, value: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn RemoveFromCollection<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param1: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, value: Param1) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), value.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn RemoveFromCollectionAt<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0, index: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), index).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn ClearCollection<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>>(&self, xamldirectobject: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn AddEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xamldirectobject: Param0, eventindex: XamlEventIndex, handler: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), eventindex, handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn AddEventHandler_HandledEventsToo<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xamldirectobject: Param0, eventindex: XamlEventIndex, handler: Param2, handledeventstoo: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), eventindex, handler.into_param().abi(), handledeventstoo).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn RemoveEventHandler<'a, Param0: ::windows::runtime::IntoParam<'a, IXamlDirectObject>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>>(&self, xamldirectobject: Param0, eventindex: XamlEventIndex, handler: Param2) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), xamldirectobject.into_param().abi(), eventindex, handler.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
    pub fn GetDefault() -> ::windows::runtime::Result<XamlDirect> {
        Self::IXamlDirectStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<XamlDirect>(result__)
        })
    }
    pub fn IXamlDirectStatics<R, F: FnOnce(&IXamlDirectStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<XamlDirect, IXamlDirectStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for XamlDirect {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Core.Direct.XamlDirect;{5ffa1295-add2-590f-a051-70989b866ade})");
}
unsafe impl ::windows::runtime::Interface for XamlDirect {
    type Vtable = IXamlDirect_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1610224277, 44498, 22799, [160, 81, 112, 152, 155, 134, 106, 222]);
}
impl ::windows::runtime::RuntimeName for XamlDirect {
    const NAME: &'static str = "Windows.UI.Xaml.Core.Direct.XamlDirect";
}
impl ::std::convert::From<XamlDirect> for ::windows::runtime::IUnknown {
    fn from(value: XamlDirect) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&XamlDirect> for ::windows::runtime::IUnknown {
    fn from(value: &XamlDirect) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for XamlDirect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &XamlDirect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<XamlDirect> for ::windows::runtime::IInspectable {
    fn from(value: XamlDirect) -> Self {
        value.0
    }
}
impl ::std::convert::From<&XamlDirect> for ::windows::runtime::IInspectable {
    fn from(value: &XamlDirect) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for XamlDirect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a XamlDirect {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for XamlDirect {}
unsafe impl ::std::marker::Sync for XamlDirect {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct XamlDirectContract(pub u8);
#[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XamlEventIndex(pub i32);
impl XamlEventIndex {
    pub const FrameworkElement_DataContextChanged: XamlEventIndex = XamlEventIndex(16i32);
    pub const FrameworkElement_SizeChanged: XamlEventIndex = XamlEventIndex(17i32);
    pub const FrameworkElement_LayoutUpdated: XamlEventIndex = XamlEventIndex(18i32);
    pub const UIElement_KeyUp: XamlEventIndex = XamlEventIndex(22i32);
    pub const UIElement_KeyDown: XamlEventIndex = XamlEventIndex(23i32);
    pub const UIElement_GotFocus: XamlEventIndex = XamlEventIndex(24i32);
    pub const UIElement_LostFocus: XamlEventIndex = XamlEventIndex(25i32);
    pub const UIElement_DragStarting: XamlEventIndex = XamlEventIndex(26i32);
    pub const UIElement_DropCompleted: XamlEventIndex = XamlEventIndex(27i32);
    pub const UIElement_CharacterReceived: XamlEventIndex = XamlEventIndex(28i32);
    pub const UIElement_DragEnter: XamlEventIndex = XamlEventIndex(29i32);
    pub const UIElement_DragLeave: XamlEventIndex = XamlEventIndex(30i32);
    pub const UIElement_DragOver: XamlEventIndex = XamlEventIndex(31i32);
    pub const UIElement_Drop: XamlEventIndex = XamlEventIndex(32i32);
    pub const UIElement_PointerPressed: XamlEventIndex = XamlEventIndex(38i32);
    pub const UIElement_PointerMoved: XamlEventIndex = XamlEventIndex(39i32);
    pub const UIElement_PointerReleased: XamlEventIndex = XamlEventIndex(40i32);
    pub const UIElement_PointerEntered: XamlEventIndex = XamlEventIndex(41i32);
    pub const UIElement_PointerExited: XamlEventIndex = XamlEventIndex(42i32);
    pub const UIElement_PointerCaptureLost: XamlEventIndex = XamlEventIndex(43i32);
    pub const UIElement_PointerCanceled: XamlEventIndex = XamlEventIndex(44i32);
    pub const UIElement_PointerWheelChanged: XamlEventIndex = XamlEventIndex(45i32);
    pub const UIElement_Tapped: XamlEventIndex = XamlEventIndex(46i32);
    pub const UIElement_DoubleTapped: XamlEventIndex = XamlEventIndex(47i32);
    pub const UIElement_Holding: XamlEventIndex = XamlEventIndex(48i32);
    pub const UIElement_ContextRequested: XamlEventIndex = XamlEventIndex(49i32);
    pub const UIElement_ContextCanceled: XamlEventIndex = XamlEventIndex(50i32);
    pub const UIElement_RightTapped: XamlEventIndex = XamlEventIndex(51i32);
    pub const UIElement_ManipulationStarting: XamlEventIndex = XamlEventIndex(52i32);
    pub const UIElement_ManipulationInertiaStarting: XamlEventIndex = XamlEventIndex(53i32);
    pub const UIElement_ManipulationStarted: XamlEventIndex = XamlEventIndex(54i32);
    pub const UIElement_ManipulationDelta: XamlEventIndex = XamlEventIndex(55i32);
    pub const UIElement_ManipulationCompleted: XamlEventIndex = XamlEventIndex(56i32);
    pub const UIElement_ProcessKeyboardAccelerators: XamlEventIndex = XamlEventIndex(60i32);
    pub const UIElement_GettingFocus: XamlEventIndex = XamlEventIndex(61i32);
    pub const UIElement_LosingFocus: XamlEventIndex = XamlEventIndex(62i32);
    pub const UIElement_NoFocusCandidateFound: XamlEventIndex = XamlEventIndex(63i32);
    pub const UIElement_PreviewKeyDown: XamlEventIndex = XamlEventIndex(64i32);
    pub const UIElement_PreviewKeyUp: XamlEventIndex = XamlEventIndex(65i32);
    pub const UIElement_BringIntoViewRequested: XamlEventIndex = XamlEventIndex(66i32);
    pub const AppBar_Opening: XamlEventIndex = XamlEventIndex(109i32);
    pub const AppBar_Opened: XamlEventIndex = XamlEventIndex(110i32);
    pub const AppBar_Closing: XamlEventIndex = XamlEventIndex(111i32);
    pub const AppBar_Closed: XamlEventIndex = XamlEventIndex(112i32);
    pub const AutoSuggestBox_SuggestionChosen: XamlEventIndex = XamlEventIndex(113i32);
    pub const AutoSuggestBox_TextChanged: XamlEventIndex = XamlEventIndex(114i32);
    pub const AutoSuggestBox_QuerySubmitted: XamlEventIndex = XamlEventIndex(115i32);
    pub const CalendarDatePicker_CalendarViewDayItemChanging: XamlEventIndex = XamlEventIndex(116i32);
    pub const CalendarDatePicker_DateChanged: XamlEventIndex = XamlEventIndex(117i32);
    pub const CalendarDatePicker_Opened: XamlEventIndex = XamlEventIndex(118i32);
    pub const CalendarDatePicker_Closed: XamlEventIndex = XamlEventIndex(119i32);
    pub const CalendarView_CalendarViewDayItemChanging: XamlEventIndex = XamlEventIndex(120i32);
    pub const CalendarView_SelectedDatesChanged: XamlEventIndex = XamlEventIndex(121i32);
    pub const ComboBox_DropDownClosed: XamlEventIndex = XamlEventIndex(122i32);
    pub const ComboBox_DropDownOpened: XamlEventIndex = XamlEventIndex(123i32);
    pub const CommandBar_DynamicOverflowItemsChanging: XamlEventIndex = XamlEventIndex(124i32);
    pub const ContentDialog_Closing: XamlEventIndex = XamlEventIndex(126i32);
    pub const ContentDialog_Closed: XamlEventIndex = XamlEventIndex(127i32);
    pub const ContentDialog_Opened: XamlEventIndex = XamlEventIndex(128i32);
    pub const ContentDialog_PrimaryButtonClick: XamlEventIndex = XamlEventIndex(129i32);
    pub const ContentDialog_SecondaryButtonClick: XamlEventIndex = XamlEventIndex(130i32);
    pub const ContentDialog_CloseButtonClick: XamlEventIndex = XamlEventIndex(131i32);
    pub const Control_FocusEngaged: XamlEventIndex = XamlEventIndex(132i32);
    pub const Control_FocusDisengaged: XamlEventIndex = XamlEventIndex(133i32);
    pub const DatePicker_DateChanged: XamlEventIndex = XamlEventIndex(135i32);
    pub const Frame_Navigated: XamlEventIndex = XamlEventIndex(136i32);
    pub const Frame_Navigating: XamlEventIndex = XamlEventIndex(137i32);
    pub const Frame_NavigationFailed: XamlEventIndex = XamlEventIndex(138i32);
    pub const Frame_NavigationStopped: XamlEventIndex = XamlEventIndex(139i32);
    pub const Hub_SectionHeaderClick: XamlEventIndex = XamlEventIndex(143i32);
    pub const Hub_SectionsInViewChanged: XamlEventIndex = XamlEventIndex(144i32);
    pub const ItemsPresenter_HorizontalSnapPointsChanged: XamlEventIndex = XamlEventIndex(148i32);
    pub const ItemsPresenter_VerticalSnapPointsChanged: XamlEventIndex = XamlEventIndex(149i32);
    pub const ListViewBase_ItemClick: XamlEventIndex = XamlEventIndex(150i32);
    pub const ListViewBase_DragItemsStarting: XamlEventIndex = XamlEventIndex(151i32);
    pub const ListViewBase_DragItemsCompleted: XamlEventIndex = XamlEventIndex(152i32);
    pub const ListViewBase_ContainerContentChanging: XamlEventIndex = XamlEventIndex(153i32);
    pub const ListViewBase_ChoosingItemContainer: XamlEventIndex = XamlEventIndex(154i32);
    pub const ListViewBase_ChoosingGroupHeaderContainer: XamlEventIndex = XamlEventIndex(155i32);
    pub const MediaTransportControls_ThumbnailRequested: XamlEventIndex = XamlEventIndex(167i32);
    pub const MenuFlyoutItem_Click: XamlEventIndex = XamlEventIndex(168i32);
    pub const RichEditBox_TextChanging: XamlEventIndex = XamlEventIndex(177i32);
    pub const ScrollViewer_ViewChanging: XamlEventIndex = XamlEventIndex(192i32);
    pub const ScrollViewer_ViewChanged: XamlEventIndex = XamlEventIndex(193i32);
    pub const ScrollViewer_DirectManipulationStarted: XamlEventIndex = XamlEventIndex(194i32);
    pub const ScrollViewer_DirectManipulationCompleted: XamlEventIndex = XamlEventIndex(195i32);
    pub const SearchBox_QueryChanged: XamlEventIndex = XamlEventIndex(196i32);
    pub const SearchBox_SuggestionsRequested: XamlEventIndex = XamlEventIndex(197i32);
    pub const SearchBox_QuerySubmitted: XamlEventIndex = XamlEventIndex(198i32);
    pub const SearchBox_ResultSuggestionChosen: XamlEventIndex = XamlEventIndex(199i32);
    pub const SearchBox_PrepareForFocusOnKeyboardInput: XamlEventIndex = XamlEventIndex(200i32);
    pub const SemanticZoom_ViewChangeStarted: XamlEventIndex = XamlEventIndex(201i32);
    pub const SemanticZoom_ViewChangeCompleted: XamlEventIndex = XamlEventIndex(202i32);
    pub const SettingsFlyout_BackClick: XamlEventIndex = XamlEventIndex(203i32);
    pub const StackPanel_HorizontalSnapPointsChanged: XamlEventIndex = XamlEventIndex(208i32);
    pub const StackPanel_VerticalSnapPointsChanged: XamlEventIndex = XamlEventIndex(209i32);
    pub const TimePicker_TimeChanged: XamlEventIndex = XamlEventIndex(227i32);
    pub const ToggleSwitch_Toggled: XamlEventIndex = XamlEventIndex(228i32);
    pub const ToolTip_Closed: XamlEventIndex = XamlEventIndex(229i32);
    pub const ToolTip_Opened: XamlEventIndex = XamlEventIndex(230i32);
    pub const VirtualizingStackPanel_CleanUpVirtualizedItemEvent: XamlEventIndex = XamlEventIndex(231i32);
    pub const WebView_SeparateProcessLost: XamlEventIndex = XamlEventIndex(232i32);
    pub const WebView_LoadCompleted: XamlEventIndex = XamlEventIndex(233i32);
    pub const WebView_ScriptNotify: XamlEventIndex = XamlEventIndex(234i32);
    pub const WebView_NavigationFailed: XamlEventIndex = XamlEventIndex(235i32);
    pub const WebView_NavigationStarting: XamlEventIndex = XamlEventIndex(236i32);
    pub const WebView_ContentLoading: XamlEventIndex = XamlEventIndex(237i32);
    pub const WebView_DOMContentLoaded: XamlEventIndex = XamlEventIndex(238i32);
    pub const WebView_NavigationCompleted: XamlEventIndex = XamlEventIndex(239i32);
    pub const WebView_FrameNavigationStarting: XamlEventIndex = XamlEventIndex(240i32);
    pub const WebView_FrameContentLoading: XamlEventIndex = XamlEventIndex(241i32);
    pub const WebView_FrameDOMContentLoaded: XamlEventIndex = XamlEventIndex(242i32);
    pub const WebView_FrameNavigationCompleted: XamlEventIndex = XamlEventIndex(243i32);
    pub const WebView_LongRunningScriptDetected: XamlEventIndex = XamlEventIndex(244i32);
    pub const WebView_UnsafeContentWarningDisplaying: XamlEventIndex = XamlEventIndex(245i32);
    pub const WebView_UnviewableContentIdentified: XamlEventIndex = XamlEventIndex(246i32);
    pub const WebView_ContainsFullScreenElementChanged: XamlEventIndex = XamlEventIndex(247i32);
    pub const WebView_UnsupportedUriSchemeIdentified: XamlEventIndex = XamlEventIndex(248i32);
    pub const WebView_NewWindowRequested: XamlEventIndex = XamlEventIndex(249i32);
    pub const WebView_PermissionRequested: XamlEventIndex = XamlEventIndex(250i32);
    pub const ButtonBase_Click: XamlEventIndex = XamlEventIndex(256i32);
    pub const CarouselPanel_HorizontalSnapPointsChanged: XamlEventIndex = XamlEventIndex(257i32);
    pub const CarouselPanel_VerticalSnapPointsChanged: XamlEventIndex = XamlEventIndex(258i32);
    pub const OrientedVirtualizingPanel_HorizontalSnapPointsChanged: XamlEventIndex = XamlEventIndex(263i32);
    pub const OrientedVirtualizingPanel_VerticalSnapPointsChanged: XamlEventIndex = XamlEventIndex(264i32);
    pub const RangeBase_ValueChanged: XamlEventIndex = XamlEventIndex(267i32);
    pub const ScrollBar_Scroll: XamlEventIndex = XamlEventIndex(268i32);
    pub const Selector_SelectionChanged: XamlEventIndex = XamlEventIndex(269i32);
    pub const Thumb_DragStarted: XamlEventIndex = XamlEventIndex(270i32);
    pub const Thumb_DragDelta: XamlEventIndex = XamlEventIndex(271i32);
    pub const Thumb_DragCompleted: XamlEventIndex = XamlEventIndex(272i32);
    pub const ToggleButton_Checked: XamlEventIndex = XamlEventIndex(273i32);
    pub const ToggleButton_Unchecked: XamlEventIndex = XamlEventIndex(274i32);
    pub const ToggleButton_Indeterminate: XamlEventIndex = XamlEventIndex(275i32);
    pub const WebView_WebResourceRequested: XamlEventIndex = XamlEventIndex(283i32);
    pub const ScrollViewer_AnchorRequested: XamlEventIndex = XamlEventIndex(291i32);
    pub const DatePicker_SelectedDateChanged: XamlEventIndex = XamlEventIndex(322i32);
    pub const TimePicker_SelectedTimeChanged: XamlEventIndex = XamlEventIndex(323i32);
}
impl ::std::convert::From<i32> for XamlEventIndex {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XamlEventIndex {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XamlEventIndex {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Core.Direct.XamlEventIndex;i4)");
}
#[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XamlPropertyIndex(pub i32);
impl XamlPropertyIndex {
    pub const AutomationProperties_AcceleratorKey: XamlPropertyIndex = XamlPropertyIndex(5i32);
    pub const AutomationProperties_AccessibilityView: XamlPropertyIndex = XamlPropertyIndex(6i32);
    pub const AutomationProperties_AccessKey: XamlPropertyIndex = XamlPropertyIndex(7i32);
    pub const AutomationProperties_AutomationId: XamlPropertyIndex = XamlPropertyIndex(8i32);
    pub const AutomationProperties_ControlledPeers: XamlPropertyIndex = XamlPropertyIndex(9i32);
    pub const AutomationProperties_HelpText: XamlPropertyIndex = XamlPropertyIndex(10i32);
    pub const AutomationProperties_IsRequiredForForm: XamlPropertyIndex = XamlPropertyIndex(11i32);
    pub const AutomationProperties_ItemStatus: XamlPropertyIndex = XamlPropertyIndex(12i32);
    pub const AutomationProperties_ItemType: XamlPropertyIndex = XamlPropertyIndex(13i32);
    pub const AutomationProperties_LabeledBy: XamlPropertyIndex = XamlPropertyIndex(14i32);
    pub const AutomationProperties_LiveSetting: XamlPropertyIndex = XamlPropertyIndex(15i32);
    pub const AutomationProperties_Name: XamlPropertyIndex = XamlPropertyIndex(16i32);
    pub const ToolTipService_Placement: XamlPropertyIndex = XamlPropertyIndex(24i32);
    pub const ToolTipService_PlacementTarget: XamlPropertyIndex = XamlPropertyIndex(25i32);
    pub const ToolTipService_ToolTip: XamlPropertyIndex = XamlPropertyIndex(26i32);
    pub const Typography_AnnotationAlternates: XamlPropertyIndex = XamlPropertyIndex(28i32);
    pub const Typography_Capitals: XamlPropertyIndex = XamlPropertyIndex(29i32);
    pub const Typography_CapitalSpacing: XamlPropertyIndex = XamlPropertyIndex(30i32);
    pub const Typography_CaseSensitiveForms: XamlPropertyIndex = XamlPropertyIndex(31i32);
    pub const Typography_ContextualAlternates: XamlPropertyIndex = XamlPropertyIndex(32i32);
    pub const Typography_ContextualLigatures: XamlPropertyIndex = XamlPropertyIndex(33i32);
    pub const Typography_ContextualSwashes: XamlPropertyIndex = XamlPropertyIndex(34i32);
    pub const Typography_DiscretionaryLigatures: XamlPropertyIndex = XamlPropertyIndex(35i32);
    pub const Typography_EastAsianExpertForms: XamlPropertyIndex = XamlPropertyIndex(36i32);
    pub const Typography_EastAsianLanguage: XamlPropertyIndex = XamlPropertyIndex(37i32);
    pub const Typography_EastAsianWidths: XamlPropertyIndex = XamlPropertyIndex(38i32);
    pub const Typography_Fraction: XamlPropertyIndex = XamlPropertyIndex(39i32);
    pub const Typography_HistoricalForms: XamlPropertyIndex = XamlPropertyIndex(40i32);
    pub const Typography_HistoricalLigatures: XamlPropertyIndex = XamlPropertyIndex(41i32);
    pub const Typography_Kerning: XamlPropertyIndex = XamlPropertyIndex(42i32);
    pub const Typography_MathematicalGreek: XamlPropertyIndex = XamlPropertyIndex(43i32);
    pub const Typography_NumeralAlignment: XamlPropertyIndex = XamlPropertyIndex(44i32);
    pub const Typography_NumeralStyle: XamlPropertyIndex = XamlPropertyIndex(45i32);
    pub const Typography_SlashedZero: XamlPropertyIndex = XamlPropertyIndex(46i32);
    pub const Typography_StandardLigatures: XamlPropertyIndex = XamlPropertyIndex(47i32);
    pub const Typography_StandardSwashes: XamlPropertyIndex = XamlPropertyIndex(48i32);
    pub const Typography_StylisticAlternates: XamlPropertyIndex = XamlPropertyIndex(49i32);
    pub const Typography_StylisticSet1: XamlPropertyIndex = XamlPropertyIndex(50i32);
    pub const Typography_StylisticSet10: XamlPropertyIndex = XamlPropertyIndex(51i32);
    pub const Typography_StylisticSet11: XamlPropertyIndex = XamlPropertyIndex(52i32);
    pub const Typography_StylisticSet12: XamlPropertyIndex = XamlPropertyIndex(53i32);
    pub const Typography_StylisticSet13: XamlPropertyIndex = XamlPropertyIndex(54i32);
    pub const Typography_StylisticSet14: XamlPropertyIndex = XamlPropertyIndex(55i32);
    pub const Typography_StylisticSet15: XamlPropertyIndex = XamlPropertyIndex(56i32);
    pub const Typography_StylisticSet16: XamlPropertyIndex = XamlPropertyIndex(57i32);
    pub const Typography_StylisticSet17: XamlPropertyIndex = XamlPropertyIndex(58i32);
    pub const Typography_StylisticSet18: XamlPropertyIndex = XamlPropertyIndex(59i32);
    pub const Typography_StylisticSet19: XamlPropertyIndex = XamlPropertyIndex(60i32);
    pub const Typography_StylisticSet2: XamlPropertyIndex = XamlPropertyIndex(61i32);
    pub const Typography_StylisticSet20: XamlPropertyIndex = XamlPropertyIndex(62i32);
    pub const Typography_StylisticSet3: XamlPropertyIndex = XamlPropertyIndex(63i32);
    pub const Typography_StylisticSet4: XamlPropertyIndex = XamlPropertyIndex(64i32);
    pub const Typography_StylisticSet5: XamlPropertyIndex = XamlPropertyIndex(65i32);
    pub const Typography_StylisticSet6: XamlPropertyIndex = XamlPropertyIndex(66i32);
    pub const Typography_StylisticSet7: XamlPropertyIndex = XamlPropertyIndex(67i32);
    pub const Typography_StylisticSet8: XamlPropertyIndex = XamlPropertyIndex(68i32);
    pub const Typography_StylisticSet9: XamlPropertyIndex = XamlPropertyIndex(69i32);
    pub const Typography_Variants: XamlPropertyIndex = XamlPropertyIndex(70i32);
    pub const AutomationPeer_EventsSource: XamlPropertyIndex = XamlPropertyIndex(75i32);
    pub const AutoSuggestBoxSuggestionChosenEventArgs_SelectedItem: XamlPropertyIndex = XamlPropertyIndex(76i32);
    pub const AutoSuggestBoxTextChangedEventArgs_Reason: XamlPropertyIndex = XamlPropertyIndex(77i32);
    pub const Brush_Opacity: XamlPropertyIndex = XamlPropertyIndex(78i32);
    pub const Brush_RelativeTransform: XamlPropertyIndex = XamlPropertyIndex(79i32);
    pub const Brush_Transform: XamlPropertyIndex = XamlPropertyIndex(80i32);
    pub const CollectionViewSource_IsSourceGrouped: XamlPropertyIndex = XamlPropertyIndex(81i32);
    pub const CollectionViewSource_ItemsPath: XamlPropertyIndex = XamlPropertyIndex(82i32);
    pub const CollectionViewSource_Source: XamlPropertyIndex = XamlPropertyIndex(83i32);
    pub const CollectionViewSource_View: XamlPropertyIndex = XamlPropertyIndex(84i32);
    pub const ColorKeyFrame_KeyTime: XamlPropertyIndex = XamlPropertyIndex(90i32);
    pub const ColorKeyFrame_Value: XamlPropertyIndex = XamlPropertyIndex(91i32);
    pub const ColumnDefinition_ActualWidth: XamlPropertyIndex = XamlPropertyIndex(92i32);
    pub const ColumnDefinition_MaxWidth: XamlPropertyIndex = XamlPropertyIndex(93i32);
    pub const ColumnDefinition_MinWidth: XamlPropertyIndex = XamlPropertyIndex(94i32);
    pub const ColumnDefinition_Width: XamlPropertyIndex = XamlPropertyIndex(95i32);
    pub const ComboBoxTemplateSettings_DropDownClosedHeight: XamlPropertyIndex = XamlPropertyIndex(96i32);
    pub const ComboBoxTemplateSettings_DropDownOffset: XamlPropertyIndex = XamlPropertyIndex(97i32);
    pub const ComboBoxTemplateSettings_DropDownOpenedHeight: XamlPropertyIndex = XamlPropertyIndex(98i32);
    pub const ComboBoxTemplateSettings_SelectedItemDirection: XamlPropertyIndex = XamlPropertyIndex(99i32);
    pub const DoubleKeyFrame_KeyTime: XamlPropertyIndex = XamlPropertyIndex(107i32);
    pub const DoubleKeyFrame_Value: XamlPropertyIndex = XamlPropertyIndex(108i32);
    pub const EasingFunctionBase_EasingMode: XamlPropertyIndex = XamlPropertyIndex(111i32);
    pub const FlyoutBase_AttachedFlyout: XamlPropertyIndex = XamlPropertyIndex(114i32);
    pub const FlyoutBase_Placement: XamlPropertyIndex = XamlPropertyIndex(115i32);
    pub const Geometry_Bounds: XamlPropertyIndex = XamlPropertyIndex(118i32);
    pub const Geometry_Transform: XamlPropertyIndex = XamlPropertyIndex(119i32);
    pub const GradientStop_Color: XamlPropertyIndex = XamlPropertyIndex(120i32);
    pub const GradientStop_Offset: XamlPropertyIndex = XamlPropertyIndex(121i32);
    pub const GroupStyle_ContainerStyle: XamlPropertyIndex = XamlPropertyIndex(124i32);
    pub const GroupStyle_ContainerStyleSelector: XamlPropertyIndex = XamlPropertyIndex(125i32);
    pub const GroupStyle_HeaderContainerStyle: XamlPropertyIndex = XamlPropertyIndex(126i32);
    pub const GroupStyle_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(127i32);
    pub const GroupStyle_HeaderTemplateSelector: XamlPropertyIndex = XamlPropertyIndex(128i32);
    pub const GroupStyle_HidesIfEmpty: XamlPropertyIndex = XamlPropertyIndex(129i32);
    pub const GroupStyle_Panel: XamlPropertyIndex = XamlPropertyIndex(130i32);
    pub const InertiaExpansionBehavior_DesiredDeceleration: XamlPropertyIndex = XamlPropertyIndex(144i32);
    pub const InertiaExpansionBehavior_DesiredExpansion: XamlPropertyIndex = XamlPropertyIndex(145i32);
    pub const InertiaRotationBehavior_DesiredDeceleration: XamlPropertyIndex = XamlPropertyIndex(146i32);
    pub const InertiaRotationBehavior_DesiredRotation: XamlPropertyIndex = XamlPropertyIndex(147i32);
    pub const InertiaTranslationBehavior_DesiredDeceleration: XamlPropertyIndex = XamlPropertyIndex(148i32);
    pub const InertiaTranslationBehavior_DesiredDisplacement: XamlPropertyIndex = XamlPropertyIndex(149i32);
    pub const InputScope_Names: XamlPropertyIndex = XamlPropertyIndex(150i32);
    pub const InputScopeName_NameValue: XamlPropertyIndex = XamlPropertyIndex(151i32);
    pub const KeySpline_ControlPoint1: XamlPropertyIndex = XamlPropertyIndex(153i32);
    pub const KeySpline_ControlPoint2: XamlPropertyIndex = XamlPropertyIndex(154i32);
    pub const ManipulationPivot_Center: XamlPropertyIndex = XamlPropertyIndex(159i32);
    pub const ManipulationPivot_Radius: XamlPropertyIndex = XamlPropertyIndex(160i32);
    pub const ObjectKeyFrame_KeyTime: XamlPropertyIndex = XamlPropertyIndex(183i32);
    pub const ObjectKeyFrame_Value: XamlPropertyIndex = XamlPropertyIndex(184i32);
    pub const PageStackEntry_SourcePageType: XamlPropertyIndex = XamlPropertyIndex(185i32);
    pub const PathFigure_IsClosed: XamlPropertyIndex = XamlPropertyIndex(192i32);
    pub const PathFigure_IsFilled: XamlPropertyIndex = XamlPropertyIndex(193i32);
    pub const PathFigure_Segments: XamlPropertyIndex = XamlPropertyIndex(194i32);
    pub const PathFigure_StartPoint: XamlPropertyIndex = XamlPropertyIndex(195i32);
    pub const Pointer_IsInContact: XamlPropertyIndex = XamlPropertyIndex(199i32);
    pub const Pointer_IsInRange: XamlPropertyIndex = XamlPropertyIndex(200i32);
    pub const Pointer_PointerDeviceType: XamlPropertyIndex = XamlPropertyIndex(201i32);
    pub const Pointer_PointerId: XamlPropertyIndex = XamlPropertyIndex(202i32);
    pub const PointKeyFrame_KeyTime: XamlPropertyIndex = XamlPropertyIndex(205i32);
    pub const PointKeyFrame_Value: XamlPropertyIndex = XamlPropertyIndex(206i32);
    pub const PrintDocument_DocumentSource: XamlPropertyIndex = XamlPropertyIndex(209i32);
    pub const ProgressBarTemplateSettings_ContainerAnimationEndPosition: XamlPropertyIndex = XamlPropertyIndex(211i32);
    pub const ProgressBarTemplateSettings_ContainerAnimationStartPosition: XamlPropertyIndex = XamlPropertyIndex(212i32);
    pub const ProgressBarTemplateSettings_EllipseAnimationEndPosition: XamlPropertyIndex = XamlPropertyIndex(213i32);
    pub const ProgressBarTemplateSettings_EllipseAnimationWellPosition: XamlPropertyIndex = XamlPropertyIndex(214i32);
    pub const ProgressBarTemplateSettings_EllipseDiameter: XamlPropertyIndex = XamlPropertyIndex(215i32);
    pub const ProgressBarTemplateSettings_EllipseOffset: XamlPropertyIndex = XamlPropertyIndex(216i32);
    pub const ProgressBarTemplateSettings_IndicatorLengthDelta: XamlPropertyIndex = XamlPropertyIndex(217i32);
    pub const ProgressRingTemplateSettings_EllipseDiameter: XamlPropertyIndex = XamlPropertyIndex(218i32);
    pub const ProgressRingTemplateSettings_EllipseOffset: XamlPropertyIndex = XamlPropertyIndex(219i32);
    pub const ProgressRingTemplateSettings_MaxSideLength: XamlPropertyIndex = XamlPropertyIndex(220i32);
    pub const PropertyPath_Path: XamlPropertyIndex = XamlPropertyIndex(221i32);
    pub const RowDefinition_ActualHeight: XamlPropertyIndex = XamlPropertyIndex(226i32);
    pub const RowDefinition_Height: XamlPropertyIndex = XamlPropertyIndex(227i32);
    pub const RowDefinition_MaxHeight: XamlPropertyIndex = XamlPropertyIndex(228i32);
    pub const RowDefinition_MinHeight: XamlPropertyIndex = XamlPropertyIndex(229i32);
    pub const SetterBase_IsSealed: XamlPropertyIndex = XamlPropertyIndex(233i32);
    pub const SettingsFlyoutTemplateSettings_BorderBrush: XamlPropertyIndex = XamlPropertyIndex(234i32);
    pub const SettingsFlyoutTemplateSettings_BorderThickness: XamlPropertyIndex = XamlPropertyIndex(235i32);
    pub const SettingsFlyoutTemplateSettings_ContentTransitions: XamlPropertyIndex = XamlPropertyIndex(236i32);
    pub const SettingsFlyoutTemplateSettings_HeaderBackground: XamlPropertyIndex = XamlPropertyIndex(237i32);
    pub const SettingsFlyoutTemplateSettings_HeaderForeground: XamlPropertyIndex = XamlPropertyIndex(238i32);
    pub const SettingsFlyoutTemplateSettings_IconSource: XamlPropertyIndex = XamlPropertyIndex(239i32);
    pub const Style_BasedOn: XamlPropertyIndex = XamlPropertyIndex(244i32);
    pub const Style_IsSealed: XamlPropertyIndex = XamlPropertyIndex(245i32);
    pub const Style_Setters: XamlPropertyIndex = XamlPropertyIndex(246i32);
    pub const Style_TargetType: XamlPropertyIndex = XamlPropertyIndex(247i32);
    pub const TextElement_CharacterSpacing: XamlPropertyIndex = XamlPropertyIndex(249i32);
    pub const TextElement_FontFamily: XamlPropertyIndex = XamlPropertyIndex(250i32);
    pub const TextElement_FontSize: XamlPropertyIndex = XamlPropertyIndex(251i32);
    pub const TextElement_FontStretch: XamlPropertyIndex = XamlPropertyIndex(252i32);
    pub const TextElement_FontStyle: XamlPropertyIndex = XamlPropertyIndex(253i32);
    pub const TextElement_FontWeight: XamlPropertyIndex = XamlPropertyIndex(254i32);
    pub const TextElement_Foreground: XamlPropertyIndex = XamlPropertyIndex(255i32);
    pub const TextElement_IsTextScaleFactorEnabled: XamlPropertyIndex = XamlPropertyIndex(256i32);
    pub const TextElement_Language: XamlPropertyIndex = XamlPropertyIndex(257i32);
    pub const Timeline_AutoReverse: XamlPropertyIndex = XamlPropertyIndex(263i32);
    pub const Timeline_BeginTime: XamlPropertyIndex = XamlPropertyIndex(264i32);
    pub const Timeline_Duration: XamlPropertyIndex = XamlPropertyIndex(265i32);
    pub const Timeline_FillBehavior: XamlPropertyIndex = XamlPropertyIndex(266i32);
    pub const Timeline_RepeatBehavior: XamlPropertyIndex = XamlPropertyIndex(267i32);
    pub const Timeline_SpeedRatio: XamlPropertyIndex = XamlPropertyIndex(268i32);
    pub const TimelineMarker_Text: XamlPropertyIndex = XamlPropertyIndex(269i32);
    pub const TimelineMarker_Time: XamlPropertyIndex = XamlPropertyIndex(270i32);
    pub const TimelineMarker_Type: XamlPropertyIndex = XamlPropertyIndex(271i32);
    pub const ToggleSwitchTemplateSettings_CurtainCurrentToOffOffset: XamlPropertyIndex = XamlPropertyIndex(273i32);
    pub const ToggleSwitchTemplateSettings_CurtainCurrentToOnOffset: XamlPropertyIndex = XamlPropertyIndex(274i32);
    pub const ToggleSwitchTemplateSettings_CurtainOffToOnOffset: XamlPropertyIndex = XamlPropertyIndex(275i32);
    pub const ToggleSwitchTemplateSettings_CurtainOnToOffOffset: XamlPropertyIndex = XamlPropertyIndex(276i32);
    pub const ToggleSwitchTemplateSettings_KnobCurrentToOffOffset: XamlPropertyIndex = XamlPropertyIndex(277i32);
    pub const ToggleSwitchTemplateSettings_KnobCurrentToOnOffset: XamlPropertyIndex = XamlPropertyIndex(278i32);
    pub const ToggleSwitchTemplateSettings_KnobOffToOnOffset: XamlPropertyIndex = XamlPropertyIndex(279i32);
    pub const ToggleSwitchTemplateSettings_KnobOnToOffOffset: XamlPropertyIndex = XamlPropertyIndex(280i32);
    pub const ToolTipTemplateSettings_FromHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(281i32);
    pub const ToolTipTemplateSettings_FromVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(282i32);
    pub const UIElement_AllowDrop: XamlPropertyIndex = XamlPropertyIndex(292i32);
    pub const UIElement_CacheMode: XamlPropertyIndex = XamlPropertyIndex(293i32);
    pub const UIElement_Clip: XamlPropertyIndex = XamlPropertyIndex(295i32);
    pub const UIElement_CompositeMode: XamlPropertyIndex = XamlPropertyIndex(296i32);
    pub const UIElement_IsDoubleTapEnabled: XamlPropertyIndex = XamlPropertyIndex(297i32);
    pub const UIElement_IsHitTestVisible: XamlPropertyIndex = XamlPropertyIndex(298i32);
    pub const UIElement_IsHoldingEnabled: XamlPropertyIndex = XamlPropertyIndex(299i32);
    pub const UIElement_IsRightTapEnabled: XamlPropertyIndex = XamlPropertyIndex(300i32);
    pub const UIElement_IsTapEnabled: XamlPropertyIndex = XamlPropertyIndex(301i32);
    pub const UIElement_ManipulationMode: XamlPropertyIndex = XamlPropertyIndex(302i32);
    pub const UIElement_Opacity: XamlPropertyIndex = XamlPropertyIndex(303i32);
    pub const UIElement_PointerCaptures: XamlPropertyIndex = XamlPropertyIndex(304i32);
    pub const UIElement_Projection: XamlPropertyIndex = XamlPropertyIndex(305i32);
    pub const UIElement_RenderSize: XamlPropertyIndex = XamlPropertyIndex(306i32);
    pub const UIElement_RenderTransform: XamlPropertyIndex = XamlPropertyIndex(307i32);
    pub const UIElement_RenderTransformOrigin: XamlPropertyIndex = XamlPropertyIndex(308i32);
    pub const UIElement_Transitions: XamlPropertyIndex = XamlPropertyIndex(309i32);
    pub const UIElement_UseLayoutRounding: XamlPropertyIndex = XamlPropertyIndex(311i32);
    pub const UIElement_Visibility: XamlPropertyIndex = XamlPropertyIndex(312i32);
    pub const VisualState_Storyboard: XamlPropertyIndex = XamlPropertyIndex(322i32);
    pub const VisualStateGroup_States: XamlPropertyIndex = XamlPropertyIndex(323i32);
    pub const VisualStateGroup_Transitions: XamlPropertyIndex = XamlPropertyIndex(324i32);
    pub const VisualStateManager_CustomVisualStateManager: XamlPropertyIndex = XamlPropertyIndex(325i32);
    pub const VisualStateManager_VisualStateGroups: XamlPropertyIndex = XamlPropertyIndex(326i32);
    pub const VisualTransition_From: XamlPropertyIndex = XamlPropertyIndex(327i32);
    pub const VisualTransition_GeneratedDuration: XamlPropertyIndex = XamlPropertyIndex(328i32);
    pub const VisualTransition_GeneratedEasingFunction: XamlPropertyIndex = XamlPropertyIndex(329i32);
    pub const VisualTransition_Storyboard: XamlPropertyIndex = XamlPropertyIndex(330i32);
    pub const VisualTransition_To: XamlPropertyIndex = XamlPropertyIndex(331i32);
    pub const ArcSegment_IsLargeArc: XamlPropertyIndex = XamlPropertyIndex(332i32);
    pub const ArcSegment_Point: XamlPropertyIndex = XamlPropertyIndex(333i32);
    pub const ArcSegment_RotationAngle: XamlPropertyIndex = XamlPropertyIndex(334i32);
    pub const ArcSegment_Size: XamlPropertyIndex = XamlPropertyIndex(335i32);
    pub const ArcSegment_SweepDirection: XamlPropertyIndex = XamlPropertyIndex(336i32);
    pub const BackEase_Amplitude: XamlPropertyIndex = XamlPropertyIndex(337i32);
    pub const BeginStoryboard_Storyboard: XamlPropertyIndex = XamlPropertyIndex(338i32);
    pub const BezierSegment_Point1: XamlPropertyIndex = XamlPropertyIndex(339i32);
    pub const BezierSegment_Point2: XamlPropertyIndex = XamlPropertyIndex(340i32);
    pub const BezierSegment_Point3: XamlPropertyIndex = XamlPropertyIndex(341i32);
    pub const BitmapSource_PixelHeight: XamlPropertyIndex = XamlPropertyIndex(342i32);
    pub const BitmapSource_PixelWidth: XamlPropertyIndex = XamlPropertyIndex(343i32);
    pub const Block_LineHeight: XamlPropertyIndex = XamlPropertyIndex(344i32);
    pub const Block_LineStackingStrategy: XamlPropertyIndex = XamlPropertyIndex(345i32);
    pub const Block_Margin: XamlPropertyIndex = XamlPropertyIndex(346i32);
    pub const Block_TextAlignment: XamlPropertyIndex = XamlPropertyIndex(347i32);
    pub const BounceEase_Bounces: XamlPropertyIndex = XamlPropertyIndex(348i32);
    pub const BounceEase_Bounciness: XamlPropertyIndex = XamlPropertyIndex(349i32);
    pub const ColorAnimation_By: XamlPropertyIndex = XamlPropertyIndex(350i32);
    pub const ColorAnimation_EasingFunction: XamlPropertyIndex = XamlPropertyIndex(351i32);
    pub const ColorAnimation_EnableDependentAnimation: XamlPropertyIndex = XamlPropertyIndex(352i32);
    pub const ColorAnimation_From: XamlPropertyIndex = XamlPropertyIndex(353i32);
    pub const ColorAnimation_To: XamlPropertyIndex = XamlPropertyIndex(354i32);
    pub const ColorAnimationUsingKeyFrames_EnableDependentAnimation: XamlPropertyIndex = XamlPropertyIndex(355i32);
    pub const ColorAnimationUsingKeyFrames_KeyFrames: XamlPropertyIndex = XamlPropertyIndex(356i32);
    pub const ContentThemeTransition_HorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(357i32);
    pub const ContentThemeTransition_VerticalOffset: XamlPropertyIndex = XamlPropertyIndex(358i32);
    pub const ControlTemplate_TargetType: XamlPropertyIndex = XamlPropertyIndex(359i32);
    pub const DispatcherTimer_Interval: XamlPropertyIndex = XamlPropertyIndex(362i32);
    pub const DoubleAnimation_By: XamlPropertyIndex = XamlPropertyIndex(363i32);
    pub const DoubleAnimation_EasingFunction: XamlPropertyIndex = XamlPropertyIndex(364i32);
    pub const DoubleAnimation_EnableDependentAnimation: XamlPropertyIndex = XamlPropertyIndex(365i32);
    pub const DoubleAnimation_From: XamlPropertyIndex = XamlPropertyIndex(366i32);
    pub const DoubleAnimation_To: XamlPropertyIndex = XamlPropertyIndex(367i32);
    pub const DoubleAnimationUsingKeyFrames_EnableDependentAnimation: XamlPropertyIndex = XamlPropertyIndex(368i32);
    pub const DoubleAnimationUsingKeyFrames_KeyFrames: XamlPropertyIndex = XamlPropertyIndex(369i32);
    pub const EasingColorKeyFrame_EasingFunction: XamlPropertyIndex = XamlPropertyIndex(372i32);
    pub const EasingDoubleKeyFrame_EasingFunction: XamlPropertyIndex = XamlPropertyIndex(373i32);
    pub const EasingPointKeyFrame_EasingFunction: XamlPropertyIndex = XamlPropertyIndex(374i32);
    pub const EdgeUIThemeTransition_Edge: XamlPropertyIndex = XamlPropertyIndex(375i32);
    pub const ElasticEase_Oscillations: XamlPropertyIndex = XamlPropertyIndex(376i32);
    pub const ElasticEase_Springiness: XamlPropertyIndex = XamlPropertyIndex(377i32);
    pub const EllipseGeometry_Center: XamlPropertyIndex = XamlPropertyIndex(378i32);
    pub const EllipseGeometry_RadiusX: XamlPropertyIndex = XamlPropertyIndex(379i32);
    pub const EllipseGeometry_RadiusY: XamlPropertyIndex = XamlPropertyIndex(380i32);
    pub const EntranceThemeTransition_FromHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(381i32);
    pub const EntranceThemeTransition_FromVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(382i32);
    pub const EntranceThemeTransition_IsStaggeringEnabled: XamlPropertyIndex = XamlPropertyIndex(383i32);
    pub const EventTrigger_Actions: XamlPropertyIndex = XamlPropertyIndex(384i32);
    pub const EventTrigger_RoutedEvent: XamlPropertyIndex = XamlPropertyIndex(385i32);
    pub const ExponentialEase_Exponent: XamlPropertyIndex = XamlPropertyIndex(386i32);
    pub const Flyout_Content: XamlPropertyIndex = XamlPropertyIndex(387i32);
    pub const Flyout_FlyoutPresenterStyle: XamlPropertyIndex = XamlPropertyIndex(388i32);
    pub const FrameworkElement_ActualHeight: XamlPropertyIndex = XamlPropertyIndex(389i32);
    pub const FrameworkElement_ActualWidth: XamlPropertyIndex = XamlPropertyIndex(390i32);
    pub const FrameworkElement_DataContext: XamlPropertyIndex = XamlPropertyIndex(391i32);
    pub const FrameworkElement_FlowDirection: XamlPropertyIndex = XamlPropertyIndex(392i32);
    pub const FrameworkElement_Height: XamlPropertyIndex = XamlPropertyIndex(393i32);
    pub const FrameworkElement_HorizontalAlignment: XamlPropertyIndex = XamlPropertyIndex(394i32);
    pub const FrameworkElement_Language: XamlPropertyIndex = XamlPropertyIndex(396i32);
    pub const FrameworkElement_Margin: XamlPropertyIndex = XamlPropertyIndex(397i32);
    pub const FrameworkElement_MaxHeight: XamlPropertyIndex = XamlPropertyIndex(398i32);
    pub const FrameworkElement_MaxWidth: XamlPropertyIndex = XamlPropertyIndex(399i32);
    pub const FrameworkElement_MinHeight: XamlPropertyIndex = XamlPropertyIndex(400i32);
    pub const FrameworkElement_MinWidth: XamlPropertyIndex = XamlPropertyIndex(401i32);
    pub const FrameworkElement_Parent: XamlPropertyIndex = XamlPropertyIndex(402i32);
    pub const FrameworkElement_RequestedTheme: XamlPropertyIndex = XamlPropertyIndex(403i32);
    pub const FrameworkElement_Resources: XamlPropertyIndex = XamlPropertyIndex(404i32);
    pub const FrameworkElement_Style: XamlPropertyIndex = XamlPropertyIndex(405i32);
    pub const FrameworkElement_Tag: XamlPropertyIndex = XamlPropertyIndex(406i32);
    pub const FrameworkElement_Triggers: XamlPropertyIndex = XamlPropertyIndex(407i32);
    pub const FrameworkElement_VerticalAlignment: XamlPropertyIndex = XamlPropertyIndex(408i32);
    pub const FrameworkElement_Width: XamlPropertyIndex = XamlPropertyIndex(409i32);
    pub const FrameworkElementAutomationPeer_Owner: XamlPropertyIndex = XamlPropertyIndex(410i32);
    pub const GeometryGroup_Children: XamlPropertyIndex = XamlPropertyIndex(411i32);
    pub const GeometryGroup_FillRule: XamlPropertyIndex = XamlPropertyIndex(412i32);
    pub const GradientBrush_ColorInterpolationMode: XamlPropertyIndex = XamlPropertyIndex(413i32);
    pub const GradientBrush_GradientStops: XamlPropertyIndex = XamlPropertyIndex(414i32);
    pub const GradientBrush_MappingMode: XamlPropertyIndex = XamlPropertyIndex(415i32);
    pub const GradientBrush_SpreadMethod: XamlPropertyIndex = XamlPropertyIndex(416i32);
    pub const GridViewItemTemplateSettings_DragItemsCount: XamlPropertyIndex = XamlPropertyIndex(417i32);
    pub const ItemAutomationPeer_Item: XamlPropertyIndex = XamlPropertyIndex(419i32);
    pub const ItemAutomationPeer_ItemsControlAutomationPeer: XamlPropertyIndex = XamlPropertyIndex(420i32);
    pub const LineGeometry_EndPoint: XamlPropertyIndex = XamlPropertyIndex(422i32);
    pub const LineGeometry_StartPoint: XamlPropertyIndex = XamlPropertyIndex(423i32);
    pub const LineSegment_Point: XamlPropertyIndex = XamlPropertyIndex(424i32);
    pub const ListViewItemTemplateSettings_DragItemsCount: XamlPropertyIndex = XamlPropertyIndex(425i32);
    pub const Matrix3DProjection_ProjectionMatrix: XamlPropertyIndex = XamlPropertyIndex(426i32);
    pub const MenuFlyout_Items: XamlPropertyIndex = XamlPropertyIndex(427i32);
    pub const MenuFlyout_MenuFlyoutPresenterStyle: XamlPropertyIndex = XamlPropertyIndex(428i32);
    pub const ObjectAnimationUsingKeyFrames_EnableDependentAnimation: XamlPropertyIndex = XamlPropertyIndex(429i32);
    pub const ObjectAnimationUsingKeyFrames_KeyFrames: XamlPropertyIndex = XamlPropertyIndex(430i32);
    pub const PaneThemeTransition_Edge: XamlPropertyIndex = XamlPropertyIndex(431i32);
    pub const PathGeometry_Figures: XamlPropertyIndex = XamlPropertyIndex(432i32);
    pub const PathGeometry_FillRule: XamlPropertyIndex = XamlPropertyIndex(433i32);
    pub const PlaneProjection_CenterOfRotationX: XamlPropertyIndex = XamlPropertyIndex(434i32);
    pub const PlaneProjection_CenterOfRotationY: XamlPropertyIndex = XamlPropertyIndex(435i32);
    pub const PlaneProjection_CenterOfRotationZ: XamlPropertyIndex = XamlPropertyIndex(436i32);
    pub const PlaneProjection_GlobalOffsetX: XamlPropertyIndex = XamlPropertyIndex(437i32);
    pub const PlaneProjection_GlobalOffsetY: XamlPropertyIndex = XamlPropertyIndex(438i32);
    pub const PlaneProjection_GlobalOffsetZ: XamlPropertyIndex = XamlPropertyIndex(439i32);
    pub const PlaneProjection_LocalOffsetX: XamlPropertyIndex = XamlPropertyIndex(440i32);
    pub const PlaneProjection_LocalOffsetY: XamlPropertyIndex = XamlPropertyIndex(441i32);
    pub const PlaneProjection_LocalOffsetZ: XamlPropertyIndex = XamlPropertyIndex(442i32);
    pub const PlaneProjection_ProjectionMatrix: XamlPropertyIndex = XamlPropertyIndex(443i32);
    pub const PlaneProjection_RotationX: XamlPropertyIndex = XamlPropertyIndex(444i32);
    pub const PlaneProjection_RotationY: XamlPropertyIndex = XamlPropertyIndex(445i32);
    pub const PlaneProjection_RotationZ: XamlPropertyIndex = XamlPropertyIndex(446i32);
    pub const PointAnimation_By: XamlPropertyIndex = XamlPropertyIndex(447i32);
    pub const PointAnimation_EasingFunction: XamlPropertyIndex = XamlPropertyIndex(448i32);
    pub const PointAnimation_EnableDependentAnimation: XamlPropertyIndex = XamlPropertyIndex(449i32);
    pub const PointAnimation_From: XamlPropertyIndex = XamlPropertyIndex(450i32);
    pub const PointAnimation_To: XamlPropertyIndex = XamlPropertyIndex(451i32);
    pub const PointAnimationUsingKeyFrames_EnableDependentAnimation: XamlPropertyIndex = XamlPropertyIndex(452i32);
    pub const PointAnimationUsingKeyFrames_KeyFrames: XamlPropertyIndex = XamlPropertyIndex(453i32);
    pub const PolyBezierSegment_Points: XamlPropertyIndex = XamlPropertyIndex(456i32);
    pub const PolyLineSegment_Points: XamlPropertyIndex = XamlPropertyIndex(457i32);
    pub const PolyQuadraticBezierSegment_Points: XamlPropertyIndex = XamlPropertyIndex(458i32);
    pub const PopupThemeTransition_FromHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(459i32);
    pub const PopupThemeTransition_FromVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(460i32);
    pub const PowerEase_Power: XamlPropertyIndex = XamlPropertyIndex(461i32);
    pub const QuadraticBezierSegment_Point1: XamlPropertyIndex = XamlPropertyIndex(466i32);
    pub const QuadraticBezierSegment_Point2: XamlPropertyIndex = XamlPropertyIndex(467i32);
    pub const RectangleGeometry_Rect: XamlPropertyIndex = XamlPropertyIndex(470i32);
    pub const RelativeSource_Mode: XamlPropertyIndex = XamlPropertyIndex(471i32);
    pub const RenderTargetBitmap_PixelHeight: XamlPropertyIndex = XamlPropertyIndex(472i32);
    pub const RenderTargetBitmap_PixelWidth: XamlPropertyIndex = XamlPropertyIndex(473i32);
    pub const Setter_Property: XamlPropertyIndex = XamlPropertyIndex(474i32);
    pub const Setter_Value: XamlPropertyIndex = XamlPropertyIndex(475i32);
    pub const SolidColorBrush_Color: XamlPropertyIndex = XamlPropertyIndex(476i32);
    pub const SplineColorKeyFrame_KeySpline: XamlPropertyIndex = XamlPropertyIndex(477i32);
    pub const SplineDoubleKeyFrame_KeySpline: XamlPropertyIndex = XamlPropertyIndex(478i32);
    pub const SplinePointKeyFrame_KeySpline: XamlPropertyIndex = XamlPropertyIndex(479i32);
    pub const TileBrush_AlignmentX: XamlPropertyIndex = XamlPropertyIndex(483i32);
    pub const TileBrush_AlignmentY: XamlPropertyIndex = XamlPropertyIndex(484i32);
    pub const TileBrush_Stretch: XamlPropertyIndex = XamlPropertyIndex(485i32);
    pub const Binding_Converter: XamlPropertyIndex = XamlPropertyIndex(487i32);
    pub const Binding_ConverterLanguage: XamlPropertyIndex = XamlPropertyIndex(488i32);
    pub const Binding_ConverterParameter: XamlPropertyIndex = XamlPropertyIndex(489i32);
    pub const Binding_ElementName: XamlPropertyIndex = XamlPropertyIndex(490i32);
    pub const Binding_FallbackValue: XamlPropertyIndex = XamlPropertyIndex(491i32);
    pub const Binding_Mode: XamlPropertyIndex = XamlPropertyIndex(492i32);
    pub const Binding_Path: XamlPropertyIndex = XamlPropertyIndex(493i32);
    pub const Binding_RelativeSource: XamlPropertyIndex = XamlPropertyIndex(494i32);
    pub const Binding_Source: XamlPropertyIndex = XamlPropertyIndex(495i32);
    pub const Binding_TargetNullValue: XamlPropertyIndex = XamlPropertyIndex(496i32);
    pub const Binding_UpdateSourceTrigger: XamlPropertyIndex = XamlPropertyIndex(497i32);
    pub const BitmapImage_CreateOptions: XamlPropertyIndex = XamlPropertyIndex(498i32);
    pub const BitmapImage_DecodePixelHeight: XamlPropertyIndex = XamlPropertyIndex(499i32);
    pub const BitmapImage_DecodePixelType: XamlPropertyIndex = XamlPropertyIndex(500i32);
    pub const BitmapImage_DecodePixelWidth: XamlPropertyIndex = XamlPropertyIndex(501i32);
    pub const BitmapImage_UriSource: XamlPropertyIndex = XamlPropertyIndex(502i32);
    pub const Border_Background: XamlPropertyIndex = XamlPropertyIndex(503i32);
    pub const Border_BorderBrush: XamlPropertyIndex = XamlPropertyIndex(504i32);
    pub const Border_BorderThickness: XamlPropertyIndex = XamlPropertyIndex(505i32);
    pub const Border_Child: XamlPropertyIndex = XamlPropertyIndex(506i32);
    pub const Border_ChildTransitions: XamlPropertyIndex = XamlPropertyIndex(507i32);
    pub const Border_CornerRadius: XamlPropertyIndex = XamlPropertyIndex(508i32);
    pub const Border_Padding: XamlPropertyIndex = XamlPropertyIndex(509i32);
    pub const CaptureElement_Source: XamlPropertyIndex = XamlPropertyIndex(510i32);
    pub const CaptureElement_Stretch: XamlPropertyIndex = XamlPropertyIndex(511i32);
    pub const CompositeTransform_CenterX: XamlPropertyIndex = XamlPropertyIndex(514i32);
    pub const CompositeTransform_CenterY: XamlPropertyIndex = XamlPropertyIndex(515i32);
    pub const CompositeTransform_Rotation: XamlPropertyIndex = XamlPropertyIndex(516i32);
    pub const CompositeTransform_ScaleX: XamlPropertyIndex = XamlPropertyIndex(517i32);
    pub const CompositeTransform_ScaleY: XamlPropertyIndex = XamlPropertyIndex(518i32);
    pub const CompositeTransform_SkewX: XamlPropertyIndex = XamlPropertyIndex(519i32);
    pub const CompositeTransform_SkewY: XamlPropertyIndex = XamlPropertyIndex(520i32);
    pub const CompositeTransform_TranslateX: XamlPropertyIndex = XamlPropertyIndex(521i32);
    pub const CompositeTransform_TranslateY: XamlPropertyIndex = XamlPropertyIndex(522i32);
    pub const ContentPresenter_CharacterSpacing: XamlPropertyIndex = XamlPropertyIndex(523i32);
    pub const ContentPresenter_Content: XamlPropertyIndex = XamlPropertyIndex(524i32);
    pub const ContentPresenter_ContentTemplate: XamlPropertyIndex = XamlPropertyIndex(525i32);
    pub const ContentPresenter_ContentTemplateSelector: XamlPropertyIndex = XamlPropertyIndex(526i32);
    pub const ContentPresenter_ContentTransitions: XamlPropertyIndex = XamlPropertyIndex(527i32);
    pub const ContentPresenter_FontFamily: XamlPropertyIndex = XamlPropertyIndex(528i32);
    pub const ContentPresenter_FontSize: XamlPropertyIndex = XamlPropertyIndex(529i32);
    pub const ContentPresenter_FontStretch: XamlPropertyIndex = XamlPropertyIndex(530i32);
    pub const ContentPresenter_FontStyle: XamlPropertyIndex = XamlPropertyIndex(531i32);
    pub const ContentPresenter_FontWeight: XamlPropertyIndex = XamlPropertyIndex(532i32);
    pub const ContentPresenter_Foreground: XamlPropertyIndex = XamlPropertyIndex(533i32);
    pub const ContentPresenter_IsTextScaleFactorEnabled: XamlPropertyIndex = XamlPropertyIndex(534i32);
    pub const ContentPresenter_LineStackingStrategy: XamlPropertyIndex = XamlPropertyIndex(535i32);
    pub const ContentPresenter_MaxLines: XamlPropertyIndex = XamlPropertyIndex(536i32);
    pub const ContentPresenter_OpticalMarginAlignment: XamlPropertyIndex = XamlPropertyIndex(537i32);
    pub const ContentPresenter_TextLineBounds: XamlPropertyIndex = XamlPropertyIndex(539i32);
    pub const ContentPresenter_TextWrapping: XamlPropertyIndex = XamlPropertyIndex(540i32);
    pub const Control_Background: XamlPropertyIndex = XamlPropertyIndex(541i32);
    pub const Control_BorderBrush: XamlPropertyIndex = XamlPropertyIndex(542i32);
    pub const Control_BorderThickness: XamlPropertyIndex = XamlPropertyIndex(543i32);
    pub const Control_CharacterSpacing: XamlPropertyIndex = XamlPropertyIndex(544i32);
    pub const Control_FocusState: XamlPropertyIndex = XamlPropertyIndex(546i32);
    pub const Control_FontFamily: XamlPropertyIndex = XamlPropertyIndex(547i32);
    pub const Control_FontSize: XamlPropertyIndex = XamlPropertyIndex(548i32);
    pub const Control_FontStretch: XamlPropertyIndex = XamlPropertyIndex(549i32);
    pub const Control_FontStyle: XamlPropertyIndex = XamlPropertyIndex(550i32);
    pub const Control_FontWeight: XamlPropertyIndex = XamlPropertyIndex(551i32);
    pub const Control_Foreground: XamlPropertyIndex = XamlPropertyIndex(552i32);
    pub const Control_HorizontalContentAlignment: XamlPropertyIndex = XamlPropertyIndex(553i32);
    pub const Control_IsEnabled: XamlPropertyIndex = XamlPropertyIndex(554i32);
    pub const Control_IsTabStop: XamlPropertyIndex = XamlPropertyIndex(555i32);
    pub const Control_IsTextScaleFactorEnabled: XamlPropertyIndex = XamlPropertyIndex(556i32);
    pub const Control_Padding: XamlPropertyIndex = XamlPropertyIndex(557i32);
    pub const Control_TabIndex: XamlPropertyIndex = XamlPropertyIndex(558i32);
    pub const Control_TabNavigation: XamlPropertyIndex = XamlPropertyIndex(559i32);
    pub const Control_Template: XamlPropertyIndex = XamlPropertyIndex(560i32);
    pub const Control_VerticalContentAlignment: XamlPropertyIndex = XamlPropertyIndex(561i32);
    pub const DragItemThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(565i32);
    pub const DragOverThemeAnimation_Direction: XamlPropertyIndex = XamlPropertyIndex(566i32);
    pub const DragOverThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(567i32);
    pub const DragOverThemeAnimation_ToOffset: XamlPropertyIndex = XamlPropertyIndex(568i32);
    pub const DropTargetItemThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(569i32);
    pub const FadeInThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(570i32);
    pub const FadeOutThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(571i32);
    pub const Glyphs_Fill: XamlPropertyIndex = XamlPropertyIndex(574i32);
    pub const Glyphs_FontRenderingEmSize: XamlPropertyIndex = XamlPropertyIndex(575i32);
    pub const Glyphs_FontUri: XamlPropertyIndex = XamlPropertyIndex(576i32);
    pub const Glyphs_Indices: XamlPropertyIndex = XamlPropertyIndex(577i32);
    pub const Glyphs_OriginX: XamlPropertyIndex = XamlPropertyIndex(578i32);
    pub const Glyphs_OriginY: XamlPropertyIndex = XamlPropertyIndex(579i32);
    pub const Glyphs_StyleSimulations: XamlPropertyIndex = XamlPropertyIndex(580i32);
    pub const Glyphs_UnicodeString: XamlPropertyIndex = XamlPropertyIndex(581i32);
    pub const IconElement_Foreground: XamlPropertyIndex = XamlPropertyIndex(584i32);
    pub const Image_NineGrid: XamlPropertyIndex = XamlPropertyIndex(586i32);
    pub const Image_PlayToSource: XamlPropertyIndex = XamlPropertyIndex(587i32);
    pub const Image_Source: XamlPropertyIndex = XamlPropertyIndex(588i32);
    pub const Image_Stretch: XamlPropertyIndex = XamlPropertyIndex(589i32);
    pub const ImageBrush_ImageSource: XamlPropertyIndex = XamlPropertyIndex(591i32);
    pub const InlineUIContainer_Child: XamlPropertyIndex = XamlPropertyIndex(592i32);
    pub const ItemsPresenter_Footer: XamlPropertyIndex = XamlPropertyIndex(594i32);
    pub const ItemsPresenter_FooterTemplate: XamlPropertyIndex = XamlPropertyIndex(595i32);
    pub const ItemsPresenter_FooterTransitions: XamlPropertyIndex = XamlPropertyIndex(596i32);
    pub const ItemsPresenter_Header: XamlPropertyIndex = XamlPropertyIndex(597i32);
    pub const ItemsPresenter_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(598i32);
    pub const ItemsPresenter_HeaderTransitions: XamlPropertyIndex = XamlPropertyIndex(599i32);
    pub const ItemsPresenter_Padding: XamlPropertyIndex = XamlPropertyIndex(601i32);
    pub const LinearGradientBrush_EndPoint: XamlPropertyIndex = XamlPropertyIndex(602i32);
    pub const LinearGradientBrush_StartPoint: XamlPropertyIndex = XamlPropertyIndex(603i32);
    pub const MatrixTransform_Matrix: XamlPropertyIndex = XamlPropertyIndex(604i32);
    pub const MediaElement_ActualStereo3DVideoPackingMode: XamlPropertyIndex = XamlPropertyIndex(605i32);
    pub const MediaElement_AreTransportControlsEnabled: XamlPropertyIndex = XamlPropertyIndex(606i32);
    pub const MediaElement_AspectRatioHeight: XamlPropertyIndex = XamlPropertyIndex(607i32);
    pub const MediaElement_AspectRatioWidth: XamlPropertyIndex = XamlPropertyIndex(608i32);
    pub const MediaElement_AudioCategory: XamlPropertyIndex = XamlPropertyIndex(609i32);
    pub const MediaElement_AudioDeviceType: XamlPropertyIndex = XamlPropertyIndex(610i32);
    pub const MediaElement_AudioStreamCount: XamlPropertyIndex = XamlPropertyIndex(611i32);
    pub const MediaElement_AudioStreamIndex: XamlPropertyIndex = XamlPropertyIndex(612i32);
    pub const MediaElement_AutoPlay: XamlPropertyIndex = XamlPropertyIndex(613i32);
    pub const MediaElement_Balance: XamlPropertyIndex = XamlPropertyIndex(614i32);
    pub const MediaElement_BufferingProgress: XamlPropertyIndex = XamlPropertyIndex(615i32);
    pub const MediaElement_CanPause: XamlPropertyIndex = XamlPropertyIndex(616i32);
    pub const MediaElement_CanSeek: XamlPropertyIndex = XamlPropertyIndex(617i32);
    pub const MediaElement_CurrentState: XamlPropertyIndex = XamlPropertyIndex(618i32);
    pub const MediaElement_DefaultPlaybackRate: XamlPropertyIndex = XamlPropertyIndex(619i32);
    pub const MediaElement_DownloadProgress: XamlPropertyIndex = XamlPropertyIndex(620i32);
    pub const MediaElement_DownloadProgressOffset: XamlPropertyIndex = XamlPropertyIndex(621i32);
    pub const MediaElement_IsAudioOnly: XamlPropertyIndex = XamlPropertyIndex(623i32);
    pub const MediaElement_IsFullWindow: XamlPropertyIndex = XamlPropertyIndex(624i32);
    pub const MediaElement_IsLooping: XamlPropertyIndex = XamlPropertyIndex(625i32);
    pub const MediaElement_IsMuted: XamlPropertyIndex = XamlPropertyIndex(626i32);
    pub const MediaElement_IsStereo3DVideo: XamlPropertyIndex = XamlPropertyIndex(627i32);
    pub const MediaElement_Markers: XamlPropertyIndex = XamlPropertyIndex(628i32);
    pub const MediaElement_NaturalDuration: XamlPropertyIndex = XamlPropertyIndex(629i32);
    pub const MediaElement_NaturalVideoHeight: XamlPropertyIndex = XamlPropertyIndex(630i32);
    pub const MediaElement_NaturalVideoWidth: XamlPropertyIndex = XamlPropertyIndex(631i32);
    pub const MediaElement_PlaybackRate: XamlPropertyIndex = XamlPropertyIndex(632i32);
    pub const MediaElement_PlayToPreferredSourceUri: XamlPropertyIndex = XamlPropertyIndex(633i32);
    pub const MediaElement_PlayToSource: XamlPropertyIndex = XamlPropertyIndex(634i32);
    pub const MediaElement_Position: XamlPropertyIndex = XamlPropertyIndex(635i32);
    pub const MediaElement_PosterSource: XamlPropertyIndex = XamlPropertyIndex(636i32);
    pub const MediaElement_ProtectionManager: XamlPropertyIndex = XamlPropertyIndex(637i32);
    pub const MediaElement_RealTimePlayback: XamlPropertyIndex = XamlPropertyIndex(638i32);
    pub const MediaElement_Source: XamlPropertyIndex = XamlPropertyIndex(639i32);
    pub const MediaElement_Stereo3DVideoPackingMode: XamlPropertyIndex = XamlPropertyIndex(640i32);
    pub const MediaElement_Stereo3DVideoRenderMode: XamlPropertyIndex = XamlPropertyIndex(641i32);
    pub const MediaElement_Stretch: XamlPropertyIndex = XamlPropertyIndex(642i32);
    pub const MediaElement_TransportControls: XamlPropertyIndex = XamlPropertyIndex(643i32);
    pub const MediaElement_Volume: XamlPropertyIndex = XamlPropertyIndex(644i32);
    pub const Panel_Background: XamlPropertyIndex = XamlPropertyIndex(647i32);
    pub const Panel_Children: XamlPropertyIndex = XamlPropertyIndex(648i32);
    pub const Panel_ChildrenTransitions: XamlPropertyIndex = XamlPropertyIndex(649i32);
    pub const Panel_IsItemsHost: XamlPropertyIndex = XamlPropertyIndex(651i32);
    pub const Paragraph_Inlines: XamlPropertyIndex = XamlPropertyIndex(652i32);
    pub const Paragraph_TextIndent: XamlPropertyIndex = XamlPropertyIndex(653i32);
    pub const PointerDownThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(660i32);
    pub const PointerUpThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(662i32);
    pub const PopInThemeAnimation_FromHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(664i32);
    pub const PopInThemeAnimation_FromVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(665i32);
    pub const PopInThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(666i32);
    pub const PopOutThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(667i32);
    pub const Popup_Child: XamlPropertyIndex = XamlPropertyIndex(668i32);
    pub const Popup_ChildTransitions: XamlPropertyIndex = XamlPropertyIndex(669i32);
    pub const Popup_HorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(670i32);
    pub const Popup_IsLightDismissEnabled: XamlPropertyIndex = XamlPropertyIndex(673i32);
    pub const Popup_IsOpen: XamlPropertyIndex = XamlPropertyIndex(674i32);
    pub const Popup_VerticalOffset: XamlPropertyIndex = XamlPropertyIndex(676i32);
    pub const RepositionThemeAnimation_FromHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(683i32);
    pub const RepositionThemeAnimation_FromVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(684i32);
    pub const RepositionThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(685i32);
    pub const ResourceDictionary_MergedDictionaries: XamlPropertyIndex = XamlPropertyIndex(687i32);
    pub const ResourceDictionary_Source: XamlPropertyIndex = XamlPropertyIndex(688i32);
    pub const ResourceDictionary_ThemeDictionaries: XamlPropertyIndex = XamlPropertyIndex(689i32);
    pub const RichTextBlock_Blocks: XamlPropertyIndex = XamlPropertyIndex(691i32);
    pub const RichTextBlock_CharacterSpacing: XamlPropertyIndex = XamlPropertyIndex(692i32);
    pub const RichTextBlock_FontFamily: XamlPropertyIndex = XamlPropertyIndex(693i32);
    pub const RichTextBlock_FontSize: XamlPropertyIndex = XamlPropertyIndex(694i32);
    pub const RichTextBlock_FontStretch: XamlPropertyIndex = XamlPropertyIndex(695i32);
    pub const RichTextBlock_FontStyle: XamlPropertyIndex = XamlPropertyIndex(696i32);
    pub const RichTextBlock_FontWeight: XamlPropertyIndex = XamlPropertyIndex(697i32);
    pub const RichTextBlock_Foreground: XamlPropertyIndex = XamlPropertyIndex(698i32);
    pub const RichTextBlock_HasOverflowContent: XamlPropertyIndex = XamlPropertyIndex(699i32);
    pub const RichTextBlock_IsColorFontEnabled: XamlPropertyIndex = XamlPropertyIndex(700i32);
    pub const RichTextBlock_IsTextScaleFactorEnabled: XamlPropertyIndex = XamlPropertyIndex(701i32);
    pub const RichTextBlock_IsTextSelectionEnabled: XamlPropertyIndex = XamlPropertyIndex(702i32);
    pub const RichTextBlock_LineHeight: XamlPropertyIndex = XamlPropertyIndex(703i32);
    pub const RichTextBlock_LineStackingStrategy: XamlPropertyIndex = XamlPropertyIndex(704i32);
    pub const RichTextBlock_MaxLines: XamlPropertyIndex = XamlPropertyIndex(705i32);
    pub const RichTextBlock_OpticalMarginAlignment: XamlPropertyIndex = XamlPropertyIndex(706i32);
    pub const RichTextBlock_OverflowContentTarget: XamlPropertyIndex = XamlPropertyIndex(707i32);
    pub const RichTextBlock_Padding: XamlPropertyIndex = XamlPropertyIndex(708i32);
    pub const RichTextBlock_SelectedText: XamlPropertyIndex = XamlPropertyIndex(709i32);
    pub const RichTextBlock_SelectionHighlightColor: XamlPropertyIndex = XamlPropertyIndex(710i32);
    pub const RichTextBlock_TextAlignment: XamlPropertyIndex = XamlPropertyIndex(711i32);
    pub const RichTextBlock_TextIndent: XamlPropertyIndex = XamlPropertyIndex(712i32);
    pub const RichTextBlock_TextLineBounds: XamlPropertyIndex = XamlPropertyIndex(713i32);
    pub const RichTextBlock_TextReadingOrder: XamlPropertyIndex = XamlPropertyIndex(714i32);
    pub const RichTextBlock_TextTrimming: XamlPropertyIndex = XamlPropertyIndex(715i32);
    pub const RichTextBlock_TextWrapping: XamlPropertyIndex = XamlPropertyIndex(716i32);
    pub const RichTextBlockOverflow_HasOverflowContent: XamlPropertyIndex = XamlPropertyIndex(717i32);
    pub const RichTextBlockOverflow_MaxLines: XamlPropertyIndex = XamlPropertyIndex(718i32);
    pub const RichTextBlockOverflow_OverflowContentTarget: XamlPropertyIndex = XamlPropertyIndex(719i32);
    pub const RichTextBlockOverflow_Padding: XamlPropertyIndex = XamlPropertyIndex(720i32);
    pub const RotateTransform_Angle: XamlPropertyIndex = XamlPropertyIndex(721i32);
    pub const RotateTransform_CenterX: XamlPropertyIndex = XamlPropertyIndex(722i32);
    pub const RotateTransform_CenterY: XamlPropertyIndex = XamlPropertyIndex(723i32);
    pub const Run_FlowDirection: XamlPropertyIndex = XamlPropertyIndex(725i32);
    pub const Run_Text: XamlPropertyIndex = XamlPropertyIndex(726i32);
    pub const ScaleTransform_CenterX: XamlPropertyIndex = XamlPropertyIndex(727i32);
    pub const ScaleTransform_CenterY: XamlPropertyIndex = XamlPropertyIndex(728i32);
    pub const ScaleTransform_ScaleX: XamlPropertyIndex = XamlPropertyIndex(729i32);
    pub const ScaleTransform_ScaleY: XamlPropertyIndex = XamlPropertyIndex(730i32);
    pub const SetterBaseCollection_IsSealed: XamlPropertyIndex = XamlPropertyIndex(732i32);
    pub const Shape_Fill: XamlPropertyIndex = XamlPropertyIndex(733i32);
    pub const Shape_GeometryTransform: XamlPropertyIndex = XamlPropertyIndex(734i32);
    pub const Shape_Stretch: XamlPropertyIndex = XamlPropertyIndex(735i32);
    pub const Shape_Stroke: XamlPropertyIndex = XamlPropertyIndex(736i32);
    pub const Shape_StrokeDashArray: XamlPropertyIndex = XamlPropertyIndex(737i32);
    pub const Shape_StrokeDashCap: XamlPropertyIndex = XamlPropertyIndex(738i32);
    pub const Shape_StrokeDashOffset: XamlPropertyIndex = XamlPropertyIndex(739i32);
    pub const Shape_StrokeEndLineCap: XamlPropertyIndex = XamlPropertyIndex(740i32);
    pub const Shape_StrokeLineJoin: XamlPropertyIndex = XamlPropertyIndex(741i32);
    pub const Shape_StrokeMiterLimit: XamlPropertyIndex = XamlPropertyIndex(742i32);
    pub const Shape_StrokeStartLineCap: XamlPropertyIndex = XamlPropertyIndex(743i32);
    pub const Shape_StrokeThickness: XamlPropertyIndex = XamlPropertyIndex(744i32);
    pub const SkewTransform_AngleX: XamlPropertyIndex = XamlPropertyIndex(745i32);
    pub const SkewTransform_AngleY: XamlPropertyIndex = XamlPropertyIndex(746i32);
    pub const SkewTransform_CenterX: XamlPropertyIndex = XamlPropertyIndex(747i32);
    pub const SkewTransform_CenterY: XamlPropertyIndex = XamlPropertyIndex(748i32);
    pub const Span_Inlines: XamlPropertyIndex = XamlPropertyIndex(749i32);
    pub const SplitCloseThemeAnimation_ClosedLength: XamlPropertyIndex = XamlPropertyIndex(750i32);
    pub const SplitCloseThemeAnimation_ClosedTarget: XamlPropertyIndex = XamlPropertyIndex(751i32);
    pub const SplitCloseThemeAnimation_ClosedTargetName: XamlPropertyIndex = XamlPropertyIndex(752i32);
    pub const SplitCloseThemeAnimation_ContentTarget: XamlPropertyIndex = XamlPropertyIndex(753i32);
    pub const SplitCloseThemeAnimation_ContentTargetName: XamlPropertyIndex = XamlPropertyIndex(754i32);
    pub const SplitCloseThemeAnimation_ContentTranslationDirection: XamlPropertyIndex = XamlPropertyIndex(755i32);
    pub const SplitCloseThemeAnimation_ContentTranslationOffset: XamlPropertyIndex = XamlPropertyIndex(756i32);
    pub const SplitCloseThemeAnimation_OffsetFromCenter: XamlPropertyIndex = XamlPropertyIndex(757i32);
    pub const SplitCloseThemeAnimation_OpenedLength: XamlPropertyIndex = XamlPropertyIndex(758i32);
    pub const SplitCloseThemeAnimation_OpenedTarget: XamlPropertyIndex = XamlPropertyIndex(759i32);
    pub const SplitCloseThemeAnimation_OpenedTargetName: XamlPropertyIndex = XamlPropertyIndex(760i32);
    pub const SplitOpenThemeAnimation_ClosedLength: XamlPropertyIndex = XamlPropertyIndex(761i32);
    pub const SplitOpenThemeAnimation_ClosedTarget: XamlPropertyIndex = XamlPropertyIndex(762i32);
    pub const SplitOpenThemeAnimation_ClosedTargetName: XamlPropertyIndex = XamlPropertyIndex(763i32);
    pub const SplitOpenThemeAnimation_ContentTarget: XamlPropertyIndex = XamlPropertyIndex(764i32);
    pub const SplitOpenThemeAnimation_ContentTargetName: XamlPropertyIndex = XamlPropertyIndex(765i32);
    pub const SplitOpenThemeAnimation_ContentTranslationDirection: XamlPropertyIndex = XamlPropertyIndex(766i32);
    pub const SplitOpenThemeAnimation_ContentTranslationOffset: XamlPropertyIndex = XamlPropertyIndex(767i32);
    pub const SplitOpenThemeAnimation_OffsetFromCenter: XamlPropertyIndex = XamlPropertyIndex(768i32);
    pub const SplitOpenThemeAnimation_OpenedLength: XamlPropertyIndex = XamlPropertyIndex(769i32);
    pub const SplitOpenThemeAnimation_OpenedTarget: XamlPropertyIndex = XamlPropertyIndex(770i32);
    pub const SplitOpenThemeAnimation_OpenedTargetName: XamlPropertyIndex = XamlPropertyIndex(771i32);
    pub const Storyboard_Children: XamlPropertyIndex = XamlPropertyIndex(772i32);
    pub const Storyboard_TargetName: XamlPropertyIndex = XamlPropertyIndex(774i32);
    pub const Storyboard_TargetProperty: XamlPropertyIndex = XamlPropertyIndex(775i32);
    pub const SwipeBackThemeAnimation_FromHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(776i32);
    pub const SwipeBackThemeAnimation_FromVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(777i32);
    pub const SwipeBackThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(778i32);
    pub const SwipeHintThemeAnimation_TargetName: XamlPropertyIndex = XamlPropertyIndex(779i32);
    pub const SwipeHintThemeAnimation_ToHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(780i32);
    pub const SwipeHintThemeAnimation_ToVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(781i32);
    pub const TextBlock_CharacterSpacing: XamlPropertyIndex = XamlPropertyIndex(782i32);
    pub const TextBlock_FontFamily: XamlPropertyIndex = XamlPropertyIndex(783i32);
    pub const TextBlock_FontSize: XamlPropertyIndex = XamlPropertyIndex(784i32);
    pub const TextBlock_FontStretch: XamlPropertyIndex = XamlPropertyIndex(785i32);
    pub const TextBlock_FontStyle: XamlPropertyIndex = XamlPropertyIndex(786i32);
    pub const TextBlock_FontWeight: XamlPropertyIndex = XamlPropertyIndex(787i32);
    pub const TextBlock_Foreground: XamlPropertyIndex = XamlPropertyIndex(788i32);
    pub const TextBlock_Inlines: XamlPropertyIndex = XamlPropertyIndex(789i32);
    pub const TextBlock_IsColorFontEnabled: XamlPropertyIndex = XamlPropertyIndex(790i32);
    pub const TextBlock_IsTextScaleFactorEnabled: XamlPropertyIndex = XamlPropertyIndex(791i32);
    pub const TextBlock_IsTextSelectionEnabled: XamlPropertyIndex = XamlPropertyIndex(792i32);
    pub const TextBlock_LineHeight: XamlPropertyIndex = XamlPropertyIndex(793i32);
    pub const TextBlock_LineStackingStrategy: XamlPropertyIndex = XamlPropertyIndex(794i32);
    pub const TextBlock_MaxLines: XamlPropertyIndex = XamlPropertyIndex(795i32);
    pub const TextBlock_OpticalMarginAlignment: XamlPropertyIndex = XamlPropertyIndex(796i32);
    pub const TextBlock_Padding: XamlPropertyIndex = XamlPropertyIndex(797i32);
    pub const TextBlock_SelectedText: XamlPropertyIndex = XamlPropertyIndex(798i32);
    pub const TextBlock_SelectionHighlightColor: XamlPropertyIndex = XamlPropertyIndex(799i32);
    pub const TextBlock_Text: XamlPropertyIndex = XamlPropertyIndex(800i32);
    pub const TextBlock_TextAlignment: XamlPropertyIndex = XamlPropertyIndex(801i32);
    pub const TextBlock_TextDecorations: XamlPropertyIndex = XamlPropertyIndex(802i32);
    pub const TextBlock_TextLineBounds: XamlPropertyIndex = XamlPropertyIndex(803i32);
    pub const TextBlock_TextReadingOrder: XamlPropertyIndex = XamlPropertyIndex(804i32);
    pub const TextBlock_TextTrimming: XamlPropertyIndex = XamlPropertyIndex(805i32);
    pub const TextBlock_TextWrapping: XamlPropertyIndex = XamlPropertyIndex(806i32);
    pub const TransformGroup_Children: XamlPropertyIndex = XamlPropertyIndex(811i32);
    pub const TransformGroup_Value: XamlPropertyIndex = XamlPropertyIndex(812i32);
    pub const TranslateTransform_X: XamlPropertyIndex = XamlPropertyIndex(814i32);
    pub const TranslateTransform_Y: XamlPropertyIndex = XamlPropertyIndex(815i32);
    pub const Viewbox_Child: XamlPropertyIndex = XamlPropertyIndex(819i32);
    pub const Viewbox_Stretch: XamlPropertyIndex = XamlPropertyIndex(820i32);
    pub const Viewbox_StretchDirection: XamlPropertyIndex = XamlPropertyIndex(821i32);
    pub const WebViewBrush_SourceName: XamlPropertyIndex = XamlPropertyIndex(825i32);
    pub const AppBarSeparator_IsCompact: XamlPropertyIndex = XamlPropertyIndex(826i32);
    pub const BitmapIcon_UriSource: XamlPropertyIndex = XamlPropertyIndex(827i32);
    pub const Canvas_Left: XamlPropertyIndex = XamlPropertyIndex(828i32);
    pub const Canvas_Top: XamlPropertyIndex = XamlPropertyIndex(829i32);
    pub const Canvas_ZIndex: XamlPropertyIndex = XamlPropertyIndex(830i32);
    pub const ContentControl_Content: XamlPropertyIndex = XamlPropertyIndex(832i32);
    pub const ContentControl_ContentTemplate: XamlPropertyIndex = XamlPropertyIndex(833i32);
    pub const ContentControl_ContentTemplateSelector: XamlPropertyIndex = XamlPropertyIndex(834i32);
    pub const ContentControl_ContentTransitions: XamlPropertyIndex = XamlPropertyIndex(835i32);
    pub const DatePicker_CalendarIdentifier: XamlPropertyIndex = XamlPropertyIndex(837i32);
    pub const DatePicker_Date: XamlPropertyIndex = XamlPropertyIndex(838i32);
    pub const DatePicker_DayFormat: XamlPropertyIndex = XamlPropertyIndex(839i32);
    pub const DatePicker_DayVisible: XamlPropertyIndex = XamlPropertyIndex(840i32);
    pub const DatePicker_Header: XamlPropertyIndex = XamlPropertyIndex(841i32);
    pub const DatePicker_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(842i32);
    pub const DatePicker_MaxYear: XamlPropertyIndex = XamlPropertyIndex(843i32);
    pub const DatePicker_MinYear: XamlPropertyIndex = XamlPropertyIndex(844i32);
    pub const DatePicker_MonthFormat: XamlPropertyIndex = XamlPropertyIndex(845i32);
    pub const DatePicker_MonthVisible: XamlPropertyIndex = XamlPropertyIndex(846i32);
    pub const DatePicker_Orientation: XamlPropertyIndex = XamlPropertyIndex(847i32);
    pub const DatePicker_YearFormat: XamlPropertyIndex = XamlPropertyIndex(848i32);
    pub const DatePicker_YearVisible: XamlPropertyIndex = XamlPropertyIndex(849i32);
    pub const FontIcon_FontFamily: XamlPropertyIndex = XamlPropertyIndex(851i32);
    pub const FontIcon_FontSize: XamlPropertyIndex = XamlPropertyIndex(852i32);
    pub const FontIcon_FontStyle: XamlPropertyIndex = XamlPropertyIndex(853i32);
    pub const FontIcon_FontWeight: XamlPropertyIndex = XamlPropertyIndex(854i32);
    pub const FontIcon_Glyph: XamlPropertyIndex = XamlPropertyIndex(855i32);
    pub const FontIcon_IsTextScaleFactorEnabled: XamlPropertyIndex = XamlPropertyIndex(856i32);
    pub const Grid_Column: XamlPropertyIndex = XamlPropertyIndex(857i32);
    pub const Grid_ColumnDefinitions: XamlPropertyIndex = XamlPropertyIndex(858i32);
    pub const Grid_ColumnSpan: XamlPropertyIndex = XamlPropertyIndex(859i32);
    pub const Grid_Row: XamlPropertyIndex = XamlPropertyIndex(860i32);
    pub const Grid_RowDefinitions: XamlPropertyIndex = XamlPropertyIndex(861i32);
    pub const Grid_RowSpan: XamlPropertyIndex = XamlPropertyIndex(862i32);
    pub const Hub_DefaultSectionIndex: XamlPropertyIndex = XamlPropertyIndex(863i32);
    pub const Hub_Header: XamlPropertyIndex = XamlPropertyIndex(864i32);
    pub const Hub_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(865i32);
    pub const Hub_IsActiveView: XamlPropertyIndex = XamlPropertyIndex(866i32);
    pub const Hub_IsZoomedInView: XamlPropertyIndex = XamlPropertyIndex(867i32);
    pub const Hub_Orientation: XamlPropertyIndex = XamlPropertyIndex(868i32);
    pub const Hub_SectionHeaders: XamlPropertyIndex = XamlPropertyIndex(869i32);
    pub const Hub_Sections: XamlPropertyIndex = XamlPropertyIndex(870i32);
    pub const Hub_SectionsInView: XamlPropertyIndex = XamlPropertyIndex(871i32);
    pub const Hub_SemanticZoomOwner: XamlPropertyIndex = XamlPropertyIndex(872i32);
    pub const HubSection_ContentTemplate: XamlPropertyIndex = XamlPropertyIndex(873i32);
    pub const HubSection_Header: XamlPropertyIndex = XamlPropertyIndex(874i32);
    pub const HubSection_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(875i32);
    pub const HubSection_IsHeaderInteractive: XamlPropertyIndex = XamlPropertyIndex(876i32);
    pub const Hyperlink_NavigateUri: XamlPropertyIndex = XamlPropertyIndex(877i32);
    pub const ItemsControl_DisplayMemberPath: XamlPropertyIndex = XamlPropertyIndex(879i32);
    pub const ItemsControl_GroupStyle: XamlPropertyIndex = XamlPropertyIndex(880i32);
    pub const ItemsControl_GroupStyleSelector: XamlPropertyIndex = XamlPropertyIndex(881i32);
    pub const ItemsControl_IsGrouping: XamlPropertyIndex = XamlPropertyIndex(882i32);
    pub const ItemsControl_ItemContainerStyle: XamlPropertyIndex = XamlPropertyIndex(884i32);
    pub const ItemsControl_ItemContainerStyleSelector: XamlPropertyIndex = XamlPropertyIndex(885i32);
    pub const ItemsControl_ItemContainerTransitions: XamlPropertyIndex = XamlPropertyIndex(886i32);
    pub const ItemsControl_Items: XamlPropertyIndex = XamlPropertyIndex(887i32);
    pub const ItemsControl_ItemsPanel: XamlPropertyIndex = XamlPropertyIndex(889i32);
    pub const ItemsControl_ItemsSource: XamlPropertyIndex = XamlPropertyIndex(890i32);
    pub const ItemsControl_ItemTemplate: XamlPropertyIndex = XamlPropertyIndex(891i32);
    pub const ItemsControl_ItemTemplateSelector: XamlPropertyIndex = XamlPropertyIndex(892i32);
    pub const Line_X1: XamlPropertyIndex = XamlPropertyIndex(893i32);
    pub const Line_X2: XamlPropertyIndex = XamlPropertyIndex(894i32);
    pub const Line_Y1: XamlPropertyIndex = XamlPropertyIndex(895i32);
    pub const Line_Y2: XamlPropertyIndex = XamlPropertyIndex(896i32);
    pub const MediaTransportControls_IsFastForwardButtonVisible: XamlPropertyIndex = XamlPropertyIndex(898i32);
    pub const MediaTransportControls_IsFastRewindButtonVisible: XamlPropertyIndex = XamlPropertyIndex(900i32);
    pub const MediaTransportControls_IsFullWindowButtonVisible: XamlPropertyIndex = XamlPropertyIndex(902i32);
    pub const MediaTransportControls_IsPlaybackRateButtonVisible: XamlPropertyIndex = XamlPropertyIndex(904i32);
    pub const MediaTransportControls_IsSeekBarVisible: XamlPropertyIndex = XamlPropertyIndex(905i32);
    pub const MediaTransportControls_IsStopButtonVisible: XamlPropertyIndex = XamlPropertyIndex(908i32);
    pub const MediaTransportControls_IsVolumeButtonVisible: XamlPropertyIndex = XamlPropertyIndex(910i32);
    pub const MediaTransportControls_IsZoomButtonVisible: XamlPropertyIndex = XamlPropertyIndex(912i32);
    pub const PasswordBox_Header: XamlPropertyIndex = XamlPropertyIndex(913i32);
    pub const PasswordBox_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(914i32);
    pub const PasswordBox_IsPasswordRevealButtonEnabled: XamlPropertyIndex = XamlPropertyIndex(915i32);
    pub const PasswordBox_MaxLength: XamlPropertyIndex = XamlPropertyIndex(916i32);
    pub const PasswordBox_Password: XamlPropertyIndex = XamlPropertyIndex(917i32);
    pub const PasswordBox_PasswordChar: XamlPropertyIndex = XamlPropertyIndex(918i32);
    pub const PasswordBox_PlaceholderText: XamlPropertyIndex = XamlPropertyIndex(919i32);
    pub const PasswordBox_PreventKeyboardDisplayOnProgrammaticFocus: XamlPropertyIndex = XamlPropertyIndex(920i32);
    pub const PasswordBox_SelectionHighlightColor: XamlPropertyIndex = XamlPropertyIndex(921i32);
    pub const Path_Data: XamlPropertyIndex = XamlPropertyIndex(922i32);
    pub const PathIcon_Data: XamlPropertyIndex = XamlPropertyIndex(923i32);
    pub const Polygon_FillRule: XamlPropertyIndex = XamlPropertyIndex(924i32);
    pub const Polygon_Points: XamlPropertyIndex = XamlPropertyIndex(925i32);
    pub const Polyline_FillRule: XamlPropertyIndex = XamlPropertyIndex(926i32);
    pub const Polyline_Points: XamlPropertyIndex = XamlPropertyIndex(927i32);
    pub const ProgressRing_IsActive: XamlPropertyIndex = XamlPropertyIndex(928i32);
    pub const ProgressRing_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(929i32);
    pub const RangeBase_LargeChange: XamlPropertyIndex = XamlPropertyIndex(930i32);
    pub const RangeBase_Maximum: XamlPropertyIndex = XamlPropertyIndex(931i32);
    pub const RangeBase_Minimum: XamlPropertyIndex = XamlPropertyIndex(932i32);
    pub const RangeBase_SmallChange: XamlPropertyIndex = XamlPropertyIndex(933i32);
    pub const RangeBase_Value: XamlPropertyIndex = XamlPropertyIndex(934i32);
    pub const Rectangle_RadiusX: XamlPropertyIndex = XamlPropertyIndex(935i32);
    pub const Rectangle_RadiusY: XamlPropertyIndex = XamlPropertyIndex(936i32);
    pub const RichEditBox_AcceptsReturn: XamlPropertyIndex = XamlPropertyIndex(937i32);
    pub const RichEditBox_Header: XamlPropertyIndex = XamlPropertyIndex(938i32);
    pub const RichEditBox_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(939i32);
    pub const RichEditBox_InputScope: XamlPropertyIndex = XamlPropertyIndex(940i32);
    pub const RichEditBox_IsColorFontEnabled: XamlPropertyIndex = XamlPropertyIndex(941i32);
    pub const RichEditBox_IsReadOnly: XamlPropertyIndex = XamlPropertyIndex(942i32);
    pub const RichEditBox_IsSpellCheckEnabled: XamlPropertyIndex = XamlPropertyIndex(943i32);
    pub const RichEditBox_IsTextPredictionEnabled: XamlPropertyIndex = XamlPropertyIndex(944i32);
    pub const RichEditBox_PlaceholderText: XamlPropertyIndex = XamlPropertyIndex(945i32);
    pub const RichEditBox_PreventKeyboardDisplayOnProgrammaticFocus: XamlPropertyIndex = XamlPropertyIndex(946i32);
    pub const RichEditBox_SelectionHighlightColor: XamlPropertyIndex = XamlPropertyIndex(947i32);
    pub const RichEditBox_TextAlignment: XamlPropertyIndex = XamlPropertyIndex(948i32);
    pub const RichEditBox_TextWrapping: XamlPropertyIndex = XamlPropertyIndex(949i32);
    pub const SearchBox_ChooseSuggestionOnEnter: XamlPropertyIndex = XamlPropertyIndex(950i32);
    pub const SearchBox_FocusOnKeyboardInput: XamlPropertyIndex = XamlPropertyIndex(951i32);
    pub const SearchBox_PlaceholderText: XamlPropertyIndex = XamlPropertyIndex(952i32);
    pub const SearchBox_QueryText: XamlPropertyIndex = XamlPropertyIndex(953i32);
    pub const SearchBox_SearchHistoryContext: XamlPropertyIndex = XamlPropertyIndex(954i32);
    pub const SearchBox_SearchHistoryEnabled: XamlPropertyIndex = XamlPropertyIndex(955i32);
    pub const SemanticZoom_CanChangeViews: XamlPropertyIndex = XamlPropertyIndex(956i32);
    pub const SemanticZoom_IsZoomedInViewActive: XamlPropertyIndex = XamlPropertyIndex(957i32);
    pub const SemanticZoom_IsZoomOutButtonEnabled: XamlPropertyIndex = XamlPropertyIndex(958i32);
    pub const SemanticZoom_ZoomedInView: XamlPropertyIndex = XamlPropertyIndex(959i32);
    pub const SemanticZoom_ZoomedOutView: XamlPropertyIndex = XamlPropertyIndex(960i32);
    pub const StackPanel_AreScrollSnapPointsRegular: XamlPropertyIndex = XamlPropertyIndex(961i32);
    pub const StackPanel_Orientation: XamlPropertyIndex = XamlPropertyIndex(962i32);
    pub const SymbolIcon_Symbol: XamlPropertyIndex = XamlPropertyIndex(963i32);
    pub const TextBox_AcceptsReturn: XamlPropertyIndex = XamlPropertyIndex(964i32);
    pub const TextBox_Header: XamlPropertyIndex = XamlPropertyIndex(965i32);
    pub const TextBox_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(966i32);
    pub const TextBox_InputScope: XamlPropertyIndex = XamlPropertyIndex(967i32);
    pub const TextBox_IsColorFontEnabled: XamlPropertyIndex = XamlPropertyIndex(968i32);
    pub const TextBox_IsReadOnly: XamlPropertyIndex = XamlPropertyIndex(971i32);
    pub const TextBox_IsSpellCheckEnabled: XamlPropertyIndex = XamlPropertyIndex(972i32);
    pub const TextBox_IsTextPredictionEnabled: XamlPropertyIndex = XamlPropertyIndex(973i32);
    pub const TextBox_MaxLength: XamlPropertyIndex = XamlPropertyIndex(974i32);
    pub const TextBox_PlaceholderText: XamlPropertyIndex = XamlPropertyIndex(975i32);
    pub const TextBox_PreventKeyboardDisplayOnProgrammaticFocus: XamlPropertyIndex = XamlPropertyIndex(976i32);
    pub const TextBox_SelectedText: XamlPropertyIndex = XamlPropertyIndex(977i32);
    pub const TextBox_SelectionHighlightColor: XamlPropertyIndex = XamlPropertyIndex(978i32);
    pub const TextBox_SelectionLength: XamlPropertyIndex = XamlPropertyIndex(979i32);
    pub const TextBox_SelectionStart: XamlPropertyIndex = XamlPropertyIndex(980i32);
    pub const TextBox_Text: XamlPropertyIndex = XamlPropertyIndex(981i32);
    pub const TextBox_TextAlignment: XamlPropertyIndex = XamlPropertyIndex(982i32);
    pub const TextBox_TextWrapping: XamlPropertyIndex = XamlPropertyIndex(983i32);
    pub const Thumb_IsDragging: XamlPropertyIndex = XamlPropertyIndex(984i32);
    pub const TickBar_Fill: XamlPropertyIndex = XamlPropertyIndex(985i32);
    pub const TimePicker_ClockIdentifier: XamlPropertyIndex = XamlPropertyIndex(986i32);
    pub const TimePicker_Header: XamlPropertyIndex = XamlPropertyIndex(987i32);
    pub const TimePicker_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(988i32);
    pub const TimePicker_MinuteIncrement: XamlPropertyIndex = XamlPropertyIndex(989i32);
    pub const TimePicker_Time: XamlPropertyIndex = XamlPropertyIndex(990i32);
    pub const ToggleSwitch_Header: XamlPropertyIndex = XamlPropertyIndex(991i32);
    pub const ToggleSwitch_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(992i32);
    pub const ToggleSwitch_IsOn: XamlPropertyIndex = XamlPropertyIndex(993i32);
    pub const ToggleSwitch_OffContent: XamlPropertyIndex = XamlPropertyIndex(994i32);
    pub const ToggleSwitch_OffContentTemplate: XamlPropertyIndex = XamlPropertyIndex(995i32);
    pub const ToggleSwitch_OnContent: XamlPropertyIndex = XamlPropertyIndex(996i32);
    pub const ToggleSwitch_OnContentTemplate: XamlPropertyIndex = XamlPropertyIndex(997i32);
    pub const ToggleSwitch_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(998i32);
    pub const UserControl_Content: XamlPropertyIndex = XamlPropertyIndex(999i32);
    pub const VariableSizedWrapGrid_ColumnSpan: XamlPropertyIndex = XamlPropertyIndex(1000i32);
    pub const VariableSizedWrapGrid_HorizontalChildrenAlignment: XamlPropertyIndex = XamlPropertyIndex(1001i32);
    pub const VariableSizedWrapGrid_ItemHeight: XamlPropertyIndex = XamlPropertyIndex(1002i32);
    pub const VariableSizedWrapGrid_ItemWidth: XamlPropertyIndex = XamlPropertyIndex(1003i32);
    pub const VariableSizedWrapGrid_MaximumRowsOrColumns: XamlPropertyIndex = XamlPropertyIndex(1004i32);
    pub const VariableSizedWrapGrid_Orientation: XamlPropertyIndex = XamlPropertyIndex(1005i32);
    pub const VariableSizedWrapGrid_RowSpan: XamlPropertyIndex = XamlPropertyIndex(1006i32);
    pub const VariableSizedWrapGrid_VerticalChildrenAlignment: XamlPropertyIndex = XamlPropertyIndex(1007i32);
    pub const WebView_AllowedScriptNotifyUris: XamlPropertyIndex = XamlPropertyIndex(1008i32);
    pub const WebView_CanGoBack: XamlPropertyIndex = XamlPropertyIndex(1009i32);
    pub const WebView_CanGoForward: XamlPropertyIndex = XamlPropertyIndex(1010i32);
    pub const WebView_ContainsFullScreenElement: XamlPropertyIndex = XamlPropertyIndex(1011i32);
    pub const WebView_DataTransferPackage: XamlPropertyIndex = XamlPropertyIndex(1012i32);
    pub const WebView_DefaultBackgroundColor: XamlPropertyIndex = XamlPropertyIndex(1013i32);
    pub const WebView_DocumentTitle: XamlPropertyIndex = XamlPropertyIndex(1014i32);
    pub const WebView_Source: XamlPropertyIndex = XamlPropertyIndex(1015i32);
    pub const AppBar_ClosedDisplayMode: XamlPropertyIndex = XamlPropertyIndex(1016i32);
    pub const AppBar_IsOpen: XamlPropertyIndex = XamlPropertyIndex(1017i32);
    pub const AppBar_IsSticky: XamlPropertyIndex = XamlPropertyIndex(1018i32);
    pub const AutoSuggestBox_AutoMaximizeSuggestionArea: XamlPropertyIndex = XamlPropertyIndex(1019i32);
    pub const AutoSuggestBox_Header: XamlPropertyIndex = XamlPropertyIndex(1020i32);
    pub const AutoSuggestBox_IsSuggestionListOpen: XamlPropertyIndex = XamlPropertyIndex(1021i32);
    pub const AutoSuggestBox_MaxSuggestionListHeight: XamlPropertyIndex = XamlPropertyIndex(1022i32);
    pub const AutoSuggestBox_PlaceholderText: XamlPropertyIndex = XamlPropertyIndex(1023i32);
    pub const AutoSuggestBox_Text: XamlPropertyIndex = XamlPropertyIndex(1024i32);
    pub const AutoSuggestBox_TextBoxStyle: XamlPropertyIndex = XamlPropertyIndex(1025i32);
    pub const AutoSuggestBox_TextMemberPath: XamlPropertyIndex = XamlPropertyIndex(1026i32);
    pub const AutoSuggestBox_UpdateTextOnSelect: XamlPropertyIndex = XamlPropertyIndex(1027i32);
    pub const ButtonBase_ClickMode: XamlPropertyIndex = XamlPropertyIndex(1029i32);
    pub const ButtonBase_Command: XamlPropertyIndex = XamlPropertyIndex(1030i32);
    pub const ButtonBase_CommandParameter: XamlPropertyIndex = XamlPropertyIndex(1031i32);
    pub const ButtonBase_IsPointerOver: XamlPropertyIndex = XamlPropertyIndex(1032i32);
    pub const ButtonBase_IsPressed: XamlPropertyIndex = XamlPropertyIndex(1033i32);
    pub const ContentDialog_FullSizeDesired: XamlPropertyIndex = XamlPropertyIndex(1034i32);
    pub const ContentDialog_IsPrimaryButtonEnabled: XamlPropertyIndex = XamlPropertyIndex(1035i32);
    pub const ContentDialog_IsSecondaryButtonEnabled: XamlPropertyIndex = XamlPropertyIndex(1036i32);
    pub const ContentDialog_PrimaryButtonCommand: XamlPropertyIndex = XamlPropertyIndex(1037i32);
    pub const ContentDialog_PrimaryButtonCommandParameter: XamlPropertyIndex = XamlPropertyIndex(1038i32);
    pub const ContentDialog_PrimaryButtonText: XamlPropertyIndex = XamlPropertyIndex(1039i32);
    pub const ContentDialog_SecondaryButtonCommand: XamlPropertyIndex = XamlPropertyIndex(1040i32);
    pub const ContentDialog_SecondaryButtonCommandParameter: XamlPropertyIndex = XamlPropertyIndex(1041i32);
    pub const ContentDialog_SecondaryButtonText: XamlPropertyIndex = XamlPropertyIndex(1042i32);
    pub const ContentDialog_Title: XamlPropertyIndex = XamlPropertyIndex(1043i32);
    pub const ContentDialog_TitleTemplate: XamlPropertyIndex = XamlPropertyIndex(1044i32);
    pub const Frame_BackStack: XamlPropertyIndex = XamlPropertyIndex(1045i32);
    pub const Frame_BackStackDepth: XamlPropertyIndex = XamlPropertyIndex(1046i32);
    pub const Frame_CacheSize: XamlPropertyIndex = XamlPropertyIndex(1047i32);
    pub const Frame_CanGoBack: XamlPropertyIndex = XamlPropertyIndex(1048i32);
    pub const Frame_CanGoForward: XamlPropertyIndex = XamlPropertyIndex(1049i32);
    pub const Frame_CurrentSourcePageType: XamlPropertyIndex = XamlPropertyIndex(1050i32);
    pub const Frame_ForwardStack: XamlPropertyIndex = XamlPropertyIndex(1051i32);
    pub const Frame_SourcePageType: XamlPropertyIndex = XamlPropertyIndex(1052i32);
    pub const GridViewItemPresenter_CheckBrush: XamlPropertyIndex = XamlPropertyIndex(1053i32);
    pub const GridViewItemPresenter_CheckHintBrush: XamlPropertyIndex = XamlPropertyIndex(1054i32);
    pub const GridViewItemPresenter_CheckSelectingBrush: XamlPropertyIndex = XamlPropertyIndex(1055i32);
    pub const GridViewItemPresenter_ContentMargin: XamlPropertyIndex = XamlPropertyIndex(1056i32);
    pub const GridViewItemPresenter_DisabledOpacity: XamlPropertyIndex = XamlPropertyIndex(1057i32);
    pub const GridViewItemPresenter_DragBackground: XamlPropertyIndex = XamlPropertyIndex(1058i32);
    pub const GridViewItemPresenter_DragForeground: XamlPropertyIndex = XamlPropertyIndex(1059i32);
    pub const GridViewItemPresenter_DragOpacity: XamlPropertyIndex = XamlPropertyIndex(1060i32);
    pub const GridViewItemPresenter_FocusBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1061i32);
    pub const GridViewItemPresenter_GridViewItemPresenterHorizontalContentAlignment: XamlPropertyIndex = XamlPropertyIndex(1062i32);
    pub const GridViewItemPresenter_GridViewItemPresenterPadding: XamlPropertyIndex = XamlPropertyIndex(1063i32);
    pub const GridViewItemPresenter_PlaceholderBackground: XamlPropertyIndex = XamlPropertyIndex(1064i32);
    pub const GridViewItemPresenter_PointerOverBackground: XamlPropertyIndex = XamlPropertyIndex(1065i32);
    pub const GridViewItemPresenter_PointerOverBackgroundMargin: XamlPropertyIndex = XamlPropertyIndex(1066i32);
    pub const GridViewItemPresenter_ReorderHintOffset: XamlPropertyIndex = XamlPropertyIndex(1067i32);
    pub const GridViewItemPresenter_SelectedBackground: XamlPropertyIndex = XamlPropertyIndex(1068i32);
    pub const GridViewItemPresenter_SelectedBorderThickness: XamlPropertyIndex = XamlPropertyIndex(1069i32);
    pub const GridViewItemPresenter_SelectedForeground: XamlPropertyIndex = XamlPropertyIndex(1070i32);
    pub const GridViewItemPresenter_SelectedPointerOverBackground: XamlPropertyIndex = XamlPropertyIndex(1071i32);
    pub const GridViewItemPresenter_SelectedPointerOverBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1072i32);
    pub const GridViewItemPresenter_SelectionCheckMarkVisualEnabled: XamlPropertyIndex = XamlPropertyIndex(1073i32);
    pub const GridViewItemPresenter_GridViewItemPresenterVerticalContentAlignment: XamlPropertyIndex = XamlPropertyIndex(1074i32);
    pub const ItemsStackPanel_CacheLength: XamlPropertyIndex = XamlPropertyIndex(1076i32);
    pub const ItemsStackPanel_GroupHeaderPlacement: XamlPropertyIndex = XamlPropertyIndex(1077i32);
    pub const ItemsStackPanel_GroupPadding: XamlPropertyIndex = XamlPropertyIndex(1078i32);
    pub const ItemsStackPanel_ItemsUpdatingScrollMode: XamlPropertyIndex = XamlPropertyIndex(1079i32);
    pub const ItemsStackPanel_Orientation: XamlPropertyIndex = XamlPropertyIndex(1080i32);
    pub const ItemsWrapGrid_CacheLength: XamlPropertyIndex = XamlPropertyIndex(1081i32);
    pub const ItemsWrapGrid_GroupHeaderPlacement: XamlPropertyIndex = XamlPropertyIndex(1082i32);
    pub const ItemsWrapGrid_GroupPadding: XamlPropertyIndex = XamlPropertyIndex(1083i32);
    pub const ItemsWrapGrid_ItemHeight: XamlPropertyIndex = XamlPropertyIndex(1084i32);
    pub const ItemsWrapGrid_ItemWidth: XamlPropertyIndex = XamlPropertyIndex(1085i32);
    pub const ItemsWrapGrid_MaximumRowsOrColumns: XamlPropertyIndex = XamlPropertyIndex(1086i32);
    pub const ItemsWrapGrid_Orientation: XamlPropertyIndex = XamlPropertyIndex(1087i32);
    pub const ListViewItemPresenter_CheckBrush: XamlPropertyIndex = XamlPropertyIndex(1088i32);
    pub const ListViewItemPresenter_CheckHintBrush: XamlPropertyIndex = XamlPropertyIndex(1089i32);
    pub const ListViewItemPresenter_CheckSelectingBrush: XamlPropertyIndex = XamlPropertyIndex(1090i32);
    pub const ListViewItemPresenter_ContentMargin: XamlPropertyIndex = XamlPropertyIndex(1091i32);
    pub const ListViewItemPresenter_DisabledOpacity: XamlPropertyIndex = XamlPropertyIndex(1092i32);
    pub const ListViewItemPresenter_DragBackground: XamlPropertyIndex = XamlPropertyIndex(1093i32);
    pub const ListViewItemPresenter_DragForeground: XamlPropertyIndex = XamlPropertyIndex(1094i32);
    pub const ListViewItemPresenter_DragOpacity: XamlPropertyIndex = XamlPropertyIndex(1095i32);
    pub const ListViewItemPresenter_FocusBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1096i32);
    pub const ListViewItemPresenter_ListViewItemPresenterHorizontalContentAlignment: XamlPropertyIndex = XamlPropertyIndex(1097i32);
    pub const ListViewItemPresenter_ListViewItemPresenterPadding: XamlPropertyIndex = XamlPropertyIndex(1098i32);
    pub const ListViewItemPresenter_PlaceholderBackground: XamlPropertyIndex = XamlPropertyIndex(1099i32);
    pub const ListViewItemPresenter_PointerOverBackground: XamlPropertyIndex = XamlPropertyIndex(1100i32);
    pub const ListViewItemPresenter_PointerOverBackgroundMargin: XamlPropertyIndex = XamlPropertyIndex(1101i32);
    pub const ListViewItemPresenter_ReorderHintOffset: XamlPropertyIndex = XamlPropertyIndex(1102i32);
    pub const ListViewItemPresenter_SelectedBackground: XamlPropertyIndex = XamlPropertyIndex(1103i32);
    pub const ListViewItemPresenter_SelectedBorderThickness: XamlPropertyIndex = XamlPropertyIndex(1104i32);
    pub const ListViewItemPresenter_SelectedForeground: XamlPropertyIndex = XamlPropertyIndex(1105i32);
    pub const ListViewItemPresenter_SelectedPointerOverBackground: XamlPropertyIndex = XamlPropertyIndex(1106i32);
    pub const ListViewItemPresenter_SelectedPointerOverBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1107i32);
    pub const ListViewItemPresenter_SelectionCheckMarkVisualEnabled: XamlPropertyIndex = XamlPropertyIndex(1108i32);
    pub const ListViewItemPresenter_ListViewItemPresenterVerticalContentAlignment: XamlPropertyIndex = XamlPropertyIndex(1109i32);
    pub const MenuFlyoutItem_Command: XamlPropertyIndex = XamlPropertyIndex(1110i32);
    pub const MenuFlyoutItem_CommandParameter: XamlPropertyIndex = XamlPropertyIndex(1111i32);
    pub const MenuFlyoutItem_Text: XamlPropertyIndex = XamlPropertyIndex(1112i32);
    pub const Page_BottomAppBar: XamlPropertyIndex = XamlPropertyIndex(1114i32);
    pub const Page_Frame: XamlPropertyIndex = XamlPropertyIndex(1115i32);
    pub const Page_NavigationCacheMode: XamlPropertyIndex = XamlPropertyIndex(1116i32);
    pub const Page_TopAppBar: XamlPropertyIndex = XamlPropertyIndex(1117i32);
    pub const ProgressBar_IsIndeterminate: XamlPropertyIndex = XamlPropertyIndex(1118i32);
    pub const ProgressBar_ShowError: XamlPropertyIndex = XamlPropertyIndex(1119i32);
    pub const ProgressBar_ShowPaused: XamlPropertyIndex = XamlPropertyIndex(1120i32);
    pub const ProgressBar_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1121i32);
    pub const ScrollBar_IndicatorMode: XamlPropertyIndex = XamlPropertyIndex(1122i32);
    pub const ScrollBar_Orientation: XamlPropertyIndex = XamlPropertyIndex(1123i32);
    pub const ScrollBar_ViewportSize: XamlPropertyIndex = XamlPropertyIndex(1124i32);
    pub const Selector_IsSynchronizedWithCurrentItem: XamlPropertyIndex = XamlPropertyIndex(1126i32);
    pub const Selector_SelectedIndex: XamlPropertyIndex = XamlPropertyIndex(1127i32);
    pub const Selector_SelectedItem: XamlPropertyIndex = XamlPropertyIndex(1128i32);
    pub const Selector_SelectedValue: XamlPropertyIndex = XamlPropertyIndex(1129i32);
    pub const Selector_SelectedValuePath: XamlPropertyIndex = XamlPropertyIndex(1130i32);
    pub const SelectorItem_IsSelected: XamlPropertyIndex = XamlPropertyIndex(1131i32);
    pub const SettingsFlyout_HeaderBackground: XamlPropertyIndex = XamlPropertyIndex(1132i32);
    pub const SettingsFlyout_HeaderForeground: XamlPropertyIndex = XamlPropertyIndex(1133i32);
    pub const SettingsFlyout_IconSource: XamlPropertyIndex = XamlPropertyIndex(1134i32);
    pub const SettingsFlyout_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1135i32);
    pub const SettingsFlyout_Title: XamlPropertyIndex = XamlPropertyIndex(1136i32);
    pub const Slider_Header: XamlPropertyIndex = XamlPropertyIndex(1137i32);
    pub const Slider_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(1138i32);
    pub const Slider_IntermediateValue: XamlPropertyIndex = XamlPropertyIndex(1139i32);
    pub const Slider_IsDirectionReversed: XamlPropertyIndex = XamlPropertyIndex(1140i32);
    pub const Slider_IsThumbToolTipEnabled: XamlPropertyIndex = XamlPropertyIndex(1141i32);
    pub const Slider_Orientation: XamlPropertyIndex = XamlPropertyIndex(1142i32);
    pub const Slider_SnapsTo: XamlPropertyIndex = XamlPropertyIndex(1143i32);
    pub const Slider_StepFrequency: XamlPropertyIndex = XamlPropertyIndex(1144i32);
    pub const Slider_ThumbToolTipValueConverter: XamlPropertyIndex = XamlPropertyIndex(1145i32);
    pub const Slider_TickFrequency: XamlPropertyIndex = XamlPropertyIndex(1146i32);
    pub const Slider_TickPlacement: XamlPropertyIndex = XamlPropertyIndex(1147i32);
    pub const SwapChainPanel_CompositionScaleX: XamlPropertyIndex = XamlPropertyIndex(1148i32);
    pub const SwapChainPanel_CompositionScaleY: XamlPropertyIndex = XamlPropertyIndex(1149i32);
    pub const ToolTip_HorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(1150i32);
    pub const ToolTip_IsOpen: XamlPropertyIndex = XamlPropertyIndex(1151i32);
    pub const ToolTip_Placement: XamlPropertyIndex = XamlPropertyIndex(1152i32);
    pub const ToolTip_PlacementTarget: XamlPropertyIndex = XamlPropertyIndex(1153i32);
    pub const ToolTip_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1154i32);
    pub const ToolTip_VerticalOffset: XamlPropertyIndex = XamlPropertyIndex(1155i32);
    pub const Button_Flyout: XamlPropertyIndex = XamlPropertyIndex(1156i32);
    pub const ComboBox_Header: XamlPropertyIndex = XamlPropertyIndex(1157i32);
    pub const ComboBox_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(1158i32);
    pub const ComboBox_IsDropDownOpen: XamlPropertyIndex = XamlPropertyIndex(1159i32);
    pub const ComboBox_IsEditable: XamlPropertyIndex = XamlPropertyIndex(1160i32);
    pub const ComboBox_IsSelectionBoxHighlighted: XamlPropertyIndex = XamlPropertyIndex(1161i32);
    pub const ComboBox_MaxDropDownHeight: XamlPropertyIndex = XamlPropertyIndex(1162i32);
    pub const ComboBox_PlaceholderText: XamlPropertyIndex = XamlPropertyIndex(1163i32);
    pub const ComboBox_SelectionBoxItem: XamlPropertyIndex = XamlPropertyIndex(1164i32);
    pub const ComboBox_SelectionBoxItemTemplate: XamlPropertyIndex = XamlPropertyIndex(1165i32);
    pub const ComboBox_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1166i32);
    pub const CommandBar_PrimaryCommands: XamlPropertyIndex = XamlPropertyIndex(1167i32);
    pub const CommandBar_SecondaryCommands: XamlPropertyIndex = XamlPropertyIndex(1168i32);
    pub const FlipView_UseTouchAnimationsForAllNavigation: XamlPropertyIndex = XamlPropertyIndex(1169i32);
    pub const HyperlinkButton_NavigateUri: XamlPropertyIndex = XamlPropertyIndex(1170i32);
    pub const ListBox_SelectedItems: XamlPropertyIndex = XamlPropertyIndex(1171i32);
    pub const ListBox_SelectionMode: XamlPropertyIndex = XamlPropertyIndex(1172i32);
    pub const ListViewBase_CanDragItems: XamlPropertyIndex = XamlPropertyIndex(1173i32);
    pub const ListViewBase_CanReorderItems: XamlPropertyIndex = XamlPropertyIndex(1174i32);
    pub const ListViewBase_DataFetchSize: XamlPropertyIndex = XamlPropertyIndex(1175i32);
    pub const ListViewBase_Footer: XamlPropertyIndex = XamlPropertyIndex(1176i32);
    pub const ListViewBase_FooterTemplate: XamlPropertyIndex = XamlPropertyIndex(1177i32);
    pub const ListViewBase_FooterTransitions: XamlPropertyIndex = XamlPropertyIndex(1178i32);
    pub const ListViewBase_Header: XamlPropertyIndex = XamlPropertyIndex(1179i32);
    pub const ListViewBase_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(1180i32);
    pub const ListViewBase_HeaderTransitions: XamlPropertyIndex = XamlPropertyIndex(1181i32);
    pub const ListViewBase_IncrementalLoadingThreshold: XamlPropertyIndex = XamlPropertyIndex(1182i32);
    pub const ListViewBase_IncrementalLoadingTrigger: XamlPropertyIndex = XamlPropertyIndex(1183i32);
    pub const ListViewBase_IsActiveView: XamlPropertyIndex = XamlPropertyIndex(1184i32);
    pub const ListViewBase_IsItemClickEnabled: XamlPropertyIndex = XamlPropertyIndex(1185i32);
    pub const ListViewBase_IsSwipeEnabled: XamlPropertyIndex = XamlPropertyIndex(1186i32);
    pub const ListViewBase_IsZoomedInView: XamlPropertyIndex = XamlPropertyIndex(1187i32);
    pub const ListViewBase_ReorderMode: XamlPropertyIndex = XamlPropertyIndex(1188i32);
    pub const ListViewBase_SelectedItems: XamlPropertyIndex = XamlPropertyIndex(1189i32);
    pub const ListViewBase_SelectionMode: XamlPropertyIndex = XamlPropertyIndex(1190i32);
    pub const ListViewBase_SemanticZoomOwner: XamlPropertyIndex = XamlPropertyIndex(1191i32);
    pub const ListViewBase_ShowsScrollingPlaceholders: XamlPropertyIndex = XamlPropertyIndex(1192i32);
    pub const RepeatButton_Delay: XamlPropertyIndex = XamlPropertyIndex(1193i32);
    pub const RepeatButton_Interval: XamlPropertyIndex = XamlPropertyIndex(1194i32);
    pub const ScrollViewer_BringIntoViewOnFocusChange: XamlPropertyIndex = XamlPropertyIndex(1195i32);
    pub const ScrollViewer_ComputedHorizontalScrollBarVisibility: XamlPropertyIndex = XamlPropertyIndex(1196i32);
    pub const ScrollViewer_ComputedVerticalScrollBarVisibility: XamlPropertyIndex = XamlPropertyIndex(1197i32);
    pub const ScrollViewer_ExtentHeight: XamlPropertyIndex = XamlPropertyIndex(1198i32);
    pub const ScrollViewer_ExtentWidth: XamlPropertyIndex = XamlPropertyIndex(1199i32);
    pub const ScrollViewer_HorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(1200i32);
    pub const ScrollViewer_HorizontalScrollBarVisibility: XamlPropertyIndex = XamlPropertyIndex(1201i32);
    pub const ScrollViewer_HorizontalScrollMode: XamlPropertyIndex = XamlPropertyIndex(1202i32);
    pub const ScrollViewer_HorizontalSnapPointsAlignment: XamlPropertyIndex = XamlPropertyIndex(1203i32);
    pub const ScrollViewer_HorizontalSnapPointsType: XamlPropertyIndex = XamlPropertyIndex(1204i32);
    pub const ScrollViewer_IsDeferredScrollingEnabled: XamlPropertyIndex = XamlPropertyIndex(1205i32);
    pub const ScrollViewer_IsHorizontalRailEnabled: XamlPropertyIndex = XamlPropertyIndex(1206i32);
    pub const ScrollViewer_IsHorizontalScrollChainingEnabled: XamlPropertyIndex = XamlPropertyIndex(1207i32);
    pub const ScrollViewer_IsScrollInertiaEnabled: XamlPropertyIndex = XamlPropertyIndex(1208i32);
    pub const ScrollViewer_IsVerticalRailEnabled: XamlPropertyIndex = XamlPropertyIndex(1209i32);
    pub const ScrollViewer_IsVerticalScrollChainingEnabled: XamlPropertyIndex = XamlPropertyIndex(1210i32);
    pub const ScrollViewer_IsZoomChainingEnabled: XamlPropertyIndex = XamlPropertyIndex(1211i32);
    pub const ScrollViewer_IsZoomInertiaEnabled: XamlPropertyIndex = XamlPropertyIndex(1212i32);
    pub const ScrollViewer_LeftHeader: XamlPropertyIndex = XamlPropertyIndex(1213i32);
    pub const ScrollViewer_MaxZoomFactor: XamlPropertyIndex = XamlPropertyIndex(1214i32);
    pub const ScrollViewer_MinZoomFactor: XamlPropertyIndex = XamlPropertyIndex(1215i32);
    pub const ScrollViewer_ScrollableHeight: XamlPropertyIndex = XamlPropertyIndex(1216i32);
    pub const ScrollViewer_ScrollableWidth: XamlPropertyIndex = XamlPropertyIndex(1217i32);
    pub const ScrollViewer_TopHeader: XamlPropertyIndex = XamlPropertyIndex(1218i32);
    pub const ScrollViewer_TopLeftHeader: XamlPropertyIndex = XamlPropertyIndex(1219i32);
    pub const ScrollViewer_VerticalOffset: XamlPropertyIndex = XamlPropertyIndex(1220i32);
    pub const ScrollViewer_VerticalScrollBarVisibility: XamlPropertyIndex = XamlPropertyIndex(1221i32);
    pub const ScrollViewer_VerticalScrollMode: XamlPropertyIndex = XamlPropertyIndex(1222i32);
    pub const ScrollViewer_VerticalSnapPointsAlignment: XamlPropertyIndex = XamlPropertyIndex(1223i32);
    pub const ScrollViewer_VerticalSnapPointsType: XamlPropertyIndex = XamlPropertyIndex(1224i32);
    pub const ScrollViewer_ViewportHeight: XamlPropertyIndex = XamlPropertyIndex(1225i32);
    pub const ScrollViewer_ViewportWidth: XamlPropertyIndex = XamlPropertyIndex(1226i32);
    pub const ScrollViewer_ZoomFactor: XamlPropertyIndex = XamlPropertyIndex(1227i32);
    pub const ScrollViewer_ZoomMode: XamlPropertyIndex = XamlPropertyIndex(1228i32);
    pub const ScrollViewer_ZoomSnapPoints: XamlPropertyIndex = XamlPropertyIndex(1229i32);
    pub const ScrollViewer_ZoomSnapPointsType: XamlPropertyIndex = XamlPropertyIndex(1230i32);
    pub const ToggleButton_IsChecked: XamlPropertyIndex = XamlPropertyIndex(1231i32);
    pub const ToggleButton_IsThreeState: XamlPropertyIndex = XamlPropertyIndex(1232i32);
    pub const ToggleMenuFlyoutItem_IsChecked: XamlPropertyIndex = XamlPropertyIndex(1233i32);
    pub const VirtualizingStackPanel_AreScrollSnapPointsRegular: XamlPropertyIndex = XamlPropertyIndex(1234i32);
    pub const VirtualizingStackPanel_IsVirtualizing: XamlPropertyIndex = XamlPropertyIndex(1236i32);
    pub const VirtualizingStackPanel_Orientation: XamlPropertyIndex = XamlPropertyIndex(1237i32);
    pub const VirtualizingStackPanel_VirtualizationMode: XamlPropertyIndex = XamlPropertyIndex(1238i32);
    pub const WrapGrid_HorizontalChildrenAlignment: XamlPropertyIndex = XamlPropertyIndex(1239i32);
    pub const WrapGrid_ItemHeight: XamlPropertyIndex = XamlPropertyIndex(1240i32);
    pub const WrapGrid_ItemWidth: XamlPropertyIndex = XamlPropertyIndex(1241i32);
    pub const WrapGrid_MaximumRowsOrColumns: XamlPropertyIndex = XamlPropertyIndex(1242i32);
    pub const WrapGrid_Orientation: XamlPropertyIndex = XamlPropertyIndex(1243i32);
    pub const WrapGrid_VerticalChildrenAlignment: XamlPropertyIndex = XamlPropertyIndex(1244i32);
    pub const AppBarButton_Icon: XamlPropertyIndex = XamlPropertyIndex(1245i32);
    pub const AppBarButton_IsCompact: XamlPropertyIndex = XamlPropertyIndex(1246i32);
    pub const AppBarButton_Label: XamlPropertyIndex = XamlPropertyIndex(1247i32);
    pub const AppBarToggleButton_Icon: XamlPropertyIndex = XamlPropertyIndex(1248i32);
    pub const AppBarToggleButton_IsCompact: XamlPropertyIndex = XamlPropertyIndex(1249i32);
    pub const AppBarToggleButton_Label: XamlPropertyIndex = XamlPropertyIndex(1250i32);
    pub const GridViewItem_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1251i32);
    pub const ListViewItem_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1252i32);
    pub const RadioButton_GroupName: XamlPropertyIndex = XamlPropertyIndex(1253i32);
    pub const Glyphs_ColorFontPaletteIndex: XamlPropertyIndex = XamlPropertyIndex(1267i32);
    pub const Glyphs_IsColorFontEnabled: XamlPropertyIndex = XamlPropertyIndex(1268i32);
    pub const CalendarViewTemplateSettings_HasMoreContentAfter: XamlPropertyIndex = XamlPropertyIndex(1274i32);
    pub const CalendarViewTemplateSettings_HasMoreContentBefore: XamlPropertyIndex = XamlPropertyIndex(1275i32);
    pub const CalendarViewTemplateSettings_HasMoreViews: XamlPropertyIndex = XamlPropertyIndex(1276i32);
    pub const CalendarViewTemplateSettings_HeaderText: XamlPropertyIndex = XamlPropertyIndex(1277i32);
    pub const CalendarViewTemplateSettings_WeekDay1: XamlPropertyIndex = XamlPropertyIndex(1280i32);
    pub const CalendarViewTemplateSettings_WeekDay2: XamlPropertyIndex = XamlPropertyIndex(1281i32);
    pub const CalendarViewTemplateSettings_WeekDay3: XamlPropertyIndex = XamlPropertyIndex(1282i32);
    pub const CalendarViewTemplateSettings_WeekDay4: XamlPropertyIndex = XamlPropertyIndex(1283i32);
    pub const CalendarViewTemplateSettings_WeekDay5: XamlPropertyIndex = XamlPropertyIndex(1284i32);
    pub const CalendarViewTemplateSettings_WeekDay6: XamlPropertyIndex = XamlPropertyIndex(1285i32);
    pub const CalendarViewTemplateSettings_WeekDay7: XamlPropertyIndex = XamlPropertyIndex(1286i32);
    pub const CalendarView_CalendarIdentifier: XamlPropertyIndex = XamlPropertyIndex(1291i32);
    pub const CalendarView_DayOfWeekFormat: XamlPropertyIndex = XamlPropertyIndex(1299i32);
    pub const CalendarView_DisplayMode: XamlPropertyIndex = XamlPropertyIndex(1302i32);
    pub const CalendarView_FirstDayOfWeek: XamlPropertyIndex = XamlPropertyIndex(1303i32);
    pub const CalendarView_IsOutOfScopeEnabled: XamlPropertyIndex = XamlPropertyIndex(1317i32);
    pub const CalendarView_IsTodayHighlighted: XamlPropertyIndex = XamlPropertyIndex(1318i32);
    pub const CalendarView_MaxDate: XamlPropertyIndex = XamlPropertyIndex(1320i32);
    pub const CalendarView_MinDate: XamlPropertyIndex = XamlPropertyIndex(1321i32);
    pub const CalendarView_NumberOfWeeksInView: XamlPropertyIndex = XamlPropertyIndex(1327i32);
    pub const CalendarView_SelectedDates: XamlPropertyIndex = XamlPropertyIndex(1333i32);
    pub const CalendarView_SelectionMode: XamlPropertyIndex = XamlPropertyIndex(1335i32);
    pub const CalendarView_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1336i32);
    pub const CalendarViewDayItem_Date: XamlPropertyIndex = XamlPropertyIndex(1339i32);
    pub const CalendarViewDayItem_IsBlackout: XamlPropertyIndex = XamlPropertyIndex(1340i32);
    pub const MediaTransportControls_IsFastForwardEnabled: XamlPropertyIndex = XamlPropertyIndex(1382i32);
    pub const MediaTransportControls_IsFastRewindEnabled: XamlPropertyIndex = XamlPropertyIndex(1383i32);
    pub const MediaTransportControls_IsFullWindowEnabled: XamlPropertyIndex = XamlPropertyIndex(1384i32);
    pub const MediaTransportControls_IsPlaybackRateEnabled: XamlPropertyIndex = XamlPropertyIndex(1385i32);
    pub const MediaTransportControls_IsSeekEnabled: XamlPropertyIndex = XamlPropertyIndex(1386i32);
    pub const MediaTransportControls_IsStopEnabled: XamlPropertyIndex = XamlPropertyIndex(1387i32);
    pub const MediaTransportControls_IsVolumeEnabled: XamlPropertyIndex = XamlPropertyIndex(1388i32);
    pub const MediaTransportControls_IsZoomEnabled: XamlPropertyIndex = XamlPropertyIndex(1389i32);
    pub const ContentPresenter_LineHeight: XamlPropertyIndex = XamlPropertyIndex(1425i32);
    pub const CalendarViewTemplateSettings_MinViewWidth: XamlPropertyIndex = XamlPropertyIndex(1435i32);
    pub const ListViewBase_SelectedRanges: XamlPropertyIndex = XamlPropertyIndex(1459i32);
    pub const SplitViewTemplateSettings_CompactPaneGridLength: XamlPropertyIndex = XamlPropertyIndex(1462i32);
    pub const SplitViewTemplateSettings_NegativeOpenPaneLength: XamlPropertyIndex = XamlPropertyIndex(1463i32);
    pub const SplitViewTemplateSettings_NegativeOpenPaneLengthMinusCompactLength: XamlPropertyIndex = XamlPropertyIndex(1464i32);
    pub const SplitViewTemplateSettings_OpenPaneGridLength: XamlPropertyIndex = XamlPropertyIndex(1465i32);
    pub const SplitViewTemplateSettings_OpenPaneLengthMinusCompactLength: XamlPropertyIndex = XamlPropertyIndex(1466i32);
    pub const SplitView_CompactPaneLength: XamlPropertyIndex = XamlPropertyIndex(1467i32);
    pub const SplitView_Content: XamlPropertyIndex = XamlPropertyIndex(1468i32);
    pub const SplitView_DisplayMode: XamlPropertyIndex = XamlPropertyIndex(1469i32);
    pub const SplitView_IsPaneOpen: XamlPropertyIndex = XamlPropertyIndex(1470i32);
    pub const SplitView_OpenPaneLength: XamlPropertyIndex = XamlPropertyIndex(1471i32);
    pub const SplitView_Pane: XamlPropertyIndex = XamlPropertyIndex(1472i32);
    pub const SplitView_PanePlacement: XamlPropertyIndex = XamlPropertyIndex(1473i32);
    pub const SplitView_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1474i32);
    pub const UIElement_Transform3D: XamlPropertyIndex = XamlPropertyIndex(1475i32);
    pub const CompositeTransform3D_CenterX: XamlPropertyIndex = XamlPropertyIndex(1476i32);
    pub const CompositeTransform3D_CenterY: XamlPropertyIndex = XamlPropertyIndex(1478i32);
    pub const CompositeTransform3D_CenterZ: XamlPropertyIndex = XamlPropertyIndex(1480i32);
    pub const CompositeTransform3D_RotationX: XamlPropertyIndex = XamlPropertyIndex(1482i32);
    pub const CompositeTransform3D_RotationY: XamlPropertyIndex = XamlPropertyIndex(1484i32);
    pub const CompositeTransform3D_RotationZ: XamlPropertyIndex = XamlPropertyIndex(1486i32);
    pub const CompositeTransform3D_ScaleX: XamlPropertyIndex = XamlPropertyIndex(1488i32);
    pub const CompositeTransform3D_ScaleY: XamlPropertyIndex = XamlPropertyIndex(1490i32);
    pub const CompositeTransform3D_ScaleZ: XamlPropertyIndex = XamlPropertyIndex(1492i32);
    pub const CompositeTransform3D_TranslateX: XamlPropertyIndex = XamlPropertyIndex(1494i32);
    pub const CompositeTransform3D_TranslateY: XamlPropertyIndex = XamlPropertyIndex(1496i32);
    pub const CompositeTransform3D_TranslateZ: XamlPropertyIndex = XamlPropertyIndex(1498i32);
    pub const PerspectiveTransform3D_Depth: XamlPropertyIndex = XamlPropertyIndex(1500i32);
    pub const PerspectiveTransform3D_OffsetX: XamlPropertyIndex = XamlPropertyIndex(1501i32);
    pub const PerspectiveTransform3D_OffsetY: XamlPropertyIndex = XamlPropertyIndex(1502i32);
    pub const RelativePanel_Above: XamlPropertyIndex = XamlPropertyIndex(1508i32);
    pub const RelativePanel_AlignBottomWith: XamlPropertyIndex = XamlPropertyIndex(1509i32);
    pub const RelativePanel_AlignLeftWith: XamlPropertyIndex = XamlPropertyIndex(1510i32);
    pub const RelativePanel_AlignRightWith: XamlPropertyIndex = XamlPropertyIndex(1515i32);
    pub const RelativePanel_AlignTopWith: XamlPropertyIndex = XamlPropertyIndex(1516i32);
    pub const RelativePanel_Below: XamlPropertyIndex = XamlPropertyIndex(1517i32);
    pub const RelativePanel_LeftOf: XamlPropertyIndex = XamlPropertyIndex(1520i32);
    pub const RelativePanel_RightOf: XamlPropertyIndex = XamlPropertyIndex(1521i32);
    pub const SplitViewTemplateSettings_OpenPaneLength: XamlPropertyIndex = XamlPropertyIndex(1524i32);
    pub const PasswordBox_PasswordRevealMode: XamlPropertyIndex = XamlPropertyIndex(1527i32);
    pub const SplitView_PaneBackground: XamlPropertyIndex = XamlPropertyIndex(1528i32);
    pub const ItemsStackPanel_AreStickyGroupHeadersEnabled: XamlPropertyIndex = XamlPropertyIndex(1529i32);
    pub const ItemsWrapGrid_AreStickyGroupHeadersEnabled: XamlPropertyIndex = XamlPropertyIndex(1530i32);
    pub const MenuFlyoutSubItem_Items: XamlPropertyIndex = XamlPropertyIndex(1531i32);
    pub const MenuFlyoutSubItem_Text: XamlPropertyIndex = XamlPropertyIndex(1532i32);
    pub const UIElement_CanDrag: XamlPropertyIndex = XamlPropertyIndex(1534i32);
    pub const DataTemplate_ExtensionInstance: XamlPropertyIndex = XamlPropertyIndex(1535i32);
    pub const RelativePanel_AlignHorizontalCenterWith: XamlPropertyIndex = XamlPropertyIndex(1552i32);
    pub const RelativePanel_AlignVerticalCenterWith: XamlPropertyIndex = XamlPropertyIndex(1553i32);
    pub const TargetPropertyPath_Path: XamlPropertyIndex = XamlPropertyIndex(1555i32);
    pub const TargetPropertyPath_Target: XamlPropertyIndex = XamlPropertyIndex(1556i32);
    pub const VisualState_Setters: XamlPropertyIndex = XamlPropertyIndex(1558i32);
    pub const VisualState_StateTriggers: XamlPropertyIndex = XamlPropertyIndex(1559i32);
    pub const AdaptiveTrigger_MinWindowHeight: XamlPropertyIndex = XamlPropertyIndex(1560i32);
    pub const AdaptiveTrigger_MinWindowWidth: XamlPropertyIndex = XamlPropertyIndex(1561i32);
    pub const Setter_Target: XamlPropertyIndex = XamlPropertyIndex(1562i32);
    pub const CalendarView_BlackoutForeground: XamlPropertyIndex = XamlPropertyIndex(1565i32);
    pub const CalendarView_CalendarItemBackground: XamlPropertyIndex = XamlPropertyIndex(1566i32);
    pub const CalendarView_CalendarItemBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1567i32);
    pub const CalendarView_CalendarItemBorderThickness: XamlPropertyIndex = XamlPropertyIndex(1568i32);
    pub const CalendarView_CalendarItemForeground: XamlPropertyIndex = XamlPropertyIndex(1569i32);
    pub const CalendarView_CalendarViewDayItemStyle: XamlPropertyIndex = XamlPropertyIndex(1570i32);
    pub const CalendarView_DayItemFontFamily: XamlPropertyIndex = XamlPropertyIndex(1571i32);
    pub const CalendarView_DayItemFontSize: XamlPropertyIndex = XamlPropertyIndex(1572i32);
    pub const CalendarView_DayItemFontStyle: XamlPropertyIndex = XamlPropertyIndex(1573i32);
    pub const CalendarView_DayItemFontWeight: XamlPropertyIndex = XamlPropertyIndex(1574i32);
    pub const CalendarView_FirstOfMonthLabelFontFamily: XamlPropertyIndex = XamlPropertyIndex(1575i32);
    pub const CalendarView_FirstOfMonthLabelFontSize: XamlPropertyIndex = XamlPropertyIndex(1576i32);
    pub const CalendarView_FirstOfMonthLabelFontStyle: XamlPropertyIndex = XamlPropertyIndex(1577i32);
    pub const CalendarView_FirstOfMonthLabelFontWeight: XamlPropertyIndex = XamlPropertyIndex(1578i32);
    pub const CalendarView_FirstOfYearDecadeLabelFontFamily: XamlPropertyIndex = XamlPropertyIndex(1579i32);
    pub const CalendarView_FirstOfYearDecadeLabelFontSize: XamlPropertyIndex = XamlPropertyIndex(1580i32);
    pub const CalendarView_FirstOfYearDecadeLabelFontStyle: XamlPropertyIndex = XamlPropertyIndex(1581i32);
    pub const CalendarView_FirstOfYearDecadeLabelFontWeight: XamlPropertyIndex = XamlPropertyIndex(1582i32);
    pub const CalendarView_FocusBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1583i32);
    pub const CalendarView_HorizontalDayItemAlignment: XamlPropertyIndex = XamlPropertyIndex(1584i32);
    pub const CalendarView_HorizontalFirstOfMonthLabelAlignment: XamlPropertyIndex = XamlPropertyIndex(1585i32);
    pub const CalendarView_HoverBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1586i32);
    pub const CalendarView_MonthYearItemFontFamily: XamlPropertyIndex = XamlPropertyIndex(1588i32);
    pub const CalendarView_MonthYearItemFontSize: XamlPropertyIndex = XamlPropertyIndex(1589i32);
    pub const CalendarView_MonthYearItemFontStyle: XamlPropertyIndex = XamlPropertyIndex(1590i32);
    pub const CalendarView_MonthYearItemFontWeight: XamlPropertyIndex = XamlPropertyIndex(1591i32);
    pub const CalendarView_OutOfScopeBackground: XamlPropertyIndex = XamlPropertyIndex(1592i32);
    pub const CalendarView_OutOfScopeForeground: XamlPropertyIndex = XamlPropertyIndex(1593i32);
    pub const CalendarView_PressedBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1594i32);
    pub const CalendarView_PressedForeground: XamlPropertyIndex = XamlPropertyIndex(1595i32);
    pub const CalendarView_SelectedBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1596i32);
    pub const CalendarView_SelectedForeground: XamlPropertyIndex = XamlPropertyIndex(1597i32);
    pub const CalendarView_SelectedHoverBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1598i32);
    pub const CalendarView_SelectedPressedBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1599i32);
    pub const CalendarView_TodayFontWeight: XamlPropertyIndex = XamlPropertyIndex(1600i32);
    pub const CalendarView_TodayForeground: XamlPropertyIndex = XamlPropertyIndex(1601i32);
    pub const CalendarView_VerticalDayItemAlignment: XamlPropertyIndex = XamlPropertyIndex(1602i32);
    pub const CalendarView_VerticalFirstOfMonthLabelAlignment: XamlPropertyIndex = XamlPropertyIndex(1603i32);
    pub const MediaTransportControls_IsCompact: XamlPropertyIndex = XamlPropertyIndex(1605i32);
    pub const RelativePanel_AlignBottomWithPanel: XamlPropertyIndex = XamlPropertyIndex(1606i32);
    pub const RelativePanel_AlignHorizontalCenterWithPanel: XamlPropertyIndex = XamlPropertyIndex(1607i32);
    pub const RelativePanel_AlignLeftWithPanel: XamlPropertyIndex = XamlPropertyIndex(1608i32);
    pub const RelativePanel_AlignRightWithPanel: XamlPropertyIndex = XamlPropertyIndex(1609i32);
    pub const RelativePanel_AlignTopWithPanel: XamlPropertyIndex = XamlPropertyIndex(1610i32);
    pub const RelativePanel_AlignVerticalCenterWithPanel: XamlPropertyIndex = XamlPropertyIndex(1611i32);
    pub const ListViewBase_IsMultiSelectCheckBoxEnabled: XamlPropertyIndex = XamlPropertyIndex(1612i32);
    pub const AutomationProperties_Level: XamlPropertyIndex = XamlPropertyIndex(1614i32);
    pub const AutomationProperties_PositionInSet: XamlPropertyIndex = XamlPropertyIndex(1615i32);
    pub const AutomationProperties_SizeOfSet: XamlPropertyIndex = XamlPropertyIndex(1616i32);
    pub const ListViewItemPresenter_CheckBoxBrush: XamlPropertyIndex = XamlPropertyIndex(1617i32);
    pub const ListViewItemPresenter_CheckMode: XamlPropertyIndex = XamlPropertyIndex(1618i32);
    pub const ListViewItemPresenter_PressedBackground: XamlPropertyIndex = XamlPropertyIndex(1620i32);
    pub const ListViewItemPresenter_SelectedPressedBackground: XamlPropertyIndex = XamlPropertyIndex(1621i32);
    pub const Control_IsTemplateFocusTarget: XamlPropertyIndex = XamlPropertyIndex(1623i32);
    pub const Control_UseSystemFocusVisuals: XamlPropertyIndex = XamlPropertyIndex(1624i32);
    pub const ListViewItemPresenter_FocusSecondaryBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1628i32);
    pub const ListViewItemPresenter_PointerOverForeground: XamlPropertyIndex = XamlPropertyIndex(1630i32);
    pub const FontIcon_MirroredWhenRightToLeft: XamlPropertyIndex = XamlPropertyIndex(1631i32);
    pub const CalendarViewTemplateSettings_CenterX: XamlPropertyIndex = XamlPropertyIndex(1632i32);
    pub const CalendarViewTemplateSettings_CenterY: XamlPropertyIndex = XamlPropertyIndex(1633i32);
    pub const CalendarViewTemplateSettings_ClipRect: XamlPropertyIndex = XamlPropertyIndex(1634i32);
    pub const PasswordBox_TextReadingOrder: XamlPropertyIndex = XamlPropertyIndex(1650i32);
    pub const RichEditBox_TextReadingOrder: XamlPropertyIndex = XamlPropertyIndex(1651i32);
    pub const TextBox_TextReadingOrder: XamlPropertyIndex = XamlPropertyIndex(1652i32);
    pub const WebView_ExecutionMode: XamlPropertyIndex = XamlPropertyIndex(1653i32);
    pub const WebView_DeferredPermissionRequests: XamlPropertyIndex = XamlPropertyIndex(1655i32);
    pub const WebView_Settings: XamlPropertyIndex = XamlPropertyIndex(1656i32);
    pub const RichEditBox_DesiredCandidateWindowAlignment: XamlPropertyIndex = XamlPropertyIndex(1660i32);
    pub const TextBox_DesiredCandidateWindowAlignment: XamlPropertyIndex = XamlPropertyIndex(1662i32);
    pub const CalendarDatePicker_CalendarIdentifier: XamlPropertyIndex = XamlPropertyIndex(1663i32);
    pub const CalendarDatePicker_CalendarViewStyle: XamlPropertyIndex = XamlPropertyIndex(1664i32);
    pub const CalendarDatePicker_Date: XamlPropertyIndex = XamlPropertyIndex(1665i32);
    pub const CalendarDatePicker_DateFormat: XamlPropertyIndex = XamlPropertyIndex(1666i32);
    pub const CalendarDatePicker_DayOfWeekFormat: XamlPropertyIndex = XamlPropertyIndex(1667i32);
    pub const CalendarDatePicker_DisplayMode: XamlPropertyIndex = XamlPropertyIndex(1668i32);
    pub const CalendarDatePicker_FirstDayOfWeek: XamlPropertyIndex = XamlPropertyIndex(1669i32);
    pub const CalendarDatePicker_Header: XamlPropertyIndex = XamlPropertyIndex(1670i32);
    pub const CalendarDatePicker_HeaderTemplate: XamlPropertyIndex = XamlPropertyIndex(1671i32);
    pub const CalendarDatePicker_IsCalendarOpen: XamlPropertyIndex = XamlPropertyIndex(1672i32);
    pub const CalendarDatePicker_IsGroupLabelVisible: XamlPropertyIndex = XamlPropertyIndex(1673i32);
    pub const CalendarDatePicker_IsOutOfScopeEnabled: XamlPropertyIndex = XamlPropertyIndex(1674i32);
    pub const CalendarDatePicker_IsTodayHighlighted: XamlPropertyIndex = XamlPropertyIndex(1675i32);
    pub const CalendarDatePicker_MaxDate: XamlPropertyIndex = XamlPropertyIndex(1676i32);
    pub const CalendarDatePicker_MinDate: XamlPropertyIndex = XamlPropertyIndex(1677i32);
    pub const CalendarDatePicker_PlaceholderText: XamlPropertyIndex = XamlPropertyIndex(1678i32);
    pub const CalendarView_IsGroupLabelVisible: XamlPropertyIndex = XamlPropertyIndex(1679i32);
    pub const ContentPresenter_Background: XamlPropertyIndex = XamlPropertyIndex(1680i32);
    pub const ContentPresenter_BorderBrush: XamlPropertyIndex = XamlPropertyIndex(1681i32);
    pub const ContentPresenter_BorderThickness: XamlPropertyIndex = XamlPropertyIndex(1682i32);
    pub const ContentPresenter_CornerRadius: XamlPropertyIndex = XamlPropertyIndex(1683i32);
    pub const ContentPresenter_Padding: XamlPropertyIndex = XamlPropertyIndex(1684i32);
    pub const Grid_BorderBrush: XamlPropertyIndex = XamlPropertyIndex(1685i32);
    pub const Grid_BorderThickness: XamlPropertyIndex = XamlPropertyIndex(1686i32);
    pub const Grid_CornerRadius: XamlPropertyIndex = XamlPropertyIndex(1687i32);
    pub const Grid_Padding: XamlPropertyIndex = XamlPropertyIndex(1688i32);
    pub const RelativePanel_BorderBrush: XamlPropertyIndex = XamlPropertyIndex(1689i32);
    pub const RelativePanel_BorderThickness: XamlPropertyIndex = XamlPropertyIndex(1690i32);
    pub const RelativePanel_CornerRadius: XamlPropertyIndex = XamlPropertyIndex(1691i32);
    pub const RelativePanel_Padding: XamlPropertyIndex = XamlPropertyIndex(1692i32);
    pub const StackPanel_BorderBrush: XamlPropertyIndex = XamlPropertyIndex(1693i32);
    pub const StackPanel_BorderThickness: XamlPropertyIndex = XamlPropertyIndex(1694i32);
    pub const StackPanel_CornerRadius: XamlPropertyIndex = XamlPropertyIndex(1695i32);
    pub const StackPanel_Padding: XamlPropertyIndex = XamlPropertyIndex(1696i32);
    pub const PasswordBox_InputScope: XamlPropertyIndex = XamlPropertyIndex(1697i32);
    pub const MediaTransportControlsHelper_DropoutOrder: XamlPropertyIndex = XamlPropertyIndex(1698i32);
    pub const AutoSuggestBoxQuerySubmittedEventArgs_ChosenSuggestion: XamlPropertyIndex = XamlPropertyIndex(1699i32);
    pub const AutoSuggestBoxQuerySubmittedEventArgs_QueryText: XamlPropertyIndex = XamlPropertyIndex(1700i32);
    pub const AutoSuggestBox_QueryIcon: XamlPropertyIndex = XamlPropertyIndex(1701i32);
    pub const StateTrigger_IsActive: XamlPropertyIndex = XamlPropertyIndex(1702i32);
    pub const ContentPresenter_HorizontalContentAlignment: XamlPropertyIndex = XamlPropertyIndex(1703i32);
    pub const ContentPresenter_VerticalContentAlignment: XamlPropertyIndex = XamlPropertyIndex(1704i32);
    pub const AppBarTemplateSettings_ClipRect: XamlPropertyIndex = XamlPropertyIndex(1705i32);
    pub const AppBarTemplateSettings_CompactRootMargin: XamlPropertyIndex = XamlPropertyIndex(1706i32);
    pub const AppBarTemplateSettings_CompactVerticalDelta: XamlPropertyIndex = XamlPropertyIndex(1707i32);
    pub const AppBarTemplateSettings_HiddenRootMargin: XamlPropertyIndex = XamlPropertyIndex(1708i32);
    pub const AppBarTemplateSettings_HiddenVerticalDelta: XamlPropertyIndex = XamlPropertyIndex(1709i32);
    pub const AppBarTemplateSettings_MinimalRootMargin: XamlPropertyIndex = XamlPropertyIndex(1710i32);
    pub const AppBarTemplateSettings_MinimalVerticalDelta: XamlPropertyIndex = XamlPropertyIndex(1711i32);
    pub const CommandBarTemplateSettings_ContentHeight: XamlPropertyIndex = XamlPropertyIndex(1712i32);
    pub const CommandBarTemplateSettings_NegativeOverflowContentHeight: XamlPropertyIndex = XamlPropertyIndex(1713i32);
    pub const CommandBarTemplateSettings_OverflowContentClipRect: XamlPropertyIndex = XamlPropertyIndex(1714i32);
    pub const CommandBarTemplateSettings_OverflowContentHeight: XamlPropertyIndex = XamlPropertyIndex(1715i32);
    pub const CommandBarTemplateSettings_OverflowContentHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(1716i32);
    pub const CommandBarTemplateSettings_OverflowContentMaxHeight: XamlPropertyIndex = XamlPropertyIndex(1717i32);
    pub const CommandBarTemplateSettings_OverflowContentMinWidth: XamlPropertyIndex = XamlPropertyIndex(1718i32);
    pub const AppBar_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1719i32);
    pub const CommandBar_CommandBarOverflowPresenterStyle: XamlPropertyIndex = XamlPropertyIndex(1720i32);
    pub const CommandBar_CommandBarTemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1721i32);
    pub const DrillInThemeAnimation_EntranceTarget: XamlPropertyIndex = XamlPropertyIndex(1722i32);
    pub const DrillInThemeAnimation_EntranceTargetName: XamlPropertyIndex = XamlPropertyIndex(1723i32);
    pub const DrillInThemeAnimation_ExitTarget: XamlPropertyIndex = XamlPropertyIndex(1724i32);
    pub const DrillInThemeAnimation_ExitTargetName: XamlPropertyIndex = XamlPropertyIndex(1725i32);
    pub const DrillOutThemeAnimation_EntranceTarget: XamlPropertyIndex = XamlPropertyIndex(1726i32);
    pub const DrillOutThemeAnimation_EntranceTargetName: XamlPropertyIndex = XamlPropertyIndex(1727i32);
    pub const DrillOutThemeAnimation_ExitTarget: XamlPropertyIndex = XamlPropertyIndex(1728i32);
    pub const DrillOutThemeAnimation_ExitTargetName: XamlPropertyIndex = XamlPropertyIndex(1729i32);
    pub const XamlBindingHelper_DataTemplateComponent: XamlPropertyIndex = XamlPropertyIndex(1730i32);
    pub const AutomationProperties_Annotations: XamlPropertyIndex = XamlPropertyIndex(1732i32);
    pub const AutomationAnnotation_Element: XamlPropertyIndex = XamlPropertyIndex(1733i32);
    pub const AutomationAnnotation_Type: XamlPropertyIndex = XamlPropertyIndex(1734i32);
    pub const AutomationPeerAnnotation_Peer: XamlPropertyIndex = XamlPropertyIndex(1735i32);
    pub const AutomationPeerAnnotation_Type: XamlPropertyIndex = XamlPropertyIndex(1736i32);
    pub const Hyperlink_UnderlineStyle: XamlPropertyIndex = XamlPropertyIndex(1741i32);
    pub const CalendarView_DisabledForeground: XamlPropertyIndex = XamlPropertyIndex(1742i32);
    pub const CalendarView_TodayBackground: XamlPropertyIndex = XamlPropertyIndex(1743i32);
    pub const CalendarView_TodayBlackoutBackground: XamlPropertyIndex = XamlPropertyIndex(1744i32);
    pub const CalendarView_TodaySelectedInnerBorderBrush: XamlPropertyIndex = XamlPropertyIndex(1747i32);
    pub const Control_IsFocusEngaged: XamlPropertyIndex = XamlPropertyIndex(1749i32);
    pub const Control_IsFocusEngagementEnabled: XamlPropertyIndex = XamlPropertyIndex(1752i32);
    pub const RichEditBox_ClipboardCopyFormat: XamlPropertyIndex = XamlPropertyIndex(1754i32);
    pub const CommandBarTemplateSettings_OverflowContentMaxWidth: XamlPropertyIndex = XamlPropertyIndex(1757i32);
    pub const ComboBoxTemplateSettings_DropDownContentMinWidth: XamlPropertyIndex = XamlPropertyIndex(1758i32);
    pub const MenuFlyoutPresenterTemplateSettings_FlyoutContentMinWidth: XamlPropertyIndex = XamlPropertyIndex(1762i32);
    pub const MenuFlyoutPresenter_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(1763i32);
    pub const AutomationProperties_LandmarkType: XamlPropertyIndex = XamlPropertyIndex(1766i32);
    pub const AutomationProperties_LocalizedLandmarkType: XamlPropertyIndex = XamlPropertyIndex(1767i32);
    pub const RepositionThemeTransition_IsStaggeringEnabled: XamlPropertyIndex = XamlPropertyIndex(1769i32);
    pub const ListBox_SingleSelectionFollowsFocus: XamlPropertyIndex = XamlPropertyIndex(1770i32);
    pub const ListViewBase_SingleSelectionFollowsFocus: XamlPropertyIndex = XamlPropertyIndex(1771i32);
    pub const BitmapImage_AutoPlay: XamlPropertyIndex = XamlPropertyIndex(1773i32);
    pub const BitmapImage_IsAnimatedBitmap: XamlPropertyIndex = XamlPropertyIndex(1774i32);
    pub const BitmapImage_IsPlaying: XamlPropertyIndex = XamlPropertyIndex(1775i32);
    pub const AutomationProperties_FullDescription: XamlPropertyIndex = XamlPropertyIndex(1776i32);
    pub const AutomationProperties_IsDataValidForForm: XamlPropertyIndex = XamlPropertyIndex(1777i32);
    pub const AutomationProperties_IsPeripheral: XamlPropertyIndex = XamlPropertyIndex(1778i32);
    pub const AutomationProperties_LocalizedControlType: XamlPropertyIndex = XamlPropertyIndex(1779i32);
    pub const FlyoutBase_AllowFocusOnInteraction: XamlPropertyIndex = XamlPropertyIndex(1780i32);
    pub const TextElement_AllowFocusOnInteraction: XamlPropertyIndex = XamlPropertyIndex(1781i32);
    pub const FrameworkElement_AllowFocusOnInteraction: XamlPropertyIndex = XamlPropertyIndex(1782i32);
    pub const Control_RequiresPointer: XamlPropertyIndex = XamlPropertyIndex(1783i32);
    pub const UIElement_ContextFlyout: XamlPropertyIndex = XamlPropertyIndex(1785i32);
    pub const TextElement_AccessKey: XamlPropertyIndex = XamlPropertyIndex(1786i32);
    pub const UIElement_AccessKeyScopeOwner: XamlPropertyIndex = XamlPropertyIndex(1787i32);
    pub const UIElement_IsAccessKeyScope: XamlPropertyIndex = XamlPropertyIndex(1788i32);
    pub const AutomationProperties_DescribedBy: XamlPropertyIndex = XamlPropertyIndex(1790i32);
    pub const UIElement_AccessKey: XamlPropertyIndex = XamlPropertyIndex(1803i32);
    pub const Control_XYFocusDown: XamlPropertyIndex = XamlPropertyIndex(1804i32);
    pub const Control_XYFocusLeft: XamlPropertyIndex = XamlPropertyIndex(1805i32);
    pub const Control_XYFocusRight: XamlPropertyIndex = XamlPropertyIndex(1806i32);
    pub const Control_XYFocusUp: XamlPropertyIndex = XamlPropertyIndex(1807i32);
    pub const Hyperlink_XYFocusDown: XamlPropertyIndex = XamlPropertyIndex(1808i32);
    pub const Hyperlink_XYFocusLeft: XamlPropertyIndex = XamlPropertyIndex(1809i32);
    pub const Hyperlink_XYFocusRight: XamlPropertyIndex = XamlPropertyIndex(1810i32);
    pub const Hyperlink_XYFocusUp: XamlPropertyIndex = XamlPropertyIndex(1811i32);
    pub const WebView_XYFocusDown: XamlPropertyIndex = XamlPropertyIndex(1812i32);
    pub const WebView_XYFocusLeft: XamlPropertyIndex = XamlPropertyIndex(1813i32);
    pub const WebView_XYFocusRight: XamlPropertyIndex = XamlPropertyIndex(1814i32);
    pub const WebView_XYFocusUp: XamlPropertyIndex = XamlPropertyIndex(1815i32);
    pub const CommandBarTemplateSettings_EffectiveOverflowButtonVisibility: XamlPropertyIndex = XamlPropertyIndex(1816i32);
    pub const AppBarSeparator_IsInOverflow: XamlPropertyIndex = XamlPropertyIndex(1817i32);
    pub const CommandBar_DefaultLabelPosition: XamlPropertyIndex = XamlPropertyIndex(1818i32);
    pub const CommandBar_IsDynamicOverflowEnabled: XamlPropertyIndex = XamlPropertyIndex(1819i32);
    pub const CommandBar_OverflowButtonVisibility: XamlPropertyIndex = XamlPropertyIndex(1820i32);
    pub const AppBarButton_IsInOverflow: XamlPropertyIndex = XamlPropertyIndex(1821i32);
    pub const AppBarButton_LabelPosition: XamlPropertyIndex = XamlPropertyIndex(1822i32);
    pub const AppBarToggleButton_IsInOverflow: XamlPropertyIndex = XamlPropertyIndex(1823i32);
    pub const AppBarToggleButton_LabelPosition: XamlPropertyIndex = XamlPropertyIndex(1824i32);
    pub const FlyoutBase_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1825i32);
    pub const Popup_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1827i32);
    pub const CalendarDatePicker_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1829i32);
    pub const DatePicker_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1830i32);
    pub const SplitView_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1831i32);
    pub const TimePicker_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1832i32);
    pub const AppBar_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1833i32);
    pub const AutoSuggestBox_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1834i32);
    pub const ComboBox_LightDismissOverlayMode: XamlPropertyIndex = XamlPropertyIndex(1835i32);
    pub const AppBarSeparator_DynamicOverflowOrder: XamlPropertyIndex = XamlPropertyIndex(1836i32);
    pub const AppBarButton_DynamicOverflowOrder: XamlPropertyIndex = XamlPropertyIndex(1837i32);
    pub const AppBarToggleButton_DynamicOverflowOrder: XamlPropertyIndex = XamlPropertyIndex(1838i32);
    pub const FrameworkElement_FocusVisualMargin: XamlPropertyIndex = XamlPropertyIndex(1839i32);
    pub const FrameworkElement_FocusVisualPrimaryBrush: XamlPropertyIndex = XamlPropertyIndex(1840i32);
    pub const FrameworkElement_FocusVisualPrimaryThickness: XamlPropertyIndex = XamlPropertyIndex(1841i32);
    pub const FrameworkElement_FocusVisualSecondaryBrush: XamlPropertyIndex = XamlPropertyIndex(1842i32);
    pub const FrameworkElement_FocusVisualSecondaryThickness: XamlPropertyIndex = XamlPropertyIndex(1843i32);
    pub const FlyoutBase_AllowFocusWhenDisabled: XamlPropertyIndex = XamlPropertyIndex(1846i32);
    pub const FrameworkElement_AllowFocusWhenDisabled: XamlPropertyIndex = XamlPropertyIndex(1847i32);
    pub const ComboBox_IsTextSearchEnabled: XamlPropertyIndex = XamlPropertyIndex(1848i32);
    pub const TextElement_ExitDisplayModeOnAccessKeyInvoked: XamlPropertyIndex = XamlPropertyIndex(1849i32);
    pub const UIElement_ExitDisplayModeOnAccessKeyInvoked: XamlPropertyIndex = XamlPropertyIndex(1850i32);
    pub const MediaPlayerPresenter_IsFullWindow: XamlPropertyIndex = XamlPropertyIndex(1851i32);
    pub const MediaPlayerPresenter_MediaPlayer: XamlPropertyIndex = XamlPropertyIndex(1852i32);
    pub const MediaPlayerPresenter_Stretch: XamlPropertyIndex = XamlPropertyIndex(1853i32);
    pub const MediaPlayerElement_AreTransportControlsEnabled: XamlPropertyIndex = XamlPropertyIndex(1854i32);
    pub const MediaPlayerElement_AutoPlay: XamlPropertyIndex = XamlPropertyIndex(1855i32);
    pub const MediaPlayerElement_IsFullWindow: XamlPropertyIndex = XamlPropertyIndex(1856i32);
    pub const MediaPlayerElement_MediaPlayer: XamlPropertyIndex = XamlPropertyIndex(1857i32);
    pub const MediaPlayerElement_PosterSource: XamlPropertyIndex = XamlPropertyIndex(1858i32);
    pub const MediaPlayerElement_Source: XamlPropertyIndex = XamlPropertyIndex(1859i32);
    pub const MediaPlayerElement_Stretch: XamlPropertyIndex = XamlPropertyIndex(1860i32);
    pub const MediaPlayerElement_TransportControls: XamlPropertyIndex = XamlPropertyIndex(1861i32);
    pub const MediaTransportControls_FastPlayFallbackBehaviour: XamlPropertyIndex = XamlPropertyIndex(1862i32);
    pub const MediaTransportControls_IsNextTrackButtonVisible: XamlPropertyIndex = XamlPropertyIndex(1863i32);
    pub const MediaTransportControls_IsPreviousTrackButtonVisible: XamlPropertyIndex = XamlPropertyIndex(1864i32);
    pub const MediaTransportControls_IsSkipBackwardButtonVisible: XamlPropertyIndex = XamlPropertyIndex(1865i32);
    pub const MediaTransportControls_IsSkipBackwardEnabled: XamlPropertyIndex = XamlPropertyIndex(1866i32);
    pub const MediaTransportControls_IsSkipForwardButtonVisible: XamlPropertyIndex = XamlPropertyIndex(1867i32);
    pub const MediaTransportControls_IsSkipForwardEnabled: XamlPropertyIndex = XamlPropertyIndex(1868i32);
    pub const FlyoutBase_ElementSoundMode: XamlPropertyIndex = XamlPropertyIndex(1869i32);
    pub const Control_ElementSoundMode: XamlPropertyIndex = XamlPropertyIndex(1870i32);
    pub const Hyperlink_ElementSoundMode: XamlPropertyIndex = XamlPropertyIndex(1871i32);
    pub const AutomationProperties_FlowsFrom: XamlPropertyIndex = XamlPropertyIndex(1876i32);
    pub const AutomationProperties_FlowsTo: XamlPropertyIndex = XamlPropertyIndex(1877i32);
    pub const TextElement_TextDecorations: XamlPropertyIndex = XamlPropertyIndex(1879i32);
    pub const RichTextBlock_TextDecorations: XamlPropertyIndex = XamlPropertyIndex(1881i32);
    pub const Control_DefaultStyleResourceUri: XamlPropertyIndex = XamlPropertyIndex(1882i32);
    pub const ContentDialog_PrimaryButtonStyle: XamlPropertyIndex = XamlPropertyIndex(1884i32);
    pub const ContentDialog_SecondaryButtonStyle: XamlPropertyIndex = XamlPropertyIndex(1885i32);
    pub const TextElement_KeyTipHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(1890i32);
    pub const TextElement_KeyTipPlacementMode: XamlPropertyIndex = XamlPropertyIndex(1891i32);
    pub const TextElement_KeyTipVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(1892i32);
    pub const UIElement_KeyTipHorizontalOffset: XamlPropertyIndex = XamlPropertyIndex(1893i32);
    pub const UIElement_KeyTipPlacementMode: XamlPropertyIndex = XamlPropertyIndex(1894i32);
    pub const UIElement_KeyTipVerticalOffset: XamlPropertyIndex = XamlPropertyIndex(1895i32);
    pub const FlyoutBase_OverlayInputPassThroughElement: XamlPropertyIndex = XamlPropertyIndex(1896i32);
    pub const UIElement_XYFocusKeyboardNavigation: XamlPropertyIndex = XamlPropertyIndex(1897i32);
    pub const AutomationProperties_Culture: XamlPropertyIndex = XamlPropertyIndex(1898i32);
    pub const UIElement_XYFocusDownNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(1918i32);
    pub const UIElement_XYFocusLeftNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(1919i32);
    pub const UIElement_XYFocusRightNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(1920i32);
    pub const UIElement_XYFocusUpNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(1921i32);
    pub const Hyperlink_XYFocusDownNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(1922i32);
    pub const Hyperlink_XYFocusLeftNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(1923i32);
    pub const Hyperlink_XYFocusRightNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(1924i32);
    pub const Hyperlink_XYFocusUpNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(1925i32);
    pub const TextElement_AccessKeyScopeOwner: XamlPropertyIndex = XamlPropertyIndex(1926i32);
    pub const TextElement_IsAccessKeyScope: XamlPropertyIndex = XamlPropertyIndex(1927i32);
    pub const Hyperlink_FocusState: XamlPropertyIndex = XamlPropertyIndex(1934i32);
    pub const ContentDialog_CloseButtonCommand: XamlPropertyIndex = XamlPropertyIndex(1936i32);
    pub const ContentDialog_CloseButtonCommandParameter: XamlPropertyIndex = XamlPropertyIndex(1937i32);
    pub const ContentDialog_CloseButtonStyle: XamlPropertyIndex = XamlPropertyIndex(1938i32);
    pub const ContentDialog_CloseButtonText: XamlPropertyIndex = XamlPropertyIndex(1939i32);
    pub const ContentDialog_DefaultButton: XamlPropertyIndex = XamlPropertyIndex(1940i32);
    pub const RichEditBox_SelectionHighlightColorWhenNotFocused: XamlPropertyIndex = XamlPropertyIndex(1941i32);
    pub const TextBox_SelectionHighlightColorWhenNotFocused: XamlPropertyIndex = XamlPropertyIndex(1942i32);
    pub const SvgImageSource_RasterizePixelHeight: XamlPropertyIndex = XamlPropertyIndex(1948i32);
    pub const SvgImageSource_RasterizePixelWidth: XamlPropertyIndex = XamlPropertyIndex(1949i32);
    pub const SvgImageSource_UriSource: XamlPropertyIndex = XamlPropertyIndex(1950i32);
    pub const LoadedImageSurface_DecodedPhysicalSize: XamlPropertyIndex = XamlPropertyIndex(1955i32);
    pub const LoadedImageSurface_DecodedSize: XamlPropertyIndex = XamlPropertyIndex(1956i32);
    pub const LoadedImageSurface_NaturalSize: XamlPropertyIndex = XamlPropertyIndex(1957i32);
    pub const ComboBox_SelectionChangedTrigger: XamlPropertyIndex = XamlPropertyIndex(1958i32);
    pub const XamlCompositionBrushBase_FallbackColor: XamlPropertyIndex = XamlPropertyIndex(1960i32);
    pub const UIElement_Lights: XamlPropertyIndex = XamlPropertyIndex(1962i32);
    pub const MenuFlyoutItem_Icon: XamlPropertyIndex = XamlPropertyIndex(1963i32);
    pub const MenuFlyoutSubItem_Icon: XamlPropertyIndex = XamlPropertyIndex(1964i32);
    pub const BitmapIcon_ShowAsMonochrome: XamlPropertyIndex = XamlPropertyIndex(1965i32);
    pub const UIElement_HighContrastAdjustment: XamlPropertyIndex = XamlPropertyIndex(1967i32);
    pub const RichEditBox_MaxLength: XamlPropertyIndex = XamlPropertyIndex(1968i32);
    pub const UIElement_TabFocusNavigation: XamlPropertyIndex = XamlPropertyIndex(1969i32);
    pub const Control_IsTemplateKeyTipTarget: XamlPropertyIndex = XamlPropertyIndex(1970i32);
    pub const Hyperlink_IsTabStop: XamlPropertyIndex = XamlPropertyIndex(1972i32);
    pub const Hyperlink_TabIndex: XamlPropertyIndex = XamlPropertyIndex(1973i32);
    pub const MediaTransportControls_IsRepeatButtonVisible: XamlPropertyIndex = XamlPropertyIndex(1974i32);
    pub const MediaTransportControls_IsRepeatEnabled: XamlPropertyIndex = XamlPropertyIndex(1975i32);
    pub const MediaTransportControls_ShowAndHideAutomatically: XamlPropertyIndex = XamlPropertyIndex(1976i32);
    pub const RichEditBox_DisabledFormattingAccelerators: XamlPropertyIndex = XamlPropertyIndex(1977i32);
    pub const RichEditBox_CharacterCasing: XamlPropertyIndex = XamlPropertyIndex(1978i32);
    pub const TextBox_CharacterCasing: XamlPropertyIndex = XamlPropertyIndex(1979i32);
    pub const RichTextBlock_IsTextTrimmed: XamlPropertyIndex = XamlPropertyIndex(1980i32);
    pub const RichTextBlockOverflow_IsTextTrimmed: XamlPropertyIndex = XamlPropertyIndex(1981i32);
    pub const TextBlock_IsTextTrimmed: XamlPropertyIndex = XamlPropertyIndex(1982i32);
    pub const TextHighlighter_Background: XamlPropertyIndex = XamlPropertyIndex(1985i32);
    pub const TextHighlighter_Foreground: XamlPropertyIndex = XamlPropertyIndex(1986i32);
    pub const TextHighlighter_Ranges: XamlPropertyIndex = XamlPropertyIndex(1987i32);
    pub const RichTextBlock_TextHighlighters: XamlPropertyIndex = XamlPropertyIndex(1988i32);
    pub const TextBlock_TextHighlighters: XamlPropertyIndex = XamlPropertyIndex(1989i32);
    pub const FrameworkElement_ActualTheme: XamlPropertyIndex = XamlPropertyIndex(1992i32);
    pub const Grid_ColumnSpacing: XamlPropertyIndex = XamlPropertyIndex(1993i32);
    pub const Grid_RowSpacing: XamlPropertyIndex = XamlPropertyIndex(1994i32);
    pub const StackPanel_Spacing: XamlPropertyIndex = XamlPropertyIndex(1995i32);
    pub const Block_HorizontalTextAlignment: XamlPropertyIndex = XamlPropertyIndex(1996i32);
    pub const RichTextBlock_HorizontalTextAlignment: XamlPropertyIndex = XamlPropertyIndex(1997i32);
    pub const TextBlock_HorizontalTextAlignment: XamlPropertyIndex = XamlPropertyIndex(1998i32);
    pub const RichEditBox_HorizontalTextAlignment: XamlPropertyIndex = XamlPropertyIndex(1999i32);
    pub const TextBox_HorizontalTextAlignment: XamlPropertyIndex = XamlPropertyIndex(2000i32);
    pub const TextBox_PlaceholderForeground: XamlPropertyIndex = XamlPropertyIndex(2001i32);
    pub const ComboBox_PlaceholderForeground: XamlPropertyIndex = XamlPropertyIndex(2002i32);
    pub const KeyboardAccelerator_IsEnabled: XamlPropertyIndex = XamlPropertyIndex(2003i32);
    pub const KeyboardAccelerator_Key: XamlPropertyIndex = XamlPropertyIndex(2004i32);
    pub const KeyboardAccelerator_Modifiers: XamlPropertyIndex = XamlPropertyIndex(2005i32);
    pub const KeyboardAccelerator_ScopeOwner: XamlPropertyIndex = XamlPropertyIndex(2006i32);
    pub const UIElement_KeyboardAccelerators: XamlPropertyIndex = XamlPropertyIndex(2007i32);
    pub const ListViewItemPresenter_RevealBackground: XamlPropertyIndex = XamlPropertyIndex(2009i32);
    pub const ListViewItemPresenter_RevealBackgroundShowsAboveContent: XamlPropertyIndex = XamlPropertyIndex(2010i32);
    pub const ListViewItemPresenter_RevealBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2011i32);
    pub const ListViewItemPresenter_RevealBorderThickness: XamlPropertyIndex = XamlPropertyIndex(2012i32);
    pub const UIElement_KeyTipTarget: XamlPropertyIndex = XamlPropertyIndex(2014i32);
    pub const AppBarButtonTemplateSettings_KeyboardAcceleratorTextMinWidth: XamlPropertyIndex = XamlPropertyIndex(2015i32);
    pub const AppBarToggleButtonTemplateSettings_KeyboardAcceleratorTextMinWidth: XamlPropertyIndex = XamlPropertyIndex(2016i32);
    pub const MenuFlyoutItemTemplateSettings_KeyboardAcceleratorTextMinWidth: XamlPropertyIndex = XamlPropertyIndex(2017i32);
    pub const MenuFlyoutItem_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(2019i32);
    pub const AppBarButton_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(2021i32);
    pub const AppBarToggleButton_TemplateSettings: XamlPropertyIndex = XamlPropertyIndex(2023i32);
    pub const UIElement_KeyboardAcceleratorPlacementMode: XamlPropertyIndex = XamlPropertyIndex(2028i32);
    pub const MediaTransportControls_IsCompactOverlayButtonVisible: XamlPropertyIndex = XamlPropertyIndex(2032i32);
    pub const MediaTransportControls_IsCompactOverlayEnabled: XamlPropertyIndex = XamlPropertyIndex(2033i32);
    pub const UIElement_KeyboardAcceleratorPlacementTarget: XamlPropertyIndex = XamlPropertyIndex(2061i32);
    pub const UIElement_CenterPoint: XamlPropertyIndex = XamlPropertyIndex(2062i32);
    pub const UIElement_Rotation: XamlPropertyIndex = XamlPropertyIndex(2063i32);
    pub const UIElement_RotationAxis: XamlPropertyIndex = XamlPropertyIndex(2064i32);
    pub const UIElement_Scale: XamlPropertyIndex = XamlPropertyIndex(2065i32);
    pub const UIElement_TransformMatrix: XamlPropertyIndex = XamlPropertyIndex(2066i32);
    pub const UIElement_Translation: XamlPropertyIndex = XamlPropertyIndex(2067i32);
    pub const TextBox_HandwritingView: XamlPropertyIndex = XamlPropertyIndex(2068i32);
    pub const AutomationProperties_HeadingLevel: XamlPropertyIndex = XamlPropertyIndex(2069i32);
    pub const TextBox_IsHandwritingViewEnabled: XamlPropertyIndex = XamlPropertyIndex(2076i32);
    pub const RichEditBox_ContentLinkProviders: XamlPropertyIndex = XamlPropertyIndex(2078i32);
    pub const RichEditBox_ContentLinkBackgroundColor: XamlPropertyIndex = XamlPropertyIndex(2079i32);
    pub const RichEditBox_ContentLinkForegroundColor: XamlPropertyIndex = XamlPropertyIndex(2080i32);
    pub const HandwritingView_AreCandidatesEnabled: XamlPropertyIndex = XamlPropertyIndex(2081i32);
    pub const HandwritingView_IsOpen: XamlPropertyIndex = XamlPropertyIndex(2082i32);
    pub const HandwritingView_PlacementTarget: XamlPropertyIndex = XamlPropertyIndex(2084i32);
    pub const HandwritingView_PlacementAlignment: XamlPropertyIndex = XamlPropertyIndex(2085i32);
    pub const RichEditBox_HandwritingView: XamlPropertyIndex = XamlPropertyIndex(2086i32);
    pub const RichEditBox_IsHandwritingViewEnabled: XamlPropertyIndex = XamlPropertyIndex(2087i32);
    pub const MenuFlyoutItem_KeyboardAcceleratorTextOverride: XamlPropertyIndex = XamlPropertyIndex(2090i32);
    pub const AppBarButton_KeyboardAcceleratorTextOverride: XamlPropertyIndex = XamlPropertyIndex(2091i32);
    pub const AppBarToggleButton_KeyboardAcceleratorTextOverride: XamlPropertyIndex = XamlPropertyIndex(2092i32);
    pub const ContentLink_Background: XamlPropertyIndex = XamlPropertyIndex(2093i32);
    pub const ContentLink_Cursor: XamlPropertyIndex = XamlPropertyIndex(2094i32);
    pub const ContentLink_ElementSoundMode: XamlPropertyIndex = XamlPropertyIndex(2095i32);
    pub const ContentLink_FocusState: XamlPropertyIndex = XamlPropertyIndex(2096i32);
    pub const ContentLink_IsTabStop: XamlPropertyIndex = XamlPropertyIndex(2097i32);
    pub const ContentLink_TabIndex: XamlPropertyIndex = XamlPropertyIndex(2098i32);
    pub const ContentLink_XYFocusDown: XamlPropertyIndex = XamlPropertyIndex(2099i32);
    pub const ContentLink_XYFocusDownNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(2100i32);
    pub const ContentLink_XYFocusLeft: XamlPropertyIndex = XamlPropertyIndex(2101i32);
    pub const ContentLink_XYFocusLeftNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(2102i32);
    pub const ContentLink_XYFocusRight: XamlPropertyIndex = XamlPropertyIndex(2103i32);
    pub const ContentLink_XYFocusRightNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(2104i32);
    pub const ContentLink_XYFocusUp: XamlPropertyIndex = XamlPropertyIndex(2105i32);
    pub const ContentLink_XYFocusUpNavigationStrategy: XamlPropertyIndex = XamlPropertyIndex(2106i32);
    pub const IconSource_Foreground: XamlPropertyIndex = XamlPropertyIndex(2112i32);
    pub const BitmapIconSource_ShowAsMonochrome: XamlPropertyIndex = XamlPropertyIndex(2113i32);
    pub const BitmapIconSource_UriSource: XamlPropertyIndex = XamlPropertyIndex(2114i32);
    pub const FontIconSource_FontFamily: XamlPropertyIndex = XamlPropertyIndex(2115i32);
    pub const FontIconSource_FontSize: XamlPropertyIndex = XamlPropertyIndex(2116i32);
    pub const FontIconSource_FontStyle: XamlPropertyIndex = XamlPropertyIndex(2117i32);
    pub const FontIconSource_FontWeight: XamlPropertyIndex = XamlPropertyIndex(2118i32);
    pub const FontIconSource_Glyph: XamlPropertyIndex = XamlPropertyIndex(2119i32);
    pub const FontIconSource_IsTextScaleFactorEnabled: XamlPropertyIndex = XamlPropertyIndex(2120i32);
    pub const FontIconSource_MirroredWhenRightToLeft: XamlPropertyIndex = XamlPropertyIndex(2121i32);
    pub const PathIconSource_Data: XamlPropertyIndex = XamlPropertyIndex(2122i32);
    pub const SymbolIconSource_Symbol: XamlPropertyIndex = XamlPropertyIndex(2123i32);
    pub const UIElement_Shadow: XamlPropertyIndex = XamlPropertyIndex(2130i32);
    pub const IconSourceElement_IconSource: XamlPropertyIndex = XamlPropertyIndex(2131i32);
    pub const PasswordBox_CanPasteClipboardContent: XamlPropertyIndex = XamlPropertyIndex(2137i32);
    pub const TextBox_CanPasteClipboardContent: XamlPropertyIndex = XamlPropertyIndex(2138i32);
    pub const TextBox_CanRedo: XamlPropertyIndex = XamlPropertyIndex(2139i32);
    pub const TextBox_CanUndo: XamlPropertyIndex = XamlPropertyIndex(2140i32);
    pub const FlyoutBase_ShowMode: XamlPropertyIndex = XamlPropertyIndex(2141i32);
    pub const FlyoutBase_Target: XamlPropertyIndex = XamlPropertyIndex(2142i32);
    pub const Control_CornerRadius: XamlPropertyIndex = XamlPropertyIndex(2143i32);
    pub const AutomationProperties_IsDialog: XamlPropertyIndex = XamlPropertyIndex(2149i32);
    pub const AppBarElementContainer_DynamicOverflowOrder: XamlPropertyIndex = XamlPropertyIndex(2150i32);
    pub const AppBarElementContainer_IsCompact: XamlPropertyIndex = XamlPropertyIndex(2151i32);
    pub const AppBarElementContainer_IsInOverflow: XamlPropertyIndex = XamlPropertyIndex(2152i32);
    pub const ScrollContentPresenter_CanContentRenderOutsideBounds: XamlPropertyIndex = XamlPropertyIndex(2157i32);
    pub const ScrollViewer_CanContentRenderOutsideBounds: XamlPropertyIndex = XamlPropertyIndex(2158i32);
    pub const RichEditBox_SelectionFlyout: XamlPropertyIndex = XamlPropertyIndex(2159i32);
    pub const TextBox_SelectionFlyout: XamlPropertyIndex = XamlPropertyIndex(2160i32);
    pub const Border_BackgroundSizing: XamlPropertyIndex = XamlPropertyIndex(2161i32);
    pub const ContentPresenter_BackgroundSizing: XamlPropertyIndex = XamlPropertyIndex(2162i32);
    pub const Control_BackgroundSizing: XamlPropertyIndex = XamlPropertyIndex(2163i32);
    pub const Grid_BackgroundSizing: XamlPropertyIndex = XamlPropertyIndex(2164i32);
    pub const RelativePanel_BackgroundSizing: XamlPropertyIndex = XamlPropertyIndex(2165i32);
    pub const StackPanel_BackgroundSizing: XamlPropertyIndex = XamlPropertyIndex(2166i32);
    pub const ScrollViewer_HorizontalAnchorRatio: XamlPropertyIndex = XamlPropertyIndex(2170i32);
    pub const ScrollViewer_VerticalAnchorRatio: XamlPropertyIndex = XamlPropertyIndex(2171i32);
    pub const ComboBox_Text: XamlPropertyIndex = XamlPropertyIndex(2208i32);
    pub const TextBox_Description: XamlPropertyIndex = XamlPropertyIndex(2217i32);
    pub const ToolTip_PlacementRect: XamlPropertyIndex = XamlPropertyIndex(2218i32);
    pub const RichTextBlock_SelectionFlyout: XamlPropertyIndex = XamlPropertyIndex(2219i32);
    pub const TextBlock_SelectionFlyout: XamlPropertyIndex = XamlPropertyIndex(2220i32);
    pub const PasswordBox_SelectionFlyout: XamlPropertyIndex = XamlPropertyIndex(2221i32);
    pub const Border_BackgroundTransition: XamlPropertyIndex = XamlPropertyIndex(2222i32);
    pub const ContentPresenter_BackgroundTransition: XamlPropertyIndex = XamlPropertyIndex(2223i32);
    pub const Panel_BackgroundTransition: XamlPropertyIndex = XamlPropertyIndex(2224i32);
    pub const ColorPaletteResources_Accent: XamlPropertyIndex = XamlPropertyIndex(2227i32);
    pub const ColorPaletteResources_AltHigh: XamlPropertyIndex = XamlPropertyIndex(2228i32);
    pub const ColorPaletteResources_AltLow: XamlPropertyIndex = XamlPropertyIndex(2229i32);
    pub const ColorPaletteResources_AltMedium: XamlPropertyIndex = XamlPropertyIndex(2230i32);
    pub const ColorPaletteResources_AltMediumHigh: XamlPropertyIndex = XamlPropertyIndex(2231i32);
    pub const ColorPaletteResources_AltMediumLow: XamlPropertyIndex = XamlPropertyIndex(2232i32);
    pub const ColorPaletteResources_BaseHigh: XamlPropertyIndex = XamlPropertyIndex(2233i32);
    pub const ColorPaletteResources_BaseLow: XamlPropertyIndex = XamlPropertyIndex(2234i32);
    pub const ColorPaletteResources_BaseMedium: XamlPropertyIndex = XamlPropertyIndex(2235i32);
    pub const ColorPaletteResources_BaseMediumHigh: XamlPropertyIndex = XamlPropertyIndex(2236i32);
    pub const ColorPaletteResources_BaseMediumLow: XamlPropertyIndex = XamlPropertyIndex(2237i32);
    pub const ColorPaletteResources_ChromeAltLow: XamlPropertyIndex = XamlPropertyIndex(2238i32);
    pub const ColorPaletteResources_ChromeBlackHigh: XamlPropertyIndex = XamlPropertyIndex(2239i32);
    pub const ColorPaletteResources_ChromeBlackLow: XamlPropertyIndex = XamlPropertyIndex(2240i32);
    pub const ColorPaletteResources_ChromeBlackMedium: XamlPropertyIndex = XamlPropertyIndex(2241i32);
    pub const ColorPaletteResources_ChromeBlackMediumLow: XamlPropertyIndex = XamlPropertyIndex(2242i32);
    pub const ColorPaletteResources_ChromeDisabledHigh: XamlPropertyIndex = XamlPropertyIndex(2243i32);
    pub const ColorPaletteResources_ChromeDisabledLow: XamlPropertyIndex = XamlPropertyIndex(2244i32);
    pub const ColorPaletteResources_ChromeGray: XamlPropertyIndex = XamlPropertyIndex(2245i32);
    pub const ColorPaletteResources_ChromeHigh: XamlPropertyIndex = XamlPropertyIndex(2246i32);
    pub const ColorPaletteResources_ChromeLow: XamlPropertyIndex = XamlPropertyIndex(2247i32);
    pub const ColorPaletteResources_ChromeMedium: XamlPropertyIndex = XamlPropertyIndex(2248i32);
    pub const ColorPaletteResources_ChromeMediumLow: XamlPropertyIndex = XamlPropertyIndex(2249i32);
    pub const ColorPaletteResources_ChromeWhite: XamlPropertyIndex = XamlPropertyIndex(2250i32);
    pub const ColorPaletteResources_ErrorText: XamlPropertyIndex = XamlPropertyIndex(2252i32);
    pub const ColorPaletteResources_ListLow: XamlPropertyIndex = XamlPropertyIndex(2253i32);
    pub const ColorPaletteResources_ListMedium: XamlPropertyIndex = XamlPropertyIndex(2254i32);
    pub const UIElement_TranslationTransition: XamlPropertyIndex = XamlPropertyIndex(2255i32);
    pub const UIElement_OpacityTransition: XamlPropertyIndex = XamlPropertyIndex(2256i32);
    pub const UIElement_RotationTransition: XamlPropertyIndex = XamlPropertyIndex(2257i32);
    pub const UIElement_ScaleTransition: XamlPropertyIndex = XamlPropertyIndex(2258i32);
    pub const BrushTransition_Duration: XamlPropertyIndex = XamlPropertyIndex(2261i32);
    pub const ScalarTransition_Duration: XamlPropertyIndex = XamlPropertyIndex(2262i32);
    pub const Vector3Transition_Duration: XamlPropertyIndex = XamlPropertyIndex(2263i32);
    pub const Vector3Transition_Components: XamlPropertyIndex = XamlPropertyIndex(2266i32);
    pub const FlyoutBase_IsOpen: XamlPropertyIndex = XamlPropertyIndex(2267i32);
    pub const StandardUICommand_Kind: XamlPropertyIndex = XamlPropertyIndex(2275i32);
    pub const UIElement_CanBeScrollAnchor: XamlPropertyIndex = XamlPropertyIndex(2276i32);
    pub const ThemeShadow_Receivers: XamlPropertyIndex = XamlPropertyIndex(2279i32);
    pub const ScrollContentPresenter_SizesContentToTemplatedParent: XamlPropertyIndex = XamlPropertyIndex(2280i32);
    pub const ComboBox_TextBoxStyle: XamlPropertyIndex = XamlPropertyIndex(2281i32);
    pub const Frame_IsNavigationStackEnabled: XamlPropertyIndex = XamlPropertyIndex(2282i32);
    pub const RichEditBox_ProofingMenuFlyout: XamlPropertyIndex = XamlPropertyIndex(2283i32);
    pub const TextBox_ProofingMenuFlyout: XamlPropertyIndex = XamlPropertyIndex(2284i32);
    pub const ScrollViewer_ReduceViewportForCoreInputViewOcclusions: XamlPropertyIndex = XamlPropertyIndex(2295i32);
    pub const FlyoutBase_AreOpenCloseAnimationsEnabled: XamlPropertyIndex = XamlPropertyIndex(2296i32);
    pub const FlyoutBase_InputDevicePrefersPrimaryCommands: XamlPropertyIndex = XamlPropertyIndex(2297i32);
    pub const CalendarDatePicker_Description: XamlPropertyIndex = XamlPropertyIndex(2300i32);
    pub const PasswordBox_Description: XamlPropertyIndex = XamlPropertyIndex(2308i32);
    pub const RichEditBox_Description: XamlPropertyIndex = XamlPropertyIndex(2316i32);
    pub const AutoSuggestBox_Description: XamlPropertyIndex = XamlPropertyIndex(2331i32);
    pub const ComboBox_Description: XamlPropertyIndex = XamlPropertyIndex(2339i32);
    pub const XamlUICommand_AccessKey: XamlPropertyIndex = XamlPropertyIndex(2347i32);
    pub const XamlUICommand_Command: XamlPropertyIndex = XamlPropertyIndex(2348i32);
    pub const XamlUICommand_Description: XamlPropertyIndex = XamlPropertyIndex(2349i32);
    pub const XamlUICommand_IconSource: XamlPropertyIndex = XamlPropertyIndex(2350i32);
    pub const XamlUICommand_KeyboardAccelerators: XamlPropertyIndex = XamlPropertyIndex(2351i32);
    pub const XamlUICommand_Label: XamlPropertyIndex = XamlPropertyIndex(2352i32);
    pub const DatePicker_SelectedDate: XamlPropertyIndex = XamlPropertyIndex(2355i32);
    pub const TimePicker_SelectedTime: XamlPropertyIndex = XamlPropertyIndex(2356i32);
    pub const AppBarTemplateSettings_NegativeCompactVerticalDelta: XamlPropertyIndex = XamlPropertyIndex(2367i32);
    pub const AppBarTemplateSettings_NegativeHiddenVerticalDelta: XamlPropertyIndex = XamlPropertyIndex(2368i32);
    pub const AppBarTemplateSettings_NegativeMinimalVerticalDelta: XamlPropertyIndex = XamlPropertyIndex(2369i32);
    pub const FlyoutBase_ShouldConstrainToRootBounds: XamlPropertyIndex = XamlPropertyIndex(2378i32);
    pub const Popup_ShouldConstrainToRootBounds: XamlPropertyIndex = XamlPropertyIndex(2379i32);
    pub const FlyoutPresenter_IsDefaultShadowEnabled: XamlPropertyIndex = XamlPropertyIndex(2380i32);
    pub const MenuFlyoutPresenter_IsDefaultShadowEnabled: XamlPropertyIndex = XamlPropertyIndex(2381i32);
    pub const UIElement_ActualOffset: XamlPropertyIndex = XamlPropertyIndex(2382i32);
    pub const UIElement_ActualSize: XamlPropertyIndex = XamlPropertyIndex(2383i32);
    pub const CommandBarTemplateSettings_OverflowContentCompactYTranslation: XamlPropertyIndex = XamlPropertyIndex(2384i32);
    pub const CommandBarTemplateSettings_OverflowContentHiddenYTranslation: XamlPropertyIndex = XamlPropertyIndex(2385i32);
    pub const CommandBarTemplateSettings_OverflowContentMinimalYTranslation: XamlPropertyIndex = XamlPropertyIndex(2386i32);
    pub const HandwritingView_IsCommandBarOpen: XamlPropertyIndex = XamlPropertyIndex(2395i32);
    pub const HandwritingView_IsSwitchToKeyboardEnabled: XamlPropertyIndex = XamlPropertyIndex(2396i32);
    pub const ListViewItemPresenter_SelectionIndicatorVisualEnabled: XamlPropertyIndex = XamlPropertyIndex(2399i32);
    pub const ListViewItemPresenter_SelectionIndicatorBrush: XamlPropertyIndex = XamlPropertyIndex(2400i32);
    pub const ListViewItemPresenter_SelectionIndicatorMode: XamlPropertyIndex = XamlPropertyIndex(2401i32);
    pub const ListViewItemPresenter_SelectionIndicatorPointerOverBrush: XamlPropertyIndex = XamlPropertyIndex(2402i32);
    pub const ListViewItemPresenter_SelectionIndicatorPressedBrush: XamlPropertyIndex = XamlPropertyIndex(2403i32);
    pub const ListViewItemPresenter_SelectedBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2410i32);
    pub const ListViewItemPresenter_SelectedInnerBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2411i32);
    pub const ListViewItemPresenter_CheckBoxCornerRadius: XamlPropertyIndex = XamlPropertyIndex(2412i32);
    pub const ListViewItemPresenter_SelectionIndicatorCornerRadius: XamlPropertyIndex = XamlPropertyIndex(2413i32);
    pub const ListViewItemPresenter_SelectedDisabledBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2414i32);
    pub const ListViewItemPresenter_SelectedPressedBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2415i32);
    pub const ListViewItemPresenter_SelectedDisabledBackground: XamlPropertyIndex = XamlPropertyIndex(2416i32);
    pub const ListViewItemPresenter_PointerOverBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2417i32);
    pub const ListViewItemPresenter_CheckBoxPointerOverBrush: XamlPropertyIndex = XamlPropertyIndex(2418i32);
    pub const ListViewItemPresenter_CheckBoxPressedBrush: XamlPropertyIndex = XamlPropertyIndex(2419i32);
    pub const ListViewItemPresenter_CheckDisabledBrush: XamlPropertyIndex = XamlPropertyIndex(2420i32);
    pub const ListViewItemPresenter_CheckPressedBrush: XamlPropertyIndex = XamlPropertyIndex(2421i32);
    pub const ListViewItemPresenter_CheckBoxBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2422i32);
    pub const ListViewItemPresenter_CheckBoxDisabledBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2423i32);
    pub const ListViewItemPresenter_CheckBoxPressedBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2424i32);
    pub const ListViewItemPresenter_CheckBoxDisabledBrush: XamlPropertyIndex = XamlPropertyIndex(2425i32);
    pub const ListViewItemPresenter_CheckBoxSelectedBrush: XamlPropertyIndex = XamlPropertyIndex(2426i32);
    pub const ListViewItemPresenter_CheckBoxSelectedDisabledBrush: XamlPropertyIndex = XamlPropertyIndex(2427i32);
    pub const ListViewItemPresenter_CheckBoxSelectedPointerOverBrush: XamlPropertyIndex = XamlPropertyIndex(2428i32);
    pub const ListViewItemPresenter_CheckBoxSelectedPressedBrush: XamlPropertyIndex = XamlPropertyIndex(2429i32);
    pub const ListViewItemPresenter_CheckBoxPointerOverBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2430i32);
    pub const ListViewItemPresenter_SelectionIndicatorDisabledBrush: XamlPropertyIndex = XamlPropertyIndex(2431i32);
    pub const CalendarView_BlackoutBackground: XamlPropertyIndex = XamlPropertyIndex(2432i32);
    pub const CalendarView_BlackoutStrikethroughBrush: XamlPropertyIndex = XamlPropertyIndex(2433i32);
    pub const CalendarView_CalendarItemCornerRadius: XamlPropertyIndex = XamlPropertyIndex(2434i32);
    pub const CalendarView_CalendarItemDisabledBackground: XamlPropertyIndex = XamlPropertyIndex(2435i32);
    pub const CalendarView_CalendarItemHoverBackground: XamlPropertyIndex = XamlPropertyIndex(2436i32);
    pub const CalendarView_CalendarItemPressedBackground: XamlPropertyIndex = XamlPropertyIndex(2437i32);
    pub const CalendarView_DayItemMargin: XamlPropertyIndex = XamlPropertyIndex(2438i32);
    pub const CalendarView_FirstOfMonthLabelMargin: XamlPropertyIndex = XamlPropertyIndex(2439i32);
    pub const CalendarView_FirstOfYearDecadeLabelMargin: XamlPropertyIndex = XamlPropertyIndex(2440i32);
    pub const CalendarView_MonthYearItemMargin: XamlPropertyIndex = XamlPropertyIndex(2441i32);
    pub const CalendarView_OutOfScopeHoverForeground: XamlPropertyIndex = XamlPropertyIndex(2442i32);
    pub const CalendarView_OutOfScopePressedForeground: XamlPropertyIndex = XamlPropertyIndex(2443i32);
    pub const CalendarView_SelectedDisabledBorderBrush: XamlPropertyIndex = XamlPropertyIndex(2444i32);
    pub const CalendarView_SelectedDisabledForeground: XamlPropertyIndex = XamlPropertyIndex(2445i32);
    pub const CalendarView_SelectedHoverForeground: XamlPropertyIndex = XamlPropertyIndex(2446i32);
    pub const CalendarView_SelectedPressedForeground: XamlPropertyIndex = XamlPropertyIndex(2447i32);
    pub const CalendarView_TodayBlackoutForeground: XamlPropertyIndex = XamlPropertyIndex(2448i32);
    pub const CalendarView_TodayDisabledBackground: XamlPropertyIndex = XamlPropertyIndex(2449i32);
    pub const CalendarView_TodayHoverBackground: XamlPropertyIndex = XamlPropertyIndex(2450i32);
    pub const CalendarView_TodayPressedBackground: XamlPropertyIndex = XamlPropertyIndex(2451i32);
    pub const Popup_ActualPlacement: XamlPropertyIndex = XamlPropertyIndex(2452i32);
    pub const Popup_DesiredPlacement: XamlPropertyIndex = XamlPropertyIndex(2453i32);
    pub const Popup_PlacementTarget: XamlPropertyIndex = XamlPropertyIndex(2454i32);
    pub const AutomationProperties_AutomationControlType: XamlPropertyIndex = XamlPropertyIndex(2455i32);
}
impl ::std::convert::From<i32> for XamlPropertyIndex {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XamlPropertyIndex {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XamlPropertyIndex {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Core.Direct.XamlPropertyIndex;i4)");
}
#[doc = "*Required features: `UI_Xaml_Core_Direct`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XamlTypeIndex(pub i32);
impl XamlTypeIndex {
    pub const AutoSuggestBoxSuggestionChosenEventArgs: XamlTypeIndex = XamlTypeIndex(34i32);
    pub const AutoSuggestBoxTextChangedEventArgs: XamlTypeIndex = XamlTypeIndex(35i32);
    pub const CollectionViewSource: XamlTypeIndex = XamlTypeIndex(41i32);
    pub const ColumnDefinition: XamlTypeIndex = XamlTypeIndex(44i32);
    pub const GradientStop: XamlTypeIndex = XamlTypeIndex(64i32);
    pub const InputScope: XamlTypeIndex = XamlTypeIndex(74i32);
    pub const InputScopeName: XamlTypeIndex = XamlTypeIndex(75i32);
    pub const KeySpline: XamlTypeIndex = XamlTypeIndex(78i32);
    pub const PathFigure: XamlTypeIndex = XamlTypeIndex(93i32);
    pub const PrintDocument: XamlTypeIndex = XamlTypeIndex(100i32);
    pub const RowDefinition: XamlTypeIndex = XamlTypeIndex(106i32);
    pub const Style: XamlTypeIndex = XamlTypeIndex(114i32);
    pub const TimelineMarker: XamlTypeIndex = XamlTypeIndex(126i32);
    pub const VisualState: XamlTypeIndex = XamlTypeIndex(137i32);
    pub const VisualStateGroup: XamlTypeIndex = XamlTypeIndex(138i32);
    pub const VisualStateManager: XamlTypeIndex = XamlTypeIndex(139i32);
    pub const VisualTransition: XamlTypeIndex = XamlTypeIndex(140i32);
    pub const AddDeleteThemeTransition: XamlTypeIndex = XamlTypeIndex(177i32);
    pub const ArcSegment: XamlTypeIndex = XamlTypeIndex(178i32);
    pub const BackEase: XamlTypeIndex = XamlTypeIndex(179i32);
    pub const BeginStoryboard: XamlTypeIndex = XamlTypeIndex(180i32);
    pub const BezierSegment: XamlTypeIndex = XamlTypeIndex(181i32);
    pub const BindingBase: XamlTypeIndex = XamlTypeIndex(182i32);
    pub const BitmapCache: XamlTypeIndex = XamlTypeIndex(183i32);
    pub const BounceEase: XamlTypeIndex = XamlTypeIndex(186i32);
    pub const CircleEase: XamlTypeIndex = XamlTypeIndex(187i32);
    pub const ColorAnimation: XamlTypeIndex = XamlTypeIndex(188i32);
    pub const ColorAnimationUsingKeyFrames: XamlTypeIndex = XamlTypeIndex(189i32);
    pub const ContentThemeTransition: XamlTypeIndex = XamlTypeIndex(190i32);
    pub const ControlTemplate: XamlTypeIndex = XamlTypeIndex(191i32);
    pub const CubicEase: XamlTypeIndex = XamlTypeIndex(192i32);
    pub const DataTemplate: XamlTypeIndex = XamlTypeIndex(194i32);
    pub const DiscreteColorKeyFrame: XamlTypeIndex = XamlTypeIndex(195i32);
    pub const DiscreteDoubleKeyFrame: XamlTypeIndex = XamlTypeIndex(196i32);
    pub const DiscreteObjectKeyFrame: XamlTypeIndex = XamlTypeIndex(197i32);
    pub const DiscretePointKeyFrame: XamlTypeIndex = XamlTypeIndex(198i32);
    pub const DoubleAnimation: XamlTypeIndex = XamlTypeIndex(200i32);
    pub const DoubleAnimationUsingKeyFrames: XamlTypeIndex = XamlTypeIndex(201i32);
    pub const EasingColorKeyFrame: XamlTypeIndex = XamlTypeIndex(204i32);
    pub const EasingDoubleKeyFrame: XamlTypeIndex = XamlTypeIndex(205i32);
    pub const EasingPointKeyFrame: XamlTypeIndex = XamlTypeIndex(206i32);
    pub const EdgeUIThemeTransition: XamlTypeIndex = XamlTypeIndex(207i32);
    pub const ElasticEase: XamlTypeIndex = XamlTypeIndex(208i32);
    pub const EllipseGeometry: XamlTypeIndex = XamlTypeIndex(209i32);
    pub const EntranceThemeTransition: XamlTypeIndex = XamlTypeIndex(210i32);
    pub const EventTrigger: XamlTypeIndex = XamlTypeIndex(211i32);
    pub const ExponentialEase: XamlTypeIndex = XamlTypeIndex(212i32);
    pub const Flyout: XamlTypeIndex = XamlTypeIndex(213i32);
    pub const GeometryGroup: XamlTypeIndex = XamlTypeIndex(216i32);
    pub const ItemsPanelTemplate: XamlTypeIndex = XamlTypeIndex(227i32);
    pub const LinearColorKeyFrame: XamlTypeIndex = XamlTypeIndex(230i32);
    pub const LinearDoubleKeyFrame: XamlTypeIndex = XamlTypeIndex(231i32);
    pub const LinearPointKeyFrame: XamlTypeIndex = XamlTypeIndex(232i32);
    pub const LineGeometry: XamlTypeIndex = XamlTypeIndex(233i32);
    pub const LineSegment: XamlTypeIndex = XamlTypeIndex(234i32);
    pub const Matrix3DProjection: XamlTypeIndex = XamlTypeIndex(236i32);
    pub const MenuFlyout: XamlTypeIndex = XamlTypeIndex(238i32);
    pub const ObjectAnimationUsingKeyFrames: XamlTypeIndex = XamlTypeIndex(240i32);
    pub const PaneThemeTransition: XamlTypeIndex = XamlTypeIndex(241i32);
    pub const PathGeometry: XamlTypeIndex = XamlTypeIndex(243i32);
    pub const PlaneProjection: XamlTypeIndex = XamlTypeIndex(244i32);
    pub const PointAnimation: XamlTypeIndex = XamlTypeIndex(245i32);
    pub const PointAnimationUsingKeyFrames: XamlTypeIndex = XamlTypeIndex(246i32);
    pub const PolyBezierSegment: XamlTypeIndex = XamlTypeIndex(248i32);
    pub const PolyLineSegment: XamlTypeIndex = XamlTypeIndex(249i32);
    pub const PolyQuadraticBezierSegment: XamlTypeIndex = XamlTypeIndex(250i32);
    pub const PopupThemeTransition: XamlTypeIndex = XamlTypeIndex(251i32);
    pub const PowerEase: XamlTypeIndex = XamlTypeIndex(252i32);
    pub const QuadraticBezierSegment: XamlTypeIndex = XamlTypeIndex(254i32);
    pub const QuadraticEase: XamlTypeIndex = XamlTypeIndex(255i32);
    pub const QuarticEase: XamlTypeIndex = XamlTypeIndex(256i32);
    pub const QuinticEase: XamlTypeIndex = XamlTypeIndex(257i32);
    pub const RectangleGeometry: XamlTypeIndex = XamlTypeIndex(258i32);
    pub const RelativeSource: XamlTypeIndex = XamlTypeIndex(259i32);
    pub const RenderTargetBitmap: XamlTypeIndex = XamlTypeIndex(260i32);
    pub const ReorderThemeTransition: XamlTypeIndex = XamlTypeIndex(261i32);
    pub const RepositionThemeTransition: XamlTypeIndex = XamlTypeIndex(262i32);
    pub const Setter: XamlTypeIndex = XamlTypeIndex(263i32);
    pub const SineEase: XamlTypeIndex = XamlTypeIndex(264i32);
    pub const SolidColorBrush: XamlTypeIndex = XamlTypeIndex(265i32);
    pub const SplineColorKeyFrame: XamlTypeIndex = XamlTypeIndex(266i32);
    pub const SplineDoubleKeyFrame: XamlTypeIndex = XamlTypeIndex(267i32);
    pub const SplinePointKeyFrame: XamlTypeIndex = XamlTypeIndex(268i32);
    pub const BitmapImage: XamlTypeIndex = XamlTypeIndex(285i32);
    pub const Border: XamlTypeIndex = XamlTypeIndex(286i32);
    pub const CaptureElement: XamlTypeIndex = XamlTypeIndex(288i32);
    pub const CompositeTransform: XamlTypeIndex = XamlTypeIndex(295i32);
    pub const ContentPresenter: XamlTypeIndex = XamlTypeIndex(296i32);
    pub const DragItemThemeAnimation: XamlTypeIndex = XamlTypeIndex(302i32);
    pub const DragOverThemeAnimation: XamlTypeIndex = XamlTypeIndex(303i32);
    pub const DropTargetItemThemeAnimation: XamlTypeIndex = XamlTypeIndex(304i32);
    pub const FadeInThemeAnimation: XamlTypeIndex = XamlTypeIndex(306i32);
    pub const FadeOutThemeAnimation: XamlTypeIndex = XamlTypeIndex(307i32);
    pub const Glyphs: XamlTypeIndex = XamlTypeIndex(312i32);
    pub const Image: XamlTypeIndex = XamlTypeIndex(326i32);
    pub const ImageBrush: XamlTypeIndex = XamlTypeIndex(328i32);
    pub const InlineUIContainer: XamlTypeIndex = XamlTypeIndex(329i32);
    pub const ItemsPresenter: XamlTypeIndex = XamlTypeIndex(332i32);
    pub const LinearGradientBrush: XamlTypeIndex = XamlTypeIndex(334i32);
    pub const LineBreak: XamlTypeIndex = XamlTypeIndex(335i32);
    pub const MatrixTransform: XamlTypeIndex = XamlTypeIndex(340i32);
    pub const MediaElement: XamlTypeIndex = XamlTypeIndex(342i32);
    pub const Paragraph: XamlTypeIndex = XamlTypeIndex(349i32);
    pub const PointerDownThemeAnimation: XamlTypeIndex = XamlTypeIndex(357i32);
    pub const PointerUpThemeAnimation: XamlTypeIndex = XamlTypeIndex(359i32);
    pub const PopInThemeAnimation: XamlTypeIndex = XamlTypeIndex(361i32);
    pub const PopOutThemeAnimation: XamlTypeIndex = XamlTypeIndex(362i32);
    pub const Popup: XamlTypeIndex = XamlTypeIndex(363i32);
    pub const RepositionThemeAnimation: XamlTypeIndex = XamlTypeIndex(370i32);
    pub const ResourceDictionary: XamlTypeIndex = XamlTypeIndex(371i32);
    pub const RichTextBlock: XamlTypeIndex = XamlTypeIndex(374i32);
    pub const RichTextBlockOverflow: XamlTypeIndex = XamlTypeIndex(376i32);
    pub const RotateTransform: XamlTypeIndex = XamlTypeIndex(378i32);
    pub const Run: XamlTypeIndex = XamlTypeIndex(380i32);
    pub const ScaleTransform: XamlTypeIndex = XamlTypeIndex(381i32);
    pub const SkewTransform: XamlTypeIndex = XamlTypeIndex(389i32);
    pub const Span: XamlTypeIndex = XamlTypeIndex(390i32);
    pub const SplitCloseThemeAnimation: XamlTypeIndex = XamlTypeIndex(391i32);
    pub const SplitOpenThemeAnimation: XamlTypeIndex = XamlTypeIndex(392i32);
    pub const Storyboard: XamlTypeIndex = XamlTypeIndex(393i32);
    pub const SwipeBackThemeAnimation: XamlTypeIndex = XamlTypeIndex(394i32);
    pub const SwipeHintThemeAnimation: XamlTypeIndex = XamlTypeIndex(395i32);
    pub const TextBlock: XamlTypeIndex = XamlTypeIndex(396i32);
    pub const TransformGroup: XamlTypeIndex = XamlTypeIndex(411i32);
    pub const TranslateTransform: XamlTypeIndex = XamlTypeIndex(413i32);
    pub const Viewbox: XamlTypeIndex = XamlTypeIndex(417i32);
    pub const WebViewBrush: XamlTypeIndex = XamlTypeIndex(423i32);
    pub const AppBarSeparator: XamlTypeIndex = XamlTypeIndex(427i32);
    pub const BitmapIcon: XamlTypeIndex = XamlTypeIndex(429i32);
    pub const Bold: XamlTypeIndex = XamlTypeIndex(430i32);
    pub const Canvas: XamlTypeIndex = XamlTypeIndex(432i32);
    pub const ContentControl: XamlTypeIndex = XamlTypeIndex(435i32);
    pub const DatePicker: XamlTypeIndex = XamlTypeIndex(436i32);
    pub const DependencyObjectCollection: XamlTypeIndex = XamlTypeIndex(437i32);
    pub const Ellipse: XamlTypeIndex = XamlTypeIndex(438i32);
    pub const FontIcon: XamlTypeIndex = XamlTypeIndex(440i32);
    pub const Grid: XamlTypeIndex = XamlTypeIndex(442i32);
    pub const Hub: XamlTypeIndex = XamlTypeIndex(445i32);
    pub const HubSection: XamlTypeIndex = XamlTypeIndex(446i32);
    pub const Hyperlink: XamlTypeIndex = XamlTypeIndex(447i32);
    pub const Italic: XamlTypeIndex = XamlTypeIndex(449i32);
    pub const ItemsControl: XamlTypeIndex = XamlTypeIndex(451i32);
    pub const Line: XamlTypeIndex = XamlTypeIndex(452i32);
    pub const MediaTransportControls: XamlTypeIndex = XamlTypeIndex(458i32);
    pub const PasswordBox: XamlTypeIndex = XamlTypeIndex(462i32);
    pub const Path: XamlTypeIndex = XamlTypeIndex(463i32);
    pub const PathIcon: XamlTypeIndex = XamlTypeIndex(464i32);
    pub const Polygon: XamlTypeIndex = XamlTypeIndex(465i32);
    pub const Polyline: XamlTypeIndex = XamlTypeIndex(466i32);
    pub const ProgressRing: XamlTypeIndex = XamlTypeIndex(468i32);
    pub const Rectangle: XamlTypeIndex = XamlTypeIndex(470i32);
    pub const RichEditBox: XamlTypeIndex = XamlTypeIndex(473i32);
    pub const ScrollContentPresenter: XamlTypeIndex = XamlTypeIndex(476i32);
    pub const SearchBox: XamlTypeIndex = XamlTypeIndex(477i32);
    pub const SemanticZoom: XamlTypeIndex = XamlTypeIndex(479i32);
    pub const StackPanel: XamlTypeIndex = XamlTypeIndex(481i32);
    pub const SymbolIcon: XamlTypeIndex = XamlTypeIndex(482i32);
    pub const TextBox: XamlTypeIndex = XamlTypeIndex(483i32);
    pub const Thumb: XamlTypeIndex = XamlTypeIndex(485i32);
    pub const TickBar: XamlTypeIndex = XamlTypeIndex(486i32);
    pub const TimePicker: XamlTypeIndex = XamlTypeIndex(487i32);
    pub const ToggleSwitch: XamlTypeIndex = XamlTypeIndex(489i32);
    pub const Underline: XamlTypeIndex = XamlTypeIndex(490i32);
    pub const UserControl: XamlTypeIndex = XamlTypeIndex(491i32);
    pub const VariableSizedWrapGrid: XamlTypeIndex = XamlTypeIndex(492i32);
    pub const WebView: XamlTypeIndex = XamlTypeIndex(494i32);
    pub const AppBar: XamlTypeIndex = XamlTypeIndex(495i32);
    pub const AutoSuggestBox: XamlTypeIndex = XamlTypeIndex(499i32);
    pub const CarouselPanel: XamlTypeIndex = XamlTypeIndex(502i32);
    pub const ContentDialog: XamlTypeIndex = XamlTypeIndex(506i32);
    pub const FlyoutPresenter: XamlTypeIndex = XamlTypeIndex(508i32);
    pub const Frame: XamlTypeIndex = XamlTypeIndex(509i32);
    pub const GridViewItemPresenter: XamlTypeIndex = XamlTypeIndex(511i32);
    pub const GroupItem: XamlTypeIndex = XamlTypeIndex(512i32);
    pub const ItemsStackPanel: XamlTypeIndex = XamlTypeIndex(514i32);
    pub const ItemsWrapGrid: XamlTypeIndex = XamlTypeIndex(515i32);
    pub const ListViewItemPresenter: XamlTypeIndex = XamlTypeIndex(520i32);
    pub const MenuFlyoutItem: XamlTypeIndex = XamlTypeIndex(521i32);
    pub const MenuFlyoutPresenter: XamlTypeIndex = XamlTypeIndex(522i32);
    pub const MenuFlyoutSeparator: XamlTypeIndex = XamlTypeIndex(523i32);
    pub const Page: XamlTypeIndex = XamlTypeIndex(525i32);
    pub const ProgressBar: XamlTypeIndex = XamlTypeIndex(528i32);
    pub const ScrollBar: XamlTypeIndex = XamlTypeIndex(530i32);
    pub const SettingsFlyout: XamlTypeIndex = XamlTypeIndex(533i32);
    pub const Slider: XamlTypeIndex = XamlTypeIndex(534i32);
    pub const SwapChainBackgroundPanel: XamlTypeIndex = XamlTypeIndex(535i32);
    pub const SwapChainPanel: XamlTypeIndex = XamlTypeIndex(536i32);
    pub const ToolTip: XamlTypeIndex = XamlTypeIndex(538i32);
    pub const Button: XamlTypeIndex = XamlTypeIndex(540i32);
    pub const ComboBoxItem: XamlTypeIndex = XamlTypeIndex(541i32);
    pub const CommandBar: XamlTypeIndex = XamlTypeIndex(542i32);
    pub const FlipViewItem: XamlTypeIndex = XamlTypeIndex(543i32);
    pub const GridViewHeaderItem: XamlTypeIndex = XamlTypeIndex(545i32);
    pub const HyperlinkButton: XamlTypeIndex = XamlTypeIndex(546i32);
    pub const ListBoxItem: XamlTypeIndex = XamlTypeIndex(547i32);
    pub const ListViewHeaderItem: XamlTypeIndex = XamlTypeIndex(550i32);
    pub const RepeatButton: XamlTypeIndex = XamlTypeIndex(551i32);
    pub const ScrollViewer: XamlTypeIndex = XamlTypeIndex(552i32);
    pub const ToggleButton: XamlTypeIndex = XamlTypeIndex(553i32);
    pub const ToggleMenuFlyoutItem: XamlTypeIndex = XamlTypeIndex(554i32);
    pub const VirtualizingStackPanel: XamlTypeIndex = XamlTypeIndex(555i32);
    pub const WrapGrid: XamlTypeIndex = XamlTypeIndex(556i32);
    pub const AppBarButton: XamlTypeIndex = XamlTypeIndex(557i32);
    pub const AppBarToggleButton: XamlTypeIndex = XamlTypeIndex(558i32);
    pub const CheckBox: XamlTypeIndex = XamlTypeIndex(559i32);
    pub const GridViewItem: XamlTypeIndex = XamlTypeIndex(560i32);
    pub const ListViewItem: XamlTypeIndex = XamlTypeIndex(561i32);
    pub const RadioButton: XamlTypeIndex = XamlTypeIndex(562i32);
    pub const Binding: XamlTypeIndex = XamlTypeIndex(564i32);
    pub const ComboBox: XamlTypeIndex = XamlTypeIndex(566i32);
    pub const FlipView: XamlTypeIndex = XamlTypeIndex(567i32);
    pub const ListBox: XamlTypeIndex = XamlTypeIndex(568i32);
    pub const GridView: XamlTypeIndex = XamlTypeIndex(570i32);
    pub const ListView: XamlTypeIndex = XamlTypeIndex(571i32);
    pub const CalendarView: XamlTypeIndex = XamlTypeIndex(707i32);
    pub const CalendarViewDayItem: XamlTypeIndex = XamlTypeIndex(709i32);
    pub const CalendarPanel: XamlTypeIndex = XamlTypeIndex(723i32);
    pub const SplitView: XamlTypeIndex = XamlTypeIndex(728i32);
    pub const CompositeTransform3D: XamlTypeIndex = XamlTypeIndex(732i32);
    pub const PerspectiveTransform3D: XamlTypeIndex = XamlTypeIndex(733i32);
    pub const RelativePanel: XamlTypeIndex = XamlTypeIndex(744i32);
    pub const InkCanvas: XamlTypeIndex = XamlTypeIndex(748i32);
    pub const MenuFlyoutSubItem: XamlTypeIndex = XamlTypeIndex(749i32);
    pub const AdaptiveTrigger: XamlTypeIndex = XamlTypeIndex(757i32);
    pub const SoftwareBitmapSource: XamlTypeIndex = XamlTypeIndex(761i32);
    pub const StateTrigger: XamlTypeIndex = XamlTypeIndex(767i32);
    pub const CalendarDatePicker: XamlTypeIndex = XamlTypeIndex(774i32);
    pub const AutoSuggestBoxQuerySubmittedEventArgs: XamlTypeIndex = XamlTypeIndex(778i32);
    pub const CommandBarOverflowPresenter: XamlTypeIndex = XamlTypeIndex(781i32);
    pub const DrillInThemeAnimation: XamlTypeIndex = XamlTypeIndex(782i32);
    pub const DrillOutThemeAnimation: XamlTypeIndex = XamlTypeIndex(783i32);
    pub const AutomationAnnotation: XamlTypeIndex = XamlTypeIndex(789i32);
    pub const AutomationPeerAnnotation: XamlTypeIndex = XamlTypeIndex(790i32);
    pub const MediaPlayerPresenter: XamlTypeIndex = XamlTypeIndex(828i32);
    pub const MediaPlayerElement: XamlTypeIndex = XamlTypeIndex(829i32);
    pub const XamlLight: XamlTypeIndex = XamlTypeIndex(855i32);
    pub const SvgImageSource: XamlTypeIndex = XamlTypeIndex(860i32);
    pub const KeyboardAccelerator: XamlTypeIndex = XamlTypeIndex(897i32);
    pub const HandwritingView: XamlTypeIndex = XamlTypeIndex(920i32);
    pub const ContentLink: XamlTypeIndex = XamlTypeIndex(925i32);
    pub const BitmapIconSource: XamlTypeIndex = XamlTypeIndex(929i32);
    pub const FontIconSource: XamlTypeIndex = XamlTypeIndex(930i32);
    pub const PathIconSource: XamlTypeIndex = XamlTypeIndex(931i32);
    pub const SymbolIconSource: XamlTypeIndex = XamlTypeIndex(933i32);
    pub const IconSourceElement: XamlTypeIndex = XamlTypeIndex(939i32);
    pub const AppBarElementContainer: XamlTypeIndex = XamlTypeIndex(945i32);
    pub const ColorPaletteResources: XamlTypeIndex = XamlTypeIndex(952i32);
    pub const StandardUICommand: XamlTypeIndex = XamlTypeIndex(961i32);
    pub const ThemeShadow: XamlTypeIndex = XamlTypeIndex(964i32);
    pub const XamlUICommand: XamlTypeIndex = XamlTypeIndex(969i32);
}
impl ::std::convert::From<i32> for XamlTypeIndex {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XamlTypeIndex {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for XamlTypeIndex {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Core.Direct.XamlTypeIndex;i4)");
}
