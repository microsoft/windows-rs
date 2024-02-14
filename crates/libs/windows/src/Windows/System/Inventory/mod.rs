::windows_core::imp::com_interface!(IInstalledDesktopApp, IInstalledDesktopApp_Vtbl, 0x75eab8ed_c0bc_5364_4c28_166e0545167a);
#[repr(C)]
pub struct IInstalledDesktopApp_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayName: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub Publisher: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub DisplayVersion: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IInstalledDesktopAppStatics, IInstalledDesktopAppStatics_Vtbl, 0x264cf74e_21cd_5f9b_6056_7866ad72489a);
#[repr(C)]
pub struct IInstalledDesktopAppStatics_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetInventoryAsync: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetInventoryAsync: usize,
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::fmt::Debug, ::core::clone::Clone)]
pub struct InstalledDesktopApp(::windows_core::IUnknown);
::windows_core::imp::interface_hierarchy!(InstalledDesktopApp, ::windows_core::IUnknown, ::windows_core::IInspectable);
::windows_core::imp::required_hierarchy!(InstalledDesktopApp, super::super::Foundation::IStringable);
impl InstalledDesktopApp {
    pub fn Id(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Id)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayName)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn Publisher(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Publisher)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    pub fn DisplayVersion(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).DisplayVersion)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetInventoryAsync() -> ::windows_core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<InstalledDesktopApp>>> {
        Self::IInstalledDesktopAppStatics(|this| unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).GetInventoryAsync)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        })
    }
    pub fn ToString(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = &::windows_core::Interface::cast::<super::super::Foundation::IStringable>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ToString)(::windows_core::Interface::as_raw(this), &mut result__).and_then(|| ::windows_core::Type::from_abi(result__))
        }
    }
    #[doc(hidden)]
    pub fn IInstalledDesktopAppStatics<R, F: FnOnce(&IInstalledDesktopAppStatics) -> ::windows_core::Result<R>>(callback: F) -> ::windows_core::Result<R> {
        static SHARED: ::windows_core::imp::FactoryCache<InstalledDesktopApp, IInstalledDesktopAppStatics> = ::windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::windows_core::RuntimeType for InstalledDesktopApp {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::for_class::<Self>();
}
unsafe impl ::windows_core::Interface for InstalledDesktopApp {
    type Vtable = IInstalledDesktopApp_Vtbl;
    const IID: ::windows_core::GUID = <IInstalledDesktopApp as ::windows_core::Interface>::IID;
}
impl ::windows_core::RuntimeName for InstalledDesktopApp {
    const NAME: &'static str = "Windows.System.Inventory.InstalledDesktopApp";
}
unsafe impl ::core::marker::Send for InstalledDesktopApp {}
unsafe impl ::core::marker::Sync for InstalledDesktopApp {}
