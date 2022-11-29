#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoCapturedEventArgs(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVariablePhotoCapturedEventArgs {
    type Vtable = IVariablePhotoCapturedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for IVariablePhotoCapturedEventArgs {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd1eb4c5c_1b53_4e4a_8b5c_db7887ac949b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoCapturedEventArgs_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Foundation")]
    pub CaptureTimeOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    CaptureTimeOffset: usize,
    #[cfg(feature = "Foundation")]
    pub UsedFrameControllerIndex: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UsedFrameControllerIndex: usize,
    pub CapturedFrameControlValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVariablePhotoSequenceCapture {
    type Vtable = IVariablePhotoSequenceCapture_Vtbl;
}
unsafe impl ::windows::core::Interface for IVariablePhotoSequenceCapture {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0112d1d_031e_4041_a6d6_bd742476a8ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceCapture_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub StartAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StartAsync: usize,
    #[cfg(feature = "Foundation")]
    pub StopAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    StopAsync: usize,
    #[cfg(feature = "Foundation")]
    pub FinishAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    FinishAsync: usize,
    #[cfg(feature = "Foundation")]
    pub PhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePhotoCaptured: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePhotoCaptured: usize,
    #[cfg(feature = "Foundation")]
    pub Stopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, handler: *mut ::core::ffi::c_void, result__: *mut super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    Stopped: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveStopped: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveStopped: usize,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture2(::windows::core::IUnknown);
unsafe impl ::windows::core::Vtable for IVariablePhotoSequenceCapture2 {
    type Vtable = IVariablePhotoSequenceCapture2_Vtbl;
}
unsafe impl ::windows::core::Interface for IVariablePhotoSequenceCapture2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe2c62bc_50b0_43e3_917c_e3b92798942f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IVariablePhotoSequenceCapture2_Vtbl {
    pub base__: ::windows::core::IInspectable_Vtbl,
    #[cfg(feature = "Foundation")]
    pub UpdateSettingsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    UpdateSettingsAsync: usize,
}
#[doc = "*Required features: `\"Media_Capture_Core\"`*"]
#[repr(transparent)]
pub struct VariablePhotoCapturedEventArgs(::windows::core::IUnknown);
impl VariablePhotoCapturedEventArgs {
    pub fn Frame(&self) -> ::windows::core::Result<super::CapturedFrame> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Frame)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CaptureTimeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CaptureTimeOffset)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UsedFrameControllerIndex(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UsedFrameControllerIndex)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    pub fn CapturedFrameControlValues(&self) -> ::windows::core::Result<super::CapturedFrameControlValues> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).CapturedFrameControlValues)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for VariablePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for VariablePhotoCapturedEventArgs {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs;{d1eb4c5c-1b53-4e4a-8b5c-db7887ac949b})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VariablePhotoCapturedEventArgs {
    type Vtable = IVariablePhotoCapturedEventArgs_Vtbl;
}
unsafe impl ::windows::core::Interface for VariablePhotoCapturedEventArgs {
    const IID: ::windows::core::GUID = <IVariablePhotoCapturedEventArgs as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VariablePhotoCapturedEventArgs {
    const NAME: &'static str = "Windows.Media.Capture.Core.VariablePhotoCapturedEventArgs";
}
::windows::core::interface_hierarchy!(VariablePhotoCapturedEventArgs, ::windows::core::IUnknown, ::windows::core::IInspectable);
unsafe impl ::core::marker::Send for VariablePhotoCapturedEventArgs {}
unsafe impl ::core::marker::Sync for VariablePhotoCapturedEventArgs {}
#[doc = "*Required features: `\"Media_Capture_Core\"`*"]
#[repr(transparent)]
pub struct VariablePhotoSequenceCapture(::windows::core::IUnknown);
impl VariablePhotoSequenceCapture {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StartAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).StopAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn FinishAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).FinishAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PhotoCaptured(&self, handler: &super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, VariablePhotoCapturedEventArgs>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).PhotoCaptured)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePhotoCaptured(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemovePhotoCaptured)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn Stopped(&self, handler: &super::super::super::Foundation::TypedEventHandler<VariablePhotoSequenceCapture, ::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).Stopped)(::windows::core::Vtable::as_raw(this), ::core::mem::transmute_copy(handler), result__.as_mut_ptr()).from_abi(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveStopped(&self, token: super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Vtable::vtable(this).RemoveStopped)(::windows::core::Vtable::as_raw(this), token).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn UpdateSettingsAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<IVariablePhotoSequenceCapture2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Vtable::vtable(this).UpdateSettingsAsync)(::windows::core::Vtable::as_raw(this), result__.as_mut_ptr()).from_abi(result__)
        }
    }
}
impl ::core::clone::Clone for VariablePhotoSequenceCapture {
    fn clone(&self) -> Self {
        Self(self.0.clone())
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
unsafe impl ::windows::core::RuntimeType for VariablePhotoSequenceCapture {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Media.Capture.Core.VariablePhotoSequenceCapture;{d0112d1d-031e-4041-a6d6-bd742476a8ee})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Vtable for VariablePhotoSequenceCapture {
    type Vtable = IVariablePhotoSequenceCapture_Vtbl;
}
unsafe impl ::windows::core::Interface for VariablePhotoSequenceCapture {
    const IID: ::windows::core::GUID = <IVariablePhotoSequenceCapture as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for VariablePhotoSequenceCapture {
    const NAME: &'static str = "Windows.Media.Capture.Core.VariablePhotoSequenceCapture";
}
::windows::core::interface_hierarchy!(VariablePhotoSequenceCapture, ::windows::core::IUnknown, ::windows::core::IInspectable);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
