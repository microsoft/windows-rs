impl ::core::cmp::PartialEq for IMarshal {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshal {}
impl ::core::fmt::Debug for IMarshal {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshal").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMarshal2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshal2 {}
impl ::core::fmt::Debug for IMarshal2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshal2").field(&self.0).finish()
    }
}
impl IMarshal2 {
    pub unsafe fn GetUnmarshalClass(&self, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*mut ::core::ffi::c_void>, mshlflags: u32, pcid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetUnmarshalClass)(::windows::core::Vtable::as_raw(self), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null_mut())), mshlflags, pcid).ok()
    }
    pub unsafe fn GetMarshalSizeMax(&self, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*mut ::core::ffi::c_void>, mshlflags: u32, psize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMarshalSizeMax)(::windows::core::Vtable::as_raw(self), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null_mut())), mshlflags, psize).ok()
    }
    pub unsafe fn MarshalInterface<P0>(&self, pstm: P0, riid: *const ::windows::core::GUID, pv: ::core::option::Option<*const ::core::ffi::c_void>, dwdestcontext: u32, pvdestcontext: ::core::option::Option<*mut ::core::ffi::c_void>, mshlflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.MarshalInterface)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), riid, ::core::mem::transmute(pv.unwrap_or(::std::ptr::null())), dwdestcontext, ::core::mem::transmute(pvdestcontext.unwrap_or(::std::ptr::null_mut())), mshlflags).ok()
    }
    pub unsafe fn UnmarshalInterface<P0>(&self, pstm: P0, riid: *const ::windows::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UnmarshalInterface)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), riid, ppv).ok()
    }
    pub unsafe fn ReleaseMarshalData<P0>(&self, pstm: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReleaseMarshalData)(::windows::core::Vtable::as_raw(self), pstm.into().abi()).ok()
    }
    pub unsafe fn DisconnectObject(&self, dwreserved: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.DisconnectObject)(::windows::core::Vtable::as_raw(self), dwreserved).ok()
    }
}
impl ::core::cmp::PartialEq for IMarshalingStream {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMarshalingStream {}
impl ::core::fmt::Debug for IMarshalingStream {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMarshalingStream").field(&self.0).finish()
    }
}
impl IMarshalingStream {
    pub unsafe fn Read(&self, pv: *mut ::core::ffi::c_void, cb: u32, pcbread: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Read)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Write(&self, pv: *const ::core::ffi::c_void, cb: u32, pcbwritten: ::core::option::Option<*mut u32>) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Write)(::windows::core::Vtable::as_raw(self), pv, cb, ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn Seek(&self, dlibmove: i64, dworigin: super::STREAM_SEEK, plibnewposition: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Seek)(::windows::core::Vtable::as_raw(self), dlibmove, dworigin, ::core::mem::transmute(plibnewposition.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetSize(&self, libnewsize: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSize)(::windows::core::Vtable::as_raw(self), libnewsize).ok()
    }
    pub unsafe fn CopyTo<P0>(&self, pstm: P0, cb: u64, pcbread: ::core::option::Option<*mut u64>, pcbwritten: ::core::option::Option<*mut u64>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTo)(::windows::core::Vtable::as_raw(self), pstm.into().abi(), cb, ::core::mem::transmute(pcbread.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pcbwritten.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Commit(&self, grfcommitflags: super::STGC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Commit)(::windows::core::Vtable::as_raw(self), grfcommitflags).ok()
    }
    pub unsafe fn Revert(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Revert)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn LockRegion(&self, liboffset: u64, cb: u64, dwlocktype: super::LOCKTYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.LockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    pub unsafe fn UnlockRegion(&self, liboffset: u64, cb: u64, dwlocktype: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnlockRegion)(::windows::core::Vtable::as_raw(self), liboffset, cb, dwlocktype).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Stat(&self, pstatstg: *mut super::STATSTG, grfstatflag: super::STATFLAG) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Stat)(::windows::core::Vtable::as_raw(self), pstatstg, grfstatflag).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<super::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::default::Default for STDMSHLFLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for STDMSHLFLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STDMSHLFLAGS").field(&self.0).finish()
    }
}
