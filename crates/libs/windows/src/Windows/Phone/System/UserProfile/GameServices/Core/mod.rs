windows_core::imp::define_interface!(IGameService, IGameService_Vtbl, 0x2e2d5098_48a9_4efc_afd6_8e6da09003fb);
impl windows_core::RuntimeType for IGameService {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameService_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub ServiceUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetGamerProfileAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInstalledGameItemsAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPartnerTokenAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPrivilegesAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GrantAchievement: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GrantAvatarAward: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(feature = "Storage_Streams")]
    pub PostResult: unsafe extern "system" fn(*mut core::ffi::c_void, u32, GameServiceScoreKind, i64, GameServiceGameOutcome, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Storage_Streams"))]
    PostResult: usize,
}
windows_core::imp::define_interface!(IGameService2, IGameService2_Vtbl, 0xd2364ef6_ea17_4be5_8d8a_c860885e051f);
impl windows_core::RuntimeType for IGameService2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameService2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub NotifyPartnerTokenExpired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAuthenticationStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IGameServicePropertyCollection, IGameServicePropertyCollection_Vtbl, 0x07e57fc8_debb_4609_9cc8_529d16bc2bd9);
impl windows_core::RuntimeType for IGameServicePropertyCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
pub struct IGameServicePropertyCollection_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetPropertyAsync: unsafe extern "system" fn(*mut core::ffi::c_void, core::mem::MaybeUninit<windows_core::HSTRING>, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub struct GameService;
impl GameService {}
impl windows_core::RuntimeName for GameService {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameService";
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct GameServicePropertyCollection(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(GameServicePropertyCollection, windows_core::IUnknown, windows_core::IInspectable);
impl GameServicePropertyCollection {
    pub fn GetPropertyAsync(&self, propertyName: &windows_core::HSTRING) -> windows_core::Result<super::super::super::super::super::Foundation::IAsyncOperation<windows_core::IInspectable>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetPropertyAsync)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(propertyName), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
impl windows_core::RuntimeType for GameServicePropertyCollection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IGameServicePropertyCollection>();
}
unsafe impl windows_core::Interface for GameServicePropertyCollection {
    type Vtable = <IGameServicePropertyCollection as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IGameServicePropertyCollection as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for GameServicePropertyCollection {
    const NAME: &'static str = "Windows.Phone.System.UserProfile.GameServices.Core.GameServicePropertyCollection";
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GameServiceGameOutcome(pub i32);
impl GameServiceGameOutcome {
    pub const None: Self = Self(0i32);
    pub const Win: Self = Self(1i32);
    pub const Loss: Self = Self(2i32);
    pub const Tie: Self = Self(3i32);
}
impl windows_core::TypeKind for GameServiceGameOutcome {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameServiceGameOutcome {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceGameOutcome;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct GameServiceScoreKind(pub i32);
impl GameServiceScoreKind {
    pub const Number: Self = Self(0i32);
    pub const Time: Self = Self(1i32);
}
impl windows_core::TypeKind for GameServiceScoreKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for GameServiceScoreKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.Phone.System.UserProfile.GameServices.Core.GameServiceScoreKind;i4)");
}
