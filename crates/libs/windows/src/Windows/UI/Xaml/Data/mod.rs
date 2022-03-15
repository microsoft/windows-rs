#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct Binding(::windows::core::IUnknown);
impl Binding {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Path(&self) -> ::windows::core::Result<super::PropertyPath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PropertyPath>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetPath<'a, Param0: ::windows::core::IntoParam<'a, super::PropertyPath>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPath)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Mode(&self) -> ::windows::core::Result<BindingMode> {
        let this = self;
        unsafe {
            let mut result__: BindingMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<BindingMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetMode(&self, value: BindingMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn RelativeSource(&self) -> ::windows::core::Result<RelativeSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).RelativeSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RelativeSource>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetRelativeSource<'a, Param0: ::windows::core::IntoParam<'a, RelativeSource>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetRelativeSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ElementName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ElementName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetElementName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetElementName)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Converter(&self) -> ::windows::core::Result<IValueConverter> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Converter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<IValueConverter>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetConverter<'a, Param0: ::windows::core::IntoParam<'a, IValueConverter>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetConverter)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ConverterParameter(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConverterParameter)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetConverterParameter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetConverterParameter)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ConverterLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConverterLanguage)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetConverterLanguage<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetConverterLanguage)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn FallbackValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IBinding2>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FallbackValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetFallbackValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBinding2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetFallbackValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn TargetNullValue(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<IBinding2>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).TargetNullValue)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetTargetNullValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBinding2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetTargetNullValue)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn UpdateSourceTrigger(&self) -> ::windows::core::Result<UpdateSourceTrigger> {
        let this = &::windows::core::Interface::cast::<IBinding2>(self)?;
        unsafe {
            let mut result__: UpdateSourceTrigger = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).UpdateSourceTrigger)(::core::mem::transmute_copy(this), &mut result__).from_abi::<UpdateSourceTrigger>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetUpdateSourceTrigger(&self, value: UpdateSourceTrigger) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<IBinding2>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetUpdateSourceTrigger)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn new() -> ::windows::core::Result<Binding> {
        Self::IBindingFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<Binding>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<Binding> {
        Self::IBindingFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<Binding>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBindingFactory<R, F: FnOnce(&IBindingFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<Binding, IBindingFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for Binding {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for Binding {}
impl ::core::fmt::Debug for Binding {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Binding").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for Binding {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.Binding;{3f7a0c6b-d00f-4730-8c1d-48e16c46f9ca})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for Binding {
    type Vtable = IBinding_Vtbl;
    const IID: ::windows::core::GUID = <IBinding as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for Binding {
    const NAME: &'static str = "Windows.UI.Xaml.Data.Binding";
}
impl ::core::convert::From<Binding> for ::windows::core::IUnknown {
    fn from(value: Binding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Binding> for ::windows::core::IUnknown {
    fn from(value: &Binding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for Binding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a Binding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Binding> for ::windows::core::IInspectable {
    fn from(value: Binding) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&Binding> for ::windows::core::IInspectable {
    fn from(value: &Binding) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for Binding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a Binding {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<Binding> for BindingBase {
    fn from(value: Binding) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Binding> for BindingBase {
    fn from(value: &Binding) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, BindingBase> for Binding {
    fn into_param(self) -> ::windows::core::Param<'a, BindingBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, BindingBase> for &Binding {
    fn into_param(self) -> ::windows::core::Param<'a, BindingBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<BindingBase>::into(self))
    }
}
impl ::core::convert::From<Binding> for super::DependencyObject {
    fn from(value: Binding) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&Binding> for super::DependencyObject {
    fn from(value: &Binding) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for Binding {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &Binding {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for Binding {}
unsafe impl ::core::marker::Sync for Binding {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingBase(::windows::core::IUnknown);
impl BindingBase {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn new() -> ::windows::core::Result<BindingBase> {
        Self::IBindingBaseFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<BindingBase>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<BindingBase> {
        Self::IBindingBaseFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<BindingBase>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IBindingBaseFactory<R, F: FnOnce(&IBindingBaseFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BindingBase, IBindingBaseFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BindingBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindingBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindingBase {}
impl ::core::fmt::Debug for BindingBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.BindingBase;{1589a2ab-3d15-49bc-a447-8a5448e58870})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BindingBase {
    type Vtable = IBindingBase_Vtbl;
    const IID: ::windows::core::GUID = <IBindingBase as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BindingBase {
    const NAME: &'static str = "Windows.UI.Xaml.Data.BindingBase";
}
impl ::core::convert::From<BindingBase> for ::windows::core::IUnknown {
    fn from(value: BindingBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingBase> for ::windows::core::IUnknown {
    fn from(value: &BindingBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingBase> for ::windows::core::IInspectable {
    fn from(value: BindingBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingBase> for ::windows::core::IInspectable {
    fn from(value: &BindingBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingBase> for super::DependencyObject {
    fn from(value: BindingBase) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BindingBase> for super::DependencyObject {
    fn from(value: &BindingBase) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &BindingBase {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for BindingBase {}
unsafe impl ::core::marker::Sync for BindingBase {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingExpression(::windows::core::IUnknown);
impl BindingExpression {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn DataItem(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DataItem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ParentBinding(&self) -> ::windows::core::Result<Binding> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ParentBinding)(::core::mem::transmute_copy(this), &mut result__).from_abi::<Binding>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn UpdateSource(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).UpdateSource)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::clone::Clone for BindingExpression {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindingExpression {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindingExpression {}
impl ::core::fmt::Debug for BindingExpression {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingExpression").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingExpression {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.BindingExpression;{516a19a5-c2fd-4a9e-9fd3-9aa42f995a3c})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BindingExpression {
    type Vtable = IBindingExpression_Vtbl;
    const IID: ::windows::core::GUID = <IBindingExpression as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BindingExpression {
    const NAME: &'static str = "Windows.UI.Xaml.Data.BindingExpression";
}
impl ::core::convert::From<BindingExpression> for ::windows::core::IUnknown {
    fn from(value: BindingExpression) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingExpression> for ::windows::core::IUnknown {
    fn from(value: &BindingExpression) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingExpression> for ::windows::core::IInspectable {
    fn from(value: BindingExpression) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingExpression> for ::windows::core::IInspectable {
    fn from(value: &BindingExpression) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingExpression> for BindingExpressionBase {
    fn from(value: BindingExpression) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&BindingExpression> for BindingExpressionBase {
    fn from(value: &BindingExpression) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, BindingExpressionBase> for BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, BindingExpressionBase> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, BindingExpressionBase> for &BindingExpression {
    fn into_param(self) -> ::windows::core::Param<'a, BindingExpressionBase> {
        ::windows::core::Param::Owned(::core::convert::Into::<BindingExpressionBase>::into(self))
    }
}
unsafe impl ::core::marker::Send for BindingExpression {}
unsafe impl ::core::marker::Sync for BindingExpression {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingExpressionBase(::windows::core::IUnknown);
impl BindingExpressionBase {}
impl ::core::clone::Clone for BindingExpressionBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindingExpressionBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindingExpressionBase {}
impl ::core::fmt::Debug for BindingExpressionBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingExpressionBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingExpressionBase {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.BindingExpressionBase;{fded3154-e954-4f67-8fb6-6ed79b3a1cb3})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BindingExpressionBase {
    type Vtable = IBindingExpressionBase_Vtbl;
    const IID: ::windows::core::GUID = <IBindingExpressionBase as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BindingExpressionBase {
    const NAME: &'static str = "Windows.UI.Xaml.Data.BindingExpressionBase";
}
impl ::core::convert::From<BindingExpressionBase> for ::windows::core::IUnknown {
    fn from(value: BindingExpressionBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingExpressionBase> for ::windows::core::IUnknown {
    fn from(value: &BindingExpressionBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BindingExpressionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BindingExpressionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingExpressionBase> for ::windows::core::IInspectable {
    fn from(value: BindingExpressionBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingExpressionBase> for ::windows::core::IInspectable {
    fn from(value: &BindingExpressionBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BindingExpressionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BindingExpressionBase {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BindingExpressionBase {}
unsafe impl ::core::marker::Sync for BindingExpressionBase {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BindingMode(pub i32);
impl BindingMode {
    pub const OneWay: Self = Self(1i32);
    pub const OneTime: Self = Self(2i32);
    pub const TwoWay: Self = Self(3i32);
}
impl ::core::marker::Copy for BindingMode {}
impl ::core::clone::Clone for BindingMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for BindingMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for BindingMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for BindingMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Data.BindingMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct BindingOperations(::windows::core::IUnknown);
impl BindingOperations {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetBinding<'a, Param0: ::windows::core::IntoParam<'a, super::DependencyObject>, Param1: ::windows::core::IntoParam<'a, super::DependencyProperty>, Param2: ::windows::core::IntoParam<'a, BindingBase>>(target: Param0, dp: Param1, binding: Param2) -> ::windows::core::Result<()> {
        Self::IBindingOperationsStatics(|this| unsafe { (::windows::core::Interface::vtable(this).SetBinding)(::core::mem::transmute_copy(this), target.into_param().abi(), dp.into_param().abi(), binding.into_param().abi()).ok() })
    }
    #[doc(hidden)]
    pub fn IBindingOperationsStatics<R, F: FnOnce(&IBindingOperationsStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<BindingOperations, IBindingOperationsStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for BindingOperations {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for BindingOperations {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for BindingOperations {}
impl ::core::fmt::Debug for BindingOperations {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BindingOperations").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for BindingOperations {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.BindingOperations;{6fffd738-9839-419c-a17a-4b3604e1524e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for BindingOperations {
    type Vtable = IBindingOperations_Vtbl;
    const IID: ::windows::core::GUID = <IBindingOperations as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for BindingOperations {
    const NAME: &'static str = "Windows.UI.Xaml.Data.BindingOperations";
}
impl ::core::convert::From<BindingOperations> for ::windows::core::IUnknown {
    fn from(value: BindingOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingOperations> for ::windows::core::IUnknown {
    fn from(value: &BindingOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for BindingOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a BindingOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<BindingOperations> for ::windows::core::IInspectable {
    fn from(value: BindingOperations) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&BindingOperations> for ::windows::core::IInspectable {
    fn from(value: &BindingOperations) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for BindingOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a BindingOperations {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for BindingOperations {}
unsafe impl ::core::marker::Sync for BindingOperations {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct CollectionViewSource(::windows::core::IUnknown);
impl CollectionViewSource {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CollectionViewSource, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Source(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Source)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetSource<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetSource)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn View(&self) -> ::windows::core::Result<ICollectionView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).View)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ICollectionView>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsSourceGrouped(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSourceGrouped)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetIsSourceGrouped(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIsSourceGrouped)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ItemsPath(&self) -> ::windows::core::Result<super::PropertyPath> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemsPath)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::PropertyPath>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetItemsPath<'a, Param0: ::windows::core::IntoParam<'a, super::PropertyPath>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetItemsPath)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).SourceProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ViewProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ViewProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsSourceGroupedProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSourceGroupedProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn ItemsPathProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::ICollectionViewSourceStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ItemsPathProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICollectionViewSourceStatics<R, F: FnOnce(&ICollectionViewSourceStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CollectionViewSource, ICollectionViewSourceStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CollectionViewSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CollectionViewSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CollectionViewSource {}
impl ::core::fmt::Debug for CollectionViewSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CollectionViewSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CollectionViewSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.CollectionViewSource;{a66a1146-d2fb-4ead-be9f-3578a466dcfe})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CollectionViewSource {
    type Vtable = ICollectionViewSource_Vtbl;
    const IID: ::windows::core::GUID = <ICollectionViewSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CollectionViewSource {
    const NAME: &'static str = "Windows.UI.Xaml.Data.CollectionViewSource";
}
impl ::core::convert::From<CollectionViewSource> for ::windows::core::IUnknown {
    fn from(value: CollectionViewSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CollectionViewSource> for ::windows::core::IUnknown {
    fn from(value: &CollectionViewSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CollectionViewSource> for ::windows::core::IInspectable {
    fn from(value: CollectionViewSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CollectionViewSource> for ::windows::core::IInspectable {
    fn from(value: &CollectionViewSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CollectionViewSource> for super::DependencyObject {
    fn from(value: CollectionViewSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&CollectionViewSource> for super::DependencyObject {
    fn from(value: &CollectionViewSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &CollectionViewSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for CollectionViewSource {}
unsafe impl ::core::marker::Sync for CollectionViewSource {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct CurrentChangingEventArgs(::windows::core::IUnknown);
impl CurrentChangingEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Cancel)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetCancel(&self, value: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCancel)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsCancelable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCancelable)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn new() -> ::windows::core::Result<CurrentChangingEventArgs> {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<CurrentChangingEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<CurrentChangingEventArgs> {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<CurrentChangingEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateWithCancelableParameter(iscancelable: bool) -> ::windows::core::Result<CurrentChangingEventArgs> {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithCancelableParameter)(::core::mem::transmute_copy(this), iscancelable, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<CurrentChangingEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateWithCancelableParameter_compose<T: ::windows::core::Compose>(iscancelable: bool, compose: T) -> ::windows::core::Result<CurrentChangingEventArgs> {
        Self::ICurrentChangingEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithCancelableParameter)(::core::mem::transmute_copy(this), iscancelable, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<CurrentChangingEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn ICurrentChangingEventArgsFactory<R, F: FnOnce(&ICurrentChangingEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<CurrentChangingEventArgs, ICurrentChangingEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for CurrentChangingEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentChangingEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentChangingEventArgs {}
impl ::core::fmt::Debug for CurrentChangingEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentChangingEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for CurrentChangingEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.CurrentChangingEventArgs;{f9891e29-51cc-47dd-a5b9-35dc4914af69})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for CurrentChangingEventArgs {
    type Vtable = ICurrentChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <ICurrentChangingEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for CurrentChangingEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Data.CurrentChangingEventArgs";
}
impl ::core::convert::From<CurrentChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: CurrentChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentChangingEventArgs> for ::windows::core::IUnknown {
    fn from(value: &CurrentChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for CurrentChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a CurrentChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<CurrentChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: CurrentChangingEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&CurrentChangingEventArgs> for ::windows::core::IInspectable {
    fn from(value: &CurrentChangingEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for CurrentChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a CurrentChangingEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for CurrentChangingEventArgs {}
unsafe impl ::core::marker::Sync for CurrentChangingEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct CurrentChangingEventHandler(pub ::windows::core::IUnknown);
impl CurrentChangingEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<CurrentChangingEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = CurrentChangingEventHandlerBox::<F> { vtable: &CurrentChangingEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, CurrentChangingEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct CurrentChangingEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<CurrentChangingEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const CurrentChangingEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<CurrentChangingEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> CurrentChangingEventHandlerBox<F> {
    const VTABLE: CurrentChangingEventHandler_Vtbl = CurrentChangingEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<CurrentChangingEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for CurrentChangingEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for CurrentChangingEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for CurrentChangingEventHandler {}
impl ::core::fmt::Debug for CurrentChangingEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CurrentChangingEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for CurrentChangingEventHandler {
    type Vtable = CurrentChangingEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf3888db8_139f_4dce_8dc9_f7f1444d1185);
}
unsafe impl ::windows::core::RuntimeType for CurrentChangingEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f3888db8-139f-4dce-8dc9-f7f1444d1185}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct CurrentChangingEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBinding(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBinding {
    type Vtable = IBinding_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3f7a0c6b_d00f_4730_8c1d_48e16c46f9ca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut BindingMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: BindingMode) -> ::windows::core::HRESULT,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RelativeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetRelativeSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ElementName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetElementName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Converter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetConverter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ConverterParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetConverterParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConverterLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetConverterLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBinding2(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBinding2 {
    type Vtable = IBinding2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34f96fcb_0406_48b3_9e82_f333ec4c6910);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBinding2_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FallbackValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFallbackValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TargetNullValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetTargetNullValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub UpdateSourceTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut UpdateSourceTrigger) -> ::windows::core::HRESULT,
    pub SetUpdateSourceTrigger: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: UpdateSourceTrigger) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingBase {
    type Vtable = IBindingBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1589a2ab_3d15_49bc_a447_8a5448e58870);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingBase_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingBaseFactory {
    type Vtable = IBindingBaseFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x22dafc3a_7701_4666_a1ba_9859bdcfec34);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingBaseFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingExpression(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingExpression {
    type Vtable = IBindingExpression_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x516a19a5_c2fd_4a9e_9fd3_9aa42f995a3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpression_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DataItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ParentBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub UpdateSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingExpressionBase(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingExpressionBase {
    type Vtable = IBindingExpressionBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfded3154_e954_4f67_8fb6_6ed79b3a1cb3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionBase_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingExpressionBaseFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingExpressionBaseFactory {
    type Vtable = IBindingExpressionBaseFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea7116a7_c2d9_4375_b471_66b9c48c7930);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionBaseFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingExpressionFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingExpressionFactory {
    type Vtable = IBindingExpressionFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cb55cd9_db72_40b3_a2b5_24ee6ea5c328);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingExpressionFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingFactory {
    type Vtable = IBindingFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff42bb08_c39e_4f7e_8434_a1569083883c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingOperations(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingOperations {
    type Vtable = IBindingOperations_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fffd738_9839_419c_a17a_4b3604e1524e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingOperations_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IBindingOperationsStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IBindingOperationsStatics {
    type Vtable = IBindingOperationsStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe155ef73_95a0_4aab_8c7d_2a47da073c79);
}
#[repr(C)]
#[doc(hidden)]
pub struct IBindingOperationsStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SetBinding: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr, dp: ::windows::core::RawPtr, binding: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICollectionView(::windows::core::IUnknown);
impl ICollectionView {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CurrentItem(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentItem)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CurrentPosition(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentPosition)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsCurrentAfterLast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentAfterLast)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsCurrentBeforeFirst(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsCurrentBeforeFirst)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn CollectionGroups(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CollectionGroups)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn HasMoreItems(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasMoreItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventHandler<::windows::core::IInspectable>>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCurrentChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CurrentChanging<'a, Param0: ::windows::core::IntoParam<'a, CurrentChangingEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentChanging)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveCurrentChanging<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveCurrentChanging)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentTo<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, item: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentTo)(::core::mem::transmute_copy(this), item.into_param().abi(), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToPosition(&self, index: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToPosition)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToFirst(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToFirst)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToLast(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToLast)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToNext(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToNext)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn MoveCurrentToPrevious(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).MoveCurrentToPrevious)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadMoreItemsAsync(&self, count: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadMoreItemsAsync)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn First(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IIterator<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).First)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IIterator<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn VectorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::VectorChangedEventHandler<::windows::core::IInspectable>>>(&self, vhnd: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).VectorChanged)(::core::mem::transmute_copy(this), vhnd.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveVectorChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveVectorChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetAt(&self, index: u32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetAt)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Size(&self) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Size)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetView(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<::windows::core::IInspectable>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn IndexOf<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0, index: &mut u32) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IndexOf)(::core::mem::transmute_copy(this), value.into_param().abi(), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn SetAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).SetAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn InsertAt<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, index: u32, value: Param1) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).InsertAt)(::core::mem::transmute_copy(this), index, value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAt(&self, index: u32) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAt)(::core::mem::transmute_copy(this), index).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Append<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Append)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoveAtEnd(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAtEnd)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn Clear(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Clear)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetMany(&self, startindex: u32, items: &mut [::core::option::Option<::windows::core::IInspectable>]) -> ::windows::core::Result<u32> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetMany)(::core::mem::transmute_copy(this), startindex, items.len() as u32, ::core::mem::transmute_copy(&items), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn ReplaceAll(&self, items: &[::core::option::Option<::windows::core::IInspectable>]) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ReplaceAll)(::core::mem::transmute_copy(this), items.len() as u32, ::core::mem::transmute(items.as_ptr())).ok() }
    }
}
impl ::core::convert::From<ICollectionView> for ::windows::core::IUnknown {
    fn from(value: ICollectionView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionView> for ::windows::core::IUnknown {
    fn from(value: &ICollectionView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICollectionView> for ::windows::core::IInspectable {
    fn from(value: ICollectionView) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionView> for ::windows::core::IInspectable {
    fn from(value: &ICollectionView) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ICollectionView> for super::super::super::Foundation::Collections::IIterable<::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: ICollectionView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ICollectionView> for super::super::super::Foundation::Collections::IIterable<::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICollectionView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>> for ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>> for &ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IIterable<::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ICollectionView> for super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: ICollectionView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ICollectionView> for super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICollectionView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>> for ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>> for &ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<ICollectionView> for super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: ICollectionView) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::convert::TryFrom<&ICollectionView> for super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable> {
    type Error = ::windows::core::Error;
    fn try_from(value: &ICollectionView) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>> for ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>> for &ICollectionView {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>> {
        ::core::convert::TryInto::<super::super::super::Foundation::Collections::IVector<::windows::core::IInspectable>>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ICollectionView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICollectionView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICollectionView {}
impl ::core::fmt::Debug for ICollectionView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICollectionView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICollectionView {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8be8bfe4-dbef-44df-8126-a31a89121ddc}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for ICollectionView {
    type Item = ::windows::core::IInspectable;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        ::core::iter::IntoIterator::into_iter(&self)
    }
}
#[cfg(feature = "Foundation_Collections")]
impl ::core::iter::IntoIterator for &ICollectionView {
    type Item = ::windows::core::IInspectable;
    type IntoIter = super::super::super::Foundation::Collections::VectorIterator<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        super::super::super::Foundation::Collections::VectorIterator::new(::core::convert::TryInto::try_into(self).ok())
    }
}
unsafe impl ::windows::core::Interface for ICollectionView {
    type Vtable = ICollectionView_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8be8bfe4_dbef_44df_8126_a31a89121ddc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionView_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CurrentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CurrentPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub IsCurrentAfterLast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsCurrentBeforeFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub CollectionGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    CollectionGroups: usize,
    pub HasMoreItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CurrentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentChanged: usize,
    #[cfg(feature = "Foundation")]
    pub CurrentChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CurrentChanging: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveCurrentChanging: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveCurrentChanging: usize,
    pub MoveCurrentTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MoveCurrentToPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MoveCurrentToFirst: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MoveCurrentToLast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MoveCurrentToNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub MoveCurrentToPrevious: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub LoadMoreItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadMoreItemsAsync: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICollectionViewFactory(::windows::core::IUnknown);
impl ICollectionViewFactory {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateView(&self) -> ::windows::core::Result<ICollectionView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateView)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ICollectionView>(result__)
        }
    }
}
impl ::core::convert::From<ICollectionViewFactory> for ::windows::core::IUnknown {
    fn from(value: ICollectionViewFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionViewFactory> for ::windows::core::IUnknown {
    fn from(value: &ICollectionViewFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICollectionViewFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICollectionViewFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICollectionViewFactory> for ::windows::core::IInspectable {
    fn from(value: ICollectionViewFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionViewFactory> for ::windows::core::IInspectable {
    fn from(value: &ICollectionViewFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICollectionViewFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICollectionViewFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICollectionViewFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICollectionViewFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICollectionViewFactory {}
impl ::core::fmt::Debug for ICollectionViewFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICollectionViewFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICollectionViewFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{34d4aaf4-8e72-4950-9192-ecd07d399d0a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICollectionViewFactory {
    type Vtable = ICollectionViewFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34d4aaf4_8e72_4950_9192_ecd07d399d0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICollectionViewGroup(::windows::core::IUnknown);
impl ICollectionViewGroup {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Group(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Group)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GroupItems(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GroupItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IObservableVector<::windows::core::IInspectable>>(result__)
        }
    }
}
impl ::core::convert::From<ICollectionViewGroup> for ::windows::core::IUnknown {
    fn from(value: ICollectionViewGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionViewGroup> for ::windows::core::IUnknown {
    fn from(value: &ICollectionViewGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICollectionViewGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICollectionViewGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICollectionViewGroup> for ::windows::core::IInspectable {
    fn from(value: ICollectionViewGroup) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICollectionViewGroup> for ::windows::core::IInspectable {
    fn from(value: &ICollectionViewGroup) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICollectionViewGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICollectionViewGroup {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICollectionViewGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICollectionViewGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICollectionViewGroup {}
impl ::core::fmt::Debug for ICollectionViewGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICollectionViewGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICollectionViewGroup {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7e01b9d8-d7b5-48b6-b31c-5bb5bdf5f09b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICollectionViewGroup {
    type Vtable = ICollectionViewGroup_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7e01b9d8_d7b5_48b6_b31c_5bb5bdf5f09b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewGroup_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Group: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GroupItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GroupItems: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICollectionViewSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICollectionViewSource {
    type Vtable = ICollectionViewSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa66a1146_d2fb_4ead_be9f_3578a466dcfe);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Source: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub View: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsSourceGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetIsSourceGrouped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub ItemsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub SetItemsPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICollectionViewSourceStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICollectionViewSourceStatics {
    type Vtable = ICollectionViewSourceStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x173a0710_46af_4c0c_818b_21b6ef81bf65);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICollectionViewSourceStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ViewProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsSourceGroupedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub ItemsPathProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentChangingEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrentChangingEventArgs {
    type Vtable = ICurrentChangingEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9891e29_51cc_47dd_a5b9_35dc4914af69);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentChangingEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SetCancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: bool) -> ::windows::core::HRESULT,
    pub IsCancelable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct ICurrentChangingEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for ICurrentChangingEventArgsFactory {
    type Vtable = ICurrentChangingEventArgsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x153bbeee_62f3_48cf_8183_8be26de3a66e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICurrentChangingEventArgsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub CreateWithCancelableParameter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iscancelable: bool, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICustomProperty(::windows::core::IUnknown);
impl ICustomProperty {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn Type(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, target: Param0) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetValue)(::core::mem::transmute_copy(this), target.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, target: Param0, value: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::core::mem::transmute_copy(this), target.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetIndexedValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, target: Param0, index: Param1) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexedValue)(::core::mem::transmute_copy(this), target.into_param().abi(), index.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetIndexedValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, target: Param0, value: Param1, index: Param2) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetIndexedValue)(::core::mem::transmute_copy(this), target.into_param().abi(), value.into_param().abi(), index.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CanWrite(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanWrite)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CanRead(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CanRead)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ICustomProperty> for ::windows::core::IUnknown {
    fn from(value: ICustomProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomProperty> for ::windows::core::IUnknown {
    fn from(value: &ICustomProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICustomProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICustomProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICustomProperty> for ::windows::core::IInspectable {
    fn from(value: ICustomProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomProperty> for ::windows::core::IInspectable {
    fn from(value: &ICustomProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICustomProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICustomProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICustomProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomProperty {}
impl ::core::fmt::Debug for ICustomProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICustomProperty {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{30da92c0-23e8-42a0-ae7c-734a0e5d2782}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICustomProperty {
    type Vtable = ICustomProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30da92c0_23e8_42a0_ae7c_734a0e5d2782);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomProperty_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Type: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIndexedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetIndexedValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, target: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, index: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CanWrite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanRead: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ICustomPropertyProvider(::windows::core::IUnknown);
impl ICustomPropertyProvider {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetCustomProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<ICustomProperty> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetCustomProperty)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<ICustomProperty>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn GetIndexedProperty<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, Param1: ::windows::core::IntoParam<'a, super::Interop::TypeName>>(&self, name: Param0, r#type: Param1) -> ::windows::core::Result<ICustomProperty> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexedProperty)(::core::mem::transmute_copy(this), name.into_param().abi(), r#type.into_param().abi(), &mut result__).from_abi::<ICustomProperty>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn GetStringRepresentation(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetStringRepresentation)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn Type(&self) -> ::windows::core::Result<super::Interop::TypeName> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<super::Interop::TypeName> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Type)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Interop::TypeName>(result__)
        }
    }
}
impl ::core::convert::From<ICustomPropertyProvider> for ::windows::core::IUnknown {
    fn from(value: ICustomPropertyProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomPropertyProvider> for ::windows::core::IUnknown {
    fn from(value: &ICustomPropertyProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICustomPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICustomPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICustomPropertyProvider> for ::windows::core::IInspectable {
    fn from(value: ICustomPropertyProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomPropertyProvider> for ::windows::core::IInspectable {
    fn from(value: &ICustomPropertyProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICustomPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICustomPropertyProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICustomPropertyProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomPropertyProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomPropertyProvider {}
impl ::core::fmt::Debug for ICustomPropertyProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomPropertyProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICustomPropertyProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7c925755-3e48-42b4-8677-76372267033f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICustomPropertyProvider {
    type Vtable = ICustomPropertyProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7c925755_3e48_42b4_8677_76372267033f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomPropertyProvider_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub GetCustomProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub GetIndexedProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    GetIndexedProperty: usize,
    pub GetStringRepresentation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Type: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Type: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItemIndexRange(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IItemIndexRange {
    type Vtable = IItemIndexRange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x83b834be_0583_4a26_9b64_8bf4a2f65704);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemIndexRange_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub FirstIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub Length: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut u32) -> ::windows::core::HRESULT,
    pub LastIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IItemIndexRangeFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IItemIndexRangeFactory {
    type Vtable = IItemIndexRangeFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x86e2c440_2e7a_4c7d_a664_e8abf07bfc7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemIndexRangeFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, firstindex: i32, length: u32, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct IItemsRangeInfo(::windows::core::IUnknown);
impl IItemsRangeInfo {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn RangesChanged<'a, Param0: ::windows::core::IntoParam<'a, ItemIndexRange>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>>(&self, visiblerange: Param0, trackeditems: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RangesChanged)(::core::mem::transmute_copy(this), visiblerange.into_param().abi(), trackeditems.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<super::super::super::Foundation::IClosable>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::core::mem::transmute_copy(this)).ok() }
    }
}
impl ::core::convert::From<IItemsRangeInfo> for ::windows::core::IUnknown {
    fn from(value: IItemsRangeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IItemsRangeInfo> for ::windows::core::IUnknown {
    fn from(value: &IItemsRangeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IItemsRangeInfo> for ::windows::core::IInspectable {
    fn from(value: IItemsRangeInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IItemsRangeInfo> for ::windows::core::IInspectable {
    fn from(value: &IItemsRangeInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<IItemsRangeInfo> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: IItemsRangeInfo) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::core::convert::TryFrom<&IItemsRangeInfo> for super::super::super::Foundation::IClosable {
    type Error = ::windows::core::Error;
    fn try_from(value: &IItemsRangeInfo) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::core::IntoParam<'a, super::super::super::Foundation::IClosable> for &IItemsRangeInfo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::super::Foundation::IClosable> {
        ::core::convert::TryInto::<super::super::super::Foundation::IClosable>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IItemsRangeInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IItemsRangeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IItemsRangeInfo {}
impl ::core::fmt::Debug for IItemsRangeInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IItemsRangeInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IItemsRangeInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f05f5665-71fd-45a2-be13-a081d294a68d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IItemsRangeInfo {
    type Vtable = IItemsRangeInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf05f5665_71fd_45a2_be13_a081d294a68d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemsRangeInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub RangesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visiblerange: ::windows::core::RawPtr, trackeditems: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    RangesChanged: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct INotifyPropertyChanged(::windows::core::IUnknown);
impl INotifyPropertyChanged {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertyChanged<'a, Param0: ::windows::core::IntoParam<'a, PropertyChangedEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PropertyChanged)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertyChanged<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertyChanged)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<INotifyPropertyChanged> for ::windows::core::IUnknown {
    fn from(value: INotifyPropertyChanged) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotifyPropertyChanged> for ::windows::core::IUnknown {
    fn from(value: &INotifyPropertyChanged) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INotifyPropertyChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INotifyPropertyChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<INotifyPropertyChanged> for ::windows::core::IInspectable {
    fn from(value: INotifyPropertyChanged) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&INotifyPropertyChanged> for ::windows::core::IInspectable {
    fn from(value: &INotifyPropertyChanged) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for INotifyPropertyChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a INotifyPropertyChanged {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for INotifyPropertyChanged {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for INotifyPropertyChanged {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INotifyPropertyChanged {}
impl ::core::fmt::Debug for INotifyPropertyChanged {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INotifyPropertyChanged").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for INotifyPropertyChanged {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cf75d69c-f2f4-486b-b302-bb4c09baebfa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for INotifyPropertyChanged {
    type Vtable = INotifyPropertyChanged_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf75d69c_f2f4_486b_b302_bb4c09baebfa);
}
#[repr(C)]
#[doc(hidden)]
pub struct INotifyPropertyChanged_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub PropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertyChanged: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertyChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertyChanged: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPropertyChangedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPropertyChangedEventArgs {
    type Vtable = IPropertyChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f33a9a0_5cf4_47a4_b16f_d7faaf17457e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangedEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPropertyChangedEventArgsFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPropertyChangedEventArgsFactory {
    type Vtable = IPropertyChangedEventArgsFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6dcc9c03_e0c7_4eee_8ea9_37e3406eeb1c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertyChangedEventArgsFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRelativeSource(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRelativeSource {
    type Vtable = IRelativeSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2397ce84_2822_483a_b499_d0f031e06c6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativeSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut RelativeSourceMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: RelativeSourceMode) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IRelativeSourceFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IRelativeSourceFactory {
    type Vtable = IRelativeSourceFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef8392cd_446e_4f93_aacb_9b1255577460);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRelativeSourceFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ISelectionInfo(::windows::core::IUnknown);
impl ISelectionInfo {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SelectRange<'a, Param0: ::windows::core::IntoParam<'a, ItemIndexRange>>(&self, itemindexrange: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SelectRange)(::core::mem::transmute_copy(this), itemindexrange.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn DeselectRange<'a, Param0: ::windows::core::IntoParam<'a, ItemIndexRange>>(&self, itemindexrange: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).DeselectRange)(::core::mem::transmute_copy(this), itemindexrange.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn IsSelected(&self, index: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).IsSelected)(::core::mem::transmute_copy(this), index, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetSelectedRanges(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetSelectedRanges)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Foundation::Collections::IVectorView<ItemIndexRange>>(result__)
        }
    }
}
impl ::core::convert::From<ISelectionInfo> for ::windows::core::IUnknown {
    fn from(value: ISelectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectionInfo> for ::windows::core::IUnknown {
    fn from(value: &ISelectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISelectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISelectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISelectionInfo> for ::windows::core::IInspectable {
    fn from(value: ISelectionInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectionInfo> for ::windows::core::IInspectable {
    fn from(value: &ISelectionInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISelectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISelectionInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISelectionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISelectionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectionInfo {}
impl ::core::fmt::Debug for ISelectionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISelectionInfo {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2e12ca86-e1ed-4245-be49-207e42aec524}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISelectionInfo {
    type Vtable = ISelectionInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e12ca86_e1ed_4245_be49_207e42aec524);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionInfo_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub SelectRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemindexrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub DeselectRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemindexrange: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub GetSelectedRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetSelectedRanges: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ISupportIncrementalLoading(::windows::core::IUnknown);
impl ISupportIncrementalLoading {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn LoadMoreItemsAsync(&self, count: u32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LoadMoreItemsAsync)(::core::mem::transmute_copy(this), count, &mut result__).from_abi::<super::super::super::Foundation::IAsyncOperation<LoadMoreItemsResult>>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn HasMoreItems(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).HasMoreItems)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<ISupportIncrementalLoading> for ::windows::core::IUnknown {
    fn from(value: ISupportIncrementalLoading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportIncrementalLoading> for ::windows::core::IUnknown {
    fn from(value: &ISupportIncrementalLoading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISupportIncrementalLoading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISupportIncrementalLoading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISupportIncrementalLoading> for ::windows::core::IInspectable {
    fn from(value: ISupportIncrementalLoading) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportIncrementalLoading> for ::windows::core::IInspectable {
    fn from(value: &ISupportIncrementalLoading) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISupportIncrementalLoading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISupportIncrementalLoading {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISupportIncrementalLoading {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportIncrementalLoading {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportIncrementalLoading {}
impl ::core::fmt::Debug for ISupportIncrementalLoading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportIncrementalLoading").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISupportIncrementalLoading {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7f5ee992-7694-4e6c-a51b-e34bf43de743}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISupportIncrementalLoading {
    type Vtable = ISupportIncrementalLoading_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7f5ee992_7694_4e6c_a51b_e34bf43de743);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportIncrementalLoading_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation")]
    pub LoadMoreItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: u32, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    LoadMoreItemsAsync: usize,
    pub HasMoreItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct IValueConverter(::windows::core::IUnknown);
impl IValueConverter {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn Convert<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::Interop::TypeName>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Convert)(::core::mem::transmute_copy(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`, `\"UI_Xaml_Interop\"`*"]
    #[cfg(feature = "UI_Xaml_Interop")]
    pub fn ConvertBack<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, super::Interop::TypeName>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param3: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0, targettype: Param1, parameter: Param2, language: Param3) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).ConvertBack)(::core::mem::transmute_copy(this), value.into_param().abi(), targettype.into_param().abi(), parameter.into_param().abi(), language.into_param().abi(), &mut result__).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<IValueConverter> for ::windows::core::IUnknown {
    fn from(value: IValueConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValueConverter> for ::windows::core::IUnknown {
    fn from(value: &IValueConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IValueConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IValueConverter> for ::windows::core::IInspectable {
    fn from(value: IValueConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValueConverter> for ::windows::core::IInspectable {
    fn from(value: &IValueConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IValueConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IValueConverter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IValueConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IValueConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValueConverter {}
impl ::core::fmt::Debug for IValueConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IValueConverter {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{e6f2fef0-0712-487f-b313-f300b8d79aa1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IValueConverter {
    type Vtable = IValueConverter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe6f2fef0_0712_487f_b313_f300b8d79aa1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueConverter_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub Convert: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    Convert: usize,
    #[cfg(feature = "UI_Xaml_Interop")]
    pub ConvertBack: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, targettype: ::core::mem::ManuallyDrop<super::Interop::TypeName>, parameter: *mut ::core::ffi::c_void, language: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Interop"))]
    ConvertBack: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct ItemIndexRange(::windows::core::IUnknown);
impl ItemIndexRange {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn FirstIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).FirstIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Length(&self) -> ::windows::core::Result<u32> {
        let this = self;
        unsafe {
            let mut result__: u32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Length)(::core::mem::transmute_copy(this), &mut result__).from_abi::<u32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn LastIndex(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).LastIndex)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance(firstindex: i32, length: u32) -> ::windows::core::Result<ItemIndexRange> {
        Self::IItemIndexRangeFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), firstindex, length, ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<ItemIndexRange>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance_compose<T: ::windows::core::Compose>(firstindex: i32, length: u32, compose: T) -> ::windows::core::Result<ItemIndexRange> {
        Self::IItemIndexRangeFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), firstindex, length, ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<ItemIndexRange>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IItemIndexRangeFactory<R, F: FnOnce(&IItemIndexRangeFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<ItemIndexRange, IItemIndexRangeFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for ItemIndexRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ItemIndexRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ItemIndexRange {}
impl ::core::fmt::Debug for ItemIndexRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ItemIndexRange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ItemIndexRange {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.ItemIndexRange;{83b834be-0583-4a26-9b64-8bf4a2f65704})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ItemIndexRange {
    type Vtable = IItemIndexRange_Vtbl;
    const IID: ::windows::core::GUID = <IItemIndexRange as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for ItemIndexRange {
    const NAME: &'static str = "Windows.UI.Xaml.Data.ItemIndexRange";
}
impl ::core::convert::From<ItemIndexRange> for ::windows::core::IUnknown {
    fn from(value: ItemIndexRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItemIndexRange> for ::windows::core::IUnknown {
    fn from(value: &ItemIndexRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ItemIndexRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ItemIndexRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ItemIndexRange> for ::windows::core::IInspectable {
    fn from(value: ItemIndexRange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ItemIndexRange> for ::windows::core::IInspectable {
    fn from(value: &ItemIndexRange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ItemIndexRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ItemIndexRange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for ItemIndexRange {}
unsafe impl ::core::marker::Sync for ItemIndexRange {}
#[repr(C)]
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
pub struct LoadMoreItemsResult {
    pub Count: u32,
}
impl ::core::marker::Copy for LoadMoreItemsResult {}
impl ::core::clone::Clone for LoadMoreItemsResult {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LoadMoreItemsResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LoadMoreItemsResult").field("Count", &self.Count).finish()
    }
}
unsafe impl ::windows::core::Abi for LoadMoreItemsResult {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for LoadMoreItemsResult {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"struct(Windows.UI.Xaml.Data.LoadMoreItemsResult;u4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
impl ::core::cmp::PartialEq for LoadMoreItemsResult {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LoadMoreItemsResult>()) == 0 }
    }
}
impl ::core::cmp::Eq for LoadMoreItemsResult {}
impl ::core::default::Default for LoadMoreItemsResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct PropertyChangedEventArgs(::windows::core::IUnknown);
impl PropertyChangedEventArgs {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn PropertyName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PropertyName)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(name: Param0) -> ::windows::core::Result<PropertyChangedEventArgs> {
        Self::IPropertyChangedEventArgsFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), name.into_param().abi(), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<PropertyChangedEventArgs>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn CreateInstance_compose<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>, T: ::windows::core::Compose>(name: Param0, compose: T) -> ::windows::core::Result<PropertyChangedEventArgs> {
        Self::IPropertyChangedEventArgsFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), name.into_param().abi(), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<PropertyChangedEventArgs>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPropertyChangedEventArgsFactory<R, F: FnOnce(&IPropertyChangedEventArgsFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PropertyChangedEventArgs, IPropertyChangedEventArgsFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PropertyChangedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PropertyChangedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PropertyChangedEventArgs {}
impl ::core::fmt::Debug for PropertyChangedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyChangedEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PropertyChangedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.PropertyChangedEventArgs;{4f33a9a0-5cf4-47a4-b16f-d7faaf17457e})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PropertyChangedEventArgs {
    type Vtable = IPropertyChangedEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPropertyChangedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PropertyChangedEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Data.PropertyChangedEventArgs";
}
impl ::core::convert::From<PropertyChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: PropertyChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PropertyChangedEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PropertyChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PropertyChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: PropertyChangedEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PropertyChangedEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PropertyChangedEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PropertyChangedEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PropertyChangedEventArgs {}
unsafe impl ::core::marker::Sync for PropertyChangedEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct PropertyChangedEventHandler(pub ::windows::core::IUnknown);
impl PropertyChangedEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PropertyChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PropertyChangedEventHandlerBox::<F> { vtable: &PropertyChangedEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, PropertyChangedEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct PropertyChangedEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PropertyChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PropertyChangedEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PropertyChangedEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> PropertyChangedEventHandlerBox<F> {
    const VTABLE: PropertyChangedEventHandler_Vtbl = PropertyChangedEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PropertyChangedEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
        if (*interface).is_null() {
            ::windows::core::HRESULT(-2147467262)
        } else {
            (*this).count.add_ref();
            ::windows::core::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::core::RawPtr) -> u32 {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            ::windows::core::alloc::boxed::Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        ((*this).invoke)(::core::mem::transmute(&sender), ::core::mem::transmute(&e)).into()
    }
}
impl ::core::clone::Clone for PropertyChangedEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PropertyChangedEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PropertyChangedEventHandler {}
impl ::core::fmt::Debug for PropertyChangedEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PropertyChangedEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for PropertyChangedEventHandler {
    type Vtable = PropertyChangedEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50f19c16_0a22_4d8e_a089_1ea9951657d2);
}
unsafe impl ::windows::core::RuntimeType for PropertyChangedEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{50f19c16-0a22-4d8e-a089-1ea9951657d2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PropertyChangedEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
pub struct RelativeSource(::windows::core::IUnknown);
impl RelativeSource {
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn Mode(&self) -> ::windows::core::Result<RelativeSourceMode> {
        let this = self;
        unsafe {
            let mut result__: RelativeSourceMode = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Mode)(::core::mem::transmute_copy(this), &mut result__).from_abi::<RelativeSourceMode>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn SetMode(&self, value: RelativeSourceMode) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetMode)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn new() -> ::windows::core::Result<RelativeSource> {
        Self::IRelativeSourceFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<RelativeSource>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<RelativeSource> {
        Self::IRelativeSourceFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<RelativeSource>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IRelativeSourceFactory<R, F: FnOnce(&IRelativeSourceFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<RelativeSource, IRelativeSourceFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for RelativeSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for RelativeSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for RelativeSource {}
impl ::core::fmt::Debug for RelativeSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RelativeSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RelativeSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Data.RelativeSource;{2397ce84-2822-483a-b499-d0f031e06c6b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for RelativeSource {
    type Vtable = IRelativeSource_Vtbl;
    const IID: ::windows::core::GUID = <IRelativeSource as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for RelativeSource {
    const NAME: &'static str = "Windows.UI.Xaml.Data.RelativeSource";
}
impl ::core::convert::From<RelativeSource> for ::windows::core::IUnknown {
    fn from(value: RelativeSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RelativeSource> for ::windows::core::IUnknown {
    fn from(value: &RelativeSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RelativeSource> for ::windows::core::IInspectable {
    fn from(value: RelativeSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&RelativeSource> for ::windows::core::IInspectable {
    fn from(value: &RelativeSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<RelativeSource> for super::DependencyObject {
    fn from(value: RelativeSource) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&RelativeSource> for super::DependencyObject {
    fn from(value: &RelativeSource) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &RelativeSource {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for RelativeSource {}
unsafe impl ::core::marker::Sync for RelativeSource {}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RelativeSourceMode(pub i32);
impl RelativeSourceMode {
    pub const None: Self = Self(0i32);
    pub const TemplatedParent: Self = Self(1i32);
    pub const Self_: Self = Self(2i32);
}
impl ::core::marker::Copy for RelativeSourceMode {}
impl ::core::clone::Clone for RelativeSourceMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RelativeSourceMode {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RelativeSourceMode {
    type Abi = Self;
}
impl ::core::fmt::Debug for RelativeSourceMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RelativeSourceMode").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for RelativeSourceMode {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Data.RelativeSourceMode;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Data\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct UpdateSourceTrigger(pub i32);
impl UpdateSourceTrigger {
    pub const Default: Self = Self(0i32);
    pub const PropertyChanged: Self = Self(1i32);
    pub const Explicit: Self = Self(2i32);
    pub const LostFocus: Self = Self(3i32);
}
impl ::core::marker::Copy for UpdateSourceTrigger {}
impl ::core::clone::Clone for UpdateSourceTrigger {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UpdateSourceTrigger {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UpdateSourceTrigger {
    type Abi = Self;
}
impl ::core::fmt::Debug for UpdateSourceTrigger {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UpdateSourceTrigger").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for UpdateSourceTrigger {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Data.UpdateSourceTrigger;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
