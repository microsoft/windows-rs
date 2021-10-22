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
pub struct IKnownRemoteSystemCapabilitiesStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IKnownRemoteSystemCapabilitiesStatics {
    type Vtable = IKnownRemoteSystemCapabilitiesStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2164843392,
        32650,
        17636,
        [146, 205, 3, 182, 70, 155, 148, 163],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnownRemoteSystemCapabilitiesStatics_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystem(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem {
    type Vtable = IRemoteSystem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3981981901,
        7696,
        19084,
        [180, 166, 78, 95, 214, 249, 119, 33],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut RemoteSystemStatus,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
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
pub struct IRemoteSystem2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem2 {
    type Vtable = IRemoteSystem2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        165668076,
        64395,
        18952,
        [167, 88, 104, 118, 67, 93, 118, 158],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem2_abi(
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
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        capabilityname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystem3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem3 {
    type Vtable = IRemoteSystem3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1924445333,
        47046,
        16574,
        [131, 27, 115, 86, 47, 18, 255, 168],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem3_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystem4(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem4 {
    type Vtable = IRemoteSystem4_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4049928165,
        47495,
        19621,
        [153, 38, 250, 4, 56, 190, 98, 115],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem4_abi(
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
        result__: *mut RemoteSystemPlatform,
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
pub struct IRemoteSystem5(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem5 {
    type Vtable = IRemoteSystem5_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3945453347,
        58850,
        19170,
        [167, 167, 161, 9, 122, 9, 142, 144],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem5_abi(
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
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystem6(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystem6 {
    type Vtable = IRemoteSystem6_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3570248002,
        49191,
        21310,
        [147, 132, 58, 25, 180, 247, 238, 243],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystem6_abi(
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
pub struct IRemoteSystemAddedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2402899471,
        58676,
        18071,
        [136, 54, 122, 190, 161, 81, 81, 110],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAddedEventArgs_abi(
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
pub struct IRemoteSystemApp(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemApp {
    type Vtable = IRemoteSystemApp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2162539709,
        54605,
        16817,
        [155, 22, 104, 16, 168, 113, 237, 79],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemApp_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut bool,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemApp2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemApp2 {
    type Vtable = IRemoteSystemApp2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1667874581,
        2710,
        22394,
        [143, 246, 195, 89, 4, 223, 168, 243],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemApp2_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystemAppRegistration(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3027847093,
        28725,
        19034,
        [184, 223, 150, 45, 143, 132, 49, 244],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistration_abi(
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
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
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
pub struct IRemoteSystemAppRegistrationStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAppRegistrationStatics {
    type Vtable = IRemoteSystemAppRegistrationStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        28940352,
        53202,
        17727,
        [174, 37, 194, 83, 159, 8, 106, 253],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAppRegistrationStatics_abi(
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        user: ::windows::runtime::RawPtr,
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
pub struct IRemoteSystemAuthorizationKindFilter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1796071054,
        1232,
        16628,
        [162, 127, 194, 172, 187, 214, 183, 52],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilter_abi(
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
        result__: *mut RemoteSystemAuthorizationKind,
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
pub struct IRemoteSystemAuthorizationKindFilterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemAuthorizationKindFilterFactory {
    type Vtable = IRemoteSystemAuthorizationKindFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2909134669,
        46698,
        17828,
        [129, 119, 140, 174, 215, 93, 158, 90],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemAuthorizationKindFilterFactory_abi(
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
        remotesystemauthorizationkind: RemoteSystemAuthorizationKind,
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
pub struct IRemoteSystemConnectionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        589794243,
        3337,
        21195,
        [156, 106, 238, 210, 148, 11, 238, 67],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfo_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfoStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionInfoStatics {
    type Vtable = IRemoteSystemConnectionInfoStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2894274093,
        26309,
        22231,
        [164, 206, 112, 93, 148, 146, 90, 214],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionInfoStatics_abi(
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
    #[cfg(feature = "ApplicationModel_AppService")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        connection: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "ApplicationModel_AppService"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2230141188,
        36190,
        19826,
        [130, 56, 118, 33, 87, 108, 122, 103],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest_abi(
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
pub struct IRemoteSystemConnectionRequest2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequest2 {
    type Vtable = IRemoteSystemConnectionRequest2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        316632431,
        49148,
        18490,
        [138, 190, 211, 74, 108, 25, 249, 43],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest2_abi(
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
pub struct IRemoteSystemConnectionRequest3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequest3 {
    type Vtable = IRemoteSystemConnectionRequest3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3733373927,
        51660,
        23120,
        [184, 217, 186, 123, 52, 187, 141, 14],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequest3_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystemConnectionRequestFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequestFactory {
    type Vtable = IRemoteSystemConnectionRequestFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2852784672,
        47851,
        17781,
        [181, 48, 129, 11, 185, 120, 99, 52],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestFactory_abi(
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
        remotesystem: ::windows::runtime::RawPtr,
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
pub struct IRemoteSystemConnectionRequestStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequestStatics {
    type Vtable = IRemoteSystemConnectionRequestStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2261390397,
        33300,
        16988,
        [137, 50, 219, 73, 3, 45, 19, 6],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics_abi(
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
        remotesystemapp: ::windows::runtime::RawPtr,
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
pub struct IRemoteSystemConnectionRequestStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemConnectionRequestStatics2 {
    type Vtable = IRemoteSystemConnectionRequestStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1175392295,
        25836,
        22926,
        [168, 0, 79, 46, 229, 141, 239, 25],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemConnectionRequestStatics2_abi(
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
        connectiontoken: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        user: ::windows::runtime::RawPtr,
        connectiontoken: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystemDiscoveryTypeFilter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1121518623,
        61018,
        17370,
        [172, 106, 111, 238, 37, 70, 7, 65],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilter_abi(
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
        result__: *mut RemoteSystemDiscoveryType,
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
pub struct IRemoteSystemDiscoveryTypeFilterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemDiscoveryTypeFilterFactory {
    type Vtable = IRemoteSystemDiscoveryTypeFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2677979539,
        49760,
        16737,
        [146, 242, 156, 2, 31, 35, 254, 93],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemDiscoveryTypeFilterFactory_abi(
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
        discoverytype: RemoteSystemDiscoveryType,
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
pub struct IRemoteSystemEnumerationCompletedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3337108831,
        16432,
        17236,
        [160, 96, 20, 241, 178, 44, 84, 93],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemEnumerationCompletedEventArgs_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IRemoteSystemFilter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemFilter {
    type Vtable = IRemoteSystemFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1245424100,
        39403,
        17899,
        [186, 22, 3, 103, 114, 143, 243, 116],
    );
}
impl IRemoteSystemFilter {}
unsafe impl ::windows::runtime::RuntimeType for IRemoteSystemFilter {
    const SIGNATURE: ::windows::runtime::ConstBuffer =
        ::windows::runtime::ConstBuffer::from_slice(b"{4a3ba9e4-99eb-45eb-ba16-0367728ff374}");
}
impl ::std::convert::From<IRemoteSystemFilter> for ::windows::runtime::IUnknown {
    fn from(value: IRemoteSystemFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IRemoteSystemFilter> for ::windows::runtime::IUnknown {
    fn from(value: &IRemoteSystemFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRemoteSystemFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IRemoteSystemFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<IRemoteSystemFilter> for ::windows::runtime::IInspectable {
    fn from(value: IRemoteSystemFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRemoteSystemFilter> for ::windows::runtime::IInspectable {
    fn from(value: &IRemoteSystemFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for IRemoteSystemFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a IRemoteSystemFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemFilter_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        954321388,
        8899,
        20214,
        [144, 26, 187, 177, 199, 170, 212, 237],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilter_abi(
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
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemKindFilterFactory {
    type Vtable = IRemoteSystemKindFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2717587694,
        39402,
        16572,
        [154, 57, 198, 112, 170, 128, 74, 40],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindFilterFactory_abi(
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
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        remotesystemkinds: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemKindStatics {
    type Vtable = IRemoteSystemKindStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4130436659,
        43796,
        16848,
        [149, 83, 121, 106, 173, 184, 130, 219],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystemKindStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemKindStatics2 {
    type Vtable = IRemoteSystemKindStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3118703568,
        1126,
        18249,
        [145, 232, 101, 249, 209, 154, 150, 165],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemKindStatics2_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystemRemovedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2336036539,
        29446,
        18922,
        [183, 223, 103, 213, 113, 76, 176, 19],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemRemovedEventArgs_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystemSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSession {
    type Vtable = IRemoteSystemSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1766287873,
        39642,
        18703,
        [149, 73, 211, 28, 177, 76, 158, 149],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSession_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        invitee: ::windows::runtime::RawPtr,
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
pub struct IRemoteSystemSessionAddedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3582318420,
        48279,
        19513,
        [153, 180, 190, 202, 118, 224, 76, 63],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionAddedEventArgs_abi(
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
pub struct IRemoteSystemSessionController(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3834326482,
        26656,
        18535,
        [180, 37, 216, 156, 10, 62, 247, 186],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionController_abi(
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
    #[cfg(feature = "Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        pparticipant: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation"))] usize,
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
pub struct IRemoteSystemSessionControllerFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionControllerFactory {
    type Vtable = IRemoteSystemSessionControllerFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3217829739,
        44093,
        16793,
        [130, 205, 102, 112, 167, 115, 239, 46],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionControllerFactory_abi(
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
        displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        displayname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        options: ::windows::runtime::RawPtr,
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
pub struct IRemoteSystemSessionCreationResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2811761346,
        14302,
        17548,
        [139, 131, 163, 10, 163, 196, 234, 214],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionCreationResult_abi(
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
        result__: *mut RemoteSystemSessionCreationStatus,
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
pub struct IRemoteSystemSessionDisconnectedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3725313691,
        30661,
        17948,
        [130, 9, 124, 108, 93, 49, 17, 171],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionDisconnectedEventArgs_abi(
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
        result__: *mut RemoteSystemSessionDisconnectedReason,
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
pub struct IRemoteSystemSessionInfo(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4283299400,
        35594,
        20122,
        [153, 5, 105, 228, 184, 65, 197, 136],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInfo_abi(
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
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
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
pub struct IRemoteSystemSessionInvitation(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1043516561,
        20951,
        18278,
        [161, 33, 37, 81, 108, 59, 130, 148],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitation_abi(
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
pub struct IRemoteSystemSessionInvitationListener(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        150208575,
        48241,
        18913,
        [135, 74, 49, 221, 255, 154, 39, 185],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationListener_abi(
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
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1586907693,
        41229,
        20187,
        [141, 234, 84, 210, 10, 193, 149, 67],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionInvitationReceivedEventArgs_abi(
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
pub struct IRemoteSystemSessionJoinRequest(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        543162472,
        31124,
        17201,
        [134, 209, 216, 157, 136, 37, 133, 238],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequest_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3687468995,
        33465,
        18454,
        [156, 36, 228, 14, 97, 119, 75, 216],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinRequestedEventArgs_abi(
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
pub struct IRemoteSystemSessionJoinResult(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3464175364,
        41022,
        16804,
        [144, 11, 30, 121, 50, 140, 18, 103],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionJoinResult_abi(
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
        result__: *mut RemoteSystemSessionJoinStatus,
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
pub struct IRemoteSystemSessionMessageChannel(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2502218026,
        29657,
        19472,
        [183, 81, 194, 103, 132, 67, 113, 39],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannel_abi(
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
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        messagedata: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        messagedata: ::windows::runtime::RawPtr,
        participant: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        messagedata: ::windows::runtime::RawPtr,
        participants: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Foundation_Collections")))] usize,
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
pub struct IRemoteSystemSessionMessageChannelFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionMessageChannelFactory {
    type Vtable = IRemoteSystemSessionMessageChannelFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        694033482,
        48406,
        17048,
        [183, 206, 65, 84, 130, 176, 225, 29],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionMessageChannelFactory_abi(
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
        session: ::windows::runtime::RawPtr,
        channelname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        session: ::windows::runtime::RawPtr,
        channelname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>,
        reliability: RemoteSystemSessionMessageChannelReliability,
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
pub struct IRemoteSystemSessionOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1947129685,
        33816,
        20225,
        [147, 83, 226, 28, 158, 204, 108, 252],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionOptions_abi(
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
pub struct IRemoteSystemSessionParticipant(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2123367820,
        44281,
        18217,
        [138, 23, 68, 231, 186, 237, 93, 204],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipant_abi(
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
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation_Collections", feature = "Networking")))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3545913304,
        51617,
        19383,
        [182, 176, 121, 187, 145, 173, 249, 61],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantAddedEventArgs_abi(
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
pub struct IRemoteSystemSessionParticipantRemovedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2255417480,
        56936,
        19135,
        [136, 161, 249, 13, 22, 39, 65, 146],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantRemovedEventArgs_abi(
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
pub struct IRemoteSystemSessionParticipantWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3705471692,
        43655,
        19833,
        [182, 204, 68, 89, 179, 233, 32, 117],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionParticipantWatcher_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut RemoteSystemSessionParticipantWatcherStatus,
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
pub struct IRemoteSystemSessionRemovedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2944569678,
        14753,
        19946,
        [157, 99, 67, 121, 141, 91, 187, 208],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionRemovedEventArgs_abi(
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
pub struct IRemoteSystemSessionStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionStatics {
    type Vtable = IRemoteSystemSessionStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2233764255,
        64800,
        17635,
        [149, 101, 231, 90, 59, 20, 198, 110],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionStatics_abi(
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
pub struct IRemoteSystemSessionUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        377966697,
        8990,
        19601,
        [142, 200, 179, 163, 157, 158, 85, 163],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionUpdatedEventArgs_abi(
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
pub struct IRemoteSystemSessionValueSetReceivedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        116594565,
        11685,
        20056,
        [167, 143, 158, 141, 7, 132, 238, 37],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionValueSetReceivedEventArgs_abi(
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
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemSessionWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2147738432,
        3137,
        19042,
        [182, 215, 189, 190, 43, 25, 190, 45],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemSessionWatcher_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut RemoteSystemSessionWatcherStatus,
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
pub struct IRemoteSystemStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatics {
    type Vtable = IRemoteSystemStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2760225682,
        65323,
        19271,
        [190, 98, 116, 63, 47, 20, 15, 48],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics_abi(
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
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        hostname: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Foundation", feature = "Networking")))] usize,
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        filters: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
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
pub struct IRemoteSystemStatics2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatics2 {
    type Vtable = IRemoteSystemStatics2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        211348938,
        28569,
        19538,
        [162, 114, 234, 79, 54, 71, 23, 68],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics2_abi(
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
        kind: RemoteSystemAuthorizationKind,
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
pub struct IRemoteSystemStatics3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatics3 {
    type Vtable = IRemoteSystemStatics3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2576740719,
        2876,
        23237,
        [179, 37, 204, 115, 244, 55, 223, 205],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatics3_abi(
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
        user: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Foundation_Collections")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        user: ::windows::runtime::RawPtr,
        filters: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        205082958,
        52150,
        18295,
        [133, 52, 46, 12, 82, 26, 255, 162],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilter_abi(
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
        result__: *mut RemoteSystemStatusType,
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
pub struct IRemoteSystemStatusTypeFilterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemStatusTypeFilterFactory {
    type Vtable = IRemoteSystemStatusTypeFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        869234938,
        55076,
        16677,
        [172, 122, 141, 40, 30, 68, 201, 73],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemStatusTypeFilterFactory_abi(
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
        remotesystemstatustype: RemoteSystemStatusType,
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
pub struct IRemoteSystemUpdatedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1963130638,
        56267,
        16725,
        [180, 202, 179, 10, 4, 242, 118, 39],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemUpdatedEventArgs_abi(
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
pub struct IRemoteSystemWatcher(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1566575742,
        11271,
        18629,
        [136, 156, 69, 93, 43, 9, 151, 113],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher_abi(
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
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
pub struct IRemoteSystemWatcher2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWatcher2 {
    type Vtable = IRemoteSystemWatcher2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1933797120,
        6602,
        18681,
        [164, 205, 120, 15, 122, 213, 140, 113],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher2_abi(
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
pub struct IRemoteSystemWatcher3(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWatcher3 {
    type Vtable = IRemoteSystemWatcher3_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4154200015,
        43283,
        21971,
        [132, 19, 65, 143, 207, 21, 186, 84],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcher3_abi(
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
pub struct IRemoteSystemWatcherErrorOccurredEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1959118511,
        20756,
        17446,
        [146, 22, 32, 216, 31, 133, 25, 174],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWatcherErrorOccurredEventArgs_abi(
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
        result__: *mut RemoteSystemWatcherError,
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
pub struct IRemoteSystemWebAccountFilter(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1068980339,
        34760,
        23951,
        [151, 126, 246, 159, 150, 214, 114, 56],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilter_abi(
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
    #[cfg(feature = "Security_Credentials")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilterFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IRemoteSystemWebAccountFilterFactory {
    type Vtable = IRemoteSystemWebAccountFilterFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        881469193,
        24397,
        20775,
        [180, 167, 191, 153, 213, 37, 43, 27],
    );
}
#[repr(C)]
#[doc(hidden)]
pub struct IRemoteSystemWebAccountFilterFactory_abi(
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
    #[cfg(feature = "Security_Credentials")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        account: ::windows::runtime::RawPtr,
        result__: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Security_Credentials"))] usize,
);
pub struct KnownRemoteSystemCapabilities {}
impl KnownRemoteSystemCapabilities {
    pub fn AppService() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn LaunchUri() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn RemoteSession() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn SpatialEntity() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IKnownRemoteSystemCapabilitiesStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IKnownRemoteSystemCapabilitiesStatics<
        R,
        F: FnOnce(&IKnownRemoteSystemCapabilitiesStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            KnownRemoteSystemCapabilities,
            IKnownRemoteSystemCapabilitiesStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for KnownRemoteSystemCapabilities {
    const NAME: &'static str = "Windows.System.RemoteSystems.KnownRemoteSystemCapabilities";
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystem(::windows::runtime::IInspectable);
impl RemoteSystem {
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn Kind(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemStatus>(result__)
        }
    }
    pub fn IsAvailableByProximity(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Networking"))]
    pub fn FindByHostNameAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Networking::HostName>,
    >(
        hostname: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<RemoteSystem>> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                hostname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<RemoteSystem>>(result__)
        })
    }
    pub fn CreateWatcher() -> ::windows::runtime::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithFilters<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>,
        >,
    >(
        filters: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                filters.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation")]
    pub fn RequestAccessAsync() -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>,
    > {
        Self::IRemoteSystemStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<RemoteSystemAccessStatus>>(
                result__,
            )
        })
    }
    pub fn IsAvailableBySpatialProximity(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetCapabilitySupportedAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        &self,
        capabilityname: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                capabilityname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn IsAuthorizationKindEnabled(
        kind: RemoteSystemAuthorizationKind,
    ) -> ::windows::runtime::Result<bool> {
        Self::IRemoteSystemStatics2(|this| unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                kind,
                &mut result__,
            )
            .from_abi::<bool>(result__)
        })
    }
    pub fn ManufacturerDisplayName(
        &self,
    ) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem3>(self)?;
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
    pub fn ModelDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem3>(self)?;
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
    pub fn Platform(&self) -> ::windows::runtime::Result<RemoteSystemPlatform> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem4>(self)?;
        unsafe {
            let mut result__: RemoteSystemPlatform = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemPlatform>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Apps(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<RemoteSystemApp>,
    > {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem5>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<RemoteSystemApp>>(
                result__,
            )
        }
    }
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystem6>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::User>(result__)
        }
    }
    pub fn CreateWatcherForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(
        user: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                user.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn CreateWatcherWithFiltersForUser<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::User>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<IRemoteSystemFilter>,
        >,
    >(
        user: Param0,
        filters: Param1,
    ) -> ::windows::runtime::Result<RemoteSystemWatcher> {
        Self::IRemoteSystemStatics3(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                user.into_param().abi(),
                filters.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemWatcher>(result__)
        })
    }
    pub fn IRemoteSystemStatics<
        R,
        F: FnOnce(&IRemoteSystemStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystem, IRemoteSystemStatics> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemStatics2<
        R,
        F: FnOnce(&IRemoteSystemStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystem, IRemoteSystemStatics2> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemStatics3<
        R,
        F: FnOnce(&IRemoteSystemStatics3) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RemoteSystem, IRemoteSystemStatics3> =
            ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystem {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.System.RemoteSystems.RemoteSystem;{ed5838cd-1e10-4a8c-b4a6-4e5fd6f97721})",
    );
}
unsafe impl ::windows::runtime::Interface for RemoteSystem {
    type Vtable = IRemoteSystem_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3981981901,
        7696,
        19084,
        [180, 166, 78, 95, 214, 249, 119, 33],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystem {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystem";
}
impl ::std::convert::From<RemoteSystem> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystem) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystem> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystem) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RemoteSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystem> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystem) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystem> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystem) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RemoteSystem {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystem {}
unsafe impl ::std::marker::Sync for RemoteSystem {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemAccessStatus(pub i32);
impl RemoteSystemAccessStatus {
    pub const Unspecified: RemoteSystemAccessStatus = RemoteSystemAccessStatus(0i32);
    pub const Allowed: RemoteSystemAccessStatus = RemoteSystemAccessStatus(1i32);
    pub const DeniedByUser: RemoteSystemAccessStatus = RemoteSystemAccessStatus(2i32);
    pub const DeniedBySystem: RemoteSystemAccessStatus = RemoteSystemAccessStatus(3i32);
}
impl ::std::convert::From<i32> for RemoteSystemAccessStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemAccessStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAccessStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemAccessStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemAddedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemAddedEventArgs {
    pub fn RemoteSystem(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAddedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemAddedEventArgs;{8f39560f-e534-4697-8836-7abea151516e})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemAddedEventArgs {
    type Vtable = IRemoteSystemAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2402899471,
        58676,
        18071,
        [136, 54, 122, 190, 161, 81, 81, 110],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAddedEventArgs";
}
impl ::std::convert::From<RemoteSystemAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemAddedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemAddedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemAddedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemAddedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemAddedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemApp(::windows::runtime::IInspectable);
impl RemoteSystemApp {
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn IsAvailableByProximity(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn IsAvailableBySpatialProximity(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<bool>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attributes(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMapView<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMapView<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemApp2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::User>(result__)
        }
    }
    pub fn ConnectionToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemApp2>(self)?;
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
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemApp {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"rc(Windows.System.RemoteSystems.RemoteSystemApp;{80e5bcbd-d54d-41b1-9b16-6810a871ed4f})",
    );
}
unsafe impl ::windows::runtime::Interface for RemoteSystemApp {
    type Vtable = IRemoteSystemApp_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2162539709,
        54605,
        16817,
        [155, 22, 104, 16, 168, 113, 237, 79],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemApp {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemApp";
}
impl ::std::convert::From<RemoteSystemApp> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemApp) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemApp> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemApp) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RemoteSystemApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemApp> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemApp) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemApp> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemApp) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RemoteSystemApp {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemApp
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemApp {}
unsafe impl ::std::marker::Sync for RemoteSystemApp {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemAppRegistration(::windows::runtime::IInspectable);
impl RemoteSystemAppRegistration {
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::User>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Attributes(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IMap<
            ::windows::runtime::HSTRING,
            ::windows::runtime::HSTRING,
        >,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IMap<
                ::windows::runtime::HSTRING,
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SaveAsync(
        &self,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn GetDefault() -> ::windows::runtime::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemAppRegistration>(result__)
        })
    }
    pub fn GetForUser<'a, Param0: ::windows::runtime::IntoParam<'a, super::User>>(
        user: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemAppRegistration> {
        Self::IRemoteSystemAppRegistrationStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                user.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemAppRegistration>(result__)
        })
    }
    pub fn IRemoteSystemAppRegistrationStatics<
        R,
        F: FnOnce(&IRemoteSystemAppRegistrationStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemAppRegistration,
            IRemoteSystemAppRegistrationStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAppRegistration {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemAppRegistration;{b47947b5-7035-4a5a-b8df-962d8f8431f4})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemAppRegistration {
    type Vtable = IRemoteSystemAppRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3027847093,
        28725,
        19034,
        [184, 223, 150, 45, 143, 132, 49, 244],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemAppRegistration {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAppRegistration";
}
impl ::std::convert::From<RemoteSystemAppRegistration> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemAppRegistration) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemAppRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemAppRegistration) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemAppRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemAppRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemAppRegistration> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemAppRegistration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemAppRegistration> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemAppRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemAppRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemAppRegistration
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemAppRegistration {}
unsafe impl ::std::marker::Sync for RemoteSystemAppRegistration {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemAuthorizationKind(pub i32);
impl RemoteSystemAuthorizationKind {
    pub const SameUser: RemoteSystemAuthorizationKind = RemoteSystemAuthorizationKind(0i32);
    pub const Anonymous: RemoteSystemAuthorizationKind = RemoteSystemAuthorizationKind(1i32);
}
impl ::std::convert::From<i32> for RemoteSystemAuthorizationKind {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemAuthorizationKind {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAuthorizationKind {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemAuthorizationKind;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemAuthorizationKindFilter(::windows::runtime::IInspectable);
impl RemoteSystemAuthorizationKindFilter {
    pub fn RemoteSystemAuthorizationKind(
        &self,
    ) -> ::windows::runtime::Result<RemoteSystemAuthorizationKind> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemAuthorizationKind = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemAuthorizationKind>(result__)
        }
    }
    pub fn Create(
        remotesystemauthorizationkind: RemoteSystemAuthorizationKind,
    ) -> ::windows::runtime::Result<RemoteSystemAuthorizationKindFilter> {
        Self::IRemoteSystemAuthorizationKindFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                remotesystemauthorizationkind,
                &mut result__,
            )
            .from_abi::<RemoteSystemAuthorizationKindFilter>(result__)
        })
    }
    pub fn IRemoteSystemAuthorizationKindFilterFactory<
        R,
        F: FnOnce(&IRemoteSystemAuthorizationKindFilterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemAuthorizationKindFilter,
            IRemoteSystemAuthorizationKindFilterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemAuthorizationKindFilter {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter;{6b0dde8e-04d0-40f4-a27f-c2acbbd6b734})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemAuthorizationKindFilter {
    type Vtable = IRemoteSystemAuthorizationKindFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1796071054,
        1232,
        16628,
        [162, 127, 194, 172, 187, 214, 183, 52],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemAuthorizationKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemAuthorizationKindFilter";
}
impl ::std::convert::From<RemoteSystemAuthorizationKindFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemAuthorizationKindFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemAuthorizationKindFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemAuthorizationKindFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemAuthorizationKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemAuthorizationKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemAuthorizationKindFilter>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemAuthorizationKindFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemAuthorizationKindFilter>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemAuthorizationKindFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemAuthorizationKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemAuthorizationKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RemoteSystemAuthorizationKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemAuthorizationKindFilter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RemoteSystemAuthorizationKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemAuthorizationKindFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter>
    for RemoteSystemAuthorizationKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter>
    for &RemoteSystemAuthorizationKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::std::convert::TryInto::<IRemoteSystemFilter>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemAuthorizationKindFilter {}
unsafe impl ::std::marker::Sync for RemoteSystemAuthorizationKindFilter {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemConnectionInfo(::windows::runtime::IInspectable);
impl RemoteSystemConnectionInfo {
    pub fn IsProximal(&self) -> ::windows::runtime::Result<bool> {
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
    #[cfg(feature = "ApplicationModel_AppService")]
    pub fn TryCreateFromAppServiceConnection<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::ApplicationModel::AppService::AppServiceConnection,
        >,
    >(
        connection: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemConnectionInfo> {
        Self::IRemoteSystemConnectionInfoStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                connection.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemConnectionInfo>(result__)
        })
    }
    pub fn IRemoteSystemConnectionInfoStatics<
        R,
        F: FnOnce(&IRemoteSystemConnectionInfoStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemConnectionInfo,
            IRemoteSystemConnectionInfoStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemConnectionInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemConnectionInfo;{23278bc3-0d09-52cb-9c6a-eed2940bee43})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemConnectionInfo {
    type Vtable = IRemoteSystemConnectionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        589794243,
        3337,
        21195,
        [156, 106, 238, 210, 148, 11, 238, 67],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemConnectionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionInfo";
}
impl ::std::convert::From<RemoteSystemConnectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemConnectionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemConnectionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemConnectionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemConnectionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemConnectionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemConnectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemConnectionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemConnectionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemConnectionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemConnectionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemConnectionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemConnectionInfo {}
unsafe impl ::std::marker::Sync for RemoteSystemConnectionInfo {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemConnectionRequest(::windows::runtime::IInspectable);
impl RemoteSystemConnectionRequest {
    pub fn RemoteSystem(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystem>(result__)
        }
    }
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystem>>(
        remotesystem: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                remotesystem.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn RemoteSystemApp(&self) -> ::windows::runtime::Result<RemoteSystemApp> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemConnectionRequest2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemApp>(result__)
        }
    }
    pub fn CreateForApp<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystemApp>>(
        remotesystemapp: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                remotesystemapp.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn ConnectionToken(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemConnectionRequest3>(self)?;
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
    pub fn CreateFromConnectionToken<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        connectiontoken: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                connectiontoken.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn CreateFromConnectionTokenForUser<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::User>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        user: Param0,
        connectiontoken: Param1,
    ) -> ::windows::runtime::Result<RemoteSystemConnectionRequest> {
        Self::IRemoteSystemConnectionRequestStatics2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                user.into_param().abi(),
                connectiontoken.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemConnectionRequest>(result__)
        })
    }
    pub fn IRemoteSystemConnectionRequestFactory<
        R,
        F: FnOnce(&IRemoteSystemConnectionRequestFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemConnectionRequest,
            IRemoteSystemConnectionRequestFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemConnectionRequestStatics<
        R,
        F: FnOnce(&IRemoteSystemConnectionRequestStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemConnectionRequest,
            IRemoteSystemConnectionRequestStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemConnectionRequestStatics2<
        R,
        F: FnOnce(&IRemoteSystemConnectionRequestStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemConnectionRequest,
            IRemoteSystemConnectionRequestStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemConnectionRequest {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemConnectionRequest;{84ed4104-8d5e-4d72-8238-7621576c7a67})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemConnectionRequest {
    type Vtable = IRemoteSystemConnectionRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2230141188,
        36190,
        19826,
        [130, 56, 118, 33, 87, 108, 122, 103],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemConnectionRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemConnectionRequest";
}
impl ::std::convert::From<RemoteSystemConnectionRequest> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemConnectionRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemConnectionRequest> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemConnectionRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemConnectionRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemConnectionRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemConnectionRequest> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemConnectionRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemConnectionRequest> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemConnectionRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemConnectionRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemConnectionRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemConnectionRequest {}
unsafe impl ::std::marker::Sync for RemoteSystemConnectionRequest {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemDiscoveryType(pub i32);
impl RemoteSystemDiscoveryType {
    pub const Any: RemoteSystemDiscoveryType = RemoteSystemDiscoveryType(0i32);
    pub const Proximal: RemoteSystemDiscoveryType = RemoteSystemDiscoveryType(1i32);
    pub const Cloud: RemoteSystemDiscoveryType = RemoteSystemDiscoveryType(2i32);
    pub const SpatiallyProximal: RemoteSystemDiscoveryType = RemoteSystemDiscoveryType(3i32);
}
impl ::std::convert::From<i32> for RemoteSystemDiscoveryType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemDiscoveryType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemDiscoveryType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemDiscoveryType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemDiscoveryTypeFilter(::windows::runtime::IInspectable);
impl RemoteSystemDiscoveryTypeFilter {
    pub fn RemoteSystemDiscoveryType(
        &self,
    ) -> ::windows::runtime::Result<RemoteSystemDiscoveryType> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemDiscoveryType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemDiscoveryType>(result__)
        }
    }
    pub fn Create(
        discoverytype: RemoteSystemDiscoveryType,
    ) -> ::windows::runtime::Result<RemoteSystemDiscoveryTypeFilter> {
        Self::IRemoteSystemDiscoveryTypeFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                discoverytype,
                &mut result__,
            )
            .from_abi::<RemoteSystemDiscoveryTypeFilter>(result__)
        })
    }
    pub fn IRemoteSystemDiscoveryTypeFilterFactory<
        R,
        F: FnOnce(&IRemoteSystemDiscoveryTypeFilterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemDiscoveryTypeFilter,
            IRemoteSystemDiscoveryTypeFilterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemDiscoveryTypeFilter {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter;{42d9041f-ee5a-43da-ac6a-6fee25460741})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemDiscoveryTypeFilter {
    type Vtable = IRemoteSystemDiscoveryTypeFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1121518623,
        61018,
        17370,
        [172, 106, 111, 238, 37, 70, 7, 65],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemDiscoveryTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemDiscoveryTypeFilter";
}
impl ::std::convert::From<RemoteSystemDiscoveryTypeFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemDiscoveryTypeFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemDiscoveryTypeFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemDiscoveryTypeFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemDiscoveryTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemDiscoveryTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemDiscoveryTypeFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemDiscoveryTypeFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemDiscoveryTypeFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemDiscoveryTypeFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemDiscoveryTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemDiscoveryTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RemoteSystemDiscoveryTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemDiscoveryTypeFilter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RemoteSystemDiscoveryTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemDiscoveryTypeFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter>
    for RemoteSystemDiscoveryTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter>
    for &RemoteSystemDiscoveryTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::std::convert::TryInto::<IRemoteSystemFilter>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemDiscoveryTypeFilter {}
unsafe impl ::std::marker::Sync for RemoteSystemDiscoveryTypeFilter {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemEnumerationCompletedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemEnumerationCompletedEventArgs {}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemEnumerationCompletedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs;{c6e83d5f-4030-4354-a060-14f1b22c545d})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemEnumerationCompletedEventArgs {
    type Vtable = IRemoteSystemEnumerationCompletedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3337108831,
        16432,
        17236,
        [160, 96, 20, 241, 178, 44, 84, 93],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemEnumerationCompletedEventArgs {
    const NAME: &'static str =
        "Windows.System.RemoteSystems.RemoteSystemEnumerationCompletedEventArgs";
}
impl ::std::convert::From<RemoteSystemEnumerationCompletedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: RemoteSystemEnumerationCompletedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemEnumerationCompletedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RemoteSystemEnumerationCompletedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemEnumerationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemEnumerationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemEnumerationCompletedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemEnumerationCompletedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemEnumerationCompletedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemEnumerationCompletedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemEnumerationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemEnumerationCompletedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemEnumerationCompletedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemEnumerationCompletedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemKindFilter(::windows::runtime::IInspectable);
impl RemoteSystemKindFilter {
    #[cfg(feature = "Foundation_Collections")]
    pub fn RemoteSystemKinds(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<::windows::runtime::HSTRING>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                ::windows::runtime::HSTRING,
            >>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<::windows::runtime::HSTRING>,
        >,
    >(
        remotesystemkinds: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemKindFilter> {
        Self::IRemoteSystemKindFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                remotesystemkinds.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemKindFilter>(result__)
        })
    }
    pub fn IRemoteSystemKindFilterFactory<
        R,
        F: FnOnce(&IRemoteSystemKindFilterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemKindFilter,
            IRemoteSystemKindFilterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemKindFilter {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemKindFilter;{38e1c9ec-22c3-4ef6-901a-bbb1c7aad4ed})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemKindFilter {
    type Vtable = IRemoteSystemKindFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        954321388,
        8899,
        20214,
        [144, 26, 187, 177, 199, 170, 212, 237],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemKindFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKindFilter";
}
impl ::std::convert::From<RemoteSystemKindFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemKindFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemKindFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemKindFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemKindFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemKindFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemKindFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemKindFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemKindFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RemoteSystemKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemKindFilter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RemoteSystemKindFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemKindFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemKindFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::std::convert::TryInto::<IRemoteSystemFilter>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemKindFilter {}
unsafe impl ::std::marker::Sync for RemoteSystemKindFilter {}
pub struct RemoteSystemKinds {}
impl RemoteSystemKinds {
    pub fn Phone() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Hub() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Holographic() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Desktop() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Xbox() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Iot() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Tablet() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn Laptop() -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        Self::IRemoteSystemKindStatics2(|this| unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<::windows::runtime::HSTRING>(result__)
        })
    }
    pub fn IRemoteSystemKindStatics<
        R,
        F: FnOnce(&IRemoteSystemKindStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemKinds,
            IRemoteSystemKindStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IRemoteSystemKindStatics2<
        R,
        F: FnOnce(&IRemoteSystemKindStatics2) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemKinds,
            IRemoteSystemKindStatics2,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for RemoteSystemKinds {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemKinds";
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
pub struct RemoteSystemPlatform(pub i32);
impl RemoteSystemPlatform {
    pub const Unknown: RemoteSystemPlatform = RemoteSystemPlatform(0i32);
    pub const Windows: RemoteSystemPlatform = RemoteSystemPlatform(1i32);
    pub const Android: RemoteSystemPlatform = RemoteSystemPlatform(2i32);
    pub const Ios: RemoteSystemPlatform = RemoteSystemPlatform(3i32);
    pub const Linux: RemoteSystemPlatform = RemoteSystemPlatform(4i32);
}
impl ::std::convert::From<i32> for RemoteSystemPlatform {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemPlatform {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemPlatform {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemPlatform;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemRemovedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemRemovedEventArgs {
    pub fn RemoteSystemId(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemRemovedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs;{8b3d16bb-7306-49ea-b7df-67d5714cb013})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemRemovedEventArgs {
    type Vtable = IRemoteSystemRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2336036539,
        29446,
        18922,
        [183, 223, 103, 213, 113, 76, 176, 19],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemRemovedEventArgs";
}
impl ::std::convert::From<RemoteSystemRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemRemovedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemRemovedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemRemovedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemRemovedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemRemovedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSession(::windows::runtime::IInspectable);
impl RemoteSystemSession {
    #[cfg(feature = "Foundation")]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this =
            &::windows::runtime::Interface::cast::<super::super::Foundation::IClosable>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn ControllerDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    #[cfg(feature = "Foundation")]
    pub fn Disconnected<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSession,
                RemoteSystemSessionDisconnectedEventArgs,
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
    pub fn RemoveDisconnected<
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
    pub fn CreateParticipantWatcher(
        &self,
    ) -> ::windows::runtime::Result<RemoteSystemSessionParticipantWatcher> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionParticipantWatcher>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn SendInvitationAsync<'a, Param0: ::windows::runtime::IntoParam<'a, RemoteSystem>>(
        &self,
        invitee: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                invitee.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    pub fn CreateWatcher() -> ::windows::runtime::Result<RemoteSystemSessionWatcher> {
        Self::IRemoteSystemSessionStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionWatcher>(result__)
        })
    }
    pub fn IRemoteSystemSessionStatics<
        R,
        F: FnOnce(&IRemoteSystemSessionStatics) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemSession,
            IRemoteSystemSessionStatics,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSession {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSession;{69476a01-9ada-490f-9549-d31cb14c9e95})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSession {
    type Vtable = IRemoteSystemSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1766287873,
        39642,
        18703,
        [149, 73, 211, 28, 177, 76, 158, 149],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSession {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSession";
}
impl ::std::convert::From<RemoteSystemSession> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSession> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RemoteSystemSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSession> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSession) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSession> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSession
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSession
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<RemoteSystemSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Foundation")]
impl ::std::convert::TryFrom<&RemoteSystemSession> for super::super::Foundation::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for RemoteSystemSession
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
#[cfg(feature = "Foundation")]
impl<'a> ::windows::runtime::IntoParam<'a, super::super::Foundation::IClosable>
    for &RemoteSystemSession
{
    fn into_param(self) -> ::windows::runtime::Param<'a, super::super::Foundation::IClosable> {
        ::std::convert::TryInto::<super::super::Foundation::IClosable>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSession {}
unsafe impl ::std::marker::Sync for RemoteSystemSession {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionAddedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionAddedEventArgs {
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionAddedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs;{d585d754-bc97-4c39-99b4-beca76e04c3f})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionAddedEventArgs {
    type Vtable = IRemoteSystemSessionAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3582318420,
        48279,
        19513,
        [153, 180, 190, 202, 118, 224, 76, 63],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionAddedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionAddedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionAddedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionAddedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionAddedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionAddedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionAddedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionAddedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionAddedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionController(::windows::runtime::IInspectable);
impl RemoteSystemSessionController {
    #[cfg(feature = "Foundation")]
    pub fn JoinRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionController,
                RemoteSystemSessionJoinRequestedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveJoinRequested<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveParticipantAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, RemoteSystemSessionParticipant>,
    >(
        &self,
        pparticipant: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                pparticipant.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn CreateSessionAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<RemoteSystemSessionCreationResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<
                RemoteSystemSessionCreationResult,
            >>(result__)
        }
    }
    pub fn CreateController<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        displayname: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                displayname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionController>(result__)
        })
    }
    pub fn CreateControllerWithSessionOptions<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
        Param1: ::windows::runtime::IntoParam<'a, RemoteSystemSessionOptions>,
    >(
        displayname: Param0,
        options: Param1,
    ) -> ::windows::runtime::Result<RemoteSystemSessionController> {
        Self::IRemoteSystemSessionControllerFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                displayname.into_param().abi(),
                options.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionController>(result__)
        })
    }
    pub fn IRemoteSystemSessionControllerFactory<
        R,
        F: FnOnce(&IRemoteSystemSessionControllerFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemSessionController,
            IRemoteSystemSessionControllerFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionController {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionController;{e48b2dd2-6820-4867-b425-d89c0a3ef7ba})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionController {
    type Vtable = IRemoteSystemSessionController_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3834326482,
        26656,
        18535,
        [180, 37, 216, 156, 10, 62, 247, 186],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionController {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionController";
}
impl ::std::convert::From<RemoteSystemSessionController> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionController) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionController> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionController) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionController> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionController) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionController> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionController) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionController
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionController {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionController {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionCreationResult(::windows::runtime::IInspectable);
impl RemoteSystemSessionCreationResult {
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemSessionCreationStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionCreationStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionCreationStatus>(result__)
        }
    }
    pub fn Session(&self) -> ::windows::runtime::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSession>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionCreationResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionCreationResult;{a79812c2-37de-448c-8b83-a30aa3c4ead6})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionCreationResult {
    type Vtable = IRemoteSystemSessionCreationResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2811761346,
        14302,
        17548,
        [139, 131, 163, 10, 163, 196, 234, 214],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionCreationResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionCreationResult";
}
impl ::std::convert::From<RemoteSystemSessionCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionCreationResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionCreationResult> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionCreationResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionCreationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionCreationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionCreationResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionCreationResult> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionCreationResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionCreationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionCreationResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionCreationResult {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionCreationResult {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemSessionCreationStatus(pub i32);
impl RemoteSystemSessionCreationStatus {
    pub const Success: RemoteSystemSessionCreationStatus = RemoteSystemSessionCreationStatus(0i32);
    pub const SessionLimitsExceeded: RemoteSystemSessionCreationStatus =
        RemoteSystemSessionCreationStatus(1i32);
    pub const OperationAborted: RemoteSystemSessionCreationStatus =
        RemoteSystemSessionCreationStatus(2i32);
}
impl ::std::convert::From<i32> for RemoteSystemSessionCreationStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionCreationStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionCreationStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemSessionCreationStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionDisconnectedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionDisconnectedEventArgs {
    pub fn Reason(&self) -> ::windows::runtime::Result<RemoteSystemSessionDisconnectedReason> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionDisconnectedReason = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionDisconnectedReason>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionDisconnectedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs;{de0bc69b-77c5-461c-8209-7c6c5d3111ab})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionDisconnectedEventArgs {
    type Vtable = IRemoteSystemSessionDisconnectedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3725313691,
        30661,
        17948,
        [130, 9, 124, 108, 93, 49, 17, 171],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionDisconnectedEventArgs {
    const NAME: &'static str =
        "Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionDisconnectedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: RemoteSystemSessionDisconnectedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionDisconnectedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RemoteSystemSessionDisconnectedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionDisconnectedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionDisconnectedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionDisconnectedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionDisconnectedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionDisconnectedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionDisconnectedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionDisconnectedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionDisconnectedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionDisconnectedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionDisconnectedEventArgs {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemSessionDisconnectedReason(pub i32);
impl RemoteSystemSessionDisconnectedReason {
    pub const SessionUnavailable: RemoteSystemSessionDisconnectedReason =
        RemoteSystemSessionDisconnectedReason(0i32);
    pub const RemovedByController: RemoteSystemSessionDisconnectedReason =
        RemoteSystemSessionDisconnectedReason(1i32);
    pub const SessionClosed: RemoteSystemSessionDisconnectedReason =
        RemoteSystemSessionDisconnectedReason(2i32);
}
impl ::std::convert::From<i32> for RemoteSystemSessionDisconnectedReason {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionDisconnectedReason {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionDisconnectedReason {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemSessionDisconnectedReason;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionInfo(::windows::runtime::IInspectable);
impl RemoteSystemSessionInfo {
    pub fn DisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    pub fn ControllerDisplayName(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
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
    #[cfg(feature = "Foundation")]
    pub fn JoinAsync(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<RemoteSystemSessionJoinResult>>(
                result__,
            )
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionInfo {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInfo;{ff4df648-8b0a-4e9a-9905-69e4b841c588})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionInfo {
    type Vtable = IRemoteSystemSessionInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4283299400,
        35594,
        20122,
        [153, 5, 105, 228, 184, 65, 197, 136],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionInfo {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInfo";
}
impl ::std::convert::From<RemoteSystemSessionInfo> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionInfo> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionInfo> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionInfo> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionInfo
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionInfo {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionInfo {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionInvitation(::windows::runtime::IInspectable);
impl RemoteSystemSessionInvitation {
    pub fn Sender(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystem>(result__)
        }
    }
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionInvitation {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitation;{3e32cc91-51d7-4766-a121-25516c3b8294})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionInvitation {
    type Vtable = IRemoteSystemSessionInvitation_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1043516561,
        20951,
        18278,
        [161, 33, 37, 81, 108, 59, 130, 148],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionInvitation {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitation";
}
impl ::std::convert::From<RemoteSystemSessionInvitation> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionInvitation) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionInvitation> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionInvitation) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionInvitation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionInvitation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionInvitation> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionInvitation) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionInvitation> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionInvitation) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionInvitation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionInvitation
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionInvitation {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionInvitation {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionInvitationListener(::windows::runtime::IInspectable);
impl RemoteSystemSessionInvitationListener {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemSessionInvitationListener,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[cfg(feature = "Foundation")]
    pub fn InvitationReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionInvitationListener,
                RemoteSystemSessionInvitationReceivedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveInvitationReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionInvitationListener {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener;{08f4003f-bc71-49e1-874a-31ddff9a27b9})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionInvitationListener {
    type Vtable = IRemoteSystemSessionInvitationListener_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        150208575,
        48241,
        18913,
        [135, 74, 49, 221, 255, 154, 39, 185],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionInvitationListener {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionInvitationListener";
}
impl ::std::convert::From<RemoteSystemSessionInvitationListener> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionInvitationListener) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionInvitationListener> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionInvitationListener) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionInvitationListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionInvitationListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionInvitationListener>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionInvitationListener) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionInvitationListener>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionInvitationListener) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionInvitationListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionInvitationListener
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionInvitationListener {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionInvitationListener {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionInvitationReceivedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionInvitationReceivedEventArgs {
    pub fn Invitation(&self) -> ::windows::runtime::Result<RemoteSystemSessionInvitation> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionInvitation>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionInvitationReceivedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs;{5e964a2d-a10d-4edb-8dea-54d20ac19543})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionInvitationReceivedEventArgs {
    type Vtable = IRemoteSystemSessionInvitationReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1586907693,
        41229,
        20187,
        [141, 234, 84, 210, 10, 193, 149, 67],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionInvitationReceivedEventArgs {
    const NAME: &'static str =
        "Windows.System.RemoteSystems.RemoteSystemSessionInvitationReceivedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionInvitationReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionInvitationReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionInvitationReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionInvitationReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionInvitationReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionInvitationReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionInvitationReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionInvitationReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionInvitationReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionInvitationReceivedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionInvitationReceivedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionJoinRequest(::windows::runtime::IInspectable);
impl RemoteSystemSessionJoinRequest {
    pub fn Participant(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
    pub fn Accept(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionJoinRequest {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest;{20600068-7994-4331-86d1-d89d882585ee})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionJoinRequest {
    type Vtable = IRemoteSystemSessionJoinRequest_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        543162472,
        31124,
        17201,
        [134, 209, 216, 157, 136, 37, 133, 238],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionJoinRequest {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequest";
}
impl ::std::convert::From<RemoteSystemSessionJoinRequest> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionJoinRequest) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionJoinRequest> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionJoinRequest) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionJoinRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionJoinRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionJoinRequest> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionJoinRequest) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionJoinRequest> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionJoinRequest) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionJoinRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionJoinRequest
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionJoinRequest {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionJoinRequest {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionJoinRequestedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionJoinRequestedEventArgs {
    pub fn JoinRequest(&self) -> ::windows::runtime::Result<RemoteSystemSessionJoinRequest> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionJoinRequest>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn GetDeferral(&self) -> ::windows::runtime::Result<super::super::Foundation::Deferral> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Deferral>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionJoinRequestedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs;{dbca4fc3-82b9-4816-9c24-e40e61774bd8})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionJoinRequestedEventArgs {
    type Vtable = IRemoteSystemSessionJoinRequestedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3687468995,
        33465,
        18454,
        [156, 36, 228, 14, 97, 119, 75, 216],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionJoinRequestedEventArgs {
    const NAME: &'static str =
        "Windows.System.RemoteSystems.RemoteSystemSessionJoinRequestedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionJoinRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionJoinRequestedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionJoinRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionJoinRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionJoinRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionJoinRequestedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionJoinRequestedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionJoinRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionJoinRequestedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionJoinRequestedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionJoinRequestedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionJoinResult(::windows::runtime::IInspectable);
impl RemoteSystemSessionJoinResult {
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemSessionJoinStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionJoinStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionJoinStatus>(result__)
        }
    }
    pub fn Session(&self) -> ::windows::runtime::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSession>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionJoinResult {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionJoinResult;{ce7b1f04-a03e-41a4-900b-1e79328c1267})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionJoinResult {
    type Vtable = IRemoteSystemSessionJoinResult_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3464175364,
        41022,
        16804,
        [144, 11, 30, 121, 50, 140, 18, 103],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionJoinResult {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionJoinResult";
}
impl ::std::convert::From<RemoteSystemSessionJoinResult> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionJoinResult) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionJoinResult> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionJoinResult) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionJoinResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionJoinResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionJoinResult> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionJoinResult) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionJoinResult> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionJoinResult) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionJoinResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionJoinResult
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionJoinResult {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionJoinResult {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemSessionJoinStatus(pub i32);
impl RemoteSystemSessionJoinStatus {
    pub const Success: RemoteSystemSessionJoinStatus = RemoteSystemSessionJoinStatus(0i32);
    pub const SessionLimitsExceeded: RemoteSystemSessionJoinStatus =
        RemoteSystemSessionJoinStatus(1i32);
    pub const OperationAborted: RemoteSystemSessionJoinStatus = RemoteSystemSessionJoinStatus(2i32);
    pub const SessionUnavailable: RemoteSystemSessionJoinStatus =
        RemoteSystemSessionJoinStatus(3i32);
    pub const RejectedByController: RemoteSystemSessionJoinStatus =
        RemoteSystemSessionJoinStatus(4i32);
}
impl ::std::convert::From<i32> for RemoteSystemSessionJoinStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionJoinStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionJoinStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemSessionJoinStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionMessageChannel(::windows::runtime::IInspectable);
impl RemoteSystemSessionMessageChannel {
    pub fn Session(&self) -> ::windows::runtime::Result<RemoteSystemSession> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSession>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn BroadcastValueSetAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>,
    >(
        &self,
        messagedata: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                messagedata.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SendValueSetAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>,
        Param1: ::windows::runtime::IntoParam<'a, RemoteSystemSessionParticipant>,
    >(
        &self,
        messagedata: Param0,
        participant: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                messagedata.into_param().abi(),
                participant.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(all(feature = "Foundation", feature = "Foundation_Collections"))]
    pub fn SendValueSetToParticipantsAsync<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::Collections::ValueSet>,
        Param1: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::Collections::IIterable<RemoteSystemSessionParticipant>,
        >,
    >(
        &self,
        messagedata: Param0,
        participants: Param1,
    ) -> ::windows::runtime::Result<super::super::Foundation::IAsyncOperation<bool>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                messagedata.into_param().abi(),
                participants.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::IAsyncOperation<bool>>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ValueSetReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionMessageChannel,
                RemoteSystemSessionValueSetReceivedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveValueSetReceived<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, RemoteSystemSession>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        session: Param0,
        channelname: Param1,
    ) -> ::windows::runtime::Result<RemoteSystemSessionMessageChannel> {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                session.into_param().abi(),
                channelname.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionMessageChannel>(result__)
        })
    }
    pub fn CreateWithReliability<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, RemoteSystemSession>,
        Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>,
    >(
        session: Param0,
        channelname: Param1,
        reliability: RemoteSystemSessionMessageChannelReliability,
    ) -> ::windows::runtime::Result<RemoteSystemSessionMessageChannel> {
        Self::IRemoteSystemSessionMessageChannelFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                session.into_param().abi(),
                channelname.into_param().abi(),
                reliability,
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionMessageChannel>(result__)
        })
    }
    pub fn IRemoteSystemSessionMessageChannelFactory<
        R,
        F: FnOnce(&IRemoteSystemSessionMessageChannelFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemSessionMessageChannel,
            IRemoteSystemSessionMessageChannelFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionMessageChannel {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel;{9524d12a-73d9-4c10-b751-c26784437127})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionMessageChannel {
    type Vtable = IRemoteSystemSessionMessageChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2502218026,
        29657,
        19472,
        [183, 81, 194, 103, 132, 67, 113, 39],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionMessageChannel {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionMessageChannel";
}
impl ::std::convert::From<RemoteSystemSessionMessageChannel> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionMessageChannel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionMessageChannel> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionMessageChannel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionMessageChannel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionMessageChannel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionMessageChannel> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionMessageChannel) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionMessageChannel> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionMessageChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionMessageChannel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionMessageChannel
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionMessageChannel {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionMessageChannel {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemSessionMessageChannelReliability(pub i32);
impl RemoteSystemSessionMessageChannelReliability {
    pub const Reliable: RemoteSystemSessionMessageChannelReliability =
        RemoteSystemSessionMessageChannelReliability(0i32);
    pub const Unreliable: RemoteSystemSessionMessageChannelReliability =
        RemoteSystemSessionMessageChannelReliability(1i32);
}
impl ::std::convert::From<i32> for RemoteSystemSessionMessageChannelReliability {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionMessageChannelReliability {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionMessageChannelReliability {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemSessionMessageChannelReliability;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionOptions(::windows::runtime::IInspectable);
impl RemoteSystemSessionOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<
        R,
        F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemSessionOptions,
            ::windows::runtime::IActivationFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn IsInviteOnly(&self) -> ::windows::runtime::Result<bool> {
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
    pub fn SetIsInviteOnly(&self, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value)
                .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionOptions {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionOptions;{740ed755-8418-4f01-9353-e21c9ecc6cfc})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionOptions {
    type Vtable = IRemoteSystemSessionOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1947129685,
        33816,
        20225,
        [147, 83, 226, 28, 158, 204, 108, 252],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionOptions {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionOptions";
}
impl ::std::convert::From<RemoteSystemSessionOptions> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionOptions) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionOptions> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionOptions) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionOptions> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionOptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionOptions> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionOptions
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionOptions {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionOptions {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionParticipant(::windows::runtime::IInspectable);
impl RemoteSystemSessionParticipant {
    pub fn RemoteSystem(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystem>(result__)
        }
    }
    #[cfg(all(feature = "Foundation_Collections", feature = "Networking"))]
    pub fn GetHostNames(
        &self,
    ) -> ::windows::runtime::Result<
        super::super::Foundation::Collections::IVectorView<super::super::Networking::HostName>,
    > {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::Collections::IVectorView<
                super::super::Networking::HostName,
            >>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipant {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipant;{7e90058c-acf9-4729-8a17-44e7baed5dcc})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionParticipant {
    type Vtable = IRemoteSystemSessionParticipant_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2123367820,
        44281,
        18217,
        [138, 23, 68, 231, 186, 237, 93, 204],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionParticipant {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipant";
}
impl ::std::convert::From<RemoteSystemSessionParticipant> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionParticipant) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionParticipant> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionParticipant) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionParticipant
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionParticipant
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionParticipant> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionParticipant) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionParticipant> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionParticipant) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionParticipant
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionParticipant
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionParticipant {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionParticipant {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionParticipantAddedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionParticipantAddedEventArgs {
    pub fn Participant(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipantAddedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs;{d35a57d8-c9a1-4bb7-b6b0-79bb91adf93d})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionParticipantAddedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantAddedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3545913304,
        51617,
        19383,
        [182, 176, 121, 187, 145, 173, 249, 61],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionParticipantAddedEventArgs {
    const NAME: &'static str =
        "Windows.System.RemoteSystems.RemoteSystemSessionParticipantAddedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionParticipantAddedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionParticipantAddedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionParticipantAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionParticipantAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionParticipantAddedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionParticipantAddedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionParticipantAddedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionParticipantAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionParticipantAddedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionParticipantAddedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionParticipantAddedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionParticipantRemovedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionParticipantRemovedEventArgs {
    pub fn Participant(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipantRemovedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs;{866ef088-de68-4abf-88a1-f90d16274192})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionParticipantRemovedEventArgs {
    type Vtable = IRemoteSystemSessionParticipantRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2255417480,
        56936,
        19135,
        [136, 161, 249, 13, 22, 39, 65, 146],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionParticipantRemovedEventArgs {
    const NAME: &'static str =
        "Windows.System.RemoteSystems.RemoteSystemSessionParticipantRemovedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionParticipantRemovedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionParticipantRemovedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionParticipantRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionParticipantRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionParticipantRemovedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionParticipantRemovedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionParticipantRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionParticipantRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionParticipantRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionParticipantRemovedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionParticipantRemovedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionParticipantWatcher(::windows::runtime::IInspectable);
impl RemoteSystemSessionParticipantWatcher {
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Status(
        &self,
    ) -> ::windows::runtime::Result<RemoteSystemSessionParticipantWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionParticipantWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionParticipantWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Added<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionParticipantWatcher,
                RemoteSystemSessionParticipantAddedEventArgs,
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
    pub fn RemoveAdded<
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
    #[cfg(feature = "Foundation")]
    pub fn Removed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionParticipantWatcher,
                RemoteSystemSessionParticipantRemovedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionParticipantWatcher,
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
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipantWatcher {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher;{dcdd02cc-aa87-4d79-b6cc-4459b3e92075})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionParticipantWatcher {
    type Vtable = IRemoteSystemSessionParticipantWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        3705471692,
        43655,
        19833,
        [182, 204, 68, 89, 179, 233, 32, 117],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionParticipantWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcher";
}
impl ::std::convert::From<RemoteSystemSessionParticipantWatcher> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionParticipantWatcher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionParticipantWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionParticipantWatcher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionParticipantWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionParticipantWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionParticipantWatcher>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionParticipantWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionParticipantWatcher>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionParticipantWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionParticipantWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionParticipantWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionParticipantWatcher {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionParticipantWatcher {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemSessionParticipantWatcherStatus(pub i32);
impl RemoteSystemSessionParticipantWatcherStatus {
    pub const Created: RemoteSystemSessionParticipantWatcherStatus =
        RemoteSystemSessionParticipantWatcherStatus(0i32);
    pub const Started: RemoteSystemSessionParticipantWatcherStatus =
        RemoteSystemSessionParticipantWatcherStatus(1i32);
    pub const EnumerationCompleted: RemoteSystemSessionParticipantWatcherStatus =
        RemoteSystemSessionParticipantWatcherStatus(2i32);
    pub const Stopping: RemoteSystemSessionParticipantWatcherStatus =
        RemoteSystemSessionParticipantWatcherStatus(3i32);
    pub const Stopped: RemoteSystemSessionParticipantWatcherStatus =
        RemoteSystemSessionParticipantWatcherStatus(4i32);
    pub const Aborted: RemoteSystemSessionParticipantWatcherStatus =
        RemoteSystemSessionParticipantWatcherStatus(5i32);
}
impl ::std::convert::From<i32> for RemoteSystemSessionParticipantWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionParticipantWatcherStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionParticipantWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemSessionParticipantWatcherStatus;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionRemovedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionRemovedEventArgs {
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionRemovedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs;{af82914e-39a1-4dea-9d63-43798d5bbbd0})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionRemovedEventArgs {
    type Vtable = IRemoteSystemSessionRemovedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2944569678,
        14753,
        19946,
        [157, 99, 67, 121, 141, 91, 187, 208],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionRemovedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionRemovedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionRemovedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionRemovedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionRemovedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionRemovedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionRemovedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionRemovedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionRemovedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionRemovedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionRemovedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionRemovedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionUpdatedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionUpdatedEventArgs {
    pub fn SessionInfo(&self) -> ::windows::runtime::Result<RemoteSystemSessionInfo> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionInfo>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionUpdatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs;{16875069-231e-4c91-8ec8-b3a39d9e55a3})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionUpdatedEventArgs {
    type Vtable = IRemoteSystemSessionUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        377966697,
        8990,
        19601,
        [142, 200, 179, 163, 157, 158, 85, 163],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionUpdatedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionUpdatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionUpdatedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionUpdatedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionValueSetReceivedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemSessionValueSetReceivedEventArgs {
    pub fn Sender(&self) -> ::windows::runtime::Result<RemoteSystemSessionParticipant> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionParticipant>(result__)
        }
    }
    #[cfg(feature = "Foundation_Collections")]
    pub fn Message(
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
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionValueSetReceivedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs;{06f31785-2da5-4e58-a78f-9e8d0784ee25})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionValueSetReceivedEventArgs {
    type Vtable = IRemoteSystemSessionValueSetReceivedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        116594565,
        11685,
        20056,
        [167, 143, 158, 141, 7, 132, 238, 37],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionValueSetReceivedEventArgs {
    const NAME: &'static str =
        "Windows.System.RemoteSystems.RemoteSystemSessionValueSetReceivedEventArgs";
}
impl ::std::convert::From<RemoteSystemSessionValueSetReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionValueSetReceivedEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionValueSetReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionValueSetReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionValueSetReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionValueSetReceivedEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemSessionValueSetReceivedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionValueSetReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionValueSetReceivedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionValueSetReceivedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionValueSetReceivedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemSessionWatcher(::windows::runtime::IInspectable);
impl RemoteSystemSessionWatcher {
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Status(&self) -> ::windows::runtime::Result<RemoteSystemSessionWatcherStatus> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemSessionWatcherStatus = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemSessionWatcherStatus>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Added<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionWatcher,
                RemoteSystemSessionAddedEventArgs,
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
    pub fn RemoveAdded<
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
    #[cfg(feature = "Foundation")]
    pub fn Updated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionWatcher,
                RemoteSystemSessionUpdatedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn Removed<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemSessionWatcher,
                RemoteSystemSessionRemovedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).14)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionWatcher {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemSessionWatcher;{8003e340-0c41-4a62-b6d7-bdbe2b19be2d})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemSessionWatcher {
    type Vtable = IRemoteSystemSessionWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        2147738432,
        3137,
        19042,
        [182, 215, 189, 190, 43, 25, 190, 45],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemSessionWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemSessionWatcher";
}
impl ::std::convert::From<RemoteSystemSessionWatcher> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemSessionWatcher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemSessionWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemSessionWatcher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemSessionWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemSessionWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemSessionWatcher> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemSessionWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemSessionWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemSessionWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemSessionWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemSessionWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemSessionWatcher {}
unsafe impl ::std::marker::Sync for RemoteSystemSessionWatcher {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemSessionWatcherStatus(pub i32);
impl RemoteSystemSessionWatcherStatus {
    pub const Created: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(0i32);
    pub const Started: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(1i32);
    pub const EnumerationCompleted: RemoteSystemSessionWatcherStatus =
        RemoteSystemSessionWatcherStatus(2i32);
    pub const Stopping: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(3i32);
    pub const Stopped: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(4i32);
    pub const Aborted: RemoteSystemSessionWatcherStatus = RemoteSystemSessionWatcherStatus(5i32);
}
impl ::std::convert::From<i32> for RemoteSystemSessionWatcherStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemSessionWatcherStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemSessionWatcherStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemSessionWatcherStatus;i4)",
    );
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
pub struct RemoteSystemStatus(pub i32);
impl RemoteSystemStatus {
    pub const Unavailable: RemoteSystemStatus = RemoteSystemStatus(0i32);
    pub const DiscoveringAvailability: RemoteSystemStatus = RemoteSystemStatus(1i32);
    pub const Available: RemoteSystemStatus = RemoteSystemStatus(2i32);
    pub const Unknown: RemoteSystemStatus = RemoteSystemStatus(3i32);
}
impl ::std::convert::From<i32> for RemoteSystemStatus {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemStatus {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemStatus {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemStatus;i4)",
    );
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
pub struct RemoteSystemStatusType(pub i32);
impl RemoteSystemStatusType {
    pub const Any: RemoteSystemStatusType = RemoteSystemStatusType(0i32);
    pub const Available: RemoteSystemStatusType = RemoteSystemStatusType(1i32);
}
impl ::std::convert::From<i32> for RemoteSystemStatusType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemStatusType {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemStatusType {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemStatusType;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemStatusTypeFilter(::windows::runtime::IInspectable);
impl RemoteSystemStatusTypeFilter {
    pub fn RemoteSystemStatusType(&self) -> ::windows::runtime::Result<RemoteSystemStatusType> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemStatusType = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemStatusType>(result__)
        }
    }
    pub fn Create(
        remotesystemstatustype: RemoteSystemStatusType,
    ) -> ::windows::runtime::Result<RemoteSystemStatusTypeFilter> {
        Self::IRemoteSystemStatusTypeFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                remotesystemstatustype,
                &mut result__,
            )
            .from_abi::<RemoteSystemStatusTypeFilter>(result__)
        })
    }
    pub fn IRemoteSystemStatusTypeFilterFactory<
        R,
        F: FnOnce(&IRemoteSystemStatusTypeFilterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemStatusTypeFilter,
            IRemoteSystemStatusTypeFilterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemStatusTypeFilter {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter;{0c39514e-cbb6-4777-8534-2e0c521affa2})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemStatusTypeFilter {
    type Vtable = IRemoteSystemStatusTypeFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        205082958,
        52150,
        18295,
        [133, 52, 46, 12, 82, 26, 255, 162],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemStatusTypeFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemStatusTypeFilter";
}
impl ::std::convert::From<RemoteSystemStatusTypeFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemStatusTypeFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemStatusTypeFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemStatusTypeFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemStatusTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemStatusTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemStatusTypeFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemStatusTypeFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemStatusTypeFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemStatusTypeFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemStatusTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemStatusTypeFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RemoteSystemStatusTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemStatusTypeFilter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RemoteSystemStatusTypeFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemStatusTypeFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemStatusTypeFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::std::convert::TryInto::<IRemoteSystemFilter>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemStatusTypeFilter {}
unsafe impl ::std::marker::Sync for RemoteSystemStatusTypeFilter {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemUpdatedEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemUpdatedEventArgs {
    pub fn RemoteSystem(&self) -> ::windows::runtime::Result<RemoteSystem> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystem>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemUpdatedEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs;{7502ff0e-dbcb-4155-b4ca-b30a04f27627})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemUpdatedEventArgs {
    type Vtable = IRemoteSystemUpdatedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1963130638,
        56267,
        16725,
        [180, 202, 179, 10, 4, 242, 118, 39],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemUpdatedEventArgs {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemUpdatedEventArgs";
}
impl ::std::convert::From<RemoteSystemUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemUpdatedEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemUpdatedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemUpdatedEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemUpdatedEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemUpdatedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemUpdatedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemUpdatedEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemUpdatedEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemUpdatedEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemWatcher(::windows::runtime::IInspectable);
impl RemoteSystemWatcher {
    pub fn Start(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok()
        }
    }
    pub fn Stop(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this)).ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoteSystemAdded<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemWatcher,
                RemoteSystemAddedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteSystemAdded<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoteSystemUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemWatcher,
                RemoteSystemUpdatedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).10)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteSystemUpdated<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).11)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoteSystemRemoved<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemWatcher,
                RemoteSystemRemovedEventArgs,
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
            (::windows::runtime::Interface::vtable(this).12)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveRemoteSystemRemoved<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe {
            (::windows::runtime::Interface::vtable(this).13)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn EnumerationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemWatcher,
                RemoteSystemEnumerationCompletedEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveEnumerationCompleted<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).7)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn ErrorOccurred<
        'a,
        Param0: ::windows::runtime::IntoParam<
            'a,
            super::super::Foundation::TypedEventHandler<
                RemoteSystemWatcher,
                RemoteSystemWatcherErrorOccurredEventArgs,
            >,
        >,
    >(
        &self,
        handler: Param0,
    ) -> ::windows::runtime::Result<super::super::Foundation::EventRegistrationToken> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            let mut result__: super::super::Foundation::EventRegistrationToken =
                ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(
                ::std::mem::transmute_copy(this),
                handler.into_param().abi(),
                &mut result__,
            )
            .from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[cfg(feature = "Foundation")]
    pub fn RemoveErrorOccurred<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::EventRegistrationToken>,
    >(
        &self,
        token: Param0,
    ) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher2>(self)?;
        unsafe {
            (::windows::runtime::Interface::vtable(this).9)(
                ::std::mem::transmute_copy(this),
                token.into_param().abi(),
            )
            .ok()
        }
    }
    pub fn User(&self) -> ::windows::runtime::Result<super::User> {
        let this = &::windows::runtime::Interface::cast::<IRemoteSystemWatcher3>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::User>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemWatcher {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemWatcher;{5d600c7e-2c07-48c5-889c-455d2b099771})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemWatcher {
    type Vtable = IRemoteSystemWatcher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1566575742,
        11271,
        18629,
        [136, 156, 69, 93, 43, 9, 151, 113],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemWatcher {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWatcher";
}
impl ::std::convert::From<RemoteSystemWatcher> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemWatcher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemWatcher> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemWatcher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RemoteSystemWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &RemoteSystemWatcher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemWatcher> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemWatcher) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemWatcher> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemWatcher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemWatcher
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemWatcher {}
unsafe impl ::std::marker::Sync for RemoteSystemWatcher {}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct RemoteSystemWatcherError(pub i32);
impl RemoteSystemWatcherError {
    pub const Unknown: RemoteSystemWatcherError = RemoteSystemWatcherError(0i32);
    pub const InternetNotAvailable: RemoteSystemWatcherError = RemoteSystemWatcherError(1i32);
    pub const AuthenticationError: RemoteSystemWatcherError = RemoteSystemWatcherError(2i32);
}
impl ::std::convert::From<i32> for RemoteSystemWatcherError {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RemoteSystemWatcherError {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemWatcherError {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.System.RemoteSystems.RemoteSystemWatcherError;i4)",
    );
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemWatcherErrorOccurredEventArgs(::windows::runtime::IInspectable);
impl RemoteSystemWatcherErrorOccurredEventArgs {
    pub fn Error(&self) -> ::windows::runtime::Result<RemoteSystemWatcherError> {
        let this = self;
        unsafe {
            let mut result__: RemoteSystemWatcherError = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<RemoteSystemWatcherError>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemWatcherErrorOccurredEventArgs {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs;{74c5c6af-5114-4426-9216-20d81f8519ae})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemWatcherErrorOccurredEventArgs {
    type Vtable = IRemoteSystemWatcherErrorOccurredEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1959118511,
        20756,
        17446,
        [146, 22, 32, 216, 31, 133, 25, 174],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemWatcherErrorOccurredEventArgs {
    const NAME: &'static str =
        "Windows.System.RemoteSystems.RemoteSystemWatcherErrorOccurredEventArgs";
}
impl ::std::convert::From<RemoteSystemWatcherErrorOccurredEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemWatcherErrorOccurredEventArgs>
    for ::windows::runtime::IUnknown
{
    fn from(value: &RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemWatcherErrorOccurredEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemWatcherErrorOccurredEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemWatcherErrorOccurredEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemWatcherErrorOccurredEventArgs>
    for ::windows::runtime::IInspectable
{
    fn from(value: &RemoteSystemWatcherErrorOccurredEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemWatcherErrorOccurredEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemWatcherErrorOccurredEventArgs
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemWatcherErrorOccurredEventArgs {}
unsafe impl ::std::marker::Sync for RemoteSystemWatcherErrorOccurredEventArgs {}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct RemoteSystemWebAccountFilter(::windows::runtime::IInspectable);
impl RemoteSystemWebAccountFilter {
    #[cfg(feature = "Security_Credentials")]
    pub fn Account(
        &self,
    ) -> ::windows::runtime::Result<super::super::Security::Credentials::WebAccount> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                &mut result__,
            )
            .from_abi::<super::super::Security::Credentials::WebAccount>(result__)
        }
    }
    #[cfg(feature = "Security_Credentials")]
    pub fn Create<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Security::Credentials::WebAccount>,
    >(
        account: Param0,
    ) -> ::windows::runtime::Result<RemoteSystemWebAccountFilter> {
        Self::IRemoteSystemWebAccountFilterFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(
                ::std::mem::transmute_copy(this),
                account.into_param().abi(),
                &mut result__,
            )
            .from_abi::<RemoteSystemWebAccountFilter>(result__)
        })
    }
    pub fn IRemoteSystemWebAccountFilterFactory<
        R,
        F: FnOnce(&IRemoteSystemWebAccountFilterFactory) -> ::windows::runtime::Result<R>,
    >(
        callback: F,
    ) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<
            RemoteSystemWebAccountFilter,
            IRemoteSystemWebAccountFilterFactory,
        > = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RemoteSystemWebAccountFilter {
    const SIGNATURE : :: windows :: runtime :: ConstBuffer = :: windows :: runtime :: ConstBuffer :: from_slice ( b"rc(Windows.System.RemoteSystems.RemoteSystemWebAccountFilter;{3fb75873-87c8-5d8f-977e-f69f96d67238})" ) ;
}
unsafe impl ::windows::runtime::Interface for RemoteSystemWebAccountFilter {
    type Vtable = IRemoteSystemWebAccountFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        1068980339,
        34760,
        23951,
        [151, 126, 246, 159, 150, 214, 114, 56],
    );
}
impl ::windows::runtime::RuntimeName for RemoteSystemWebAccountFilter {
    const NAME: &'static str = "Windows.System.RemoteSystems.RemoteSystemWebAccountFilter";
}
impl ::std::convert::From<RemoteSystemWebAccountFilter> for ::windows::runtime::IUnknown {
    fn from(value: RemoteSystemWebAccountFilter) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RemoteSystemWebAccountFilter> for ::windows::runtime::IUnknown {
    fn from(value: &RemoteSystemWebAccountFilter) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for RemoteSystemWebAccountFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>
    for &RemoteSystemWebAccountFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
impl ::std::convert::From<RemoteSystemWebAccountFilter> for ::windows::runtime::IInspectable {
    fn from(value: RemoteSystemWebAccountFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&RemoteSystemWebAccountFilter> for ::windows::runtime::IInspectable {
    fn from(value: &RemoteSystemWebAccountFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for RemoteSystemWebAccountFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable>
    for &'a RemoteSystemWebAccountFilter
{
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::TryFrom<RemoteSystemWebAccountFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: RemoteSystemWebAccountFilter) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&RemoteSystemWebAccountFilter> for IRemoteSystemFilter {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &RemoteSystemWebAccountFilter) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IRemoteSystemFilter> for &RemoteSystemWebAccountFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, IRemoteSystemFilter> {
        ::std::convert::TryInto::<IRemoteSystemFilter>::try_into(self)
            .map(::windows::runtime::Param::Owned)
            .unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for RemoteSystemWebAccountFilter {}
unsafe impl ::std::marker::Sync for RemoteSystemWebAccountFilter {}
