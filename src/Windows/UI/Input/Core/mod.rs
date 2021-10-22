#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1029144310,
        19694,
        4582,
        [181, 53, 0, 27, 220, 6, 171, 59],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Core")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Core"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerIndependentInputSource2 {
    type Vtable = IRadialControllerIndependentInputSource2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1886628568,
        13811,
        20203,
        [135, 81, 190, 77, 10, 102, 250, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSource2_abi(
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
    #[cfg(feature = "System")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "System"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRadialControllerIndependentInputSourceStatics {
    type Vtable = IRadialControllerIndependentInputSourceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1029144309,
        19694,
        4582,
        [181, 53, 0, 27, 220, 6, 171, 59],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRadialControllerIndependentInputSourceStatics_abi(
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
    #[cfg(feature = "ApplicationModel_Core")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        view: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Core"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RadialControllerIndependentInputSource(::windows::runtime::IInspectable);
impl RadialControllerIndependentInputSource {
    pub fn Controller(&self) -> ::windows::runtime::Result<super::RadialController> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::RadialController>(result__)
        }
    }
    #[cfg(feature = "UI_Core")]
    pub fn Dispatcher(&self) -> ::windows::runtime::Result<super::super::Core::CoreDispatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Core::CoreDispatcher>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Core")]
    pub fn CreateForView<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::ApplicationModel::Core::CoreApplicationView,
        >,
    >(
        view: Param0,
    ) -> ::windows::runtime::Result<RadialControllerIndependentInputSource> {
        Self::IRadialControllerIndependentInputSourceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                view.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RadialControllerIndependentInputSource>(result__)
        })
    }
    #[cfg(feature = "System")]
    pub fn DispatcherQueue(
        &self,
    ) -> ::windows::runtime::Result<super::super::super::System::DispatcherQueue> {
        let this =
            &::windows::runtime::Interface::cast::<IRadialControllerIndependentInputSource2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::super::System::DispatcherQueue>(result__)
        }
    }
    pub fn IRadialControllerIndependentInputSourceStatics<
        R,
        F: FnOnce(&IRadialControllerIndependentInputSourceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RadialControllerIndependentInputSource,
            IRadialControllerIndependentInputSourceStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RadialControllerIndependentInputSource {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.Input.Core.RadialControllerIndependentInputSource;{3d577ef6-4cee-11e6-b535-001bdc06ab3b})" ) ;
}
unsafe impl ::windows::runtime::Interface for RadialControllerIndependentInputSource {
    type Vtable = IRadialControllerIndependentInputSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1029144310,
        19694,
        4582,
        [181, 53, 0, 27, 220, 6, 171, 59],
    );
}
impl ::windows::runtime::RuntimeName for RadialControllerIndependentInputSource {
    const NAME: &'static str = "Windows.UI.Input.Core.RadialControllerIndependentInputSource";
}
impl ::std::convert::From<RadialControllerIndependentInputSource> for ::windows::runtime::IUnknown {
    fn from(value: RadialControllerIndependentInputSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RadialControllerIndependentInputSource>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RadialControllerIndependentInputSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RadialControllerIndependentInputSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RadialControllerIndependentInputSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RadialControllerIndependentInputSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: RadialControllerIndependentInputSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RadialControllerIndependentInputSource>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RadialControllerIndependentInputSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RadialControllerIndependentInputSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RadialControllerIndependentInputSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RadialControllerIndependentInputSource {}
unsafe impl ::std::marker::Sync for RadialControllerIndependentInputSource {}
