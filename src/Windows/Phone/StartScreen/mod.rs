#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Phone_StartScreen`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct DualSimTile(::windows::runtime::IInspectable);
impl DualSimTile {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DualSimTile, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Phone_StartScreen`*"]
    pub fn SetDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Phone_StartScreen`*"]
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Phone_StartScreen`*"]
    pub fn IsPinnedToStart(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_StartScreen`, `Foundation`*"]
    pub fn CreateAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_StartScreen`, `Foundation`*"]
    pub fn UpdateAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_StartScreen`, `Foundation`*"]
    pub fn DeleteAsync(&self) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[doc = "*Required features: `Phone_StartScreen`*"]
    pub fn GetTileForSim2() -> ::windows::runtime::Result<DualSimTile> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<DualSimTile>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    #[doc = "*Required features: `Phone_StartScreen`, `Foundation`*"]
    pub fn UpdateDisplayNameForSim1Async<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        })
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Phone_StartScreen`, `UI_Notifications`*"]
    pub fn CreateTileUpdaterForSim1() -> ::windows::runtime::Result<super::super::UI::Notifications::TileUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileUpdater>(result__)
        })
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Phone_StartScreen`, `UI_Notifications`*"]
    pub fn CreateTileUpdaterForSim2() -> ::windows::runtime::Result<super::super::UI::Notifications::TileUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::TileUpdater>(result__)
        })
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Phone_StartScreen`, `UI_Notifications`*"]
    pub fn CreateBadgeUpdaterForSim1() -> ::windows::runtime::Result<super::super::UI::Notifications::BadgeUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::BadgeUpdater>(result__)
        })
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Phone_StartScreen`, `UI_Notifications`*"]
    pub fn CreateBadgeUpdaterForSim2() -> ::windows::runtime::Result<super::super::UI::Notifications::BadgeUpdater> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::BadgeUpdater>(result__)
        })
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Phone_StartScreen`, `UI_Notifications`*"]
    pub fn CreateToastNotifierForSim1() -> ::windows::runtime::Result<super::super::UI::Notifications::ToastNotifier> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotifier>(result__)
        })
    }
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Phone_StartScreen`, `UI_Notifications`*"]
    pub fn CreateToastNotifierForSim2() -> ::windows::runtime::Result<super::super::UI::Notifications::ToastNotifier> {
        Self::IDualSimTileStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotifier>(result__)
        })
    }
    pub fn IDualSimTileStatics<R, F: FnOnce(&IDualSimTileStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<DualSimTile, IDualSimTileStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for DualSimTile {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Phone.StartScreen.DualSimTile;{143ab213-d05f-4041-a18c-3e3fcb75b41e})");
}
unsafe impl ::windows::runtime::Interface for DualSimTile {
    type Vtable = IDualSimTile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(339390995, 53343, 16449, [161, 140, 62, 63, 203, 117, 180, 30]);
}
impl ::windows::runtime::RuntimeName for DualSimTile {
    const NAME: &'static str = "Windows.Phone.StartScreen.DualSimTile";
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct DualSimTileContract(pub u8);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDualSimTile(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDualSimTile {
    type Vtable = IDualSimTile_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(339390995, 53343, 16449, [161, 140, 62, 63, 203, 117, 180, 30]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDualSimTile_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IDualSimTileStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IDualSimTileStatics {
    type Vtable = IDualSimTileStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1347845278, 50575, 19913, [182, 232, 250, 103, 119, 238, 235, 55]);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDualSimTileStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Phone_StartScreen`*"]
pub struct IToastNotificationManagerStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IToastNotificationManagerStatics3 {
    type Vtable = IToastNotificationManagerStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(655881547, 20703, 17493, [142, 110, 65, 224, 252, 142, 19, 206]);
}
impl IToastNotificationManagerStatics3 {
    #[cfg(feature = "UI_Notifications")]
    #[doc = "*Required features: `Phone_StartScreen`, `UI_Notifications`*"]
    pub fn CreateToastNotifierForSecondaryTile<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, tileid: Param0) -> ::windows::runtime::Result<super::super::UI::Notifications::ToastNotifier> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), tileid.into_param().abi(), &mut result__).from_abi::<super::super::UI::Notifications::ToastNotifier>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IToastNotificationManagerStatics3 {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{2717f54b-50df-4455-8e6e-41e0fc8e13ce}");
}
#[repr(C)]
#[doc(hidden)]
pub struct IToastNotificationManagerStatics3_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "UI_Notifications")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tileid: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "UI_Notifications"))] usize,
);
