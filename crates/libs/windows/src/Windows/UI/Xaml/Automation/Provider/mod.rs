#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IAnnotationProvider(::windows::core::IUnknown);
impl IAnnotationProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn AnnotationTypeId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).AnnotationTypeId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn AnnotationTypeName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).AnnotationTypeName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Author(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Author)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DateTime(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DateTime)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Target(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Target)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
}
impl ::core::convert::From<IAnnotationProvider> for ::windows::core::IUnknown {
    fn from(value: IAnnotationProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAnnotationProvider> for ::windows::core::IUnknown {
    fn from(value: &IAnnotationProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IAnnotationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IAnnotationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IAnnotationProvider> for ::windows::core::IInspectable {
    fn from(value: IAnnotationProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAnnotationProvider> for ::windows::core::IInspectable {
    fn from(value: &IAnnotationProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IAnnotationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IAnnotationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IAnnotationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAnnotationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAnnotationProvider {}
impl ::core::fmt::Debug for IAnnotationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAnnotationProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IAnnotationProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{95ba1417-4437-451b-9461-050a49b59d06}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IAnnotationProvider {
    type Vtable = IAnnotationProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x95ba1417_4437_451b_9461_050a49b59d06);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub AnnotationTypeId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub AnnotationTypeName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Author: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DateTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Target: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ICustomNavigationProvider(::windows::core::IUnknown);
impl ICustomNavigationProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Peers\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub fn NavigateCustom(&self, direction: super::Peers::AutomationNavigationDirection) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).NavigateCustom)(::windows::core::Interface::as_raw(this), direction, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<ICustomNavigationProvider> for ::windows::core::IUnknown {
    fn from(value: ICustomNavigationProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomNavigationProvider> for ::windows::core::IUnknown {
    fn from(value: &ICustomNavigationProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ICustomNavigationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ICustomNavigationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ICustomNavigationProvider> for ::windows::core::IInspectable {
    fn from(value: ICustomNavigationProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomNavigationProvider> for ::windows::core::IInspectable {
    fn from(value: &ICustomNavigationProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ICustomNavigationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ICustomNavigationProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ICustomNavigationProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomNavigationProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomNavigationProvider {}
impl ::core::fmt::Debug for ICustomNavigationProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomNavigationProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ICustomNavigationProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2bd8a6d0-2fa3-4717-b28c-4917ce54928d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ICustomNavigationProvider {
    type Vtable = ICustomNavigationProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2bd8a6d0_2fa3_4717_b28c_4917ce54928d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomNavigationProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "UI_Xaml_Automation_Peers")]
    pub NavigateCustom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, direction: super::Peers::AutomationNavigationDirection, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Peers"))]
    NavigateCustom: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IDockProvider(::windows::core::IUnknown);
impl IDockProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DockPosition(&self) -> ::windows::core::Result<super::DockPosition> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::DockPosition>::zeroed();
            (::windows::core::Interface::vtable(this).DockPosition)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::DockPosition>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SetDockPosition(&self, dockposition: super::DockPosition) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetDockPosition)(::windows::core::Interface::as_raw(this), dockposition).ok() }
    }
}
impl ::core::convert::From<IDockProvider> for ::windows::core::IUnknown {
    fn from(value: IDockProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDockProvider> for ::windows::core::IUnknown {
    fn from(value: &IDockProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDockProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDockProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDockProvider> for ::windows::core::IInspectable {
    fn from(value: IDockProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDockProvider> for ::windows::core::IInspectable {
    fn from(value: &IDockProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDockProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDockProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDockProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDockProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDockProvider {}
impl ::core::fmt::Debug for IDockProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDockProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDockProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{48c243f8-78b1-44a0-ac5f-750757bcde3c}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDockProvider {
    type Vtable = IDockProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x48c243f8_78b1_44a0_ac5f_750757bcde3c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DockPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::DockPosition) -> ::windows::core::HRESULT,
    pub SetDockPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dockposition: super::DockPosition) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IDragProvider(::windows::core::IUnknown);
impl IDragProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn IsGrabbed(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsGrabbed)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DropEffect(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DropEffect)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DropEffects(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DropEffects)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetGrabbedItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetGrabbedItems)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::convert::From<IDragProvider> for ::windows::core::IUnknown {
    fn from(value: IDragProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDragProvider> for ::windows::core::IUnknown {
    fn from(value: &IDragProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDragProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDragProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDragProvider> for ::windows::core::IInspectable {
    fn from(value: IDragProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDragProvider> for ::windows::core::IInspectable {
    fn from(value: &IDragProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDragProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDragProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDragProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDragProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDragProvider {}
impl ::core::fmt::Debug for IDragProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDragProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDragProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2e7786a9-7ffc-4f57-b965-1ef1f373f546}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDragProvider {
    type Vtable = IDragProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e7786a9_7ffc_4f57_b965_1ef1f373f546);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsGrabbed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub DropEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DropEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetGrabbedItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IDropTargetProvider(::windows::core::IUnknown);
impl IDropTargetProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DropEffect(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DropEffect)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DropEffects(&self) -> ::windows::core::Result<::windows::core::Array<::windows::core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).DropEffects)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<::windows::core::HSTRING>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::convert::From<IDropTargetProvider> for ::windows::core::IUnknown {
    fn from(value: IDropTargetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDropTargetProvider> for ::windows::core::IUnknown {
    fn from(value: &IDropTargetProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDropTargetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDropTargetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IDropTargetProvider> for ::windows::core::IInspectable {
    fn from(value: IDropTargetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDropTargetProvider> for ::windows::core::IInspectable {
    fn from(value: &IDropTargetProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IDropTargetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IDropTargetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDropTargetProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDropTargetProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDropTargetProvider {}
impl ::core::fmt::Debug for IDropTargetProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDropTargetProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IDropTargetProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7a245bdd-b458-4fe0-98c8-aac89df56d61}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IDropTargetProvider {
    type Vtable = IDropTargetProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a245bdd_b458_4fe0_98c8_aac89df56d61);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DropEffect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub DropEffects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IExpandCollapseProvider(::windows::core::IUnknown);
impl IExpandCollapseProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ExpandCollapseState(&self) -> ::windows::core::Result<super::ExpandCollapseState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ExpandCollapseState>::zeroed();
            (::windows::core::Interface::vtable(this).ExpandCollapseState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ExpandCollapseState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Collapse(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Collapse)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Expand(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Expand)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IExpandCollapseProvider> for ::windows::core::IUnknown {
    fn from(value: IExpandCollapseProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpandCollapseProvider> for ::windows::core::IUnknown {
    fn from(value: &IExpandCollapseProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IExpandCollapseProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IExpandCollapseProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IExpandCollapseProvider> for ::windows::core::IInspectable {
    fn from(value: IExpandCollapseProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IExpandCollapseProvider> for ::windows::core::IInspectable {
    fn from(value: &IExpandCollapseProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IExpandCollapseProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IExpandCollapseProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IExpandCollapseProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IExpandCollapseProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExpandCollapseProvider {}
impl ::core::fmt::Debug for IExpandCollapseProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExpandCollapseProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IExpandCollapseProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{49ac8399-d626-4543-94b9-a6d9a9593af6}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IExpandCollapseProvider {
    type Vtable = IExpandCollapseProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49ac8399_d626_4543_94b9_a6d9a9593af6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapseProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ExpandCollapseState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ExpandCollapseState) -> ::windows::core::HRESULT,
    pub Collapse: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IGridItemProvider(::windows::core::IUnknown);
impl IGridItemProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Column(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).Column)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ColumnSpan(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).ColumnSpan)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ContainingGrid(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).ContainingGrid)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Row(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).Row)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RowSpan(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).RowSpan)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
}
impl ::core::convert::From<IGridItemProvider> for ::windows::core::IUnknown {
    fn from(value: IGridItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGridItemProvider> for ::windows::core::IUnknown {
    fn from(value: &IGridItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGridItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGridItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGridItemProvider> for ::windows::core::IInspectable {
    fn from(value: IGridItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGridItemProvider> for ::windows::core::IInspectable {
    fn from(value: &IGridItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGridItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGridItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGridItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGridItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGridItemProvider {}
impl ::core::fmt::Debug for IGridItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGridItemProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGridItemProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{fff3683c-7407-45bb-a936-df3ed6d3837d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGridItemProvider {
    type Vtable = IGridItemProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfff3683c_7407_45bb_a936_df3ed6d3837d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Column: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ColumnSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub ContainingGrid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Row: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub RowSpan: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IGridProvider(::windows::core::IUnknown);
impl IGridProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ColumnCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).ColumnCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RowCount(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).RowCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetItem(&self, row: i32, column: i32) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetItem)(::windows::core::Interface::as_raw(this), row, column, result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
}
impl ::core::convert::From<IGridProvider> for ::windows::core::IUnknown {
    fn from(value: IGridProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGridProvider> for ::windows::core::IUnknown {
    fn from(value: &IGridProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IGridProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IGridProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IGridProvider> for ::windows::core::IInspectable {
    fn from(value: IGridProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IGridProvider> for ::windows::core::IInspectable {
    fn from(value: &IGridProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IGridProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IGridProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IGridProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IGridProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IGridProvider {}
impl ::core::fmt::Debug for IGridProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IGridProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IGridProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{8b62b7a0-932c-4490-9a13-02fdb39a8f5b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IGridProvider {
    type Vtable = IGridProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8b62b7a0_932c_4490_9a13_02fdb39a8f5b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub RowCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, row: i32, column: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IIRawElementProviderSimple(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IIRawElementProviderSimple {
    type Vtable = IIRawElementProviderSimple_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec752224_9b77_4720_bb21_4ac89fdb1afd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIRawElementProviderSimple_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IInvokeProvider(::windows::core::IUnknown);
impl IInvokeProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Invoke(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IInvokeProvider> for ::windows::core::IUnknown {
    fn from(value: IInvokeProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInvokeProvider> for ::windows::core::IUnknown {
    fn from(value: &IInvokeProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IInvokeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IInvokeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IInvokeProvider> for ::windows::core::IInspectable {
    fn from(value: IInvokeProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IInvokeProvider> for ::windows::core::IInspectable {
    fn from(value: &IInvokeProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IInvokeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IInvokeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IInvokeProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IInvokeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IInvokeProvider {}
impl ::core::fmt::Debug for IInvokeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IInvokeProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IInvokeProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{f7d1a187-b13c-4540-b09e-6778e2dc9ba5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IInvokeProvider {
    type Vtable = IInvokeProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7d1a187_b13c_4540_b09e_6778e2dc9ba5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInvokeProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IItemContainerProvider(::windows::core::IUnknown);
impl IItemContainerProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn FindItemByProperty<'a, Param0: ::windows::core::IntoParam<'a, IRawElementProviderSimple>, Param1: ::windows::core::IntoParam<'a, super::AutomationProperty>, Param2: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, startafter: Param0, automationproperty: Param1, value: Param2) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).FindItemByProperty)(::windows::core::Interface::as_raw(this), startafter.into_param().abi(), automationproperty.into_param().abi(), value.into_param().abi(), result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
}
impl ::core::convert::From<IItemContainerProvider> for ::windows::core::IUnknown {
    fn from(value: IItemContainerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IItemContainerProvider> for ::windows::core::IUnknown {
    fn from(value: &IItemContainerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IItemContainerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IItemContainerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IItemContainerProvider> for ::windows::core::IInspectable {
    fn from(value: IItemContainerProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IItemContainerProvider> for ::windows::core::IInspectable {
    fn from(value: &IItemContainerProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IItemContainerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IItemContainerProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IItemContainerProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IItemContainerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IItemContainerProvider {}
impl ::core::fmt::Debug for IItemContainerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IItemContainerProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IItemContainerProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ef5cd845-e1d4-40f4-bad5-c7fad44a703e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IItemContainerProvider {
    type Vtable = IItemContainerProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef5cd845_e1d4_40f4_bad5_c7fad44a703e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemContainerProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub FindItemByProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startafter: *mut ::core::ffi::c_void, automationproperty: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IMultipleViewProvider(::windows::core::IUnknown);
impl IMultipleViewProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CurrentView(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).CurrentView)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetSupportedViews(&self) -> ::windows::core::Result<::windows::core::Array<i32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<i32>>::zeroed();
            (::windows::core::Interface::vtable(this).GetSupportedViews)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<i32>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetViewName(&self, viewid: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).GetViewName)(::windows::core::Interface::as_raw(this), viewid, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SetCurrentView(&self, viewid: i32) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetCurrentView)(::windows::core::Interface::as_raw(this), viewid).ok() }
    }
}
impl ::core::convert::From<IMultipleViewProvider> for ::windows::core::IUnknown {
    fn from(value: IMultipleViewProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultipleViewProvider> for ::windows::core::IUnknown {
    fn from(value: &IMultipleViewProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMultipleViewProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMultipleViewProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IMultipleViewProvider> for ::windows::core::IInspectable {
    fn from(value: IMultipleViewProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultipleViewProvider> for ::windows::core::IInspectable {
    fn from(value: &IMultipleViewProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IMultipleViewProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IMultipleViewProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMultipleViewProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultipleViewProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultipleViewProvider {}
impl ::core::fmt::Debug for IMultipleViewProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultipleViewProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IMultipleViewProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d014e196-0e50-4843-a5d2-c22897c8845a}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IMultipleViewProvider {
    type Vtable = IMultipleViewProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd014e196_0e50_4843_a5d2_c22897c8845a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub GetSupportedViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut i32) -> ::windows::core::HRESULT,
    pub GetViewName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, viewid: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IObjectModelProvider(::windows::core::IUnknown);
impl IObjectModelProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetUnderlyingObjectModel(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetUnderlyingObjectModel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
}
impl ::core::convert::From<IObjectModelProvider> for ::windows::core::IUnknown {
    fn from(value: IObjectModelProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectModelProvider> for ::windows::core::IUnknown {
    fn from(value: &IObjectModelProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IObjectModelProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IObjectModelProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IObjectModelProvider> for ::windows::core::IInspectable {
    fn from(value: IObjectModelProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IObjectModelProvider> for ::windows::core::IInspectable {
    fn from(value: &IObjectModelProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IObjectModelProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IObjectModelProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IObjectModelProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IObjectModelProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IObjectModelProvider {}
impl ::core::fmt::Debug for IObjectModelProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IObjectModelProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IObjectModelProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{c3ca36b9-0793-4ed0-bbf4-9ff4e0f98f80}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IObjectModelProvider {
    type Vtable = IObjectModelProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc3ca36b9_0793_4ed0_bbf4_9ff4e0f98f80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectModelProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IRangeValueProvider(::windows::core::IUnknown);
impl IRangeValueProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn LargeChange(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).LargeChange)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Maximum(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).Maximum)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Minimum(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).Minimum)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SmallChange(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).SmallChange)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SetValue(&self, value: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), value).ok() }
    }
}
impl ::core::convert::From<IRangeValueProvider> for ::windows::core::IUnknown {
    fn from(value: IRangeValueProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRangeValueProvider> for ::windows::core::IUnknown {
    fn from(value: &IRangeValueProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRangeValueProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRangeValueProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRangeValueProvider> for ::windows::core::IInspectable {
    fn from(value: IRangeValueProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRangeValueProvider> for ::windows::core::IInspectable {
    fn from(value: &IRangeValueProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRangeValueProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IRangeValueProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IRangeValueProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRangeValueProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRangeValueProvider {}
impl ::core::fmt::Debug for IRangeValueProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRangeValueProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRangeValueProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{838a34a8-7d5f-4079-af03-c3d015e93413}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IRangeValueProvider {
    type Vtable = IRangeValueProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x838a34a8_7d5f_4079_af03_c3d015e93413);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValueProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Maximum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Minimum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IRawElementProviderSimple(::windows::core::IUnknown);
impl IRawElementProviderSimple {}
impl ::core::clone::Clone for IRawElementProviderSimple {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRawElementProviderSimple {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRawElementProviderSimple {}
impl ::core::fmt::Debug for IRawElementProviderSimple {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRawElementProviderSimple").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IRawElementProviderSimple {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Automation.Provider.IRawElementProviderSimple;{ec752224-9b77-4720-bb21-4ac89fdb1afd})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IRawElementProviderSimple {
    type Vtable = IIRawElementProviderSimple_Vtbl;
    const IID: ::windows::core::GUID = <IIRawElementProviderSimple as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for IRawElementProviderSimple {
    const NAME: &'static str = "Windows.UI.Xaml.Automation.Provider.IRawElementProviderSimple";
}
impl ::core::convert::From<IRawElementProviderSimple> for ::windows::core::IUnknown {
    fn from(value: IRawElementProviderSimple) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRawElementProviderSimple> for ::windows::core::IUnknown {
    fn from(value: &IRawElementProviderSimple) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IRawElementProviderSimple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IRawElementProviderSimple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRawElementProviderSimple> for ::windows::core::IInspectable {
    fn from(value: IRawElementProviderSimple) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRawElementProviderSimple> for ::windows::core::IInspectable {
    fn from(value: &IRawElementProviderSimple) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IRawElementProviderSimple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IRawElementProviderSimple {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IRawElementProviderSimple> for super::super::DependencyObject {
    fn from(value: IRawElementProviderSimple) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&IRawElementProviderSimple> for super::super::DependencyObject {
    fn from(value: &IRawElementProviderSimple) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for IRawElementProviderSimple {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::super::DependencyObject> for &IRawElementProviderSimple {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for IRawElementProviderSimple {}
unsafe impl ::core::marker::Sync for IRawElementProviderSimple {}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IScrollItemProvider(::windows::core::IUnknown);
impl IScrollItemProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ScrollIntoView(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ScrollIntoView)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IScrollItemProvider> for ::windows::core::IUnknown {
    fn from(value: IScrollItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScrollItemProvider> for ::windows::core::IUnknown {
    fn from(value: &IScrollItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IScrollItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IScrollItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IScrollItemProvider> for ::windows::core::IInspectable {
    fn from(value: IScrollItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScrollItemProvider> for ::windows::core::IInspectable {
    fn from(value: &IScrollItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IScrollItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IScrollItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IScrollItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IScrollItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScrollItemProvider {}
impl ::core::fmt::Debug for IScrollItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScrollItemProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IScrollItemProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{9a3ec090-5d2c-4e42-9ee6-9d58db100b55}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IScrollItemProvider {
    type Vtable = IScrollItemProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a3ec090_5d2c_4e42_9ee6_9d58db100b55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollItemProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IScrollProvider(::windows::core::IUnknown);
impl IScrollProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn HorizontallyScrollable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).HorizontallyScrollable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn HorizontalScrollPercent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalScrollPercent)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn HorizontalViewSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).HorizontalViewSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn VerticallyScrollable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).VerticallyScrollable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn VerticalScrollPercent(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).VerticalScrollPercent)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn VerticalViewSize(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).VerticalViewSize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Scroll(&self, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Scroll)(::windows::core::Interface::as_raw(this), horizontalamount, verticalamount).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetScrollPercent)(::windows::core::Interface::as_raw(this), horizontalpercent, verticalpercent).ok() }
    }
}
impl ::core::convert::From<IScrollProvider> for ::windows::core::IUnknown {
    fn from(value: IScrollProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScrollProvider> for ::windows::core::IUnknown {
    fn from(value: &IScrollProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IScrollProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IScrollProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IScrollProvider> for ::windows::core::IInspectable {
    fn from(value: IScrollProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IScrollProvider> for ::windows::core::IInspectable {
    fn from(value: &IScrollProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IScrollProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IScrollProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IScrollProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IScrollProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IScrollProvider {}
impl ::core::fmt::Debug for IScrollProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IScrollProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IScrollProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{374bf581-7716-4bbc-82eb-d997006ea999}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IScrollProvider {
    type Vtable = IScrollProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x374bf581_7716_4bbc_82eb_d997006ea999);
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub HorizontallyScrollable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub HorizontalScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub HorizontalViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub VerticallyScrollable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub VerticalScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub VerticalViewSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Scroll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> ::windows::core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ISelectionItemProvider(::windows::core::IUnknown);
impl ISelectionItemProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn IsSelected(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsSelected)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SelectionContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).SelectionContainer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn AddToSelection(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddToSelection)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFromSelection)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Select(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Select)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<ISelectionItemProvider> for ::windows::core::IUnknown {
    fn from(value: ISelectionItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectionItemProvider> for ::windows::core::IUnknown {
    fn from(value: &ISelectionItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISelectionItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISelectionItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISelectionItemProvider> for ::windows::core::IInspectable {
    fn from(value: ISelectionItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectionItemProvider> for ::windows::core::IInspectable {
    fn from(value: &ISelectionItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISelectionItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISelectionItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISelectionItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISelectionItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectionItemProvider {}
impl ::core::fmt::Debug for ISelectionItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectionItemProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISelectionItemProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{6a4977c1-830d-42d2-bf62-042ebddecc19}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISelectionItemProvider {
    type Vtable = ISelectionItemProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6a4977c1_830d_42d2_bf62_042ebddecc19);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub SelectionContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ISelectionProvider(::windows::core::IUnknown);
impl ISelectionProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CanSelectMultiple(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanSelectMultiple)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn IsSelectionRequired(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsSelectionRequired)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetSelection)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::convert::From<ISelectionProvider> for ::windows::core::IUnknown {
    fn from(value: ISelectionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectionProvider> for ::windows::core::IUnknown {
    fn from(value: &ISelectionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISelectionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISelectionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISelectionProvider> for ::windows::core::IInspectable {
    fn from(value: ISelectionProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISelectionProvider> for ::windows::core::IInspectable {
    fn from(value: &ISelectionProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISelectionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISelectionProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISelectionProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISelectionProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISelectionProvider {}
impl ::core::fmt::Debug for ISelectionProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISelectionProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISelectionProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1f018fca-b944-4395-8de1-88f674af51d3}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISelectionProvider {
    type Vtable = ISelectionProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f018fca_b944_4395_8de1_88f674af51d3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CanSelectMultiple: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsSelectionRequired: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ISpreadsheetItemProvider(::windows::core::IUnknown);
impl ISpreadsheetItemProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Formula(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Formula)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetAnnotationObjects(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetAnnotationObjects)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetAnnotationTypes(&self) -> ::windows::core::Result<::windows::core::Array<super::AnnotationType>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<super::AnnotationType>>::zeroed();
            (::windows::core::Interface::vtable(this).GetAnnotationTypes)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<super::AnnotationType>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::convert::From<ISpreadsheetItemProvider> for ::windows::core::IUnknown {
    fn from(value: ISpreadsheetItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpreadsheetItemProvider> for ::windows::core::IUnknown {
    fn from(value: &ISpreadsheetItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpreadsheetItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpreadsheetItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISpreadsheetItemProvider> for ::windows::core::IInspectable {
    fn from(value: ISpreadsheetItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpreadsheetItemProvider> for ::windows::core::IInspectable {
    fn from(value: &ISpreadsheetItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISpreadsheetItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISpreadsheetItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpreadsheetItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpreadsheetItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpreadsheetItemProvider {}
impl ::core::fmt::Debug for ISpreadsheetItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpreadsheetItemProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISpreadsheetItemProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ebde8f92-6015-4826-b719-47521a81c67e}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISpreadsheetItemProvider {
    type Vtable = ISpreadsheetItemProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebde8f92_6015_4826_b719_47521a81c67e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Formula: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub GetAnnotationObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAnnotationTypes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut super::AnnotationType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ISpreadsheetProvider(::windows::core::IUnknown);
impl ISpreadsheetProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetItemByName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, name: Param0) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetItemByName)(::windows::core::Interface::as_raw(this), name.into_param().abi(), result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
}
impl ::core::convert::From<ISpreadsheetProvider> for ::windows::core::IUnknown {
    fn from(value: ISpreadsheetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpreadsheetProvider> for ::windows::core::IUnknown {
    fn from(value: &ISpreadsheetProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISpreadsheetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISpreadsheetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISpreadsheetProvider> for ::windows::core::IInspectable {
    fn from(value: ISpreadsheetProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISpreadsheetProvider> for ::windows::core::IInspectable {
    fn from(value: &ISpreadsheetProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISpreadsheetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISpreadsheetProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISpreadsheetProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISpreadsheetProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISpreadsheetProvider {}
impl ::core::fmt::Debug for ISpreadsheetProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISpreadsheetProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISpreadsheetProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{15359093-bd99-4cfd-9f07-3b14b315e23d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISpreadsheetProvider {
    type Vtable = ISpreadsheetProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15359093_bd99_4cfd_9f07_3b14b315e23d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetItemByName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IStylesProvider(::windows::core::IUnknown);
impl IStylesProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ExtendedProperties(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).ExtendedProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn FillColor(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Color>::zeroed();
            (::windows::core::Interface::vtable(this).FillColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn FillPatternColor(&self) -> ::windows::core::Result<super::super::super::Color> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::super::super::Color>::zeroed();
            (::windows::core::Interface::vtable(this).FillPatternColor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::super::Color>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn FillPatternStyle(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).FillPatternStyle)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Shape(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Shape)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn StyleId(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).StyleId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn StyleName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).StyleName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
}
impl ::core::convert::From<IStylesProvider> for ::windows::core::IUnknown {
    fn from(value: IStylesProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStylesProvider> for ::windows::core::IUnknown {
    fn from(value: &IStylesProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IStylesProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IStylesProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IStylesProvider> for ::windows::core::IInspectable {
    fn from(value: IStylesProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStylesProvider> for ::windows::core::IInspectable {
    fn from(value: &IStylesProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IStylesProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IStylesProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IStylesProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStylesProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStylesProvider {}
impl ::core::fmt::Debug for IStylesProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStylesProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStylesProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1a5b7a17-7c01-4bec-9cd4-2dfa7dc246cd}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStylesProvider {
    type Vtable = IStylesProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a5b7a17_7c01_4bec_9cd4_2dfa7dc246cd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ExtendedProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub FillColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub FillPatternColor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Color) -> ::windows::core::HRESULT,
    pub FillPatternStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub Shape: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub StyleId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
    pub StyleName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ISynchronizedInputProvider(::windows::core::IUnknown);
impl ISynchronizedInputProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Cancel(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Cancel)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn StartListening(&self, inputtype: super::SynchronizedInputType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).StartListening)(::windows::core::Interface::as_raw(this), inputtype).ok() }
    }
}
impl ::core::convert::From<ISynchronizedInputProvider> for ::windows::core::IUnknown {
    fn from(value: ISynchronizedInputProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizedInputProvider> for ::windows::core::IUnknown {
    fn from(value: &ISynchronizedInputProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISynchronizedInputProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISynchronizedInputProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ISynchronizedInputProvider> for ::windows::core::IInspectable {
    fn from(value: ISynchronizedInputProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronizedInputProvider> for ::windows::core::IInspectable {
    fn from(value: &ISynchronizedInputProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ISynchronizedInputProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ISynchronizedInputProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ISynchronizedInputProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronizedInputProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronizedInputProvider {}
impl ::core::fmt::Debug for ISynchronizedInputProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronizedInputProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ISynchronizedInputProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3d60cecb-da54-4aa3-b915-e3244427d4ac}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ISynchronizedInputProvider {
    type Vtable = ISynchronizedInputProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d60cecb_da54_4aa3_b915_e3244427d4ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizedInputProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Cancel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StartListening: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputtype: super::SynchronizedInputType) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITableItemProvider(::windows::core::IUnknown);
impl ITableItemProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetColumnHeaderItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetColumnHeaderItems)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetRowHeaderItems(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetRowHeaderItems)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::convert::From<ITableItemProvider> for ::windows::core::IUnknown {
    fn from(value: ITableItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITableItemProvider> for ::windows::core::IUnknown {
    fn from(value: &ITableItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITableItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITableItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITableItemProvider> for ::windows::core::IInspectable {
    fn from(value: ITableItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITableItemProvider> for ::windows::core::IInspectable {
    fn from(value: &ITableItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITableItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITableItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITableItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITableItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableItemProvider {}
impl ::core::fmt::Debug for ITableItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableItemProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITableItemProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{3b2c49cd-1de2-4ee2-a3e1-fb553559d15d}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITableItemProvider {
    type Vtable = ITableItemProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b2c49cd_1de2_4ee2_a3e1_fb553559d15d);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetColumnHeaderItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRowHeaderItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITableProvider(::windows::core::IUnknown);
impl ITableProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RowOrColumnMajor(&self) -> ::windows::core::Result<super::RowOrColumnMajor> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::RowOrColumnMajor>::zeroed();
            (::windows::core::Interface::vtable(this).RowOrColumnMajor)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::RowOrColumnMajor>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetColumnHeaders(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetColumnHeaders)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetRowHeaders(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetRowHeaders)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::convert::From<ITableProvider> for ::windows::core::IUnknown {
    fn from(value: ITableProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITableProvider> for ::windows::core::IUnknown {
    fn from(value: &ITableProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITableProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITableProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITableProvider> for ::windows::core::IInspectable {
    fn from(value: ITableProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITableProvider> for ::windows::core::IInspectable {
    fn from(value: &ITableProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITableProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITableProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITableProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITableProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITableProvider {}
impl ::core::fmt::Debug for ITableProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITableProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITableProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{7a8ed399-6824-4595-bab3-464bc9a04417}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITableProvider {
    type Vtable = ITableProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7a8ed399_6824_4595_bab3_464bc9a04417);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RowOrColumnMajor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::RowOrColumnMajor) -> ::windows::core::HRESULT,
    pub GetColumnHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRowHeaders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITextChildProvider(::windows::core::IUnknown);
impl ITextChildProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn TextContainer(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TextContainer)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn TextRange(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).TextRange)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
}
impl ::core::convert::From<ITextChildProvider> for ::windows::core::IUnknown {
    fn from(value: ITextChildProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextChildProvider> for ::windows::core::IUnknown {
    fn from(value: &ITextChildProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextChildProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextChildProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextChildProvider> for ::windows::core::IInspectable {
    fn from(value: ITextChildProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextChildProvider> for ::windows::core::IInspectable {
    fn from(value: &ITextChildProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextChildProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextChildProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextChildProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextChildProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextChildProvider {}
impl ::core::fmt::Debug for ITextChildProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextChildProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextChildProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1133c336-a89b-4130-9be6-55e33334f557}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextChildProvider {
    type Vtable = ITextChildProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1133c336_a89b_4130_9be6_55e33334f557);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextChildProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub TextContainer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub TextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITextEditProvider(::windows::core::IUnknown);
impl ITextEditProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetActiveComposition(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetActiveComposition)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetConversionTarget(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetConversionTarget)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).DocumentRange)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SupportedTextSelection(&self) -> ::windows::core::Result<super::SupportedTextSelection> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::SupportedTextSelection>::zeroed();
            (::windows::core::Interface::vtable(this).SupportedTextSelection)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SupportedTextSelection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<ITextRangeProvider>>::zeroed();
            (::windows::core::Interface::vtable(this).GetSelection)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<ITextRangeProvider>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetVisibleRanges(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<ITextRangeProvider>>::zeroed();
            (::windows::core::Interface::vtable(this).GetVisibleRanges)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<ITextRangeProvider>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RangeFromChild<'a, Param0: ::windows::core::IntoParam<'a, IRawElementProviderSimple>>(&self, childelement: Param0) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RangeFromChild)(::windows::core::Interface::as_raw(this), childelement.into_param().abi(), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RangeFromPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, screenlocation: Param0) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RangeFromPoint)(::windows::core::Interface::as_raw(this), screenlocation.into_param().abi(), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
}
impl ::core::convert::From<ITextEditProvider> for ::windows::core::IUnknown {
    fn from(value: ITextEditProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextEditProvider> for ::windows::core::IUnknown {
    fn from(value: &ITextEditProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextEditProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextEditProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextEditProvider> for ::windows::core::IInspectable {
    fn from(value: ITextEditProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextEditProvider> for ::windows::core::IInspectable {
    fn from(value: &ITextEditProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextEditProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextEditProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ITextEditProvider> for ITextProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ITextEditProvider) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ITextEditProvider> for ITextProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ITextEditProvider) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextProvider> for ITextEditProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ITextProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextProvider> for &ITextEditProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ITextProvider> {
        ::core::convert::TryInto::<ITextProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ITextEditProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextEditProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextEditProvider {}
impl ::core::fmt::Debug for ITextEditProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextEditProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextEditProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ea3605b4-3a05-400e-b5f9-4e91b40f6176}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextEditProvider {
    type Vtable = ITextEditProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xea3605b4_3a05_400e_b5f9_4e91b40f6176);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextEditProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub GetActiveComposition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITextProvider(::windows::core::IUnknown);
impl ITextProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).DocumentRange)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SupportedTextSelection(&self) -> ::windows::core::Result<super::SupportedTextSelection> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::SupportedTextSelection>::zeroed();
            (::windows::core::Interface::vtable(this).SupportedTextSelection)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SupportedTextSelection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<ITextRangeProvider>>::zeroed();
            (::windows::core::Interface::vtable(this).GetSelection)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<ITextRangeProvider>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetVisibleRanges(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<ITextRangeProvider>>::zeroed();
            (::windows::core::Interface::vtable(this).GetVisibleRanges)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<ITextRangeProvider>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RangeFromChild<'a, Param0: ::windows::core::IntoParam<'a, IRawElementProviderSimple>>(&self, childelement: Param0) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RangeFromChild)(::windows::core::Interface::as_raw(this), childelement.into_param().abi(), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RangeFromPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, screenlocation: Param0) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RangeFromPoint)(::windows::core::Interface::as_raw(this), screenlocation.into_param().abi(), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
}
impl ::core::convert::From<ITextProvider> for ::windows::core::IUnknown {
    fn from(value: ITextProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextProvider> for ::windows::core::IUnknown {
    fn from(value: &ITextProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextProvider> for ::windows::core::IInspectable {
    fn from(value: ITextProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextProvider> for ::windows::core::IInspectable {
    fn from(value: &ITextProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextProvider {}
impl ::core::fmt::Debug for ITextProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{db5bbc9f-4807-4f2a-8678-1b13f3c60e22}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextProvider {
    type Vtable = ITextProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdb5bbc9f_4807_4f2a_8678_1b13f3c60e22);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub DocumentRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SupportedTextSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::SupportedTextSelection) -> ::windows::core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVisibleRanges: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RangeFromChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, childelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub RangeFromPoint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, screenlocation: super::super::super::super::Foundation::Point, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RangeFromPoint: usize,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITextProvider2(::windows::core::IUnknown);
impl ITextProvider2 {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RangeFromAnnotation<'a, Param0: ::windows::core::IntoParam<'a, IRawElementProviderSimple>>(&self, annotationelement: Param0) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RangeFromAnnotation)(::windows::core::Interface::as_raw(this), annotationelement.into_param().abi(), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetCaretRange(&self, isactive: &mut bool) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetCaretRange)(::windows::core::Interface::as_raw(this), isactive, result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn DocumentRange(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).DocumentRange)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SupportedTextSelection(&self) -> ::windows::core::Result<super::SupportedTextSelection> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::SupportedTextSelection>::zeroed();
            (::windows::core::Interface::vtable(this).SupportedTextSelection)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::SupportedTextSelection>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetSelection(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<ITextRangeProvider>>::zeroed();
            (::windows::core::Interface::vtable(this).GetSelection)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<ITextRangeProvider>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetVisibleRanges(&self) -> ::windows::core::Result<::windows::core::Array<ITextRangeProvider>> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<ITextRangeProvider>>::zeroed();
            (::windows::core::Interface::vtable(this).GetVisibleRanges)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<ITextRangeProvider>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RangeFromChild<'a, Param0: ::windows::core::IntoParam<'a, IRawElementProviderSimple>>(&self, childelement: Param0) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RangeFromChild)(::windows::core::Interface::as_raw(this), childelement.into_param().abi(), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RangeFromPoint<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::super::Foundation::Point>>(&self, screenlocation: Param0) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).RangeFromPoint)(::windows::core::Interface::as_raw(this), screenlocation.into_param().abi(), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
}
impl ::core::convert::From<ITextProvider2> for ::windows::core::IUnknown {
    fn from(value: ITextProvider2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextProvider2> for ::windows::core::IUnknown {
    fn from(value: &ITextProvider2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextProvider2> for ::windows::core::IInspectable {
    fn from(value: ITextProvider2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextProvider2> for ::windows::core::IInspectable {
    fn from(value: &ITextProvider2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ITextProvider2> for ITextProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ITextProvider2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ITextProvider2> for ITextProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ITextProvider2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextProvider> for ITextProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextProvider> for &ITextProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextProvider> {
        ::core::convert::TryInto::<ITextProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ITextProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextProvider2 {}
impl ::core::fmt::Debug for ITextProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextProvider2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextProvider2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{df1d48bc-0487-4e7f-9d5e-f09e77e41246}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextProvider2 {
    type Vtable = ITextProvider2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdf1d48bc_0487_4e7f_9d5e_f09e77e41246);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub RangeFromAnnotation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, annotationelement: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCaretRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isactive: *mut bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITextRangeProvider(::windows::core::IUnknown);
impl ITextRangeProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Clone(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Clone)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Compare<'a, Param0: ::windows::core::IntoParam<'a, ITextRangeProvider>>(&self, textrangeprovider: Param0) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Compare)(::windows::core::Interface::as_raw(this), textrangeprovider.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn CompareEndpoints<'a, Param1: ::windows::core::IntoParam<'a, ITextRangeProvider>>(&self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: Param1, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).CompareEndpoints)(::windows::core::Interface::as_raw(this), endpoint, textrangeprovider.into_param().abi(), targetendpoint, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn ExpandToEnclosingUnit(&self, unit: super::Text::TextUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ExpandToEnclosingUnit)(::windows::core::Interface::as_raw(this), unit).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn FindAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, attributeid: i32, value: Param1, backward: bool) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).FindAttribute)(::windows::core::Interface::as_raw(this), attributeid, value.into_param().abi(), backward, result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn FindText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, backward: bool, ignorecase: bool) -> ::windows::core::Result<ITextRangeProvider> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).FindText)(::windows::core::Interface::as_raw(this), text.into_param().abi(), backward, ignorecase, result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetAttributeValue(&self, attributeid: i32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAttributeValue)(::windows::core::Interface::as_raw(this), attributeid, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetBoundingRectangles(&self, returnvalue: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).GetBoundingRectangles)(::windows::core::Interface::as_raw(this), returnvalue.set_abi_len(), returnvalue as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetEnclosingElement(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetEnclosingElement)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).GetText)(::windows::core::Interface::as_raw(this), maxlength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn Move(&self, unit: super::Text::TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).Move)(::windows::core::Interface::as_raw(this), unit, count, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByUnit(&self, endpoint: super::Text::TextPatternRangeEndpoint, unit: super::Text::TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).MoveEndpointByUnit)(::windows::core::Interface::as_raw(this), endpoint, unit, count, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByRange<'a, Param1: ::windows::core::IntoParam<'a, ITextRangeProvider>>(&self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: Param1, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).MoveEndpointByRange)(::windows::core::Interface::as_raw(this), endpoint, textrangeprovider.into_param().abi(), targetendpoint).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Select(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Select)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn AddToSelection(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddToSelection)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFromSelection)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ScrollIntoView(&self, aligntotop: bool) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ScrollIntoView)(::windows::core::Interface::as_raw(this), aligntotop).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetChildren(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetChildren)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::convert::From<ITextRangeProvider> for ::windows::core::IUnknown {
    fn from(value: ITextRangeProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRangeProvider> for ::windows::core::IUnknown {
    fn from(value: &ITextRangeProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextRangeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextRangeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextRangeProvider> for ::windows::core::IInspectable {
    fn from(value: ITextRangeProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRangeProvider> for ::windows::core::IInspectable {
    fn from(value: &ITextRangeProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextRangeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextRangeProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITextRangeProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextRangeProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRangeProvider {}
impl ::core::fmt::Debug for ITextRangeProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRangeProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextRangeProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0274688d-06e9-4f66-9446-28a5be98fbd0}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextRangeProvider {
    type Vtable = ITextRangeProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0274688d_06e9_4f66_9446_28a5be98fbd0);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, textrangeprovider: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub CompareEndpoints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: *mut ::core::ffi::c_void, targetendpoint: super::Text::TextPatternRangeEndpoint, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    CompareEndpoints: usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: super::Text::TextUnit) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    ExpandToEnclosingUnit: usize,
    pub FindAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributeid: i32, value: *mut ::core::ffi::c_void, backward: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, backward: bool, ignorecase: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAttributeValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, attributeid: i32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBoundingRectangles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, returnValue_array_size: *mut u32, returnvalue: *mut *mut f64) -> ::windows::core::HRESULT,
    pub GetEnclosingElement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxlength: i32, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, unit: super::Text::TextUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    Move: usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub MoveEndpointByUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, unit: super::Text::TextUnit, count: i32, result__: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    MoveEndpointByUnit: usize,
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub MoveEndpointByRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: *mut ::core::ffi::c_void, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "UI_Xaml_Automation_Text"))]
    MoveEndpointByRange: usize,
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, aligntotop: bool) -> ::windows::core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITextRangeProvider2(::windows::core::IUnknown);
impl ITextRangeProvider2 {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ShowContextMenu(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ShowContextMenu)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Clone(&self) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).Clone)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Compare<'a, Param0: ::windows::core::IntoParam<'a, ITextRangeProvider>>(&self, textrangeprovider: Param0) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Compare)(::windows::core::Interface::as_raw(this), textrangeprovider.into_param().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn CompareEndpoints<'a, Param1: ::windows::core::IntoParam<'a, ITextRangeProvider>>(&self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: Param1, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).CompareEndpoints)(::windows::core::Interface::as_raw(this), endpoint, textrangeprovider.into_param().abi(), targetendpoint, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn ExpandToEnclosingUnit(&self, unit: super::Text::TextUnit) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ExpandToEnclosingUnit)(::windows::core::Interface::as_raw(this), unit).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn FindAttribute<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>>(&self, attributeid: i32, value: Param1, backward: bool) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).FindAttribute)(::windows::core::Interface::as_raw(this), attributeid, value.into_param().abi(), backward, result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn FindText<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, text: Param0, backward: bool, ignorecase: bool) -> ::windows::core::Result<ITextRangeProvider> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).FindText)(::windows::core::Interface::as_raw(this), text.into_param().abi(), backward, ignorecase, result__.as_mut_ptr()).from_abi::<ITextRangeProvider>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetAttributeValue(&self, attributeid: i32) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetAttributeValue)(::windows::core::Interface::as_raw(this), attributeid, result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetBoundingRectangles(&self, returnvalue: &mut ::windows::core::Array<f64>) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).GetBoundingRectangles)(::windows::core::Interface::as_raw(this), returnvalue.set_abi_len(), returnvalue as *mut _ as _).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetEnclosingElement(&self) -> ::windows::core::Result<IRawElementProviderSimple> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<*mut ::core::ffi::c_void>::zeroed();
            (::windows::core::Interface::vtable(this).GetEnclosingElement)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<IRawElementProviderSimple>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetText(&self, maxlength: i32) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).GetText)(::windows::core::Interface::as_raw(this), maxlength, result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn Move(&self, unit: super::Text::TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).Move)(::windows::core::Interface::as_raw(this), unit, count, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByUnit(&self, endpoint: super::Text::TextPatternRangeEndpoint, unit: super::Text::TextUnit, count: i32) -> ::windows::core::Result<i32> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<i32>::zeroed();
            (::windows::core::Interface::vtable(this).MoveEndpointByUnit)(::windows::core::Interface::as_raw(this), endpoint, unit, count, result__.as_mut_ptr()).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`, `\"UI_Xaml_Automation_Text\"`*"]
    #[cfg(feature = "UI_Xaml_Automation_Text")]
    pub fn MoveEndpointByRange<'a, Param1: ::windows::core::IntoParam<'a, ITextRangeProvider>>(&self, endpoint: super::Text::TextPatternRangeEndpoint, textrangeprovider: Param1, targetendpoint: super::Text::TextPatternRangeEndpoint) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).MoveEndpointByRange)(::windows::core::Interface::as_raw(this), endpoint, textrangeprovider.into_param().abi(), targetendpoint).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Select(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Select)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn AddToSelection(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).AddToSelection)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn RemoveFromSelection(&self) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).RemoveFromSelection)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ScrollIntoView(&self, aligntotop: bool) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).ScrollIntoView)(::windows::core::Interface::as_raw(this), aligntotop).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn GetChildren(&self) -> ::windows::core::Result<::windows::core::Array<IRawElementProviderSimple>> {
        let this = &::windows::core::Interface::cast::<ITextRangeProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::windows::core::Array<IRawElementProviderSimple>>::zeroed();
            (::windows::core::Interface::vtable(this).GetChildren)(::windows::core::Interface::as_raw(this), ::windows::core::Array::<IRawElementProviderSimple>::set_abi_len(result__.assume_init_mut()), result__.as_mut_ptr() as *mut _ as _).and_then(|| result__.assume_init())
        }
    }
}
impl ::core::convert::From<ITextRangeProvider2> for ::windows::core::IUnknown {
    fn from(value: ITextRangeProvider2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRangeProvider2> for ::windows::core::IUnknown {
    fn from(value: &ITextRangeProvider2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITextRangeProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITextRangeProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITextRangeProvider2> for ::windows::core::IInspectable {
    fn from(value: ITextRangeProvider2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITextRangeProvider2> for ::windows::core::IInspectable {
    fn from(value: &ITextRangeProvider2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITextRangeProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITextRangeProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ITextRangeProvider2> for ITextRangeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ITextRangeProvider2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ITextRangeProvider2> for ITextRangeProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ITextRangeProvider2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRangeProvider> for ITextRangeProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRangeProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITextRangeProvider> for &ITextRangeProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITextRangeProvider> {
        ::core::convert::TryInto::<ITextRangeProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ITextRangeProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITextRangeProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITextRangeProvider2 {}
impl ::core::fmt::Debug for ITextRangeProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITextRangeProvider2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITextRangeProvider2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d3be3dfb-9f54-4642-a7a5-5c18d5ee2a3f}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITextRangeProvider2 {
    type Vtable = ITextRangeProvider2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd3be3dfb_9f54_4642_a7a5_5c18d5ee2a3f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IToggleProvider(::windows::core::IUnknown);
impl IToggleProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ToggleState(&self) -> ::windows::core::Result<super::ToggleState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::ToggleState>::zeroed();
            (::windows::core::Interface::vtable(this).ToggleState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::ToggleState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Toggle(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Toggle)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IToggleProvider> for ::windows::core::IUnknown {
    fn from(value: IToggleProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToggleProvider> for ::windows::core::IUnknown {
    fn from(value: &IToggleProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IToggleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IToggleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IToggleProvider> for ::windows::core::IInspectable {
    fn from(value: IToggleProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IToggleProvider> for ::windows::core::IInspectable {
    fn from(value: &IToggleProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IToggleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IToggleProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IToggleProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IToggleProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToggleProvider {}
impl ::core::fmt::Debug for IToggleProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToggleProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IToggleProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{93b88290-656f-44f7-aeaf-78b8f944d062}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IToggleProvider {
    type Vtable = IToggleProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x93b88290_656f_44f7_aeaf_78b8f944d062);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub ToggleState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::ToggleState) -> ::windows::core::HRESULT,
    pub Toggle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITransformProvider(::windows::core::IUnknown);
impl ITransformProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CanMove(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanMove)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CanResize(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanResize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CanRotate(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanRotate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Move(&self, x: f64, y: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Move)(::windows::core::Interface::as_raw(this), x, y).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Resize(&self, width: f64, height: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Resize)(::windows::core::Interface::as_raw(this), width, height).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Rotate(&self, degrees: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Rotate)(::windows::core::Interface::as_raw(this), degrees).ok() }
    }
}
impl ::core::convert::From<ITransformProvider> for ::windows::core::IUnknown {
    fn from(value: ITransformProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransformProvider> for ::windows::core::IUnknown {
    fn from(value: &ITransformProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransformProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransformProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITransformProvider> for ::windows::core::IInspectable {
    fn from(value: ITransformProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransformProvider> for ::windows::core::IInspectable {
    fn from(value: &ITransformProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITransformProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITransformProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for ITransformProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransformProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransformProvider {}
impl ::core::fmt::Debug for ITransformProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransformProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITransformProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{79670fdd-f6a9-4a65-af17-861db799a2da}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITransformProvider {
    type Vtable = ITransformProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79670fdd_f6a9_4a65_af17_861db799a2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CanMove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanResize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub CanRotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Move: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, x: f64, y: f64) -> ::windows::core::HRESULT,
    pub Resize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: f64, height: f64) -> ::windows::core::HRESULT,
    pub Rotate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, degrees: f64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct ITransformProvider2(::windows::core::IUnknown);
impl ITransformProvider2 {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CanZoom(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanZoom)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ZoomLevel(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).ZoomLevel)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn MaxZoom(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).MaxZoom)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn MinZoom(&self) -> ::windows::core::Result<f64> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<f64>::zeroed();
            (::windows::core::Interface::vtable(this).MinZoom)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<f64>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Zoom(&self, zoom: f64) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Zoom)(::windows::core::Interface::as_raw(this), zoom).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn ZoomByUnit(&self, zoomunit: super::ZoomUnit) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).ZoomByUnit)(::windows::core::Interface::as_raw(this), zoomunit).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CanMove(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanMove)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CanResize(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanResize)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn CanRotate(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<ITransformProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).CanRotate)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Move(&self, x: f64, y: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITransformProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Move)(::windows::core::Interface::as_raw(this), x, y).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Resize(&self, width: f64, height: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITransformProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Resize)(::windows::core::Interface::as_raw(this), width, height).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Rotate(&self, degrees: f64) -> ::windows::core::Result<()> {
        let this = &::windows::core::Interface::cast::<ITransformProvider>(self)?;
        unsafe { (::windows::core::Interface::vtable(this).Rotate)(::windows::core::Interface::as_raw(this), degrees).ok() }
    }
}
impl ::core::convert::From<ITransformProvider2> for ::windows::core::IUnknown {
    fn from(value: ITransformProvider2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransformProvider2> for ::windows::core::IUnknown {
    fn from(value: &ITransformProvider2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITransformProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITransformProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<ITransformProvider2> for ::windows::core::IInspectable {
    fn from(value: ITransformProvider2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ITransformProvider2> for ::windows::core::IInspectable {
    fn from(value: &ITransformProvider2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for ITransformProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a ITransformProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<ITransformProvider2> for ITransformProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: ITransformProvider2) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ITransformProvider2> for ITransformProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &ITransformProvider2) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransformProvider> for ITransformProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITransformProvider> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ITransformProvider> for &ITransformProvider2 {
    fn into_param(self) -> ::windows::core::Param<'a, ITransformProvider> {
        ::core::convert::TryInto::<ITransformProvider>::try_into(self).map(::windows::core::Param::Owned).unwrap_or(::windows::core::Param::None)
    }
}
impl ::core::clone::Clone for ITransformProvider2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ITransformProvider2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ITransformProvider2 {}
impl ::core::fmt::Debug for ITransformProvider2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITransformProvider2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for ITransformProvider2 {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{a8b11756-a39f-4e97-8c7d-c1ea8dd633c5}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for ITransformProvider2 {
    type Vtable = ITransformProvider2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8b11756_a39f_4e97_8c7d_c1ea8dd633c5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider2_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub CanZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MaxZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub MinZoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut f64) -> ::windows::core::HRESULT,
    pub Zoom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoom: f64) -> ::windows::core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, zoomunit: super::ZoomUnit) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IValueProvider(::windows::core::IUnknown);
impl IValueProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn IsReadOnly(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsReadOnly)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<::core::mem::ManuallyDrop<::windows::core::HSTRING>>::zeroed();
            (::windows::core::Interface::vtable(this).Value)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SetValue<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::HSTRING>>(&self, value: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetValue)(::windows::core::Interface::as_raw(this), value.into_param().abi()).ok() }
    }
}
impl ::core::convert::From<IValueProvider> for ::windows::core::IUnknown {
    fn from(value: IValueProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValueProvider> for ::windows::core::IUnknown {
    fn from(value: &IValueProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IValueProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IValueProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IValueProvider> for ::windows::core::IInspectable {
    fn from(value: IValueProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IValueProvider> for ::windows::core::IInspectable {
    fn from(value: &IValueProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IValueProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IValueProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IValueProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IValueProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IValueProvider {}
impl ::core::fmt::Debug for IValueProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IValueProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IValueProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{2086b7a7-ac0e-47d1-ab9b-2a64292afdf8}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IValueProvider {
    type Vtable = IValueProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2086b7a7_ac0e_47d1_ab9b_2a64292afdf8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsReadOnly: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IVirtualizedItemProvider(::windows::core::IUnknown);
impl IVirtualizedItemProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Realize(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Realize)(::windows::core::Interface::as_raw(this)).ok() }
    }
}
impl ::core::convert::From<IVirtualizedItemProvider> for ::windows::core::IUnknown {
    fn from(value: IVirtualizedItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVirtualizedItemProvider> for ::windows::core::IUnknown {
    fn from(value: &IVirtualizedItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IVirtualizedItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IVirtualizedItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IVirtualizedItemProvider> for ::windows::core::IInspectable {
    fn from(value: IVirtualizedItemProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IVirtualizedItemProvider> for ::windows::core::IInspectable {
    fn from(value: &IVirtualizedItemProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IVirtualizedItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IVirtualizedItemProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IVirtualizedItemProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IVirtualizedItemProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IVirtualizedItemProvider {}
impl ::core::fmt::Debug for IVirtualizedItemProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IVirtualizedItemProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IVirtualizedItemProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{17d4a04b-d658-48e0-a574-5a516c58dfa7}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IVirtualizedItemProvider {
    type Vtable = IVirtualizedItemProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17d4a04b_d658_48e0_a574_5a516c58dfa7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualizedItemProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub Realize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
#[repr(transparent)]
pub struct IWindowProvider(::windows::core::IUnknown);
impl IWindowProvider {
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn IsModal(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsModal)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn IsTopmost(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).IsTopmost)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Maximizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Maximizable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Minimizable(&self) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).Minimizable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn InteractionState(&self) -> ::windows::core::Result<super::WindowInteractionState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::WindowInteractionState>::zeroed();
            (::windows::core::Interface::vtable(this).InteractionState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::WindowInteractionState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn VisualState(&self) -> ::windows::core::Result<super::WindowVisualState> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<super::WindowVisualState>::zeroed();
            (::windows::core::Interface::vtable(this).VisualState)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::WindowVisualState>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn Close(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Close)(::windows::core::Interface::as_raw(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn SetVisualState(&self, state: super::WindowVisualState) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetVisualState)(::windows::core::Interface::as_raw(this), state).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Automation_Provider\"`*"]
    pub fn WaitForInputIdle(&self, milliseconds: i32) -> ::windows::core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::<bool>::zeroed();
            (::windows::core::Interface::vtable(this).WaitForInputIdle)(::windows::core::Interface::as_raw(this), milliseconds, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
}
impl ::core::convert::From<IWindowProvider> for ::windows::core::IUnknown {
    fn from(value: IWindowProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowProvider> for ::windows::core::IUnknown {
    fn from(value: &IWindowProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IWindowProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IWindowProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IWindowProvider> for ::windows::core::IInspectable {
    fn from(value: IWindowProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IWindowProvider> for ::windows::core::IInspectable {
    fn from(value: &IWindowProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for IWindowProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a IWindowProvider {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IWindowProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IWindowProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWindowProvider {}
impl ::core::fmt::Debug for IWindowProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWindowProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IWindowProvider {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{1baa8b3d-38cf-415a-85d3-20e43a0ec1b1}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IWindowProvider {
    type Vtable = IWindowProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1baa8b3d_38cf_415a_85d3_20e43a0ec1b1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowProvider_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    pub IsModal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub IsTopmost: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Maximizable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub Minimizable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT,
    pub InteractionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WindowInteractionState) -> ::windows::core::HRESULT,
    pub VisualState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::WindowVisualState) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetVisualState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, state: super::WindowVisualState) -> ::windows::core::HRESULT,
    pub WaitForInputIdle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, milliseconds: i32, result__: *mut bool) -> ::windows::core::HRESULT,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
