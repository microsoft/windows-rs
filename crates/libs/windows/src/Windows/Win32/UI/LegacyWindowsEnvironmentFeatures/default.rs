impl ::core::default::Default for EMPTY_VOLUME_CACHE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for EMPTY_VOLUME_CACHE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EMPTY_VOLUME_CACHE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IADesktopP2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IADesktopP2 {}
impl ::core::fmt::Debug for IADesktopP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IADesktopP2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IActiveDesktopP {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IActiveDesktopP {}
impl ::core::fmt::Debug for IActiveDesktopP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IActiveDesktopP").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IBriefcaseInitiator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IBriefcaseInitiator {}
impl ::core::fmt::Debug for IBriefcaseInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IBriefcaseInitiator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCache {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCache {}
impl ::core::fmt::Debug for IEmptyVolumeCache {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCache").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCache2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCache2 {}
impl ::core::fmt::Debug for IEmptyVolumeCache2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCache2").field(&self.0).finish()
    }
}
impl IEmptyVolumeCache2 {
    #[doc = "*Required features: `\"Win32_System_Registry\"`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub unsafe fn Initialize<P0, P1>(&self, hkregkey: P0, pcwszvolume: P1, ppwszdisplayname: *mut ::windows::core::PWSTR, ppwszdescription: *mut ::windows::core::PWSTR, pdwflags: *mut EMPTY_VOLUME_CACHE_FLAGS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::System::Registry::HKEY>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Initialize)(::windows::core::Vtable::as_raw(self), hkregkey.into(), pcwszvolume.into().abi(), ppwszdisplayname, ppwszdescription, pdwflags).ok()
    }
    pub unsafe fn GetSpaceUsed<P0>(&self, pdwlspaceused: *mut u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetSpaceUsed)(::windows::core::Vtable::as_raw(self), pdwlspaceused, picb.into().abi()).ok()
    }
    pub unsafe fn Purge<P0>(&self, dwlspacetofree: u64, picb: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IEmptyVolumeCacheCallBack>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Purge)(::windows::core::Vtable::as_raw(self), dwlspacetofree, picb.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowProperties<P0>(&self, hwnd: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.ShowProperties)(::windows::core::Vtable::as_raw(self), hwnd.into()).ok()
    }
    pub unsafe fn Deactivate(&self) -> ::windows::core::Result<EMPTY_VOLUME_CACHE_FLAGS> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Deactivate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IEmptyVolumeCacheCallBack {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEmptyVolumeCacheCallBack {}
impl ::core::fmt::Debug for IEmptyVolumeCacheCallBack {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEmptyVolumeCacheCallBack").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IReconcilableObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReconcilableObject {}
impl ::core::fmt::Debug for IReconcilableObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReconcilableObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IReconcileInitiator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReconcileInitiator {}
impl ::core::fmt::Debug for IReconcileInitiator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReconcileInitiator").field(&self.0).finish()
    }
}
impl ::core::default::Default for RECONCILEF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for RECONCILEF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RECONCILEF").field(&self.0).finish()
    }
}
