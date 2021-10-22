#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "UI_WebUI_Core")]
pub mod Core;
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ActivatedDeferral(::windows::runtime::IInspectable);
impl ActivatedDeferral {
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ActivatedDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.ActivatedDeferral;{c3bd1978-a431-49d8-a76a-395a4e03dcf3})",
    );
}
unsafe impl ::windows::runtime::Interface for ActivatedDeferral {
    type Vtable = IActivatedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3283949944,
        42033,
        18904,
        [167, 106, 57, 90, 78, 3, 220, 243],
    );
}
impl ::windows::runtime::RuntimeName for ActivatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedDeferral";
}
impl ::std::convert::From<ActivatedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: ActivatedDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ActivatedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &ActivatedDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ActivatedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ActivatedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ActivatedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: ActivatedDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActivatedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &ActivatedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ActivatedDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ActivatedDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ActivatedEventHandler(::windows::runtime::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl ActivatedEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<
                    super::super::ApplicationModel::Activation::IActivatedEventArgs,
                >,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ActivatedEventHandler_box::<F> {
            vtable: &ActivatedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >,
    >(
        &self,
        sender: Param0,
        eventargs: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                eventargs.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for ActivatedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({50f1e730-c5d1-4b6b-9adb-8a11756be29c})",
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for ActivatedEventHandler {
    type Vtable = ActivatedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1358030640,
        50641,
        19307,
        [154, 219, 138, 17, 117, 107, 226, 156],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct ActivatedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        eventargs: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct ActivatedEventHandler_box<
    F: FnMut(
            &::std::option::Option<::windows::runtime::IInspectable>,
            &::std::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const ActivatedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<
                    super::super::ApplicationModel::Activation::IActivatedEventArgs,
                >,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > ActivatedEventHandler_box<F>
{
    const VTABLE: ActivatedEventHandler_abi = ActivatedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ActivatedEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        eventargs: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & eventargs as * const < super::super::ApplicationModel::Activation:: IActivatedEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < super::super::ApplicationModel::Activation:: IActivatedEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ActivatedOperation(::windows::runtime::IInspectable);
impl ActivatedOperation {
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<ActivatedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ActivatedOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.ActivatedOperation;{b6a0b4bc-c6ca-42fd-9818-71904e45fed7})",
    );
}
unsafe impl ::windows::runtime::Interface for ActivatedOperation {
    type Vtable = IActivatedOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3063985340,
        50890,
        17149,
        [152, 24, 113, 144, 78, 69, 254, 215],
    );
}
impl ::windows::runtime::RuntimeName for ActivatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.ActivatedOperation";
}
impl ::std::convert::From<ActivatedOperation> for ::windows::runtime::IUnknown {
    fn from(value: ActivatedOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ActivatedOperation> for ::windows::runtime::IUnknown {
    fn from(value: &ActivatedOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ActivatedOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &ActivatedOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<ActivatedOperation> for ::windows::runtime::IInspectable {
    fn from(value: ActivatedOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ActivatedOperation> for ::windows::runtime::IInspectable {
    fn from(value: &ActivatedOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for ActivatedOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a ActivatedOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Background"
    ))]
    pub fn TaskInstance(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Background::IBackgroundTaskInstance,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Background::IBackgroundTaskInstance>(
                result__,
            )
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for BackgroundActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.BackgroundActivatedEventArgs;{ab14bee0-e760-440e-a91c-44796de3a92d})",
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for BackgroundActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2870263520,
        59232,
        17422,
        [169, 28, 68, 121, 109, 227, 169, 45],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for BackgroundActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.BackgroundActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<BackgroundActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&BackgroundActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for BackgroundActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &BackgroundActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<BackgroundActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&BackgroundActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for BackgroundActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a BackgroundActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<BackgroundActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs
{
    fn from(value: BackgroundActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&BackgroundActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs
{
    fn from(value: &BackgroundActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
    > for BackgroundActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
    > for &BackgroundActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Send for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Sync for BackgroundActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct BackgroundActivatedEventHandler(::windows::runtime::IUnknown);
#[cfg(feature = "ApplicationModel_Activation")]
impl BackgroundActivatedEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<
                    super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
                >,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = BackgroundActivatedEventHandler_box::<F> {
            vtable: &BackgroundActivatedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
        >,
    >(
        &self,
        sender: Param0,
        eventargs: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                eventargs.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for BackgroundActivatedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({edb19fbb-0761-47cc-9a77-24d7072965ca})",
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for BackgroundActivatedEventHandler {
    type Vtable = BackgroundActivatedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3987840955,
        1889,
        18380,
        [154, 119, 36, 215, 7, 41, 101, 202],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
#[doc(hidden)]
pub struct BackgroundActivatedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        eventargs: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(C)]
struct BackgroundActivatedEventHandler_box<
    F: FnMut(
            &::std::option::Option<::windows::runtime::IInspectable>,
            &::std::option::Option<
                super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
            >,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const BackgroundActivatedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<
                    super::super::ApplicationModel::Activation::IBackgroundActivatedEventArgs,
                >,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > BackgroundActivatedEventHandler_box<F>
{
    const VTABLE: BackgroundActivatedEventHandler_abi = BackgroundActivatedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid
            == &<BackgroundActivatedEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        eventargs: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & eventargs as * const < super::super::ApplicationModel::Activation:: IBackgroundActivatedEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < super::super::ApplicationModel::Activation:: IBackgroundActivatedEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct EnteredBackgroundEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventArgs {
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::RuntimeType for EnteredBackgroundEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.EnteredBackgroundEventArgs;{f722dcc2-9827-403d-aaed-ecca9ac17398})",
    );
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::Interface for EnteredBackgroundEventArgs {
    type Vtable = super::super::ApplicationModel::IEnteredBackgroundEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4146257090,
        38951,
        16445,
        [170, 237, 236, 202, 154, 193, 115, 152],
    );
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::runtime::RuntimeName for EnteredBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.EnteredBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<EnteredBackgroundEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&EnteredBackgroundEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for EnteredBackgroundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &EnteredBackgroundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<EnteredBackgroundEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&EnteredBackgroundEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for EnteredBackgroundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a EnteredBackgroundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<EnteredBackgroundEventArgs>
    for super::super::ApplicationModel::IEnteredBackgroundEventArgs
{
    fn from(value: EnteredBackgroundEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&EnteredBackgroundEventArgs>
    for super::super::ApplicationModel::IEnteredBackgroundEventArgs
{
    fn from(value: &EnteredBackgroundEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::IEnteredBackgroundEventArgs>
    for EnteredBackgroundEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::IEnteredBackgroundEventArgs>
    {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::IEnteredBackgroundEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::IEnteredBackgroundEventArgs>
    for &EnteredBackgroundEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::IEnteredBackgroundEventArgs>
    {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::IEnteredBackgroundEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::std::marker::Send for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::std::marker::Sync for EnteredBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct EnteredBackgroundEventHandler(::windows::runtime::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl EnteredBackgroundEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<super::super::ApplicationModel::IEnteredBackgroundEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = EnteredBackgroundEventHandler_box::<F> {
            vtable: &EnteredBackgroundEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::ApplicationModel::IEnteredBackgroundEventArgs,
        >,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::RuntimeType for EnteredBackgroundEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({2b09a173-b68e-4def-88c1-8de84e5aab2f})",
    );
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::Interface for EnteredBackgroundEventHandler {
    type Vtable = EnteredBackgroundEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        722051443,
        46734,
        19951,
        [136, 193, 141, 232, 78, 90, 171, 47],
    );
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct EnteredBackgroundEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct EnteredBackgroundEventHandler_box<
    F: FnMut(
            &::std::option::Option<::windows::runtime::IInspectable>,
            &::std::option::Option<super::super::ApplicationModel::IEnteredBackgroundEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const EnteredBackgroundEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<super::super::ApplicationModel::IEnteredBackgroundEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > EnteredBackgroundEventHandler_box<F>
{
    const VTABLE: EnteredBackgroundEventHandler_abi = EnteredBackgroundEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid
            == &<EnteredBackgroundEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & e as * const < super::super::ApplicationModel:: IEnteredBackgroundEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < super::super::ApplicationModel:: IEnteredBackgroundEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct HtmlPrintDocumentSource(::windows::runtime::IInspectable);
impl HtmlPrintDocumentSource {
    pub fn Content(&self) -> ::windows::runtime::Result<PrintContent> {
        let this = self;
        unsafe {
            let mut result__: PrintContent = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<PrintContent>(result__)
        }
    }
    pub fn SetContent(&self, value: PrintContent) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn LeftMargin(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetLeftMargin(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    pub fn TopMargin(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetTopMargin(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn RightMargin(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetRightMargin(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn BottomMargin(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetBottomMargin(&self, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn EnableHeaderFooter(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetEnableHeaderFooter(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn ShrinkToFit(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).18)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetShrinkToFit(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    pub fn PercentScale(&self) -> ::windows::runtime::Result<f32> {
        let this = self;
        unsafe {
            let mut result__: f32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<f32>(result__)
        }
    }
    pub fn SetPercentScale(&self, scalepercent: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                scalepercent,
            )
            .ok()
        }
    }
    pub fn PageRange(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn TrySetPageRange<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        strpagerange: Param0,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                strpagerange.into_param().abi(),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for HtmlPrintDocumentSource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.HtmlPrintDocumentSource;{cea6469a-0e05-467a-abc9-36ec1d4cdcb6})",
    );
}
unsafe impl ::windows::runtime::Interface for HtmlPrintDocumentSource {
    type Vtable = IHtmlPrintDocumentSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3467003546,
        3589,
        18042,
        [171, 201, 54, 236, 29, 76, 220, 182],
    );
}
impl ::windows::runtime::RuntimeName for HtmlPrintDocumentSource {
    const NAME: &'static str = "Windows.UI.WebUI.HtmlPrintDocumentSource";
}
impl ::std::convert::From<HtmlPrintDocumentSource> for ::windows::runtime::IUnknown {
    fn from(value: HtmlPrintDocumentSource) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&HtmlPrintDocumentSource> for ::windows::runtime::IUnknown {
    fn from(value: &HtmlPrintDocumentSource) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for HtmlPrintDocumentSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &HtmlPrintDocumentSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<HtmlPrintDocumentSource> for ::windows::runtime::IInspectable {
    fn from(value: HtmlPrintDocumentSource) -> Self {
        value.0
    }
}
impl ::std::convert::From<&HtmlPrintDocumentSource> for ::windows::runtime::IInspectable {
    fn from(value: &HtmlPrintDocumentSource) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for HtmlPrintDocumentSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a HtmlPrintDocumentSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<HtmlPrintDocumentSource> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: HtmlPrintDocumentSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&HtmlPrintDocumentSource> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HtmlPrintDocumentSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for HtmlPrintDocumentSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &HtmlPrintDocumentSource
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::std::convert::TryFrom<HtmlPrintDocumentSource>
    for super::super::Graphics::Printing::IPrintDocumentSource
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: HtmlPrintDocumentSource) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl ::std::convert::TryFrom<&HtmlPrintDocumentSource>
    for super::super::Graphics::Printing::IPrintDocumentSource
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &HtmlPrintDocumentSource) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Graphics::Printing::IPrintDocumentSource>
    for HtmlPrintDocumentSource
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Graphics::Printing::IPrintDocumentSource> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Graphics_Printing")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Graphics::Printing::IPrintDocumentSource>
    for &HtmlPrintDocumentSource
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::Graphics::Printing::IPrintDocumentSource> {
        ::std::convert::TryInto::<super::super::Graphics::Printing::IPrintDocumentSource>::try_into(
            self,
        )
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for HtmlPrintDocumentSource {}
unsafe impl ::std::marker::Sync for HtmlPrintDocumentSource {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IActivatedDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivatedDeferral {
    type Vtable = IActivatedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3283949944,
        42033,
        18904,
        [167, 106, 57, 90, 78, 3, 220, 243],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedDeferral_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IActivatedEventArgsDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivatedEventArgsDeferral {
    type Vtable = IActivatedEventArgsDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3396165492,
        25538,
        17574,
        [185, 123, 217, 160, 60, 32, 188, 155],
    );
}
impl IActivatedEventArgsDeferral {
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IActivatedEventArgsDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{ca6d5f74-63c2-44a6-b97b-d9a03c20bc9b}");
}
impl ::std::convert::From<IActivatedEventArgsDeferral> for ::windows::runtime::IUnknown {
    fn from(value: IActivatedEventArgsDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IActivatedEventArgsDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &IActivatedEventArgsDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IActivatedEventArgsDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IActivatedEventArgsDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IActivatedEventArgsDeferral> for ::windows::runtime::IInspectable {
    fn from(value: IActivatedEventArgsDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IActivatedEventArgsDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &IActivatedEventArgsDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IActivatedEventArgsDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IActivatedEventArgsDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedEventArgsDeferral_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IActivatedOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IActivatedOperation {
    type Vtable = IActivatedOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3063985340,
        50890,
        17149,
        [152, 24, 113, 144, 78, 69, 254, 215],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IActivatedOperation_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IHtmlPrintDocumentSource(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IHtmlPrintDocumentSource {
    type Vtable = IHtmlPrintDocumentSource_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3467003546,
        3589,
        18042,
        [171, 201, 54, 236, 29, 76, 220, 182],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IHtmlPrintDocumentSource_abi(
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
        result__: *mut PrintContent,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: PrintContent,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        scalepercent: f32,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        strpagerange: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct INewWebUIViewCreatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for INewWebUIViewCreatedEventArgs {
    type Vtable = INewWebUIViewCreatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3907105302,
        48683,
        19614,
        [133, 231, 8, 49, 67, 236, 75, 231],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct INewWebUIViewCreatedEventArgs_abi(
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
    #[cfg(feature = "ApplicationModel_Activation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_Activation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUIActivationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUIActivationStatics {
    type Vtable = IWebUIActivationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        890996413,
        17331,
        18475,
        [133, 219, 53, 216, 123, 81, 122, 217],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics_abi(
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
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUIActivationStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUIActivationStatics2 {
    type Vtable = IWebUIActivationStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3370682006,
        19832,
        19108,
        [143, 6, 42, 158, 173, 198, 196, 10],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics2_abi(
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
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUIActivationStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUIActivationStatics3 {
    type Vtable = IWebUIActivationStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2443949702,
        6901,
        17477,
        [180, 159, 148, 89, 244, 15, 200, 222],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics3_abi(
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
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        launcharguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Core", feature = "Foundation")))] usize,
    #[cfg(all(
        feature = "ApplicationModel_Core",
        feature = "Foundation",
        feature = "System"
    ))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        user: ::windows::runtime::RawPtr,
        launcharguments: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(
        feature = "ApplicationModel_Core",
        feature = "Foundation",
        feature = "System"
    )))]
    usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUIActivationStatics4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUIActivationStatics4 {
    type Vtable = IWebUIActivationStatics4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1580799017,
        6207,
        18317,
        [138, 37, 103, 248, 13, 3, 147, 91],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIActivationStatics4_abi(
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
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWebUIBackgroundTaskInstance(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUIBackgroundTaskInstance {
    type Vtable = IWebUIBackgroundTaskInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        603008037,
        58103,
        18241,
        [188, 156, 57, 69, 149, 222, 36, 220],
    );
}
impl IWebUIBackgroundTaskInstance {
    pub fn Succeeded(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                succeeded,
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IWebUIBackgroundTaskInstance {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{23f12c25-e2f7-4741-bc9c-394595de24dc}");
}
impl ::std::convert::From<IWebUIBackgroundTaskInstance> for ::windows::runtime::IUnknown {
    fn from(value: IWebUIBackgroundTaskInstance) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebUIBackgroundTaskInstance> for ::windows::runtime::IUnknown {
    fn from(value: &IWebUIBackgroundTaskInstance) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWebUIBackgroundTaskInstance
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWebUIBackgroundTaskInstance
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IWebUIBackgroundTaskInstance> for ::windows::runtime::IInspectable {
    fn from(value: IWebUIBackgroundTaskInstance) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWebUIBackgroundTaskInstance> for ::windows::runtime::IInspectable {
    fn from(value: &IWebUIBackgroundTaskInstance) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IWebUIBackgroundTaskInstance
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IWebUIBackgroundTaskInstance
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstance_abi(
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
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        succeeded: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstanceStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUIBackgroundTaskInstanceStatics {
    type Vtable = IWebUIBackgroundTaskInstanceStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2625262225,
        6574,
        19619,
        [185, 75, 254, 78, 199, 68, 167, 64],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIBackgroundTaskInstanceStatics_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUINavigatedDeferral(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUINavigatedDeferral {
    type Vtable = IWebUINavigatedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3624149069,
        33567,
        18146,
        [180, 50, 58, 252, 226, 17, 249, 98],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedDeferral_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IWebUINavigatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUINavigatedEventArgs {
    type Vtable = IWebUINavigatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2807579064,
        9369,
        16432,
        [166, 157, 21, 210, 217, 207, 229, 36],
    );
}
impl IWebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> ::windows::runtime::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<WebUINavigatedOperation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IWebUINavigatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{a75841b8-2499-4030-a69d-15d2d9cfe524}");
}
impl ::std::convert::From<IWebUINavigatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: IWebUINavigatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IWebUINavigatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &IWebUINavigatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for IWebUINavigatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &IWebUINavigatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IWebUINavigatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: IWebUINavigatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IWebUINavigatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &IWebUINavigatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IWebUINavigatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IWebUINavigatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedEventArgs_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUINavigatedOperation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUINavigatedOperation {
    type Vtable = IWebUINavigatedOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2056675080,
        33154,
        19081,
        [171, 103, 132, 146, 232, 117, 13, 75],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUINavigatedOperation_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUIView(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUIView {
    type Vtable = IWebUIView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1736701519,
        21210,
        20439,
        [190, 105, 142, 246, 40, 75, 66, 60],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIView_abi(
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
        result__: *mut i32,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "ApplicationModel_Activation", feature = "Foundation")))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        value: bool,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IWebUIViewStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IWebUIViewStatics {
    type Vtable = IWebUIViewStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3046237800,
        36441,
        17657,
        [136, 3, 27, 36, 201, 20, 157, 48],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IWebUIViewStatics_abi(
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
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        uri: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct LeavingBackgroundEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventArgs {
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::RuntimeType for LeavingBackgroundEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.LeavingBackgroundEventArgs;{39c6ec9a-ae6e-46f9-a07a-cfc23f88733e})",
    );
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::Interface for LeavingBackgroundEventArgs {
    type Vtable = super::super::ApplicationModel::ILeavingBackgroundEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        969338010,
        44654,
        18169,
        [160, 122, 207, 194, 63, 136, 115, 62],
    );
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::runtime::RuntimeName for LeavingBackgroundEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.LeavingBackgroundEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<LeavingBackgroundEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&LeavingBackgroundEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for LeavingBackgroundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &LeavingBackgroundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<LeavingBackgroundEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&LeavingBackgroundEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for LeavingBackgroundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a LeavingBackgroundEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<LeavingBackgroundEventArgs>
    for super::super::ApplicationModel::ILeavingBackgroundEventArgs
{
    fn from(value: LeavingBackgroundEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&LeavingBackgroundEventArgs>
    for super::super::ApplicationModel::ILeavingBackgroundEventArgs
{
    fn from(value: &LeavingBackgroundEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ILeavingBackgroundEventArgs>
    for LeavingBackgroundEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::ILeavingBackgroundEventArgs>
    {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::ILeavingBackgroundEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a>
    ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ILeavingBackgroundEventArgs>
    for &LeavingBackgroundEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::ILeavingBackgroundEventArgs>
    {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::ILeavingBackgroundEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::std::marker::Send for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::std::marker::Sync for LeavingBackgroundEventArgs {}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct LeavingBackgroundEventHandler(::windows::runtime::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl LeavingBackgroundEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<super::super::ApplicationModel::ILeavingBackgroundEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = LeavingBackgroundEventHandler_box::<F> {
            vtable: &LeavingBackgroundEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::ApplicationModel::ILeavingBackgroundEventArgs,
        >,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::RuntimeType for LeavingBackgroundEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({00b4ccd9-7a9c-4b6b-9ac4-13474f268bc4})",
    );
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::Interface for LeavingBackgroundEventHandler {
    type Vtable = LeavingBackgroundEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        11848921,
        31388,
        19307,
        [154, 196, 19, 71, 79, 38, 139, 196],
    );
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct LeavingBackgroundEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct LeavingBackgroundEventHandler_box<
    F: FnMut(
            &::std::option::Option<::windows::runtime::IInspectable>,
            &::std::option::Option<super::super::ApplicationModel::ILeavingBackgroundEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const LeavingBackgroundEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<super::super::ApplicationModel::ILeavingBackgroundEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > LeavingBackgroundEventHandler_box<F>
{
    const VTABLE: LeavingBackgroundEventHandler_abi = LeavingBackgroundEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid
            == &<LeavingBackgroundEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & e as * const < super::super::ApplicationModel:: ILeavingBackgroundEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < super::super::ApplicationModel:: ILeavingBackgroundEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct NavigatedEventHandler(::windows::runtime::IUnknown);
impl NavigatedEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<IWebUINavigatedEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = NavigatedEventHandler_box::<F> {
            vtable: &NavigatedEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, IWebUINavigatedEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NavigatedEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({7af46fe6-40ca-4e49-a7d6-dbdb330cd1a3})",
    );
}
unsafe impl ::windows::runtime::Interface for NavigatedEventHandler {
    type Vtable = NavigatedEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2062839782,
        16586,
        20041,
        [167, 214, 219, 219, 51, 12, 209, 163],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct NavigatedEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct NavigatedEventHandler_box<
    F: FnMut(
            &::std::option::Option<::windows::runtime::IInspectable>,
            &::std::option::Option<IWebUINavigatedEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const NavigatedEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<IWebUINavigatedEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > NavigatedEventHandler_box<F>
{
    const VTABLE: NavigatedEventHandler_abi = NavigatedEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<NavigatedEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & e as * const < IWebUINavigatedEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < IWebUINavigatedEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct NewWebUIViewCreatedEventArgs(::windows::runtime::IInspectable);
impl NewWebUIViewCreatedEventArgs {
    pub fn WebUIView(&self) -> ::windows::runtime::Result<WebUIView> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<WebUIView>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ActivatedEventArgs(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::IActivatedEventArgs>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::IActivatedEventArgs>(result__)
        }
    }
    pub fn HasPendingNavigate(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for NewWebUIViewCreatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.NewWebUIViewCreatedEventArgs;{e8e1b216-be2b-4c9e-85e7-083143ec4be7})",
    );
}
unsafe impl ::windows::runtime::Interface for NewWebUIViewCreatedEventArgs {
    type Vtable = INewWebUIViewCreatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3907105302,
        48683,
        19614,
        [133, 231, 8, 49, 67, 236, 75, 231],
    );
}
impl ::windows::runtime::RuntimeName for NewWebUIViewCreatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.NewWebUIViewCreatedEventArgs";
}
impl ::std::convert::From<NewWebUIViewCreatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: NewWebUIViewCreatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&NewWebUIViewCreatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &NewWebUIViewCreatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for NewWebUIViewCreatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &NewWebUIViewCreatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<NewWebUIViewCreatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: NewWebUIViewCreatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&NewWebUIViewCreatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &NewWebUIViewCreatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for NewWebUIViewCreatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a NewWebUIViewCreatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct PrintContent(pub i32);
impl PrintContent {
    pub const AllPages: PrintContent = PrintContent(0i32);
    pub const CurrentPage: PrintContent = PrintContent(1i32);
    pub const CustomPageRange: PrintContent = PrintContent(2i32);
    pub const CurrentSelection: PrintContent = PrintContent(3i32);
}
impl ::std::convert::From<i32> for PrintContent {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PrintContent {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for PrintContent {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.UI.WebUI.PrintContent;i4)");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct ResumingEventHandler(::windows::runtime::IUnknown);
impl ResumingEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = ResumingEventHandler_box::<F> {
            vtable: &ResumingEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
    >(
        &self,
        sender: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ResumingEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({26599ba9-a22d-4806-a728-acadc1d075fa})",
    );
}
unsafe impl ::windows::runtime::Interface for ResumingEventHandler {
    type Vtable = ResumingEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        643406761,
        41517,
        18438,
        [167, 40, 172, 173, 193, 208, 117, 250],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ResumingEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[repr(C)]
struct ResumingEventHandler_box<
    F: FnMut(
            &::std::option::Option<::windows::runtime::IInspectable>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const ResumingEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
impl<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > ResumingEventHandler_box<F>
{
    const VTABLE: ResumingEventHandler_abi = ResumingEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<ResumingEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SuspendingDeferral(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl SuspendingDeferral {
    #[cfg(feature = "ApplicationModel")]
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::RuntimeType for SuspendingDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.SuspendingDeferral;{59140509-8bc9-4eb4-b636-dabdc4f46f66})",
    );
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::Interface for SuspendingDeferral {
    type Vtable = super::super::ApplicationModel::ISuspendingDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1494484233,
        35785,
        20148,
        [182, 54, 218, 189, 196, 244, 111, 102],
    );
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::runtime::RuntimeName for SuspendingDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingDeferral";
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingDeferral> for ::windows::runtime::IUnknown {
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &SuspendingDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SuspendingDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SuspendingDeferral {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingDeferral> for ::windows::runtime::IInspectable {
    fn from(value: SuspendingDeferral) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &SuspendingDeferral) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SuspendingDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SuspendingDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingDeferral>
    for super::super::ApplicationModel::ISuspendingDeferral
{
    fn from(value: SuspendingDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingDeferral>
    for super::super::ApplicationModel::ISuspendingDeferral
{
    fn from(value: &SuspendingDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ISuspendingDeferral>
    for SuspendingDeferral
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::ISuspendingDeferral> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::ISuspendingDeferral,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ISuspendingDeferral>
    for &SuspendingDeferral
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::ISuspendingDeferral> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::ISuspendingDeferral,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SuspendingEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventArgs {
    #[cfg(feature = "ApplicationModel")]
    pub fn SuspendingOperation(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::SuspendingOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::SuspendingOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::RuntimeType for SuspendingEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.SuspendingEventArgs;{96061c05-2dba-4d08-b0bd-2b30a131c6aa})",
    );
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::Interface for SuspendingEventArgs {
    type Vtable = super::super::ApplicationModel::ISuspendingEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2516982789,
        11706,
        19720,
        [176, 189, 43, 48, 161, 49, 198, 170],
    );
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::runtime::RuntimeName for SuspendingEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingEventArgs";
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &SuspendingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SuspendingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SuspendingEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: SuspendingEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &SuspendingEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SuspendingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SuspendingEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingEventArgs>
    for super::super::ApplicationModel::ISuspendingEventArgs
{
    fn from(value: SuspendingEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingEventArgs>
    for super::super::ApplicationModel::ISuspendingEventArgs
{
    fn from(value: &SuspendingEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ISuspendingEventArgs>
    for SuspendingEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::ISuspendingEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::ISuspendingEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ISuspendingEventArgs>
    for &SuspendingEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::ISuspendingEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::ISuspendingEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SuspendingEventHandler(::windows::runtime::IUnknown);
#[cfg(feature = "ApplicationModel")]
impl SuspendingEventHandler {
    pub fn new<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<super::super::ApplicationModel::ISuspendingEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    >(
        invoke: F,
    ) -> Self {
        let com = SuspendingEventHandler_box::<F> {
            vtable: &SuspendingEventHandler_box::<F>::VTABLE,
            count: ::windows::runtime::RefCount::new(1),
            invoke,
        };
        unsafe { std::mem::transmute(::std::boxed::Box::new(com)) }
    }
    #[cfg(feature = "ApplicationModel")]
    pub fn Invoke<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ISuspendingEventArgs>,
    >(
        &self,
        sender: Param0,
        e: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).3)(
                ::std::mem::transmute_copy(this),
                sender.into_param().abi(),
                e.into_param().abi(),
            )
            .ok()
        }
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::RuntimeType for SuspendingEventHandler {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"delegate({509c429c-78e2-4883-abc8-8960dcde1b5c})",
    );
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::Interface for SuspendingEventHandler {
    type Vtable = SuspendingEventHandler_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1352417948,
        30946,
        18563,
        [171, 200, 137, 96, 220, 222, 27, 92],
    );
}
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
#[doc(hidden)]
pub struct SuspendingEventHandler_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
);
#[cfg(feature = "ApplicationModel")]
#[repr(C)]
struct SuspendingEventHandler_box<
    F: FnMut(
            &::std::option::Option<::windows::runtime::IInspectable>,
            &::std::option::Option<super::super::ApplicationModel::ISuspendingEventArgs>,
        ) -> ::windows::runtime::Result<()>
        + 'static,
> {
    vtable: *const SuspendingEventHandler_abi,
    invoke: F,
    count: ::windows::runtime::RefCount,
}
#[cfg(feature = "ApplicationModel")]
impl<
        F: FnMut(
                &::std::option::Option<::windows::runtime::IInspectable>,
                &::std::option::Option<super::super::ApplicationModel::ISuspendingEventArgs>,
            ) -> ::windows::runtime::Result<()>
            + 'static,
    > SuspendingEventHandler_box<F>
{
    const VTABLE: SuspendingEventHandler_abi = SuspendingEventHandler_abi(
        Self::QueryInterface,
        Self::AddRef,
        Self::Release,
        Self::Invoke,
    );
    unsafe extern "system" fn QueryInterface(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        *interface = if iid == &<SuspendingEventHandler as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IUnknown as ::windows::runtime::Interface>::IID
            || iid == &<::windows::runtime::IAgileObject as ::windows::runtime::Interface>::IID
        {
            &mut (*this).vtable as *mut _ as _
        } else {
            ::std::ptr::null_mut()
        };
        if (*interface).is_null() {
            ::windows::runtime::HRESULT(0x8000_4002)
        } else {
            (*this).count.add_ref();
            ::windows::runtime::HRESULT(0)
        }
    }
    unsafe extern "system" fn AddRef(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        (*this).count.add_ref()
    }
    unsafe extern "system" fn Release(this: ::windows::runtime::RawPtr) -> u32 {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        let remaining = (*this).count.release();
        if remaining == 0 {
            Box::from_raw(this);
        }
        remaining
    }
    unsafe extern "system" fn Invoke(
        this: ::windows::runtime::RawPtr,
        sender: ::windows::runtime::RawPtr,
        e: ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT {
        let this = this as *mut ::windows::runtime::RawPtr as *mut Self;
        ( ( * this ) . invoke ) ( & * ( & sender as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: Abi as * const < :: windows :: runtime :: IInspectable as :: windows :: runtime :: Abi > :: DefaultType ) , & * ( & e as * const < super::super::ApplicationModel:: ISuspendingEventArgs as :: windows :: runtime :: Abi > :: Abi as * const < super::super::ApplicationModel:: ISuspendingEventArgs as :: windows :: runtime :: Abi > :: DefaultType ) , ) . into ( )
    }
}
#[cfg(feature = "ApplicationModel")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct SuspendingOperation(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel")]
impl SuspendingOperation {
    #[cfg(feature = "ApplicationModel")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::SuspendingDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::SuspendingDeferral>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn Deadline(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::RuntimeType for SuspendingOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.SuspendingOperation;{9da4ca41-20e1-4e9b-9f65-a9f435340c3a})",
    );
}
#[cfg(feature = "ApplicationModel")]
unsafe impl ::windows::runtime::Interface for SuspendingOperation {
    type Vtable = super::super::ApplicationModel::ISuspendingOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2644822593,
        8417,
        20123,
        [159, 101, 169, 244, 53, 52, 12, 58],
    );
}
#[cfg(feature = "ApplicationModel")]
impl ::windows::runtime::RuntimeName for SuspendingOperation {
    const NAME: &'static str = "Windows.UI.WebUI.SuspendingOperation";
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingOperation> for ::windows::runtime::IUnknown {
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingOperation> for ::windows::runtime::IUnknown {
    fn from(value: &SuspendingOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for SuspendingOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &SuspendingOperation {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingOperation> for ::windows::runtime::IInspectable {
    fn from(value: SuspendingOperation) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingOperation> for ::windows::runtime::IInspectable {
    fn from(value: &SuspendingOperation) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for SuspendingOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a SuspendingOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<SuspendingOperation>
    for super::super::ApplicationModel::ISuspendingOperation
{
    fn from(value: SuspendingOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel")]
impl ::std::convert::From<&SuspendingOperation>
    for super::super::ApplicationModel::ISuspendingOperation
{
    fn from(value: &SuspendingOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ISuspendingOperation>
    for SuspendingOperation
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::ISuspendingOperation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::ISuspendingOperation,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::ApplicationModel::ISuspendingOperation>
    for &SuspendingOperation
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<'a, super::super::ApplicationModel::ISuspendingOperation> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::ISuspendingOperation,
        >::into(::std::clone::Clone::clone(self)))
    }
}
pub struct WebUIApplication {}
impl WebUIApplication {
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated<'a, Param0: ::windows::runtime::IntoParam<'a, ActivatedEventHandler>>(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn Suspending<'a, Param0: ::windows::runtime::IntoParam<'a, SuspendingEventHandler>>(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSuspending<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Resuming<'a, Param0: ::windows::runtime::IntoParam<'a, ResumingEventHandler>>(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveResuming<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn Navigated<'a, Param0: ::windows::runtime::IntoParam<'a, NavigatedEventHandler>>(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNavigated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn LeavingBackground<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, LeavingBackgroundEventHandler>,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveLeavingBackground<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(feature = "ApplicationModel", feature = "Foundation"))]
    pub fn EnteredBackground<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, EnteredBackgroundEventHandler>,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics2(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnteredBackground<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn EnablePrelaunch(value: bool) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics2(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        })
    }
    #[cfg(all(feature = "ApplicationModel_Core", feature = "Foundation"))]
    pub fn RequestRestartAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        launcharguments: Param0,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<
            super::super::ApplicationModel::Core::AppRestartFailureReason,
        >,
    > {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                launcharguments.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::super::ApplicationModel::Core::AppRestartFailureReason,
            >>(result__)
        })
    }
    #[cfg(all(
        feature = "ApplicationModel_Core",
        feature = "Foundation",
        feature = "System"
    ))]
    pub fn RequestRestartForUserAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::System::User>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        user: Param0,
        launcharguments: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<
            super::super::ApplicationModel::Core::AppRestartFailureReason,
        >,
    > {
        Self::IWebUIActivationStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                user.into_param().abi(),
                launcharguments.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::super::ApplicationModel::Core::AppRestartFailureReason,
            >>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn NewWebUIViewCreated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::EventHandler<NewWebUIViewCreatedEventArgs>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveNewWebUIViewCreated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn BackgroundActivated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, BackgroundActivatedEventHandler>,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        Self::IWebUIActivationStatics4(|this| unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveBackgroundActivated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::IWebUIActivationStatics4(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn IWebUIActivationStatics<
        R,
        F: FnOnce(&IWebUIActivationStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            WebUIApplication,
            IWebUIActivationStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebUIActivationStatics2<
        R,
        F: FnOnce(&IWebUIActivationStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            WebUIApplication,
            IWebUIActivationStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebUIActivationStatics3<
        R,
        F: FnOnce(&IWebUIActivationStatics3) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            WebUIApplication,
            IWebUIActivationStatics3,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IWebUIActivationStatics4<
        R,
        F: FnOnce(&IWebUIActivationStatics4) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            WebUIApplication,
            IWebUIActivationStatics4,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for WebUIApplication {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIApplication";
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIAppointmentsProviderAddAppointmentActivatedEventArgs(
    ::windows::runtime::IInspectable,
);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Appointments_AppointmentsProvider"
    ))]
    pub fn AddAppointmentOperation(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Appointments::AppointmentsProvider::AddAppointmentOperation,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::ApplicationModel::Appointments::AppointmentsProvider:: AddAppointmentOperation > ( result__ )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType
    for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs;{a2861367-cee5-4e4d-9ed7-41c34ec18b02})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface
    for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    type Vtable = super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs_abi ;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2726695783,
        52965,
        20045,
        [158, 215, 65, 195, 78, 193, 139, 2],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs {
    const NAME: &'static str =
        "Windows.UI.WebUI.WebUIAppointmentsProviderAddAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < WebUIAppointmentsProviderAddAppointmentActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs { fn from ( value : WebUIAppointmentsProviderAddAppointmentActivatedEventArgs ) -> Self { unsafe { :: std :: mem :: transmute ( value ) } } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < & WebUIAppointmentsProviderAddAppointmentActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs { fn from ( value : & WebUIAppointmentsProviderAddAppointmentActivatedEventArgs ) -> Self { :: std :: convert :: From :: from ( :: std :: clone :: Clone :: clone ( value ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs > for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs > :: into ( self ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs > for & WebUIAppointmentsProviderAddAppointmentActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderAddAppointmentActivatedEventArgs > :: into ( :: std :: clone :: Clone :: clone ( self ) ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderAddAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderAddAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIAppointmentsProviderAddAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs(
    ::windows::runtime::IInspectable,
);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Appointments_AppointmentsProvider"
    ))]    pub fn RemoveAppointmentOperation < > ( & self , ) -> :: windows :: runtime :: Result < super::super::ApplicationModel::Appointments::AppointmentsProvider:: RemoveAppointmentOperation >{
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::ApplicationModel::Appointments::AppointmentsProvider:: RemoveAppointmentOperation > ( result__ )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType
    for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs;{751f3ab8-0b8e-451c-9f15-966e699bac25})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface
    for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    type Vtable = super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs_abi ;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1964980920,
        2958,
        17692,
        [159, 21, 150, 110, 105, 155, 172, 37],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName
    for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    const NAME: &'static str =
        "Windows.UI.WebUI.WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs { fn from ( value : WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs ) -> Self { unsafe { :: std :: mem :: transmute ( value ) } } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < & WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs { fn from ( value : & WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs ) -> Self { :: std :: convert :: From :: from ( :: std :: clone :: Clone :: clone ( value ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs > for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs > :: into ( self ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs > for & WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderRemoveAppointmentActivatedEventArgs > :: into ( :: std :: clone :: Clone :: clone ( self ) ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIAppointmentsProviderRemoveAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs(
    ::windows::runtime::IInspectable,
);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Appointments_AppointmentsProvider"
    ))]    pub fn ReplaceAppointmentOperation < > ( & self , ) -> :: windows :: runtime :: Result < super::super::ApplicationModel::Appointments::AppointmentsProvider:: ReplaceAppointmentOperation >{
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::ApplicationModel::Appointments::AppointmentsProvider:: ReplaceAppointmentOperation > ( result__ )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType
    for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs;{1551b7d4-a981-4067-8a62-0524e4ade121})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface
    for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    type Vtable = super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs_abi ;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        357677012,
        43393,
        16487,
        [138, 98, 5, 36, 228, 173, 225, 33],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName
    for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    const NAME: &'static str =
        "Windows.UI.WebUI.WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs { fn from ( value : WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs ) -> Self { unsafe { :: std :: mem :: transmute ( value ) } } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < & WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs { fn from ( value : & WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs ) -> Self { :: std :: convert :: From :: from ( :: std :: clone :: Clone :: clone ( value ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs > for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs > :: into ( self ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs > for & WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderReplaceAppointmentActivatedEventArgs > :: into ( :: std :: clone :: Clone :: clone ( self ) ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIAppointmentsProviderReplaceAppointmentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs(
    ::windows::runtime::IInspectable,
);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs {
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn InstanceStartDate(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IReference<super::super::Foundation::DateTime>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IReference<super::super::Foundation::DateTime>>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn LocalId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn RoamingId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType
    for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs;{3958f065-9841-4ca5-999b-885198b9ef2a})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface
    for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    type Vtable = super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs_abi ;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        962130021,
        38977,
        19621,
        [153, 155, 136, 81, 152, 185, 239, 42],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName
    for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    const NAME: &'static str =
        "Windows.UI.WebUI.WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs { fn from ( value : WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs ) -> Self { unsafe { :: std :: mem :: transmute ( value ) } } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < & WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs { fn from ( value : & WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs ) -> Self { :: std :: convert :: From :: from ( :: std :: clone :: Clone :: clone ( value ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs > for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs > :: into ( self ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs > for & WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderShowAppointmentDetailsActivatedEventArgs > :: into ( :: std :: clone :: Clone :: clone ( self ) ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIAppointmentsProviderShowAppointmentDetailsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs(
    ::windows::runtime::IInspectable,
);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn TimeToShow(&self) -> ::windows::runtime::Result<super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::DateTime = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Duration(&self) -> ::windows::runtime::Result<super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::TimeSpan = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::TimeSpan>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType
    for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs;{9baeaba6-0e0b-49aa-babc-12b1dc774986})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface
    for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    type Vtable = super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs_abi ;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2611915686,
        3595,
        18858,
        [186, 188, 18, 177, 220, 119, 73, 134],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs {
    const NAME: &'static str =
        "Windows.UI.WebUI.WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs { fn from ( value : WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs ) -> Self { unsafe { :: std :: mem :: transmute ( value ) } } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: From < & WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs > for super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs { fn from ( value : & WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs ) -> Self { :: std :: convert :: From :: from ( :: std :: clone :: Clone :: clone ( value ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs > for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs > :: into ( self ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs > for & WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs > { :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IAppointmentsProviderShowTimeFrameActivatedEventArgs > :: into ( :: std :: clone :: Clone :: clone ( self ) ) ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IAppointmentsProviderActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIAppointmentsProviderShowTimeFrameActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
pub struct WebUIBackgroundTaskInstance {}
impl WebUIBackgroundTaskInstance {
    pub fn Current() -> ::windows::runtime::Result<IWebUIBackgroundTaskInstance> {
        Self::IWebUIBackgroundTaskInstanceStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<IWebUIBackgroundTaskInstance>(result__)
        })
    }
    pub fn IWebUIBackgroundTaskInstanceStatics<
        R,
        F: FnOnce(&IWebUIBackgroundTaskInstanceStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            WebUIBackgroundTaskInstance,
            IWebUIBackgroundTaskInstanceStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for WebUIBackgroundTaskInstance {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstance";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIBackgroundTaskInstanceRuntimeClass(::windows::runtime::IInspectable);
impl WebUIBackgroundTaskInstanceRuntimeClass {
    pub fn Succeeded(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetSucceeded(&self, succeeded: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                succeeded,
            )
            .ok()
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn InstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Task(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Background::BackgroundTaskRegistration,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Background::BackgroundTaskRegistration>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn Progress(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn SetProgress(&self, value: u32) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn TriggerDetails(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation"))]
    pub fn Canceled<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::ApplicationModel::Background::BackgroundTaskCanceledEventHandler,
        >,
    >(
        &self,
        cancelhandler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                cancelhandler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Background", feature = "Foundation"))]
    pub fn RemoveCanceled<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        cookie: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                cookie.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn SuspendedCount(&self) -> ::windows::runtime::Result<u32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            let mut result__: u32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<u32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Background")]
    pub fn GetDeferral(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Background::BackgroundTaskDeferral,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Background::IBackgroundTaskInstance,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Background::BackgroundTaskDeferral>(
                result__,
            )
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUIBackgroundTaskInstanceRuntimeClass {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass;{23f12c25-e2f7-4741-bc9c-394595de24dc})" ) ;
}
unsafe impl ::windows::runtime::Interface for WebUIBackgroundTaskInstanceRuntimeClass {
    type Vtable = IWebUIBackgroundTaskInstance_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        603008037,
        58103,
        18241,
        [188, 156, 57, 69, 149, 222, 36, 220],
    );
}
impl ::windows::runtime::RuntimeName for WebUIBackgroundTaskInstanceRuntimeClass {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBackgroundTaskInstanceRuntimeClass";
}
impl ::std::convert::From<WebUIBackgroundTaskInstanceRuntimeClass>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIBackgroundTaskInstanceRuntimeClass
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIBackgroundTaskInstanceRuntimeClass
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<WebUIBackgroundTaskInstanceRuntimeClass>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIBackgroundTaskInstanceRuntimeClass
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIBackgroundTaskInstanceRuntimeClass
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<WebUIBackgroundTaskInstanceRuntimeClass>
    for IWebUIBackgroundTaskInstance
{
    fn from(value: WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebUIBackgroundTaskInstanceRuntimeClass>
    for IWebUIBackgroundTaskInstance
{
    fn from(value: &WebUIBackgroundTaskInstanceRuntimeClass) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUIBackgroundTaskInstance>
    for WebUIBackgroundTaskInstanceRuntimeClass
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUIBackgroundTaskInstance> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<IWebUIBackgroundTaskInstance>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUIBackgroundTaskInstance>
    for &WebUIBackgroundTaskInstanceRuntimeClass
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUIBackgroundTaskInstance> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<IWebUIBackgroundTaskInstance>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::std::convert::TryFrom<WebUIBackgroundTaskInstanceRuntimeClass>
    for super::super::ApplicationModel::Background::IBackgroundTaskInstance
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIBackgroundTaskInstanceRuntimeClass,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl ::std::convert::TryFrom<&WebUIBackgroundTaskInstanceRuntimeClass>
    for super::super::ApplicationModel::Background::IBackgroundTaskInstance
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIBackgroundTaskInstanceRuntimeClass,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Background::IBackgroundTaskInstance,
    > for WebUIBackgroundTaskInstanceRuntimeClass
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Background::IBackgroundTaskInstance,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Background")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Background::IBackgroundTaskInstance,
    > for &WebUIBackgroundTaskInstanceRuntimeClass
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Background::IBackgroundTaskInstance,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Background:: IBackgroundTaskInstance > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIBarcodeScannerPreviewActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIBarcodeScannerPreviewActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ConnectionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs;{6772797c-99bf-4349-af22-e4123560371c})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIBarcodeScannerPreviewActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1735555452,
        39359,
        17225,
        [175, 34, 228, 18, 53, 96, 55, 28],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIBarcodeScannerPreviewActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIBarcodeScannerPreviewActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIBarcodeScannerPreviewActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIBarcodeScannerPreviewActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIBarcodeScannerPreviewActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIBarcodeScannerPreviewActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIBarcodeScannerPreviewActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIBarcodeScannerPreviewActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIBarcodeScannerPreviewActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIBarcodeScannerPreviewActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIBarcodeScannerPreviewActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIBarcodeScannerPreviewActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIBarcodeScannerPreviewActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs
{
    fn from(value: WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIBarcodeScannerPreviewActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs
{
    fn from(value: &WebUIBarcodeScannerPreviewActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs,
    > for WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs,
    > for &WebUIBarcodeScannerPreviewActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IBarcodeScannerPreviewActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Send for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Sync for WebUIBarcodeScannerPreviewActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUICachedFileUpdaterActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICachedFileUpdaterActivatedEventArgs {
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Provider"))]
    pub fn CachedFileUpdaterUI(
        &self,
    ) -> ::windows::runtime::Result<super::super::Storage::Provider::CachedFileUpdaterUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Provider::CachedFileUpdaterUI>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUICachedFileUpdaterActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs;{d06eb1c7-3805-4ecb-b757-6cf15e26fef3})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUICachedFileUpdaterActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3496915399,
        14341,
        20171,
        [183, 87, 108, 241, 94, 38, 254, 243],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUICachedFileUpdaterActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICachedFileUpdaterActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICachedFileUpdaterActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICachedFileUpdaterActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICachedFileUpdaterActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICachedFileUpdaterActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICachedFileUpdaterActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs
{
    fn from(value: WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICachedFileUpdaterActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs
{
    fn from(value: &WebUICachedFileUpdaterActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs,
    > for WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs,
    > for &WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ICachedFileUpdaterActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUICachedFileUpdaterActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUICachedFileUpdaterActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUICachedFileUpdaterActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUICachedFileUpdaterActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUICachedFileUpdaterActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUICachedFileUpdaterActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUICachedFileUpdaterActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUICachedFileUpdaterActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUICachedFileUpdaterActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUICameraSettingsActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICameraSettingsActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn VideoDeviceController(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn VideoDeviceExtension(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUICameraSettingsActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs;{fb67a508-2dad-490a-9170-dca036eb114b})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUICameraSettingsActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4217873672,
        11693,
        18698,
        [145, 112, 220, 160, 54, 235, 17, 75],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUICameraSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICameraSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICameraSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICameraSettingsActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUICameraSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUICameraSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICameraSettingsActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUICameraSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICameraSettingsActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUICameraSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUICameraSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICameraSettingsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs
{
    fn from(value: WebUICameraSettingsActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICameraSettingsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs
{
    fn from(value: &WebUICameraSettingsActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs,
    > for WebUICameraSettingsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs,
    > for &WebUICameraSettingsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ICameraSettingsActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUICameraSettingsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICameraSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUICameraSettingsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUICameraSettingsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUICameraSettingsActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICameraSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUICameraSettingsActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICameraSettingsActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUICameraSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUICameraSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUICommandLineActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUICommandLineActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Operation(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::CommandLineActivationOperation,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::CommandLineActivationOperation>(
                result__,
            )
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUICommandLineActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUICommandLineActivatedEventArgs;{4506472c-006a-48eb-8afb-d07ab25e3366})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUICommandLineActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1158039340,
        106,
        18667,
        [138, 251, 208, 122, 178, 94, 51, 102],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUICommandLineActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUICommandLineActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICommandLineActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUICommandLineActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICommandLineActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUICommandLineActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUICommandLineActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICommandLineActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUICommandLineActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICommandLineActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUICommandLineActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUICommandLineActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUICommandLineActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUICommandLineActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUICommandLineActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUICommandLineActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUICommandLineActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUICommandLineActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUICommandLineActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUICommandLineActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUICommandLineActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUICommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUICommandLineActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUICommandLineActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUICommandLineActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUICommandLineActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUICommandLineActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs
{
    fn from(value: WebUICommandLineActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUICommandLineActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs
{
    fn from(value: &WebUICommandLineActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs,
    > for WebUICommandLineActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs,
    > for &WebUICommandLineActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ICommandLineActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Send for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Sync for WebUICommandLineActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIContactCallActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts"
    ))]
    pub fn Contact(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIContactCallActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIContactCallActivatedEventArgs;{c2df14c7-30eb-41c6-b3bc-5b1694f9dab3})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIContactCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3269399751,
        12523,
        16838,
        [179, 188, 91, 22, 148, 249, 218, 179],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIContactCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIContactCallActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIContactCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIContactCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIContactCallActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactCallActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIContactCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIContactCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs
{
    fn from(value: WebUIContactCallActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs
{
    fn from(value: &WebUIContactCallActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs,
    > for WebUIContactCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs,
    > for &WebUIContactCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactCallActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIContactCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIContactCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for WebUIContactCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for &WebUIContactCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIContactCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIContactCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIContactMapActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMapActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts"
    ))]
    pub fn Address(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Contacts::ContactAddress> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::ContactAddress>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts"
    ))]
    pub fn Contact(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIContactMapActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIContactMapActivatedEventArgs;{b32bf870-eee7-4ad2-aaf1-a87effcf00a4})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIContactMapActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3006003312,
        61159,
        19154,
        [170, 241, 168, 126, 255, 207, 0, 164],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIContactMapActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMapActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactMapActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIContactMapActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactMapActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIContactMapActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIContactMapActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactMapActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIContactMapActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactMapActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIContactMapActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIContactMapActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactMapActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs
{
    fn from(value: WebUIContactMapActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactMapActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs
{
    fn from(value: &WebUIContactMapActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs,
    > for WebUIContactMapActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs,
    > for &WebUIContactMapActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactMapActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactMapActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactMapActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIContactMapActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIContactMapActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactMapActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactMapActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for WebUIContactMapActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for &WebUIContactMapActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactMapActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactMapActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactMapActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIContactMapActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIContactMapActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIContactMessageActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactMessageActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts"
    ))]
    pub fn Contact(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIContactMessageActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs;{de598db2-0e03-43b0-bf56-bcc40b3162df})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIContactMessageActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3730410930,
        3587,
        17328,
        [191, 86, 188, 196, 11, 49, 98, 223],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIContactMessageActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactMessageActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactMessageActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIContactMessageActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactMessageActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIContactMessageActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIContactMessageActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactMessageActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIContactMessageActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactMessageActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIContactMessageActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIContactMessageActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactMessageActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs
{
    fn from(value: WebUIContactMessageActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactMessageActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs
{
    fn from(value: &WebUIContactMessageActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs,
    > for WebUIContactMessageActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs,
    > for &WebUIContactMessageActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactMessageActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactMessageActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactMessageActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIContactMessageActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIContactMessageActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactMessageActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactMessageActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for WebUIContactMessageActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for &WebUIContactMessageActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactMessageActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactMessageActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactMessageActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIContactMessageActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIContactMessageActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIContactPanelActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPanelActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts"
    ))]
    pub fn ContactPanel(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Contacts::ContactPanel> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::ContactPanel>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts"
    ))]
    pub fn Contact(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIContactPanelActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs;{52bb63e4-d3d4-4b63-8051-4af2082cab80})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIContactPanelActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1388012516,
        54228,
        19299,
        [128, 81, 74, 242, 8, 44, 171, 128],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIContactPanelActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPanelActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPanelActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIContactPanelActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPanelActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIContactPanelActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIContactPanelActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPanelActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIContactPanelActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPanelActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIContactPanelActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIContactPanelActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactPanelActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactPanelActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIContactPanelActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIContactPanelActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactPanelActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactPanelActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIContactPanelActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIContactPanelActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactPanelActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactPanelActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactPanelActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIContactPanelActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIContactPanelActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPanelActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs
{
    fn from(value: WebUIContactPanelActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPanelActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs
{
    fn from(value: &WebUIContactPanelActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs,
    > for WebUIContactPanelActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs,
    > for &WebUIContactPanelActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactPanelActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Send for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Sync for WebUIContactPanelActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIContactPickerActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPickerActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts_Provider"
    ))]
    pub fn ContactPickerUI(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Contacts::Provider::ContactPickerUI,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::Provider::ContactPickerUI>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIContactPickerActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs;{ce57aae7-6449-45a7-971f-d113be7a8936})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIContactPickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3461851879,
        25673,
        17831,
        [151, 31, 209, 19, 190, 122, 137, 54],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIContactPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIContactPickerActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIContactPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIContactPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPickerActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIContactPickerActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPickerActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIContactPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIContactPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs
{
    fn from(value: WebUIContactPickerActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs
{
    fn from(value: &WebUIContactPickerActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs,
    > for WebUIContactPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs,
    > for &WebUIContactPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactPickerActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIContactPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIContactPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactPickerActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactPickerActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIContactPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIContactPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIContactPostActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactPostActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts"
    ))]
    pub fn Contact(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIContactPostActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIContactPostActivatedEventArgs;{b35a3c67-f1e7-4655-ad6e-4857588f552f})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIContactPostActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3009035367,
        61927,
        18005,
        [173, 110, 72, 87, 88, 143, 85, 47],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIContactPostActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactPostActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPostActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIContactPostActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPostActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIContactPostActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIContactPostActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPostActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIContactPostActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPostActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIContactPostActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIContactPostActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactPostActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs
{
    fn from(value: WebUIContactPostActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactPostActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs
{
    fn from(value: &WebUIContactPostActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs,
    > for WebUIContactPostActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs,
    > for &WebUIContactPostActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactPostActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactPostActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactPostActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIContactPostActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIContactPostActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactPostActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactPostActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for WebUIContactPostActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for &WebUIContactPostActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactPostActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactPostActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIContactPostActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIContactPostActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIContactPostActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIContactVideoCallActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIContactVideoCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ServiceUserId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Contacts"
    ))]
    pub fn Contact(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Contacts::Contact> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Contacts::Contact>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIContactVideoCallActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs;{61079db8-e3e7-4b4f-858d-5c63a96ef684})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIContactVideoCallActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1627889080,
        58343,
        19279,
        [133, 141, 92, 99, 169, 110, 246, 132],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIContactVideoCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIContactVideoCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactVideoCallActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactVideoCallActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactVideoCallActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIContactVideoCallActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactVideoCallActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIContactVideoCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs
{
    fn from(value: WebUIContactVideoCallActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIContactVideoCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs
{
    fn from(value: &WebUIContactVideoCallActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs,
    > for WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs,
    > for &WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IContactVideoCallActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIContactVideoCallActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIContactVideoCallActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIContactVideoCallActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IContactActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIContactVideoCallActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > for &WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContactActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIContactVideoCallActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIContactVideoCallActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIContactVideoCallActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIContactVideoCallActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIContactVideoCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIDeviceActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDeviceActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn DeviceInformationId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIDeviceActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIDeviceActivatedEventArgs;{cd50b9a9-ce10-44d2-8234-c355a073ef33})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIDeviceActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3444619689,
        52752,
        17618,
        [130, 52, 195, 85, 160, 115, 239, 51],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIDeviceActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDeviceActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDeviceActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIDeviceActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDeviceActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIDeviceActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIDeviceActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDeviceActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIDeviceActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDeviceActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIDeviceActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIDeviceActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDeviceActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs
{
    fn from(value: WebUIDeviceActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDeviceActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs
{
    fn from(value: &WebUIDeviceActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs,
    > for WebUIDeviceActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs,
    > for &WebUIDeviceActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IDeviceActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDeviceActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDeviceActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIDeviceActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIDeviceActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDeviceActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDeviceActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUIDeviceActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUIDeviceActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDeviceActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDeviceActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIDeviceActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIDeviceActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDeviceActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDeviceActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDeviceActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIDeviceActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIDeviceActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIDevicePairingActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDevicePairingActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Devices_Enumeration"
    ))]
    pub fn DeviceInformation(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Enumeration::DeviceInformation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Enumeration::DeviceInformation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIDevicePairingActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs;{eba0d1e4-ecc6-4148-94ed-f4b37ec05b3e})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIDevicePairingActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3953185252,
        60614,
        16712,
        [148, 237, 244, 179, 126, 192, 91, 62],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIDevicePairingActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDevicePairingActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDevicePairingActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDevicePairingActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIDevicePairingActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIDevicePairingActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDevicePairingActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIDevicePairingActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDevicePairingActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIDevicePairingActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIDevicePairingActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDevicePairingActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIDevicePairingActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIDevicePairingActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDevicePairingActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIDevicePairingActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIDevicePairingActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDevicePairingActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs
{
    fn from(value: WebUIDevicePairingActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDevicePairingActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs
{
    fn from(value: &WebUIDevicePairingActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs,
    > for WebUIDevicePairingActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs,
    > for &WebUIDevicePairingActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IDevicePairingActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDevicePairingActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDevicePairingActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDevicePairingActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIDevicePairingActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIDevicePairingActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIDialReceiverActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIDialReceiverActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn AppName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIDialReceiverActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs;{fb777ed7-85ee-456e-a44d-85d730e70aed})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIDialReceiverActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4218912471,
        34286,
        17774,
        [164, 77, 133, 215, 48, 231, 10, 237],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIDialReceiverActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIDialReceiverActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDialReceiverActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDialReceiverActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIDialReceiverActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIDialReceiverActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDialReceiverActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIDialReceiverActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDialReceiverActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIDialReceiverActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIDialReceiverActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs
{
    fn from(value: WebUIDialReceiverActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs
{
    fn from(value: &WebUIDialReceiverActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs,
    > for WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs,
    > for &WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IDialReceiverActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > for WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > for &WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDialReceiverActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIDialReceiverActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIDialReceiverActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIDialReceiverActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIDialReceiverActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIDialReceiverActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIFileActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub fn Files(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<super::super::Storage::IStorageItem>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                super::super::Storage::IStorageItem,
            >>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Verb(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage_Search"))]
    pub fn NeighboringFilesQuery(
        &self,
    ) -> ::windows::runtime::Result<super::super::Storage::Search::StorageFileQueryResult> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Search::StorageFileQueryResult>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIFileActivatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.WebUIFileActivatedEventArgs;{bb2afc33-93b1-42ed-8b26-236dd9c78496})",
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIFileActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3140156467,
        37809,
        17133,
        [139, 38, 35, 109, 217, 199, 132, 150],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIFileActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIFileActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIFileActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIFileActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIFileActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIFileActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIFileActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileActivatedEventArgs
{
    fn from(value: WebUIFileActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileActivatedEventArgs
{
    fn from(value: &WebUIFileActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileActivatedEventArgs,
    > for WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileActivatedEventArgs,
    > for &WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles,
    > for WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles,
    > for &WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IFileActivatedEventArgsWithNeighboringFiles,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIFileActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIFileActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIFileActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIFileOpenPickerActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileOpenPickerActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Storage_Pickers_Provider"
    ))]
    pub fn FileOpenPickerUI(
        &self,
    ) -> ::windows::runtime::Result<super::super::Storage::Pickers::Provider::FileOpenPickerUI>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Pickers::Provider::FileOpenPickerUI>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIFileOpenPickerActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs;{72827082-5525-4bf2-bc09-1f5095d4964d})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIFileOpenPickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1921151106,
        21797,
        19442,
        [188, 9, 31, 80, 149, 212, 150, 77],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIFileOpenPickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileOpenPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileOpenPickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileOpenPickerActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIFileOpenPickerActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileOpenPickerActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileOpenPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs
{
    fn from(value: WebUIFileOpenPickerActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileOpenPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs
{
    fn from(value: &WebUIFileOpenPickerActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs,
    > for WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs,
    > for &WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2,
    > for WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2,
    > for &WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IFileOpenPickerActivatedEventArgs2,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileOpenPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileOpenPickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileOpenPickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIFileOpenPickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIFileOpenPickerContinuationEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileOpenPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections",
        feature = "Storage"
    ))]
    pub fn Files(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<super::super::Storage::StorageFile>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                super::super::Storage::StorageFile,
            >>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections"
    ))]
    pub fn ContinuationData(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIFileOpenPickerContinuationEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs;{f0fa3f3a-d4e8-4ad3-9c34-2308f32fcec9})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIFileOpenPickerContinuationEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4042932026,
        54504,
        19155,
        [156, 52, 35, 8, 243, 47, 206, 201],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIFileOpenPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileOpenPickerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileOpenPickerContinuationEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileOpenPickerContinuationEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileOpenPickerContinuationEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIFileOpenPickerContinuationEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileOpenPickerContinuationEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileOpenPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs
{
    fn from(value: WebUIFileOpenPickerContinuationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileOpenPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs
{
    fn from(value: &WebUIFileOpenPickerContinuationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs,
    > for WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs,
    > for &WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileOpenPickerContinuationEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIFileOpenPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFileOpenPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIFileOpenPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFileOpenPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > for WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > for &WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIFileOpenPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFileOpenPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileOpenPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIFileOpenPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileOpenPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFileOpenPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIFileOpenPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIFileSavePickerActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileSavePickerActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Storage_Pickers_Provider"
    ))]
    pub fn FileSavePickerUI(
        &self,
    ) -> ::windows::runtime::Result<super::super::Storage::Pickers::Provider::FileSavePickerUI>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::Pickers::Provider::FileSavePickerUI>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn EnterpriseId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIFileSavePickerActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs;{81c19cf1-74e6-4387-82eb-bb8fd64b4346})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIFileSavePickerActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2176949489,
        29926,
        17287,
        [130, 235, 187, 143, 214, 75, 67, 70],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIFileSavePickerActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileSavePickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileSavePickerActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileSavePickerActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIFileSavePickerActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileSavePickerActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileSavePickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs
{
    fn from(value: WebUIFileSavePickerActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileSavePickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs
{
    fn from(value: &WebUIFileSavePickerActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs,
    > for WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs,
    > for &WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2,
    > for WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2,
    > for &WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IFileSavePickerActivatedEventArgs2,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileSavePickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileSavePickerActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIFileSavePickerActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIFileSavePickerActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIFileSavePickerContinuationEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFileSavePickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage"))]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections"
    ))]
    pub fn ContinuationData(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIFileSavePickerContinuationEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs;{2c846fe1-3bad-4f33-8c8b-e46fae824b4b})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIFileSavePickerContinuationEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        746876897,
        15277,
        20275,
        [140, 139, 228, 111, 174, 130, 75, 75],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIFileSavePickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFileSavePickerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileSavePickerContinuationEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileSavePickerContinuationEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileSavePickerContinuationEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIFileSavePickerContinuationEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileSavePickerContinuationEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFileSavePickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs
{
    fn from(value: WebUIFileSavePickerContinuationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFileSavePickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs
{
    fn from(value: &WebUIFileSavePickerContinuationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs,
    > for WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs,
    > for &WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFileSavePickerContinuationEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIFileSavePickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFileSavePickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIFileSavePickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFileSavePickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > for WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > for &WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIFileSavePickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFileSavePickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFileSavePickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIFileSavePickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFileSavePickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFileSavePickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIFileSavePickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIFolderPickerContinuationEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIFolderPickerContinuationEventArgs {
    #[cfg(feature = "deprecated")]
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Storage"))]
    pub fn Folder(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFolder> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Storage::StorageFolder>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections"
    ))]
    pub fn ContinuationData(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIFolderPickerContinuationEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs;{51882366-9f4b-498f-beb0-42684f6e1c29})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIFolderPickerContinuationEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1367876454,
        40779,
        18831,
        [190, 176, 66, 104, 79, 110, 28, 41],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIFolderPickerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIFolderPickerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFolderPickerContinuationEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFolderPickerContinuationEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIFolderPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIFolderPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFolderPickerContinuationEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIFolderPickerContinuationEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFolderPickerContinuationEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIFolderPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIFolderPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIFolderPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs
{
    fn from(value: WebUIFolderPickerContinuationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIFolderPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs
{
    fn from(value: &WebUIFolderPickerContinuationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs,
    > for WebUIFolderPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs,
    > for &WebUIFolderPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IFolderPickerContinuationEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFolderPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFolderPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIFolderPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIFolderPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFolderPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFolderPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > for WebUIFolderPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > for &WebUIFolderPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFolderPickerContinuationEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFolderPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIFolderPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIFolderPickerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIFolderPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIFolderPickerContinuationEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIFolderPickerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIFolderPickerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIFolderPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIFolderPickerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUILaunchActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILaunchActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PrelaunchActivated(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileActivatedInfo(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::TileActivatedInfo>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::TileActivatedInfo>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUILaunchActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUILaunchActivatedEventArgs;{fbc93e26-a14a-4b4f-82b0-33bed920af52})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUILaunchActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4224269862,
        41290,
        19279,
        [130, 176, 51, 190, 217, 32, 175, 82],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUILaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUILaunchActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILaunchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUILaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUILaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUILaunchActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILaunchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUILaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUILaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs
{
    fn from(value: WebUILaunchActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs
{
    fn from(value: &WebUILaunchActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > for WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > for &WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs,
    > for WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs,
    > for &WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IPrelaunchActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILaunchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUILaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUILaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILaunchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2,
    > for WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2,
    > for &WebUILaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs2,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUILockScreenActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Info(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUILockScreenActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUILockScreenActivatedEventArgs;{3ca77966-6108-4a41-8220-ee7d133c8532})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUILockScreenActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1017608550,
        24840,
        19009,
        [130, 32, 238, 125, 19, 60, 133, 50],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUILockScreenActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUILockScreenActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUILockScreenActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUILockScreenActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUILockScreenActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUILockScreenActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUILockScreenActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs
{
    fn from(value: WebUILockScreenActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs
{
    fn from(value: &WebUILockScreenActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs,
    > for WebUILockScreenActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs,
    > for &WebUILockScreenActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ILockScreenActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUILockScreenActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUILockScreenActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUILockScreenActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUILockScreenActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUILockScreenActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUILockScreenActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILockScreenActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUILockScreenActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUILockScreenActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUILockScreenCallActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenCallActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Calls"
    ))]
    pub fn CallUI(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Calls::LockScreenCallUI> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Calls::LockScreenCallUI>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Arguments(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TileId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUILockScreenCallActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs;{06f37fbe-b5f2-448b-b13e-e328ac1c516a})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUILockScreenCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        116621246,
        46578,
        17547,
        [177, 62, 227, 40, 172, 28, 81, 106],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUILockScreenCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUILockScreenCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUILockScreenCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenCallActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUILockScreenCallActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenCallActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUILockScreenCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUILockScreenCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs
{
    fn from(value: WebUILockScreenCallActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs
{
    fn from(value: &WebUILockScreenCallActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs,
    > for WebUILockScreenCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs,
    > for &WebUILockScreenCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ILockScreenCallActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUILockScreenCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUILockScreenCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUILockScreenCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUILockScreenCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > for WebUILockScreenCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > for &WebUILockScreenCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::ILaunchActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenCallActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenCallActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUILockScreenCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUILockScreenCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUILockScreenCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUILockScreenComponentActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUILockScreenComponentActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = self;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUILockScreenComponentActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUILockScreenComponentActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3479508755,
        52488,
        20440,
        [182, 151, 162, 129, 182, 84, 78, 46],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUILockScreenComponentActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUILockScreenComponentActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenComponentActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUILockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenComponentActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUILockScreenComponentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUILockScreenComponentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenComponentActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUILockScreenComponentActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenComponentActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUILockScreenComponentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUILockScreenComponentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUILockScreenComponentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    fn from(value: WebUILockScreenComponentActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUILockScreenComponentActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    fn from(value: &WebUILockScreenComponentActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUILockScreenComponentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUILockScreenComponentActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUILockScreenComponentActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUILockScreenComponentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUILockScreenComponentActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUILockScreenComponentActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUILockScreenComponentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUILockScreenComponentActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUINavigatedDeferral(::windows::runtime::IInspectable);
impl WebUINavigatedDeferral {
    pub fn Complete(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUINavigatedDeferral {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.WebUINavigatedDeferral;{d804204d-831f-46e2-b432-3afce211f962})",
    );
}
unsafe impl ::windows::runtime::Interface for WebUINavigatedDeferral {
    type Vtable = IWebUINavigatedDeferral_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3624149069,
        33567,
        18146,
        [180, 50, 58, 252, 226, 17, 249, 98],
    );
}
impl ::windows::runtime::RuntimeName for WebUINavigatedDeferral {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedDeferral";
}
impl ::std::convert::From<WebUINavigatedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: WebUINavigatedDeferral) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebUINavigatedDeferral> for ::windows::runtime::IUnknown {
    fn from(value: &WebUINavigatedDeferral) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUINavigatedDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUINavigatedDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<WebUINavigatedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: WebUINavigatedDeferral) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebUINavigatedDeferral> for ::windows::runtime::IInspectable {
    fn from(value: &WebUINavigatedDeferral) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUINavigatedDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUINavigatedDeferral
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUINavigatedEventArgs(::windows::runtime::IInspectable);
impl WebUINavigatedEventArgs {
    pub fn NavigatedOperation(&self) -> ::windows::runtime::Result<WebUINavigatedOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<WebUINavigatedOperation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUINavigatedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.WebUINavigatedEventArgs;{a75841b8-2499-4030-a69d-15d2d9cfe524})",
    );
}
unsafe impl ::windows::runtime::Interface for WebUINavigatedEventArgs {
    type Vtable = IWebUINavigatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2807579064,
        9369,
        16432,
        [166, 157, 21, 210, 217, 207, 229, 36],
    );
}
impl ::windows::runtime::RuntimeName for WebUINavigatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedEventArgs";
}
impl ::std::convert::From<WebUINavigatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUINavigatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebUINavigatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUINavigatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUINavigatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<WebUINavigatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUINavigatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebUINavigatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUINavigatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUINavigatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<WebUINavigatedEventArgs> for IWebUINavigatedEventArgs {
    fn from(value: WebUINavigatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebUINavigatedEventArgs> for IWebUINavigatedEventArgs {
    fn from(value: &WebUINavigatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUINavigatedEventArgs> for WebUINavigatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUINavigatedEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebUINavigatedEventArgs>::into(
            self,
        ))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IWebUINavigatedEventArgs> for &WebUINavigatedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, IWebUINavigatedEventArgs> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IWebUINavigatedEventArgs>::into(
            ::std::clone::Clone::clone(self),
        ))
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUINavigatedOperation(::windows::runtime::IInspectable);
impl WebUINavigatedOperation {
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<WebUINavigatedDeferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<WebUINavigatedDeferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUINavigatedOperation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.WebUINavigatedOperation;{7a965f08-8182-4a89-ab67-8492e8750d4b})",
    );
}
unsafe impl ::windows::runtime::Interface for WebUINavigatedOperation {
    type Vtable = IWebUINavigatedOperation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2056675080,
        33154,
        19081,
        [171, 103, 132, 146, 232, 117, 13, 75],
    );
}
impl ::windows::runtime::RuntimeName for WebUINavigatedOperation {
    const NAME: &'static str = "Windows.UI.WebUI.WebUINavigatedOperation";
}
impl ::std::convert::From<WebUINavigatedOperation> for ::windows::runtime::IUnknown {
    fn from(value: WebUINavigatedOperation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebUINavigatedOperation> for ::windows::runtime::IUnknown {
    fn from(value: &WebUINavigatedOperation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUINavigatedOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUINavigatedOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<WebUINavigatedOperation> for ::windows::runtime::IInspectable {
    fn from(value: WebUINavigatedOperation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebUINavigatedOperation> for ::windows::runtime::IInspectable {
    fn from(value: &WebUINavigatedOperation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUINavigatedOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUINavigatedOperation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIPhoneCallActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPhoneCallActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn LineId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::GUID>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIPhoneCallActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs;{54615221-a3c1-4ced-b62f-8c60523619ad})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIPhoneCallActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1415664161,
        41921,
        19693,
        [182, 47, 140, 96, 82, 54, 25, 173],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIPhoneCallActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPhoneCallActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPhoneCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPhoneCallActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIPhoneCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIPhoneCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPhoneCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIPhoneCallActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPhoneCallActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIPhoneCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIPhoneCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIPhoneCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIPhoneCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIPhoneCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIPhoneCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIPhoneCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIPhoneCallActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIPhoneCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIPhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIPhoneCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIPhoneCallActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIPhoneCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIPhoneCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPhoneCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs
{
    fn from(value: WebUIPhoneCallActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPhoneCallActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs
{
    fn from(value: &WebUIPhoneCallActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs,
    > for WebUIPhoneCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs,
    > for &WebUIPhoneCallActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IPhoneCallActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Send for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Sync for WebUIPhoneCallActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIPrint3DWorkflowActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrint3DWorkflowActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Devices_Printers_Extensions"
    ))]
    pub fn Workflow(
        &self,
    ) -> ::windows::runtime::Result<super::super::Devices::Printers::Extensions::Print3DWorkflow>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Printers::Extensions::Print3DWorkflow>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIPrint3DWorkflowActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs;{3f57e78b-f2ac-4619-8302-ef855e1c9b90})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIPrint3DWorkflowActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1062725515,
        62124,
        17945,
        [131, 2, 239, 133, 94, 28, 155, 144],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIPrint3DWorkflowActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrint3DWorkflowActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrint3DWorkflowActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrint3DWorkflowActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrint3DWorkflowActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs
{
    fn from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrint3DWorkflowActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs
{
    fn from(value: &WebUIPrint3DWorkflowActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs,
    > for WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs,
    > for &WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IPrint3DWorkflowActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIPrint3DWorkflowActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIPrint3DWorkflowActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIPrint3DWorkflowActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIPrint3DWorkflowActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIPrint3DWorkflowActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIPrint3DWorkflowActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIPrint3DWorkflowActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIPrintTaskSettingsActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintTaskSettingsActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Devices_Printers_Extensions"
    ))]
    pub fn Configuration(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Devices::Printers::Extensions::PrintTaskConfiguration,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Devices::Printers::Extensions::PrintTaskConfiguration>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIPrintTaskSettingsActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs;{ee30a0c9-ce56-4865-ba8e-8954ac271107})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIPrintTaskSettingsActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3996164297,
        52822,
        18533,
        [186, 142, 137, 84, 172, 39, 17, 7],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIPrintTaskSettingsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIPrintTaskSettingsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrintTaskSettingsActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrintTaskSettingsActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrintTaskSettingsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs
{
    fn from(value: WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrintTaskSettingsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs
{
    fn from(value: &WebUIPrintTaskSettingsActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs,
    > for WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs,
    > for &WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IPrintTaskSettingsActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIPrintTaskSettingsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIPrintTaskSettingsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIPrintTaskSettingsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIPrintTaskSettingsActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIPrintTaskSettingsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIPrintTaskSettingsActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIPrintTaskSettingsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIPrintTaskSettingsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIPrintWorkflowForegroundTaskActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = self;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = self;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs;{cf651713-cd08-4fd8-b697-a281b6544e2e})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3479508755,
        52488,
        20440,
        [182, 151, 162, 129, 182, 84, 78, 46],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIPrintWorkflowForegroundTaskActivatedEventArgs {
    const NAME: &'static str =
        "Windows.UI.WebUI.WebUIPrintWorkflowForegroundTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrintWorkflowForegroundTaskActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIPrintWorkflowForegroundTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIPrintWorkflowForegroundTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrintWorkflowForegroundTaskActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIPrintWorkflowForegroundTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIPrintWorkflowForegroundTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIPrintWorkflowForegroundTaskActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    fn from(value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    fn from(value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIPrintWorkflowForegroundTaskActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIPrintWorkflowForegroundTaskActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIPrintWorkflowForegroundTaskActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIPrintWorkflowForegroundTaskActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIPrintWorkflowForegroundTaskActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIPrintWorkflowForegroundTaskActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIPrintWorkflowForegroundTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIPrintWorkflowForegroundTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIProtocolActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolActivatedEventArgs {
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = & :: windows :: runtime :: Interface :: cast :: < super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > ( self ) ? ;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections"
    ))]
    pub fn Data(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = & :: windows :: runtime :: Interface :: cast :: < super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > ( self ) ? ;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIProtocolActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIProtocolActivatedEventArgs;{6095f4dd-b7c0-46ab-81fe-d90f36d00d24})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIProtocolActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1620440285,
        47040,
        18091,
        [129, 254, 217, 15, 54, 208, 13, 36],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIProtocolActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIProtocolActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIProtocolActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIProtocolActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIProtocolActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIProtocolActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIProtocolActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIProtocolActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIProtocolActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIProtocolActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIProtocolActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIProtocolActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs
{
    fn from(value: WebUIProtocolActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIProtocolActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs
{
    fn from(value: &WebUIProtocolActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
    > for WebUIProtocolActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
    > for &WebUIProtocolActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIProtocolActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIProtocolActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUIProtocolActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUIProtocolActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: TryFrom < WebUIProtocolActivatedEventArgs > for super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData { type Error = :: windows :: runtime :: Error ; fn try_from ( value : WebUIProtocolActivatedEventArgs ) -> :: windows :: runtime :: Result < Self > { :: std :: convert :: TryFrom :: try_from ( & value ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: TryFrom < & WebUIProtocolActivatedEventArgs > for super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData { type Error = :: windows :: runtime :: Error ; fn try_from ( value : & WebUIProtocolActivatedEventArgs ) -> :: windows :: runtime :: Result < Self > { :: windows :: runtime :: Interface :: cast ( value ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > for WebUIProtocolActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > { :: windows :: runtime :: IntoParam :: into_param ( & self ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > for & WebUIProtocolActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > { :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIProtocolActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIProtocolActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIProtocolActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIProtocolActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIProtocolActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIProtocolForResultsActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIProtocolForResultsActivatedEventArgs {
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn ProtocolForResultsOperation(
        &self,
    ) -> ::windows::runtime::Result<super::super::System::ProtocolForResultsOperation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::ProtocolForResultsOperation>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Uri(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CallerPackageFamilyName(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = & :: windows :: runtime :: Interface :: cast :: < super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > ( self ) ? ;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections"
    ))]
    pub fn Data(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = & :: windows :: runtime :: Interface :: cast :: < super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > ( self ) ? ;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIProtocolForResultsActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs;{e75132c2-7ae7-4517-80ac-dbe8d7cc5b9c})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIProtocolForResultsActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3880858306,
        31463,
        17687,
        [128, 172, 219, 232, 215, 204, 91, 156],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIProtocolForResultsActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIProtocolForResultsActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIProtocolForResultsActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIProtocolForResultsActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIProtocolForResultsActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIProtocolForResultsActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIProtocolForResultsActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs
{
    fn from(value: WebUIProtocolForResultsActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs
{
    fn from(value: &WebUIProtocolForResultsActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs,
    > for WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs,
    > for &WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IProtocolForResultsActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
    > for WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
    > for &WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IProtocolActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: TryFrom < WebUIProtocolForResultsActivatedEventArgs > for super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData { type Error = :: windows :: runtime :: Error ; fn try_from ( value : WebUIProtocolForResultsActivatedEventArgs ) -> :: windows :: runtime :: Result < Self > { :: std :: convert :: TryFrom :: try_from ( & value ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < > :: std :: convert :: TryFrom < & WebUIProtocolForResultsActivatedEventArgs > for super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData { type Error = :: windows :: runtime :: Error ; fn try_from ( value : & WebUIProtocolForResultsActivatedEventArgs ) -> :: windows :: runtime :: Result < Self > { :: windows :: runtime :: Interface :: cast ( value ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > for WebUIProtocolForResultsActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > { :: windows :: runtime :: IntoParam :: into_param ( & self ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl < 'a , > :: windows :: runtime :: IntoParam < 'a , super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > for & WebUIProtocolForResultsActivatedEventArgs { fn into_param ( self ) -> :: windows :: runtime :: Param < 'a , super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > { :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IProtocolActivatedEventArgsWithCallerPackageFamilyNameAndData > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None ) } }
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIProtocolForResultsActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIProtocolForResultsActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIProtocolForResultsActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIRestrictedLaunchActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIRestrictedLaunchActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SharedContext(&self) -> ::windows::runtime::Result<::windows::runtime::IInspectable> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::IInspectable>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIRestrictedLaunchActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs;{e0b7ac81-bfc3-4344-a5da-19fd5a27baae})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIRestrictedLaunchActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3770133633,
        49091,
        17220,
        [165, 218, 25, 253, 90, 39, 186, 174],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIRestrictedLaunchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIRestrictedLaunchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIRestrictedLaunchActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIRestrictedLaunchActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIRestrictedLaunchActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIRestrictedLaunchActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIRestrictedLaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs
{
    fn from(value: WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIRestrictedLaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs
{
    fn from(value: &WebUIRestrictedLaunchActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs,
    > for WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs,
    > for &WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IRestrictedLaunchActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIRestrictedLaunchActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIRestrictedLaunchActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIRestrictedLaunchActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIRestrictedLaunchActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIRestrictedLaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIRestrictedLaunchActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIRestrictedLaunchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIRestrictedLaunchActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIRestrictedLaunchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUISearchActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUISearchActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn QueryText(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Language(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn CurrentlyShownApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Search"
    ))]
    pub fn LinguisticDetails(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Search::SearchPaneQueryLinguisticDetails,
    > {
        let this = & :: windows :: runtime :: Interface :: cast :: < super::super::ApplicationModel::Activation:: ISearchActivatedEventArgsWithLinguisticDetails > ( self ) ? ;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Search::SearchPaneQueryLinguisticDetails>(
                result__,
            )
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUISearchActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUISearchActivatedEventArgs;{8cb36951-58c8-43e3-94bc-41d33f8b630e})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUISearchActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::ISearchActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2360568145,
        22728,
        17379,
        [148, 188, 65, 211, 63, 139, 99, 14],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUISearchActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUISearchActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUISearchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUISearchActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUISearchActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUISearchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUISearchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUISearchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUISearchActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUISearchActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUISearchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUISearchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUISearchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ISearchActivatedEventArgs
{
    fn from(value: WebUISearchActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUISearchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ISearchActivatedEventArgs
{
    fn from(value: &WebUISearchActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ISearchActivatedEventArgs,
    > for WebUISearchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ISearchActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ISearchActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ISearchActivatedEventArgs,
    > for &WebUISearchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ISearchActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::ISearchActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUISearchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUISearchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUISearchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUISearchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUISearchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUISearchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for WebUISearchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > for &WebUISearchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IApplicationViewActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUISearchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUISearchActivatedEventArgs>
    for super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails,
    > for WebUISearchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails,
    > for &WebUISearchActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::ISearchActivatedEventArgsWithLinguisticDetails,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: ISearchActivatedEventArgsWithLinguisticDetails > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUISearchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUISearchActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUISearchActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUISearchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUISearchActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIShareTargetActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIShareTargetActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_DataTransfer_ShareTarget"
    ))]
    pub fn ShareOperation(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::DataTransfer::ShareTarget::ShareOperation,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::DataTransfer::ShareTarget::ShareOperation>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIShareTargetActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs;{4bdaf9c8-cdb2-4acb-bfc3-6648563378ec})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIShareTargetActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1272641992,
        52658,
        19147,
        [191, 195, 102, 72, 86, 51, 120, 236],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIShareTargetActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIShareTargetActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIShareTargetActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIShareTargetActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIShareTargetActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIShareTargetActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIShareTargetActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIShareTargetActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIShareTargetActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIShareTargetActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIShareTargetActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIShareTargetActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIShareTargetActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs
{
    fn from(value: WebUIShareTargetActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIShareTargetActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs
{
    fn from(value: &WebUIShareTargetActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs,
    > for WebUIShareTargetActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs,
    > for &WebUIShareTargetActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IShareTargetActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIShareTargetActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIShareTargetActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIShareTargetActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIShareTargetActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIShareTargetActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIShareTargetActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIShareTargetActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIShareTargetActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIShareTargetActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIShareTargetActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIShareTargetActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIShareTargetActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIShareTargetActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIStartupTaskActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIStartupTaskActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn TaskId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIStartupTaskActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs;{03b11a58-5276-4d91-8621-54611864d5fa})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIStartupTaskActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        61938264,
        21110,
        19857,
        [134, 33, 84, 97, 24, 100, 213, 250],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIStartupTaskActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIStartupTaskActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIStartupTaskActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIStartupTaskActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIStartupTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIStartupTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIStartupTaskActivatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: WebUIStartupTaskActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIStartupTaskActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIStartupTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIStartupTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIStartupTaskActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIStartupTaskActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIStartupTaskActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIStartupTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIStartupTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIStartupTaskActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIStartupTaskActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIStartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIStartupTaskActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIStartupTaskActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIStartupTaskActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIStartupTaskActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIStartupTaskActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs
{
    fn from(value: WebUIStartupTaskActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIStartupTaskActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs
{
    fn from(value: &WebUIStartupTaskActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs,
    > for WebUIStartupTaskActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs,
    > for &WebUIStartupTaskActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IStartupTaskActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Send for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::std::marker::Sync for WebUIStartupTaskActivatedEventArgs {}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIToastNotificationActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIToastNotificationActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Argument(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections"
    ))]
    pub fn UserInput(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIToastNotificationActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs;{92a86f82-5290-431d-be85-c4aaeeb8685f})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIToastNotificationActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2460512130,
        21136,
        17181,
        [190, 133, 196, 170, 238, 184, 104, 95],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIToastNotificationActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIToastNotificationActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIToastNotificationActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIToastNotificationActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIToastNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIToastNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIToastNotificationActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIToastNotificationActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIToastNotificationActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIToastNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIToastNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIToastNotificationActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs
{
    fn from(value: WebUIToastNotificationActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIToastNotificationActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs
{
    fn from(value: &WebUIToastNotificationActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs,
    > for WebUIToastNotificationActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs,
    > for &WebUIToastNotificationActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IToastNotificationActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIToastNotificationActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIToastNotificationActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIToastNotificationActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIToastNotificationActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIToastNotificationActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIToastNotificationActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIToastNotificationActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIToastNotificationActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIToastNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIToastNotificationActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIToastNotificationActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIToastNotificationActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIToastNotificationActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIToastNotificationActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIToastNotificationActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIToastNotificationActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIUserDataAccountProviderActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIUserDataAccountProviderActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_UserDataAccounts_Provider"
    ))]    pub fn Operation < > ( & self , ) -> :: windows :: runtime :: Result < super::super::ApplicationModel::UserDataAccounts::Provider:: IUserDataAccountProviderOperation >{
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::ApplicationModel::UserDataAccounts::Provider:: IUserDataAccountProviderOperation > ( result__ )
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIUserDataAccountProviderActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs;{1bc9f723-8ef1-4a51-a63a-fe711eeab607})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIUserDataAccountProviderActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        466220835,
        36593,
        19025,
        [166, 58, 254, 113, 30, 234, 182, 7],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIUserDataAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIUserDataAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIUserDataAccountProviderActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIUserDataAccountProviderActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIUserDataAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIUserDataAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIUserDataAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIUserDataAccountProviderActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIUserDataAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIUserDataAccountProviderActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIUserDataAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIUserDataAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs
{
    fn from(value: WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIUserDataAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs
{
    fn from(value: &WebUIUserDataAccountProviderActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs,
    > for WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs,
    > for &WebUIUserDataAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IUserDataAccountProviderActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIView(::windows::runtime::IInspectable);
impl WebUIView {
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn Source(&self) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn SetSource<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
    >(
        &self,
        source: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn DocumentTitle(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn CanGoBack(&self) -> ::windows::runtime::Result<bool> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn CanGoForward(&self) -> ::windows::runtime::Result<bool> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn SetDefaultBackgroundColor<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::Color>,
    >(
        &self,
        value: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                value.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn DefaultBackgroundColor(&self) -> ::windows::runtime::Result<super::Color> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::Color = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::Color>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn ContainsFullScreenElement(&self) -> ::windows::runtime::Result<bool> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn Settings(
        &self,
    ) -> ::windows::runtime::Result<super::super::Web::UI::WebViewControlSettings> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Web::UI::WebViewControlSettings>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Web_UI"))]
    pub fn DeferredPermissionRequests(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<
            super::super::Web::UI::WebViewControlDeferredPermissionRequest,
        >,
    > {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                super::super::Web::UI::WebViewControlDeferredPermissionRequest,
            >>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn GoForward(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn GoBack(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn Refresh(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn Navigate<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
    >(
        &self,
        source: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).20)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn NavigateToString<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        text: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).21)(
                ::std::mem::transmute_copy(this),
                text.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web", feature = "Web_UI"))]
    pub fn NavigateToLocalStreamUri<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Web::IUriToStreamResolver>,
    >(
        &self,
        source: Param0,
        streamresolver: Param1,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).22)(
                ::std::mem::transmute_copy(this),
                source.into_param().abi(),
                streamresolver.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Web_Http", feature = "Web_UI"))]
    pub fn NavigateWithHttpRequestMessage<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Web::Http::HttpRequestMessage>,
    >(
        &self,
        requestmessage: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).23)(
                ::std::mem::transmute_copy(this),
                requestmessage.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Foundation_Collections",
        feature = "Web_UI"
    ))]
    pub fn InvokeScriptAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        &self,
        scriptname: Param0,
        arguments: Param1,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>,
    > {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).24)(
                ::std::mem::transmute_copy(this),
                scriptname.into_param().abi(),
                arguments.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<::windows::runtime::HSTRING>>(
                result__,
            )
        }
    }
    #[cfg(all(
        feature = "Foundation",
        feature = "Storage_Streams",
        feature = "Web_UI"
    ))]
    pub fn CapturePreviewToStreamAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::Streams::IRandomAccessStream>,
    >(
        &self,
        stream: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncAction> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).25)(
                ::std::mem::transmute_copy(this),
                stream.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_DataTransfer",
        feature = "Foundation",
        feature = "Web_UI"
    ))]
    pub fn CaptureSelectedContentToDataPackageAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<
            super::super::ApplicationModel::DataTransfer::DataPackage,
        >,
    > {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).26)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                super::super::ApplicationModel::DataTransfer::DataPackage,
            >>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn BuildLocalStreamUri<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        contentidentifier: Param0,
        relativepath: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::Uri> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).27)(
                ::std::mem::transmute_copy(this),
                contentidentifier.into_param().abi(),
                relativepath.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Uri>(result__)
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn GetDeferredPermissionRequestById(
        &self,
        id: u32,
        result: &mut ::std::option::Option<
            super::super::Web::UI::WebViewControlDeferredPermissionRequest,
        >,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).28)(
                ::std::mem::transmute_copy(this),
                id,
                result as *mut _ as _,
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NavigationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlNavigationStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).29)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNavigationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).30)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ContentLoading<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlContentLoadingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).31)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveContentLoading<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).32)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn DOMContentLoaded<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlDOMContentLoadedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).33)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveDOMContentLoaded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).34)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NavigationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlNavigationCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).35)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNavigationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).36)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameNavigationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlNavigationStartingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).37)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameNavigationStarting<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).38)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameContentLoading<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlContentLoadingEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).39)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameContentLoading<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).40)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameDOMContentLoaded<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlDOMContentLoadedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).41)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameDOMContentLoaded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).42)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn FrameNavigationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlNavigationCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).43)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveFrameNavigationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).44)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ScriptNotify<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlScriptNotifyEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).45)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveScriptNotify<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).46)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn LongRunningScriptDetected<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlLongRunningScriptDetectedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).47)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveLongRunningScriptDetected<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).48)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnsafeContentWarningDisplaying<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).49)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnsafeContentWarningDisplaying<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).50)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnviewableContentIdentified<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlUnviewableContentIdentifiedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).51)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnviewableContentIdentified<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).52)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn PermissionRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlPermissionRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).53)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemovePermissionRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).54)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn UnsupportedUriSchemeIdentified<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlUnsupportedUriSchemeIdentifiedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).55)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveUnsupportedUriSchemeIdentified<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).56)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn NewWindowRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlNewWindowRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).57)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveNewWindowRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).58)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn ContainsFullScreenElementChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).59)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveContainsFullScreenElementChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).60)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn WebResourceRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                super::super::Web::UI::IWebViewControl,
                super::super::Web::UI::WebViewControlWebResourceRequestedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).61)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Web_UI"))]
    pub fn RemoveWebResourceRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).62)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn ApplicationViewId(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<i32>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Closed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                WebUIView,
                ::windows::runtime::IInspectable,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveClosed<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "Foundation"))]
    pub fn Activated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                WebUIView,
                super::super::ApplicationModel::Activation::IActivatedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveActivated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn IgnoreApplicationContentUriRulesNavigationRestrictions(
        &self,
    ) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    pub fn SetIgnoreApplicationContentUriRulesNavigationRestrictions(
        &self,
        value: bool,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                value,
            )
            .ok()
        }
    }
    #[cfg(feature = "Web_UI")]
    pub fn AddInitializeScript<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        script: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Web::UI::IWebViewControl2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                script.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateAsync(
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WebUIView>> {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<WebUIView>>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateWithUriAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Uri>,
    >(
        uri: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<WebUIView>> {
        Self::IWebUIViewStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                uri.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<WebUIView>>(result__)
        })
    }
    pub fn IWebUIViewStatics<R, F: FnOnce(&IWebUIViewStatics) -> ::windows::runtime::Result<R>>(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<WebUIView, IWebUIViewStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for WebUIView {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.UI.WebUI.WebUIView;{6783f64f-52da-4fd7-be69-8ef6284b423c})",
    );
}
unsafe impl ::windows::runtime::Interface for WebUIView {
    type Vtable = IWebUIView_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1736701519,
        21210,
        20439,
        [190, 105, 142, 246, 40, 75, 66, 60],
    );
}
impl ::windows::runtime::RuntimeName for WebUIView {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIView";
}
impl ::std::convert::From<WebUIView> for ::windows::runtime::IUnknown {
    fn from(value: WebUIView) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&WebUIView> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIView) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for WebUIView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &WebUIView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<WebUIView> for ::windows::runtime::IInspectable {
    fn from(value: WebUIView) -> Self {
        value.0
    }
}
impl ::std::convert::From<&WebUIView> for ::windows::runtime::IInspectable {
    fn from(value: &WebUIView) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for WebUIView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a WebUIView {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Web_UI")]
impl ::std::convert::TryFrom<WebUIView> for super::super::Web::UI::IWebViewControl {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIView) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_UI")]
impl ::std::convert::TryFrom<&WebUIView> for super::super::Web::UI::IWebViewControl {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Web::UI::IWebViewControl> for WebUIView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Web::UI::IWebViewControl> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Web::UI::IWebViewControl> for &WebUIView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Web::UI::IWebViewControl> {
        ::std::convert::TryInto::<super::super::Web::UI::IWebViewControl>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "Web_UI")]
impl ::std::convert::TryFrom<WebUIView> for super::super::Web::UI::IWebViewControl2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIView) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Web_UI")]
impl ::std::convert::TryFrom<&WebUIView> for super::super::Web::UI::IWebViewControl2 {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIView) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Web::UI::IWebViewControl2> for WebUIView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Web::UI::IWebViewControl2> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Web_UI")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Web::UI::IWebViewControl2> for &WebUIView {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Web::UI::IWebViewControl2> {
        ::std::convert::TryInto::<super::super::Web::UI::IWebViewControl2>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIVoiceCommandActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIVoiceCommandActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Media_SpeechRecognition"
    ))]
    pub fn Result(
        &self,
    ) -> ::windows::runtime::Result<super::super::Media::SpeechRecognition::SpeechRecognitionResult>
    {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Media::SpeechRecognition::SpeechRecognitionResult>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIVoiceCommandActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs;{ab92dcfd-8d43-4de6-9775-20704b581b00})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIVoiceCommandActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2878528765,
        36163,
        19942,
        [151, 117, 32, 112, 75, 88, 27, 0],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIVoiceCommandActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIVoiceCommandActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIVoiceCommandActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIVoiceCommandActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIVoiceCommandActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIVoiceCommandActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIVoiceCommandActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIVoiceCommandActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs
{
    fn from(value: WebUIVoiceCommandActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIVoiceCommandActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs
{
    fn from(value: &WebUIVoiceCommandActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs,
    > for WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs,
    > for &WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IVoiceCommandActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIVoiceCommandActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIVoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIVoiceCommandActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIVoiceCommandActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIVoiceCommandActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIWalletActionActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWalletActionActivatedEventArgs {
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ItemId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "ApplicationModel_Wallet"
    ))]
    pub fn ActionKind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Wallet::WalletActionKind> {
        let this = self;
        unsafe {
            let mut result__: super::super::ApplicationModel::Wallet::WalletActionKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Wallet::WalletActionKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn ActionId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIWalletActionActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs;{fcfc027b-1a1a-4d22-923f-ae6f45fa52d9})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIWalletActionActivatedEventArgs {
    type Vtable = super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4244374139,
        6682,
        19746,
        [146, 63, 174, 111, 69, 250, 82, 217],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIWalletActionActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWalletActionActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWalletActionActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: WebUIWalletActionActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWalletActionActivatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIWalletActionActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIWalletActionActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWalletActionActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIWalletActionActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWalletActionActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIWalletActionActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIWalletActionActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWalletActionActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs
{
    fn from(value: WebUIWalletActionActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWalletActionActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs
{
    fn from(value: &WebUIWalletActionActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs,
    > for WebUIWalletActionActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs,
    > for &WebUIWalletActionActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IWalletActionActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIWalletActionActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIWalletActionActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIWalletActionActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIWalletActionActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIWalletActionActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIWalletActionActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: WebUIWalletActionActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIWalletActionActivatedEventArgs> for IActivatedEventArgsDeferral {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &WebUIWalletActionActivatedEventArgs) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIWalletActionActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIWalletActionActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIWebAccountProviderActivatedEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAccountProviderActivatedEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Security_Authentication_Web_Provider"
    ))]
    pub fn Operation(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Security::Authentication::Web::Provider::IWebAccountProviderOperation,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            ( :: windows :: runtime :: Interface :: vtable ( this ) .6 ) ( :: std :: mem :: transmute_copy ( this ) , & mut result__ ) . from_abi :: < super::super::Security::Authentication::Web::Provider:: IWebAccountProviderOperation > ( result__ )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
    #[cfg(all(feature = "ApplicationModel_Activation", feature = "System"))]
    pub fn User(&self) -> ::windows::runtime::Result<super::super::System::User> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::System::User>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIWebAccountProviderActivatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs;{72b71774-98ea-4ccf-9752-46d9051004f1})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIWebAccountProviderActivatedEventArgs {
    type Vtable =
        super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1924601716,
        39146,
        19663,
        [151, 82, 70, 217, 5, 16, 4, 241],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIWebAccountProviderActivatedEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAccountProviderActivatedEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWebAccountProviderActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWebAccountProviderActivatedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWebAccountProviderActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIWebAccountProviderActivatedEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWebAccountProviderActivatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWebAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs
{
    fn from(value: WebUIWebAccountProviderActivatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWebAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs
{
    fn from(value: &WebUIWebAccountProviderActivatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs,
    > for WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs,
        >::into(self))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs,
    > for &WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs,
    > {
        ::windows::runtime::Param::Owned(::std::convert::Into::<
            super::super::ApplicationModel::Activation::IWebAccountProviderActivatedEventArgs,
        >::into(::std::clone::Clone::clone(self)))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIWebAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIWebAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIWebAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIWebAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIWebAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIWebAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIWebAccountProviderActivatedEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIWebAccountProviderActivatedEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > for &WebUIWebAccountProviderActivatedEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IActivatedEventArgsWithUser,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct WebUIWebAuthenticationBrokerContinuationEventArgs(::windows::runtime::IInspectable);
#[cfg(feature = "ApplicationModel_Activation")]
impl WebUIWebAuthenticationBrokerContinuationEventArgs {
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Security_Authentication_Web"
    ))]
    pub fn WebAuthenticationResult(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Security::Authentication::Web::WebAuthenticationResult,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Security::Authentication::Web::WebAuthenticationResult>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn Kind(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::ActivationKind>
    {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: super::super::ApplicationModel::Activation::ActivationKind =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ActivationKind>(result__)
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn PreviousExecutionState(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::ApplicationModel::Activation::ApplicationExecutionState,
    > {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__ : super::super::ApplicationModel::Activation:: ApplicationExecutionState = :: std :: mem :: zeroed ( ) ;
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::ApplicationExecutionState>(
                result__,
            )
        }
    }
    #[cfg(feature = "ApplicationModel_Activation")]
    pub fn SplashScreen(
        &self,
    ) -> ::windows::runtime::Result<super::super::ApplicationModel::Activation::SplashScreen> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::ApplicationModel::Activation::SplashScreen>(result__)
        }
    }
    #[cfg(all(
        feature = "ApplicationModel_Activation",
        feature = "Foundation_Collections"
    ))]
    pub fn ContinuationData(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::Collections::ValueSet> {
        let this = &::windows::runtime::Interface::cast::<
            super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
        >(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::ValueSet>(result__)
        }
    }
    pub fn ActivatedOperation(&self) -> ::windows::runtime::Result<ActivatedOperation> {
        let this = &::windows::runtime::Interface::cast::<IActivatedEventArgsDeferral>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<ActivatedOperation>(result__)
        }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::RuntimeType for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs;{75dda3d4-7714-453d-b7ff-b95e3a1709da})" ) ;
}
#[cfg(feature = "ApplicationModel_Activation")]
unsafe impl ::windows::runtime::Interface for WebUIWebAuthenticationBrokerContinuationEventArgs {
    type Vtable = super::super::ApplicationModel::Activation:: IWebAuthenticationBrokerContinuationEventArgs_abi ;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1977459668,
        30484,
        17725,
        [183, 255, 185, 94, 58, 23, 9, 218],
    );
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::runtime::RuntimeName for WebUIWebAuthenticationBrokerContinuationEventArgs {
    const NAME: &'static str = "Windows.UI.WebUI.WebUIWebAuthenticationBrokerContinuationEventArgs";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWebAuthenticationBrokerContinuationEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWebAuthenticationBrokerContinuationEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        value.0.clone()
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<WebUIWebAuthenticationBrokerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs
{
    fn from(value: WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::From<&WebUIWebAuthenticationBrokerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs
{
    fn from(value: &WebUIWebAuthenticationBrokerContinuationEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs,
    > for WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs,
    > {
        :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IWebAuthenticationBrokerContinuationEventArgs > :: into ( self ) )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs,
    > for &WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IWebAuthenticationBrokerContinuationEventArgs,
    > {
        :: windows :: runtime :: Param :: Owned ( :: std :: convert :: Into :: < super::super::ApplicationModel::Activation:: IWebAuthenticationBrokerContinuationEventArgs > :: into ( :: std :: clone :: Clone :: clone ( self ) ) )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIWebAuthenticationBrokerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIWebAuthenticationBrokerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > for &WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IActivatedEventArgs,
    > {
        :: std :: convert :: TryInto :: < super::super::ApplicationModel::Activation:: IActivatedEventArgs > :: try_into ( self ) . map ( :: windows :: runtime :: Param :: Owned ) . unwrap_or ( :: windows :: runtime :: Param :: None )
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIWebAuthenticationBrokerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs>
    for super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIWebAuthenticationBrokerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > for WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a>
    ::windows::runtime::IntoParam<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > for &WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(
        self,
    ) -> ::windows::runtime::Param<
        'a,
        super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
    > {
        ::std::convert::TryInto::<
            super::super::ApplicationModel::Activation::IContinuationActivatedEventArgs,
        >::try_into(self)
        .map(::windows::runtime::Param::Owned)
        .unwrap_or(::windows::runtime::Param::None)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<WebUIWebAuthenticationBrokerContinuationEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: WebUIWebAuthenticationBrokerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::std::convert::TryFrom<&WebUIWebAuthenticationBrokerContinuationEventArgs>
    for IActivatedEventArgsDeferral
{
    type Error = ::windows::runtime::Error;
    fn try_from(
        value: &WebUIWebAuthenticationBrokerContinuationEventArgs,
    ) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl<'a> ::windows::runtime::IntoParam<'a, IActivatedEventArgsDeferral>
    for &WebUIWebAuthenticationBrokerContinuationEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IActivatedEventArgsDeferral> {
        ::std::convert::TryInto::<IActivatedEventArgsDeferral>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
