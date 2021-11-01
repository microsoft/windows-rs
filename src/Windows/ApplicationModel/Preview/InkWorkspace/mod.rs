#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManager(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4262099344, 24153, 19383, [138, 99, 125, 33, 140, 217, 99, 0]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, bitmap: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Graphics_Imaging")))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManagerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IInkWorkspaceHostedAppManagerStatics {
    type Vtable = IInkWorkspaceHostedAppManagerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3422391493, 41314, 19396, [132, 238, 232, 113, 109, 82, 51, 197]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IInkWorkspaceHostedAppManagerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `ApplicationModel_Preview_InkWorkspace`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct InkWorkspaceHostedAppManager(::windows::runtime::IInspectable);
impl InkWorkspaceHostedAppManager {
    #[cfg(all(feature = "Foundation", feature = "Graphics_Imaging"))]
    #[doc = "*Required features: `ApplicationModel_Preview_InkWorkspace`, `Foundation`, `Graphics_Imaging`*"]
    pub fn SetThumbnailAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::super::Graphics::Imaging::SoftwareBitmap>>(&self, bitmap: Param0) -> ::windows::runtime::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), bitmap.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `ApplicationModel_Preview_InkWorkspace`*"]
    pub fn GetForCurrentApp() -> ::windows::runtime::Result<InkWorkspaceHostedAppManager> {
        Self::IInkWorkspaceHostedAppManagerStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<InkWorkspaceHostedAppManager>(result__)
        })
    }
    pub fn IInkWorkspaceHostedAppManagerStatics<R, F: FnOnce(&IInkWorkspaceHostedAppManagerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<InkWorkspaceHostedAppManager, IInkWorkspaceHostedAppManagerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for InkWorkspaceHostedAppManager {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager;{fe0a7990-5e59-4bb7-8a63-7d218cd96300})");
}
unsafe impl ::windows::runtime::Interface for InkWorkspaceHostedAppManager {
    type Vtable = IInkWorkspaceHostedAppManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4262099344, 24153, 19383, [138, 99, 125, 33, 140, 217, 99, 0]);
}
impl ::windows::runtime::RuntimeName for InkWorkspaceHostedAppManager {
    const NAME: &'static str = "Windows.ApplicationModel.Preview.InkWorkspace.InkWorkspaceHostedAppManager";
}
impl ::std::convert::From<InkWorkspaceHostedAppManager> for ::windows::runtime::IUnknown {
    fn from(value: InkWorkspaceHostedAppManager) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&InkWorkspaceHostedAppManager> for ::windows::runtime::IUnknown {
    fn from(value: &InkWorkspaceHostedAppManager) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<InkWorkspaceHostedAppManager> for ::windows::runtime::IInspectable {
    fn from(value: InkWorkspaceHostedAppManager) -> Self {
        value.0
    }
}
impl ::std::convert::From<&InkWorkspaceHostedAppManager> for ::windows::runtime::IInspectable {
    fn from(value: &InkWorkspaceHostedAppManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a InkWorkspaceHostedAppManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for InkWorkspaceHostedAppManager {}
unsafe impl ::std::marker::Sync for InkWorkspaceHostedAppManager {}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct PreviewInkWorkspaceContract(pub u8);
