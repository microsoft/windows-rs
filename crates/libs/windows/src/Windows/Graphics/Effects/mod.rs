#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Graphics_Effects\"`*"]
#[repr(transparent)]
pub struct IGraphicsEffect(::windows::core::IUnknown);
impl IGraphicsEffect {
    #[doc = "*Required features: `\"Graphics_Effects\"`*"]
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::core::HSTRING> = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Graphics_Effects\"`*"]
    pub fn SetName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetName)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IGraphicsEffect> for ::windows::core::IUnknown {
    fn from(value: IGraphicsEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGraphicsEffect> for ::windows::core::IUnknown {
    fn from(value: &IGraphicsEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGraphicsEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGraphicsEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGraphicsEffect> for ::windows::core::IInspectable {
    fn from(value: IGraphicsEffect) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGraphicsEffect> for ::windows::core::IInspectable {
    fn from(value: &IGraphicsEffect) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGraphicsEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGraphicsEffect {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<IGraphicsEffect> for IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: IGraphicsEffect) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IGraphicsEffect> for IGraphicsEffectSource {
    type Error = ::windows::core::Error;
    fn try_from(value: &IGraphicsEffect) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGraphicsEffectSource> for IGraphicsEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IGraphicsEffectSource> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, IGraphicsEffectSource> for &IGraphicsEffect {
    fn into_param(self) -> ::windows::core::Param<'a, IGraphicsEffectSource> {
        ::core::convert::TryInto::<IGraphicsEffectSource>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for IGraphicsEffect {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGraphicsEffect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffect {}
impl ::core::fmt::Debug for IGraphicsEffect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffect").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGraphicsEffect {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{cb51c0ce-8fe6-4636-b202-861faa07d8f3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGraphicsEffect {
    type Vtable = IGraphicsEffect_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb51c0ce_8fe6_4636_b202_861faa07d8f3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffect_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Graphics_Effects\"`*"]
#[repr(transparent)]
pub struct IGraphicsEffectSource(::windows::core::IUnknown);
impl IGraphicsEffectSource {}
impl ::core::convert::From<IGraphicsEffectSource> for ::windows::core::IUnknown {
    fn from(value: IGraphicsEffectSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGraphicsEffectSource> for ::windows::core::IUnknown {
    fn from(value: &IGraphicsEffectSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGraphicsEffectSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGraphicsEffectSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGraphicsEffectSource> for ::windows::core::IInspectable {
    fn from(value: IGraphicsEffectSource) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGraphicsEffectSource> for ::windows::core::IInspectable {
    fn from(value: &IGraphicsEffectSource) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGraphicsEffectSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGraphicsEffectSource {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGraphicsEffectSource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGraphicsEffectSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGraphicsEffectSource {}
impl ::core::fmt::Debug for IGraphicsEffectSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGraphicsEffectSource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGraphicsEffectSource {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2d8f9ddc-4339-4eb9-9216-f9deb75658a2}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGraphicsEffectSource {
    type Vtable = IGraphicsEffectSource_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d8f9ddc_4339_4eb9_9216_f9deb75658a2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGraphicsEffectSource_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
