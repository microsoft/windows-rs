#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoCapturedEventArgs(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVariablePhotoCapturedEventArgs {
    type Vtable = IVariablePhotoCapturedEventArgs_Vtbl;
}
impl ::core::clone::Clone for IVariablePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVariablePhotoCapturedEventArgs {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd1eb4c5c_1b53_4e4a_8b5c_db7887ac949b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoCapturedEventArgs_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub UsedFrameControllerIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UsedFrameControllerIndex: usize,
    pub CapturedFrameControlValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVariablePhotoSequenceCapture {
    type Vtable = IVariablePhotoSequenceCapture_Vtbl;
}
impl ::core::clone::Clone for IVariablePhotoSequenceCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVariablePhotoSequenceCapture {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xd0112d1d_031e_4041_a6d6_bd742476a8ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceCapture_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture2(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IVariablePhotoSequenceCapture2 {
    type Vtable = IVariablePhotoSequenceCapture2_Vtbl;
}
impl ::core::clone::Clone for IVariablePhotoSequenceCapture2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IVariablePhotoSequenceCapture2 {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xfe2c62bc_50b0_43e3_917c_e3b92798942f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceCapture2_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub UpdateSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateSettingsAsync: usize,
}
#[doc = "*Required features: `\"Media_Capture_Core\"`*"]
#[repr(transparent)]
pub struct VariablePhotoCapturedEventArgs(::windows_core::IUnknown);
impl VariablePhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows_core::Result<super::CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Frame)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTimeOffset(&self) -> ::windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CaptureTimeOffset)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UsedFrameControllerIndex(&self) -> ::windows_core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UsedFrameControllerIndex)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn CapturedFrameControlValues(&self) -> ::windows_core::Result<super::CapturedFrameControlValues> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).CapturedFrameControlValues)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VariablePhotoCapturedEventArgs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoCapturedEventArgs {}
impl ::core::fmt::Debug for VariablePhotoCapturedEventArgs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VariablePhotoCapturedEventArgs").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VariablePhotoCapturedEventArgs {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs;{d1eb4c5c-1b53-4e4a-8b5c-db7887ac949b})");
}
impl ::core::clone::Clone for VariablePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VariablePhotoCapturedEventArgs {
    type Vtable = IVariablePhotoCapturedEventArgs_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VariablePhotoCapturedEventArgs {
    const IID: ::windows_core::GUID = <IVariablePhotoCapturedEventArgs as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VariablePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs";
}
::windows_core::imp::interface_hierarchy!(VariablePhotoCapturedEventArgs, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for VariablePhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for VariablePhotoCapturedEventArgs {}
#[doc = "*Required features: `\"Media_Capture_Core\"`*"]
#[repr(transparent)]
pub struct VariablePhotoSequenceCapture(::windows_core::IUnknown);
impl VariablePhotoSequenceCapture {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StartAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).StopAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).FinishAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhotoCaptured<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, VariablePhotoCapturedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).PhotoCaptured)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePhotoCaptured(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemovePhotoCaptured)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped<P0>(&self, handler: P0) -> ::windows_core::Result<super::super::super::Foundation::EventRegistrationToken>
    where
        P0: ::windows_core::IntoParam<super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, ::windows_core::IInspectable>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Stopped)(::windows_core::Interface::as_raw(this), handler.into_param().abi(), &mut result__).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows_core::Result<()> {
        let this = self;
        unsafe { (::windows_core::Interface::vtable(this).RemoveStopped)(::windows_core::Interface::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateSettingsAsync(&self) -> ::windows_core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows_core::ComInterface::cast::<IVariablePhotoSequenceCapture2>(self)?;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).UpdateSettingsAsync)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for VariablePhotoSequenceCapture {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for VariablePhotoSequenceCapture {}
impl ::core::fmt::Debug for VariablePhotoSequenceCapture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VariablePhotoSequenceCapture").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for VariablePhotoSequenceCapture {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Core.VariablePhotoSequenceCapture;{d0112d1d-031e-4041-a6d6-bd742476a8ee})");
}
impl ::core::clone::Clone for VariablePhotoSequenceCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for VariablePhotoSequenceCapture {
    type Vtable = IVariablePhotoSequenceCapture_Vtbl;
}
unsafe impl ::windows_core::ComInterface for VariablePhotoSequenceCapture {
    const IID: ::windows_core::GUID = <IVariablePhotoSequenceCapture as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for VariablePhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.Core.VariablePhotoSequenceCapture";
}
::windows_core::imp::interface_hierarchy!(VariablePhotoSequenceCapture, ::windows_core::IUnknown, ::windows_core::IInspectable);
