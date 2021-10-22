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
pub struct ISoundLevelBrokerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ISoundLevelBrokerStatics {
    type Vtable = ISoundLevelBrokerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1784887649,
        56301,
        17996,
        [160, 154, 51, 65, 47, 92, 170, 63],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct ISoundLevelBrokerStatics_abi(
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
        result__: *mut super::super::SoundLevel,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        handler: ::windows::runtime::RawPtr,
        result__: *mut super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        token: super::super::super::Foundation::EventRegistrationToken,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
);
pub struct SoundLevelBroker {}
impl SoundLevelBroker {
    pub fn SoundLevel() -> ::windows::runtime::Result<super::super::SoundLevel> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__: super::super::SoundLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::SoundLevel>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn SoundLevelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::super::Foundation::EventHandler<::windows::runtime::IInspectable>,
        >,
    >(
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::super::Foundation::EventRegistrationToken> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            let mut result__: super::super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::super::Foundation::EventRegistrationToken>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveSoundLevelChanged<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::super::Foundation::EventRegistrationToken>,
    >(
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        Self::ISoundLevelBrokerStatics(|this| unsafe {
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        })
    }
    pub fn ISoundLevelBrokerStatics<
        R,
        F: FnOnce(&ISoundLevelBrokerStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            SoundLevelBroker,
            ISoundLevelBrokerStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for SoundLevelBroker {
    const NAME: &'static str = "Windows.Media.Core.Preview.SoundLevelBroker";
}
