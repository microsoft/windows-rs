#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct AddPagesEventArgs(::windows::core::IUnknown);
impl AddPagesEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<AddPagesEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn PrintTaskOptions(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrintTaskOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Printing::PrintTaskOptions>(result__)
        }
    }
}
impl ::core::clone::Clone for AddPagesEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddPagesEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPagesEventArgs {}
impl ::core::fmt::Debug for AddPagesEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPagesEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for AddPagesEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Printing.AddPagesEventArgs;{e2e52be5-056c-4420-9795-cb3526ce0c20})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for AddPagesEventArgs {
    type Vtable = IAddPagesEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IAddPagesEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for AddPagesEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.AddPagesEventArgs";
}
impl ::core::convert::From<AddPagesEventArgs> for ::windows::core::IUnknown {
    fn from(value: AddPagesEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddPagesEventArgs> for ::windows::core::IUnknown {
    fn from(value: &AddPagesEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for AddPagesEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a AddPagesEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<AddPagesEventArgs> for ::windows::core::IInspectable {
    fn from(value: AddPagesEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&AddPagesEventArgs> for ::windows::core::IInspectable {
    fn from(value: &AddPagesEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for AddPagesEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a AddPagesEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for AddPagesEventArgs {}
unsafe impl ::core::marker::Sync for AddPagesEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct AddPagesEventHandler(pub ::windows::core::IUnknown);
impl AddPagesEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<AddPagesEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = AddPagesEventHandlerBox::<F> { vtable: &AddPagesEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, AddPagesEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct AddPagesEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<AddPagesEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const AddPagesEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<AddPagesEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> AddPagesEventHandlerBox<F> {
    const VTABLE: AddPagesEventHandler_Vtbl = AddPagesEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<AddPagesEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for AddPagesEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for AddPagesEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AddPagesEventHandler {}
impl ::core::fmt::Debug for AddPagesEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPagesEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for AddPagesEventHandler {
    type Vtable = AddPagesEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd4b57970_57a0_4209_847c_c093b54bc729);
}
unsafe impl ::windows::core::RuntimeType for AddPagesEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{d4b57970-57a0-4209-847c-c093b54bc729}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct AddPagesEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct GetPreviewPageEventArgs(::windows::core::IUnknown);
impl GetPreviewPageEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<GetPreviewPageEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn PageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PageNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for GetPreviewPageEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetPreviewPageEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetPreviewPageEventArgs {}
impl ::core::fmt::Debug for GetPreviewPageEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetPreviewPageEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for GetPreviewPageEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Printing.GetPreviewPageEventArgs;{a43d703d-dea9-4df6-a7ed-35049cd485c7})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for GetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IGetPreviewPageEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for GetPreviewPageEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.GetPreviewPageEventArgs";
}
impl ::core::convert::From<GetPreviewPageEventArgs> for ::windows::core::IUnknown {
    fn from(value: GetPreviewPageEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetPreviewPageEventArgs> for ::windows::core::IUnknown {
    fn from(value: &GetPreviewPageEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<GetPreviewPageEventArgs> for ::windows::core::IInspectable {
    fn from(value: GetPreviewPageEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&GetPreviewPageEventArgs> for ::windows::core::IInspectable {
    fn from(value: &GetPreviewPageEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a GetPreviewPageEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for GetPreviewPageEventArgs {}
unsafe impl ::core::marker::Sync for GetPreviewPageEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct GetPreviewPageEventHandler(pub ::windows::core::IUnknown);
impl GetPreviewPageEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<GetPreviewPageEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = GetPreviewPageEventHandlerBox::<F> { vtable: &GetPreviewPageEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, GetPreviewPageEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct GetPreviewPageEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<GetPreviewPageEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const GetPreviewPageEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<GetPreviewPageEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> GetPreviewPageEventHandlerBox<F> {
    const VTABLE: GetPreviewPageEventHandler_Vtbl = GetPreviewPageEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<GetPreviewPageEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for GetPreviewPageEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for GetPreviewPageEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for GetPreviewPageEventHandler {}
impl ::core::fmt::Debug for GetPreviewPageEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GetPreviewPageEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for GetPreviewPageEventHandler {
    type Vtable = GetPreviewPageEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xccb3e9ed_9c11_4e50_ab49_e98086bbfdef);
}
unsafe impl ::windows::core::RuntimeType for GetPreviewPageEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{ccb3e9ed-9c11-4e50-ab49-e98086bbfdef}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct GetPreviewPageEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAddPagesEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IAddPagesEventArgs {
    type Vtable = IAddPagesEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2e52be5_056c_4420_9795_cb3526ce0c20);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAddPagesEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PrintTaskOptions: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IGetPreviewPageEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IGetPreviewPageEventArgs {
    type Vtable = IGetPreviewPageEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa43d703d_dea9_4df6_a7ed_35049cd485c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IGetPreviewPageEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub PageNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPaginateEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPaginateEventArgs {
    type Vtable = IPaginateEventArgs_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed945fd6_79ab_42b7_930a_3d6e09011d21);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPaginateEventArgs_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Printing")]
    pub PrintTaskOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    PrintTaskOptions: usize,
    pub CurrentPreviewPageNumber: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocument(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintDocument {
    type Vtable = IPrintDocument_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe44327c3_a999_485b_b1d8_72dc517821e6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocument_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Graphics_Printing")]
    pub DocumentSource: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Graphics_Printing"))]
    DocumentSource: usize,
    #[cfg(feature = "Foundation")]
    pub Paginate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Paginate: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePaginate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePaginate: usize,
    #[cfg(feature = "Foundation")]
    pub GetPreviewPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    GetPreviewPage: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveGetPreviewPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveGetPreviewPage: usize,
    #[cfg(feature = "Foundation")]
    pub AddPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: ::windows::core::RawPtr, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    AddPages: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveAddPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveAddPages: usize,
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub AddPagesComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetPreviewPageCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: i32, r#type: PreviewPageCountType) -> ::windows::core::HRESULT,
    pub SetPreviewPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pagenumber: i32, pagevisual: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub InvalidatePreview: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocumentFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintDocumentFactory {
    type Vtable = IPrintDocumentFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb87b18f_2606_4a2f_99d4_a7cdbc35d7c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentFactory_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub CreateInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IPrintDocumentStatics(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IPrintDocumentStatics {
    type Vtable = IPrintDocumentStatics_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd970a3c_b152_49e0_a6bd_6aa6477e43c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPrintDocumentStatics_Vtbl {
    pub base: ::windows::core::IInspectableVtbl,
    pub DocumentSourceProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct PaginateEventArgs(::windows::core::IUnknown);
impl PaginateEventArgs {
    pub fn new() -> ::windows::core::Result<Self> {
        Self::IActivationFactory(|f| f.ActivateInstance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::core::IActivationFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PaginateEventArgs, ::windows::core::IActivationFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn PrintTaskOptions(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::PrintTaskOptions> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).PrintTaskOptions)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Printing::PrintTaskOptions>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn CurrentPreviewPageNumber(&self) -> ::windows::core::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CurrentPreviewPageNumber)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
}
impl ::core::clone::Clone for PaginateEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaginateEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaginateEventArgs {}
impl ::core::fmt::Debug for PaginateEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaginateEventArgs").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PaginateEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Printing.PaginateEventArgs;{ed945fd6-79ab-42b7-930a-3d6e09011d21})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PaginateEventArgs {
    type Vtable = IPaginateEventArgs_Vtbl;
    const IID: ::windows::core::GUID = <IPaginateEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PaginateEventArgs {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.PaginateEventArgs";
}
impl ::core::convert::From<PaginateEventArgs> for ::windows::core::IUnknown {
    fn from(value: PaginateEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaginateEventArgs> for ::windows::core::IUnknown {
    fn from(value: &PaginateEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PaginateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PaginateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PaginateEventArgs> for ::windows::core::IInspectable {
    fn from(value: PaginateEventArgs) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PaginateEventArgs> for ::windows::core::IInspectable {
    fn from(value: &PaginateEventArgs) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PaginateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PaginateEventArgs {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for PaginateEventArgs {}
unsafe impl ::core::marker::Sync for PaginateEventArgs {}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct PaginateEventHandler(pub ::windows::core::IUnknown);
impl PaginateEventHandler {
    pub fn new<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PaginateEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static>(invoke: F) -> Self {
        let com = PaginateEventHandlerBox::<F> { vtable: &PaginateEventHandlerBox::<F>::VTABLE, count: ::windows::core::RefCount::new(1), invoke };
        unsafe { ::core::mem::transmute(::windows::core::alloc::boxed::Box::new(com)) }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn Invoke<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IInspectable>, Param1: ::windows::core::IntoParam<'a, PaginateEventArgs>>(&self, sender: Param0, e: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).Invoke)(::core::mem::transmute_copy(this), sender.into_param().abi(), e.into_param().abi()).ok() }
    }
}
#[repr(C)]
struct PaginateEventHandlerBox<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PaginateEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> {
    vtable: *const PaginateEventHandler_Vtbl,
    invoke: F,
    count: ::windows::core::RefCount,
}
impl<F: FnMut(&::core::option::Option<::windows::core::IInspectable>, &::core::option::Option<PaginateEventArgs>) -> ::windows::core::Result<()> + ::core::marker::Send + 'static> PaginateEventHandlerBox<F> {
    const VTABLE: PaginateEventHandler_Vtbl = PaginateEventHandler_Vtbl { base: ::windows::core::IUnknownVtbl { QueryInterface: Self::QueryInterface, AddRef: Self::AddRef, Release: Self::Release }, Invoke: Self::Invoke };
    unsafe extern "system" fn QueryInterface(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
        let this = this as *mut ::windows::core::RawPtr as *mut Self;
        *interface = if iid == &<PaginateEventHandler as ::windows::core::Interface>::IID || iid == &<::windows::core::IUnknown as ::windows::core::Interface>::IID || iid == &<::windows::core::IAgileObject as ::windows::core::Interface>::IID { &mut (*this).vtable as *mut _ as _ } else { ::core::ptr::null_mut() };
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
impl ::core::clone::Clone for PaginateEventHandler {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PaginateEventHandler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PaginateEventHandler {}
impl ::core::fmt::Debug for PaginateEventHandler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PaginateEventHandler").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for PaginateEventHandler {
    type Vtable = PaginateEventHandler_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cc05b61_811b_4a32_9965_13eb78dbb01b);
}
unsafe impl ::windows::core::RuntimeType for PaginateEventHandler {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{0cc05b61-811b-4a32-9965-13eb78dbb01b}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct PaginateEventHandler_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Invoke: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sender: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PreviewPageCountType(pub i32);
impl PreviewPageCountType {
    pub const Final: Self = Self(0i32);
    pub const Intermediate: Self = Self(1i32);
}
impl ::core::marker::Copy for PreviewPageCountType {}
impl ::core::clone::Clone for PreviewPageCountType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PreviewPageCountType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PreviewPageCountType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PreviewPageCountType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PreviewPageCountType").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PreviewPageCountType {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.UI.Xaml.Printing.PreviewPageCountType;i4)");
    type DefaultType = Self;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        Ok(*from)
    }
}
#[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
#[repr(transparent)]
pub struct PrintDocument(::windows::core::IUnknown);
impl PrintDocument {
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Graphics_Printing\"`*"]
    #[cfg(feature = "Graphics_Printing")]
    pub fn DocumentSource(&self) -> ::windows::core::Result<super::super::super::Graphics::Printing::IPrintDocumentSource> {
        let this = self;
        unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentSource)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::super::Graphics::Printing::IPrintDocumentSource>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Paginate<'a, Param0: ::windows::core::IntoParam<'a, PaginateEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).Paginate)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePaginate<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePaginate)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetPreviewPage<'a, Param0: ::windows::core::IntoParam<'a, GetPreviewPageEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).GetPreviewPage)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveGetPreviewPage<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveGetPreviewPage)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn AddPages<'a, Param0: ::windows::core::IntoParam<'a, AddPagesEventHandler>>(&self, handler: Param0) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).AddPages)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`, `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveAddPages<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>>(&self, token: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveAddPages)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn AddPage<'a, Param0: ::windows::core::IntoParam<'a, super::UIElement>>(&self, pagevisual: Param0) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPage)(::core::mem::transmute_copy(this), pagevisual.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn AddPagesComplete(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).AddPagesComplete)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn SetPreviewPageCount(&self, count: i32, r#type: PreviewPageCountType) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreviewPageCount)(::core::mem::transmute_copy(this), count, r#type).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn SetPreviewPage<'a, Param1: ::windows::core::IntoParam<'a, super::UIElement>>(&self, pagenumber: i32, pagevisual: Param1) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).SetPreviewPage)(::core::mem::transmute_copy(this), pagenumber, pagevisual.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn InvalidatePreview(&self) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).InvalidatePreview)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn new() -> ::windows::core::Result<PrintDocument> {
        Self::IPrintDocumentFactory(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::ptr::null_mut(), &mut ::core::option::Option::<::windows::core::IInspectable>::None as *mut _ as _, &mut result__).from_abi::<PrintDocument>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn compose<T: ::windows::core::Compose>(compose: T) -> ::windows::core::Result<PrintDocument> {
        Self::IPrintDocumentFactory(|this| unsafe {
            let (derived__, base__) = ::windows::core::Compose::compose(compose);
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).CreateInstance)(::core::mem::transmute_copy(this), ::core::mem::transmute_copy(&derived__), base__ as *mut _ as _, &mut result__).from_abi::<PrintDocument>(result__)
        })
    }
    #[doc = "*Required features: `\"UI_Xaml_Printing\"`*"]
    pub fn DocumentSourceProperty() -> ::windows::core::Result<super::DependencyProperty> {
        Self::IPrintDocumentStatics(|this| unsafe {
            let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
            (::windows::core::Interface::vtable(this).DocumentSourceProperty)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::DependencyProperty>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IPrintDocumentFactory<R, F: FnOnce(&IPrintDocumentFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintDocument, IPrintDocumentFactory> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc(hidden)]
    pub fn IPrintDocumentStatics<R, F: FnOnce(&IPrintDocumentStatics) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static mut SHARED: ::windows::core::FactoryCache<PrintDocument, IPrintDocumentStatics> = ::windows::core::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::core::clone::Clone for PrintDocument {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for PrintDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for PrintDocument {}
impl ::core::fmt::Debug for PrintDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PrintDocument").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for PrintDocument {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.UI.Xaml.Printing.PrintDocument;{e44327c3-a999-485b-b1d8-72dc517821e6})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for PrintDocument {
    type Vtable = IPrintDocument_Vtbl;
    const IID: ::windows::core::GUID = <IPrintDocument as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for PrintDocument {
    const NAME: &'static str = "Windows.UI.Xaml.Printing.PrintDocument";
}
impl ::core::convert::From<PrintDocument> for ::windows::core::IUnknown {
    fn from(value: PrintDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintDocument> for ::windows::core::IUnknown {
    fn from(value: &PrintDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintDocument> for ::windows::core::IInspectable {
    fn from(value: PrintDocument) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&PrintDocument> for ::windows::core::IInspectable {
    fn from(value: &PrintDocument) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IInspectable> for &'a PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IInspectable> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<PrintDocument> for super::DependencyObject {
    fn from(value: PrintDocument) -> Self {
        ::core::convert::From::from(&value)
    }
}
impl ::core::convert::From<&PrintDocument> for super::DependencyObject {
    fn from(value: &PrintDocument) -> Self {
        ::windows::core::Interface::cast(value).unwrap()
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::DependencyObject> for &PrintDocument {
    fn into_param(self) -> ::windows::core::Param<'a, super::DependencyObject> {
        ::windows::core::Param::Owned(::core::convert::Into::<super::DependencyObject>::into(self))
    }
}
unsafe impl ::core::marker::Send for PrintDocument {}
unsafe impl ::core::marker::Sync for PrintDocument {}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
