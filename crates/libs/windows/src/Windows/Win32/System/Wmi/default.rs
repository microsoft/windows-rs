impl ::core::default::Default for CIMTYPE_ENUMERATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for CIMTYPE_ENUMERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CIMTYPE_ENUMERATION").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IEnumWbemClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumWbemClassObject {}
impl ::core::fmt::Debug for IEnumWbemClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumWbemClassObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IMofCompiler {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMofCompiler {}
impl ::core::fmt::Debug for IMofCompiler {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMofCompiler").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemDateTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemDateTime {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemDateTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemDateTime").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemEventSource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemEventSource {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemEventSource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemEventSource").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemLastError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemLastError {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemLastError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemLastError").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemLastError {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectPath>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Put_)(::windows::core::Vtable::as_raw(self), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PutAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Delete_)(::windows::core::Vtable::as_raw(self), iflags, objwbemnamedvalueset.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Instances_)(::windows::core::Vtable::as_raw(self), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InstancesAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Subclasses_)(::windows::core::Vtable::as_raw(self), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SubclassesAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Associators_<P0, P1, P2>(&self, strassocclass: &::windows::core::BSTR, strresultclass: &::windows::core::BSTR, strresultrole: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P0, bschemaonly: P1, strrequiredassocqualifier: &::windows::core::BSTR, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Associators_)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strassocclass), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strresultrole), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredassocqualifier), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsAsync_<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strassocclass: &::windows::core::BSTR, strresultclass: &::windows::core::BSTR, strresultrole: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P1, bschemaonly: P2, strrequiredassocqualifier: &::windows::core::BSTR, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P4: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AssociatorsAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strassocclass), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strresultrole), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredassocqualifier), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn References_<P0, P1, P2>(&self, strresultclass: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P0, bschemaonly: P1, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.References_)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesAsync_<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strresultclass: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P1, bschemaonly: P2, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P4: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReferencesAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod_<P0, P1>(&self, strmethodname: &::windows::core::BSTR, objwbeminparameters: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ExecMethod_)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strmethodname), objwbeminparameters.into().abi(), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync_<P0, P1, P2, P3>(&self, objwbemsink: P0, strmethodname: &::windows::core::BSTR, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecMethodAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strmethodname), objwbeminparameters.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone_(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetObjectText_)(::windows::core::Vtable::as_raw(self), iflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SpawnDerivedClass_)(::windows::core::Vtable::as_raw(self), iflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SpawnInstance_)(::windows::core::Vtable::as_raw(self), iflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareTo_<P0>(&self, objwbemobject: P0, iflags: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareTo_)(::windows::core::Vtable::as_raw(self), objwbemobject.into().abi(), iflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Qualifiers_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Properties_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Methods_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Derivation_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Security_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemLocator {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemLocator").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemMethod {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemMethod {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemMethod").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemMethodSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemMethodSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemMethodSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemMethodSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemNamedValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemNamedValue {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemNamedValue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemNamedValue").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemNamedValueSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemNamedValueSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemNamedValueSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemNamedValueSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemObject {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObject").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemObjectEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemObjectEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemObjectEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemObjectEx {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Put_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectPath>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Put_)(::windows::core::Vtable::as_raw(self), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn PutAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PutAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Delete_)(::windows::core::Vtable::as_raw(self), iflags, objwbemnamedvalueset.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Instances_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Instances_)(::windows::core::Vtable::as_raw(self), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InstancesAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Subclasses_<P0>(&self, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Subclasses_)(::windows::core::Vtable::as_raw(self), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesAsync_<P0, P1, P2>(&self, objwbemsink: P0, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SubclassesAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Associators_<P0, P1, P2>(&self, strassocclass: &::windows::core::BSTR, strresultclass: &::windows::core::BSTR, strresultrole: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P0, bschemaonly: P1, strrequiredassocqualifier: &::windows::core::BSTR, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Associators_)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strassocclass), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strresultrole), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredassocqualifier), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsAsync_<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strassocclass: &::windows::core::BSTR, strresultclass: &::windows::core::BSTR, strresultrole: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P1, bschemaonly: P2, strrequiredassocqualifier: &::windows::core::BSTR, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P4: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AssociatorsAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strassocclass), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strresultrole), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredassocqualifier), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn References_<P0, P1, P2>(&self, strresultclass: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P0, bschemaonly: P1, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.References_)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesAsync_<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strresultclass: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P1, bschemaonly: P2, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P4: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReferencesAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod_<P0, P1>(&self, strmethodname: &::windows::core::BSTR, objwbeminparameters: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ExecMethod_)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strmethodname), objwbeminparameters.into().abi(), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync_<P0, P1, P2, P3>(&self, objwbemsink: P0, strmethodname: &::windows::core::BSTR, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecMethodAsync_)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strmethodname), objwbeminparameters.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Clone_(&self) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetObjectText_(&self, iflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetObjectText_)(::windows::core::Vtable::as_raw(self), iflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnDerivedClass_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SpawnDerivedClass_)(::windows::core::Vtable::as_raw(self), iflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SpawnInstance_(&self, iflags: i32) -> ::windows::core::Result<ISWbemObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SpawnInstance_)(::windows::core::Vtable::as_raw(self), iflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CompareTo_<P0>(&self, objwbemobject: P0, iflags: i32) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CompareTo_)(::windows::core::Vtable::as_raw(self), objwbemobject.into().abi(), iflags, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Qualifiers_(&self) -> ::windows::core::Result<ISWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Qualifiers_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties_(&self) -> ::windows::core::Result<ISWbemPropertySet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Properties_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Methods_(&self) -> ::windows::core::Result<ISWbemMethodSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Methods_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Derivation_(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Derivation_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Path_(&self) -> ::windows::core::Result<ISWbemObjectPath> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Path_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Security_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemObjectPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemObjectPath {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemObjectPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectPath").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemObjectSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemObjectSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemObjectSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemObjectSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemPrivilege {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemPrivilege {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemPrivilege {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPrivilege").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemPrivilegeSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemPrivilegeSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemPrivilegeSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPrivilegeSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemProperty {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemProperty").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemPropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemPropertySet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemPropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemPropertySet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemQualifier {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemQualifier {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemQualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemQualifier").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemQualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemQualifierSet {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemQualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemQualifierSet").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemRefreshableItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemRefreshableItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemRefreshableItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemRefreshableItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemRefresher {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemRefresher").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemSecurity {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSecurity").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemServices {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemServices").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemServicesEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemServicesEx {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemServicesEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemServicesEx").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ISWbemServicesEx {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Get<P0>(&self, strobjectpath: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetAsync<P0, P1, P2>(&self, objwbemsink: P0, strobjectpath: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAsync)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Delete<P0>(&self, strobjectpath: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DeleteAsync<P0, P1, P2>(&self, objwbemsink: P0, strobjectpath: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteAsync)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strobjectpath), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesOf<P0>(&self, strclass: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.InstancesOf)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strclass), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn InstancesOfAsync<P0, P1, P2>(&self, objwbemsink: P0, strclass: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InstancesOfAsync)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strclass), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesOf<P0>(&self, strsuperclass: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SubclassesOf)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strsuperclass), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SubclassesOfAsync<P0, P1, P2>(&self, objwbemsink: P0, strsuperclass: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SubclassesOfAsync)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strsuperclass), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecQuery<P0>(&self, strquery: &::windows::core::BSTR, strquerylanguage: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ExecQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strquery), ::core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecQueryAsync<P0, P1, P2>(&self, objwbemsink: P0, strquery: &::windows::core::BSTR, strquerylanguage: &::windows::core::BSTR, lflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecQueryAsync)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strquery), ::core::mem::transmute_copy(strquerylanguage), lflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOf<P0, P1, P2>(&self, strobjectpath: &::windows::core::BSTR, strassocclass: &::windows::core::BSTR, strresultclass: &::windows::core::BSTR, strresultrole: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P0, bschemaonly: P1, strrequiredassocqualifier: &::windows::core::BSTR, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AssociatorsOf)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strobjectpath), ::core::mem::transmute_copy(strassocclass), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strresultrole), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredassocqualifier), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AssociatorsOfAsync<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strobjectpath: &::windows::core::BSTR, strassocclass: &::windows::core::BSTR, strresultclass: &::windows::core::BSTR, strresultrole: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P1, bschemaonly: P2, strrequiredassocqualifier: &::windows::core::BSTR, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P4: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AssociatorsOfAsync)(
            ::windows::core::Vtable::as_raw(self),
            objwbemsink.into().abi(),
            ::core::mem::transmute_copy(strobjectpath),
            ::core::mem::transmute_copy(strassocclass),
            ::core::mem::transmute_copy(strresultclass),
            ::core::mem::transmute_copy(strresultrole),
            ::core::mem::transmute_copy(strrole),
            bclassesonly.into(),
            bschemaonly.into(),
            ::core::mem::transmute_copy(strrequiredassocqualifier),
            ::core::mem::transmute_copy(strrequiredqualifier),
            iflags,
            objwbemnamedvalueset.into().abi(),
            objwbemasynccontext.into().abi(),
        )
        .ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesTo<P0, P1, P2>(&self, strobjectpath: &::windows::core::BSTR, strresultclass: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P0, bschemaonly: P1, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P2) -> ::windows::core::Result<ISWbemObjectSet>
    where
        P0: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ReferencesTo)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strobjectpath), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ReferencesToAsync<P0, P1, P2, P3, P4>(&self, objwbemsink: P0, strobjectpath: &::windows::core::BSTR, strresultclass: &::windows::core::BSTR, strrole: &::windows::core::BSTR, bclassesonly: P1, bschemaonly: P2, strrequiredqualifier: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P3, objwbemasynccontext: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P2: ::std::convert::Into<super::super::Foundation::VARIANT_BOOL>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P4: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ReferencesToAsync)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strobjectpath), ::core::mem::transmute_copy(strresultclass), ::core::mem::transmute_copy(strrole), bclassesonly.into(), bschemaonly.into(), ::core::mem::transmute_copy(strrequiredqualifier), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecNotificationQuery<P0>(&self, strquery: &::windows::core::BSTR, strquerylanguage: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P0) -> ::windows::core::Result<ISWbemEventSource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ExecNotificationQuery)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strquery), ::core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecNotificationQueryAsync<P0, P1, P2>(&self, objwbemsink: P0, strquery: &::windows::core::BSTR, strquerylanguage: &::windows::core::BSTR, iflags: i32, objwbemnamedvalueset: P1, objwbemasynccontext: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecNotificationQueryAsync)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strquery), ::core::mem::transmute_copy(strquerylanguage), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethod<P0, P1>(&self, strobjectpath: &::windows::core::BSTR, strmethodname: &::windows::core::BSTR, objwbeminparameters: P0, iflags: i32, objwbemnamedvalueset: P1) -> ::windows::core::Result<ISWbemObject>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ExecMethod)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute_copy(strobjectpath), ::core::mem::transmute_copy(strmethodname), objwbeminparameters.into().abi(), iflags, objwbemnamedvalueset.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ExecMethodAsync<P0, P1, P2, P3>(&self, objwbemsink: P0, strobjectpath: &::windows::core::BSTR, strmethodname: &::windows::core::BSTR, objwbeminparameters: P1, iflags: i32, objwbemnamedvalueset: P2, objwbemasynccontext: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
        P3: ::std::convert::Into<::windows::core::InParam<super::Com::IDispatch>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecMethodAsync)(::windows::core::Vtable::as_raw(self), objwbemsink.into().abi(), ::core::mem::transmute_copy(strobjectpath), ::core::mem::transmute_copy(strmethodname), objwbeminparameters.into().abi(), iflags, objwbemnamedvalueset.into().abi(), objwbemasynccontext.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Security_(&self) -> ::windows::core::Result<ISWbemSecurity> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Security_)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemSink {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSink").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISWbemSinkEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISWbemSinkEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISWbemSinkEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISWbemSinkEvents").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IUnsecuredApartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IUnsecuredApartment {}
impl ::core::fmt::Debug for IUnsecuredApartment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IUnsecuredApartment").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for IWMIExtension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for IWMIExtension {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for IWMIExtension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWMIExtension").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemAddressResolution {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemAddressResolution {}
impl ::core::fmt::Debug for IWbemAddressResolution {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemAddressResolution").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemBackupRestore {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemBackupRestore {}
impl ::core::fmt::Debug for IWbemBackupRestore {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemBackupRestore").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemBackupRestoreEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemBackupRestoreEx {}
impl ::core::fmt::Debug for IWbemBackupRestoreEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemBackupRestoreEx").field(&self.0).finish()
    }
}
impl IWbemBackupRestoreEx {
    pub unsafe fn Backup<P0>(&self, strbackuptofile: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Backup)(::windows::core::Vtable::as_raw(self), strbackuptofile.into().abi(), lflags).ok()
    }
    pub unsafe fn Restore<P0>(&self, strrestorefromfile: P0, lflags: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Restore)(::windows::core::Vtable::as_raw(self), strrestorefromfile.into().abi(), lflags).ok()
    }
}
impl ::core::cmp::PartialEq for IWbemCallResult {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemCallResult {}
impl ::core::fmt::Debug for IWbemCallResult {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemCallResult").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClassObject {}
impl ::core::fmt::Debug for IWbemClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClassObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemClientConnectionTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClientConnectionTransport {}
impl ::core::fmt::Debug for IWbemClientConnectionTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClientConnectionTransport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemClientTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemClientTransport {}
impl ::core::fmt::Debug for IWbemClientTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemClientTransport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemConfigureRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConfigureRefresher {}
impl ::core::fmt::Debug for IWbemConfigureRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConfigureRefresher").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemConnectorLogin {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConnectorLogin {}
impl ::core::fmt::Debug for IWbemConnectorLogin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConnectorLogin").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemConstructClassObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemConstructClassObject {}
impl ::core::fmt::Debug for IWbemConstructClassObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemConstructClassObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemContext {}
impl ::core::fmt::Debug for IWbemContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemContext").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemDecoupledBasicEventProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemDecoupledBasicEventProvider {}
impl ::core::fmt::Debug for IWbemDecoupledBasicEventProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemDecoupledBasicEventProvider").field(&self.0).finish()
    }
}
impl IWbemDecoupledBasicEventProvider {
    pub unsafe fn Register<P0, P1, P2, P3, P4, P5>(&self, a_flags: i32, a_context: P0, a_user: P1, a_locale: P2, a_scope: P3, a_registration: P4, piunknown: P5) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWbemContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P4: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P5: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Register)(::windows::core::Vtable::as_raw(self), a_flags, a_context.into().abi(), a_user.into().abi(), a_locale.into().abi(), a_scope.into().abi(), a_registration.into().abi(), piunknown.into().abi()).ok()
    }
    pub unsafe fn UnRegister(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.UnRegister)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IWbemDecoupledRegistrar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemDecoupledRegistrar {}
impl ::core::fmt::Debug for IWbemDecoupledRegistrar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemDecoupledRegistrar").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemEventConsumerProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventConsumerProvider {}
impl ::core::fmt::Debug for IWbemEventConsumerProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventConsumerProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemEventProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProvider {}
impl ::core::fmt::Debug for IWbemEventProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemEventProviderQuerySink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProviderQuerySink {}
impl ::core::fmt::Debug for IWbemEventProviderQuerySink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProviderQuerySink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemEventProviderSecurity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventProviderSecurity {}
impl ::core::fmt::Debug for IWbemEventProviderSecurity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventProviderSecurity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemEventSink {}
impl ::core::fmt::Debug for IWbemEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemEventSink").field(&self.0).finish()
    }
}
impl IWbemEventSink {
    pub unsafe fn Indicate(&self, apobjarray: &[IWbemClassObject]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Indicate)(::windows::core::Vtable::as_raw(self), apobjarray.len() as _, ::core::mem::transmute(apobjarray.as_ptr())).ok()
    }
    pub unsafe fn SetStatus<P0>(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: &::windows::core::BSTR, pobjparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWbemClassObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStatus)(::windows::core::Vtable::as_raw(self), lflags, hresult, ::core::mem::transmute_copy(strparam), pobjparam.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IWbemHiPerfEnum {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemHiPerfEnum {}
impl ::core::fmt::Debug for IWbemHiPerfEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemHiPerfEnum").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemHiPerfProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemHiPerfProvider {}
impl ::core::fmt::Debug for IWbemHiPerfProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemHiPerfProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemLevel1Login {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemLevel1Login {}
impl ::core::fmt::Debug for IWbemLevel1Login {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemLevel1Login").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemLocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemLocator {}
impl ::core::fmt::Debug for IWbemLocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemLocator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemObjectAccess {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectAccess {}
impl ::core::fmt::Debug for IWbemObjectAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectAccess").field(&self.0).finish()
    }
}
impl IWbemObjectAccess {
    pub unsafe fn GetQualifierSet(&self) -> ::windows::core::Result<IWbemQualifierSet> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetQualifierSet)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Get<P0>(&self, wszname: P0, lflags: i32, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Get)(::windows::core::Vtable::as_raw(self), wszname.into().abi(), lflags, pval, ptype, plflavor).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Put<P0>(&self, wszname: P0, lflags: i32, pval: *const super::Com::VARIANT, r#type: i32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Put)(::windows::core::Vtable::as_raw(self), wszname.into().abi(), lflags, pval, r#type).ok()
    }
    pub unsafe fn Delete<P0>(&self, wszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Delete)(::windows::core::Vtable::as_raw(self), wszname.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetNames<P0>(&self, wszqualifiername: P0, lflags: i32, pqualifierval: *const super::Com::VARIANT) -> ::windows::core::Result<*mut super::Com::SAFEARRAY>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetNames)(::windows::core::Vtable::as_raw(self), wszqualifiername.into().abi(), lflags, pqualifierval, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn BeginEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginEnumeration)(::windows::core::Vtable::as_raw(self), lenumflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Next(&self, lflags: i32, strname: *mut ::windows::core::BSTR, pval: *mut super::Com::VARIANT, ptype: *mut i32, plflavor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Next)(::windows::core::Vtable::as_raw(self), lflags, ::core::mem::transmute(strname), pval, ptype, plflavor).ok()
    }
    pub unsafe fn EndEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndEnumeration)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetPropertyQualifierSet<P0>(&self, wszproperty: P0) -> ::windows::core::Result<IWbemQualifierSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyQualifierSet)(::windows::core::Vtable::as_raw(self), wszproperty.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetObjectText(&self, lflags: i32) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetObjectText)(::windows::core::Vtable::as_raw(self), lflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SpawnDerivedClass(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SpawnDerivedClass)(::windows::core::Vtable::as_raw(self), lflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SpawnInstance(&self, lflags: i32) -> ::windows::core::Result<IWbemClassObject> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.SpawnInstance)(::windows::core::Vtable::as_raw(self), lflags, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CompareTo<P0>(&self, lflags: i32, pcompareto: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWbemClassObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CompareTo)(::windows::core::Vtable::as_raw(self), lflags, pcompareto.into().abi()).ok()
    }
    pub unsafe fn GetPropertyOrigin<P0>(&self, wszname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPropertyOrigin)(::windows::core::Vtable::as_raw(self), wszname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn InheritsFrom<P0>(&self, strancestor: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InheritsFrom)(::windows::core::Vtable::as_raw(self), strancestor.into().abi()).ok()
    }
    pub unsafe fn GetMethod<P0>(&self, wszname: P0, lflags: i32, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMethod)(::windows::core::Vtable::as_raw(self), wszname.into().abi(), lflags, ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn PutMethod<P0, P1, P2>(&self, wszname: P0, lflags: i32, pinsignature: P1, poutsignature: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<IWbemClassObject>>,
        P2: ::std::convert::Into<::windows::core::InParam<IWbemClassObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PutMethod)(::windows::core::Vtable::as_raw(self), wszname.into().abi(), lflags, pinsignature.into().abi(), poutsignature.into().abi()).ok()
    }
    pub unsafe fn DeleteMethod<P0>(&self, wszname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DeleteMethod)(::windows::core::Vtable::as_raw(self), wszname.into().abi()).ok()
    }
    pub unsafe fn BeginMethodEnumeration(&self, lenumflags: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.BeginMethodEnumeration)(::windows::core::Vtable::as_raw(self), lenumflags).ok()
    }
    pub unsafe fn NextMethod(&self, lflags: i32, pstrname: *mut ::windows::core::BSTR, ppinsignature: *mut ::core::option::Option<IWbemClassObject>, ppoutsignature: *mut ::core::option::Option<IWbemClassObject>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.NextMethod)(::windows::core::Vtable::as_raw(self), lflags, ::core::mem::transmute(pstrname), ::core::mem::transmute(ppinsignature), ::core::mem::transmute(ppoutsignature)).ok()
    }
    pub unsafe fn EndMethodEnumeration(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EndMethodEnumeration)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn GetMethodQualifierSet<P0>(&self, wszmethod: P0) -> ::windows::core::Result<IWbemQualifierSet>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMethodQualifierSet)(::windows::core::Vtable::as_raw(self), wszmethod.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMethodOrigin<P0>(&self, wszmethodname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMethodOrigin)(::windows::core::Vtable::as_raw(self), wszmethodname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IWbemObjectSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectSink {}
impl ::core::fmt::Debug for IWbemObjectSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemObjectSinkEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectSinkEx {}
impl ::core::fmt::Debug for IWbemObjectSinkEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectSinkEx").field(&self.0).finish()
    }
}
impl IWbemObjectSinkEx {
    pub unsafe fn Indicate(&self, apobjarray: &[IWbemClassObject]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Indicate)(::windows::core::Vtable::as_raw(self), apobjarray.len() as _, ::core::mem::transmute(apobjarray.as_ptr())).ok()
    }
    pub unsafe fn SetStatus<P0>(&self, lflags: i32, hresult: ::windows::core::HRESULT, strparam: &::windows::core::BSTR, pobjparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IWbemClassObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStatus)(::windows::core::Vtable::as_raw(self), lflags, hresult, ::core::mem::transmute_copy(strparam), pobjparam.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IWbemObjectTextSrc {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemObjectTextSrc {}
impl ::core::fmt::Debug for IWbemObjectTextSrc {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemObjectTextSrc").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPath {}
impl ::core::fmt::Debug for IWbemPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPath").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemPathKeyList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPathKeyList {}
impl ::core::fmt::Debug for IWbemPathKeyList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPathKeyList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemPropertyProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemPropertyProvider {}
impl ::core::fmt::Debug for IWbemPropertyProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemPropertyProvider").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemProviderIdentity {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderIdentity {}
impl ::core::fmt::Debug for IWbemProviderIdentity {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderIdentity").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemProviderInit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderInit {}
impl ::core::fmt::Debug for IWbemProviderInit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderInit").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemProviderInitSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemProviderInitSink {}
impl ::core::fmt::Debug for IWbemProviderInitSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemProviderInitSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemQualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemQualifierSet {}
impl ::core::fmt::Debug for IWbemQualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemQualifierSet").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemQuery {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemQuery {}
impl ::core::fmt::Debug for IWbemQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemQuery").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemRefresher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemRefresher {}
impl ::core::fmt::Debug for IWbemRefresher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemRefresher").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemServices {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemServices {}
impl ::core::fmt::Debug for IWbemServices {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemServices").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemShutdown {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemShutdown {}
impl ::core::fmt::Debug for IWbemShutdown {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemShutdown").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemStatusCodeText {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemStatusCodeText {}
impl ::core::fmt::Debug for IWbemStatusCodeText {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemStatusCodeText").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemTransport {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemTransport {}
impl ::core::fmt::Debug for IWbemTransport {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemTransport").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemUnboundObjectSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemUnboundObjectSink {}
impl ::core::fmt::Debug for IWbemUnboundObjectSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemUnboundObjectSink").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IWbemUnsecuredApartment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IWbemUnsecuredApartment {}
impl ::core::fmt::Debug for IWbemUnsecuredApartment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IWbemUnsecuredApartment").field(&self.0).finish()
    }
}
impl IWbemUnsecuredApartment {
    pub unsafe fn CreateObjectStub<P0>(&self, pobject: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateObjectStub)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::default::Default for MI_Application {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Application {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_Application {}
impl ::core::fmt::Debug for MI_Application {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Application").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_ApplicationFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ApplicationFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.NewSession == other.NewSession && self.NewHostedProvider == other.NewHostedProvider && self.NewInstance == other.NewInstance && self.NewDestinationOptions == other.NewDestinationOptions && self.NewOperationOptions == other.NewOperationOptions && self.NewSubscriptionDeliveryOptions == other.NewSubscriptionDeliveryOptions && self.NewSerializer == other.NewSerializer && self.NewDeserializer == other.NewDeserializer && self.NewInstanceFromClass == other.NewInstanceFromClass && self.NewClass == other.NewClass
    }
}
impl ::core::cmp::Eq for MI_ApplicationFT {}
impl ::core::fmt::Debug for MI_ApplicationFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ApplicationFT")
            .field("Close", &self.Close)
            .field("NewSession", &self.NewSession)
            .field("NewHostedProvider", &self.NewHostedProvider)
            .field("NewInstance", &self.NewInstance)
            .field("NewDestinationOptions", &self.NewDestinationOptions)
            .field("NewOperationOptions", &self.NewOperationOptions)
            .field("NewSubscriptionDeliveryOptions", &self.NewSubscriptionDeliveryOptions)
            .field("NewSerializer", &self.NewSerializer)
            .field("NewDeserializer", &self.NewDeserializer)
            .field("NewInstanceFromClass", &self.NewInstanceFromClass)
            .field("NewClass", &self.NewClass)
            .finish()
    }
}
impl ::core::default::Default for MI_Array {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Array {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Array {}
impl ::core::fmt::Debug for MI_Array {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Array").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ArrayField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ArrayField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ArrayField {}
impl ::core::fmt::Debug for MI_ArrayField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ArrayField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_BooleanA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_BooleanA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_BooleanA {}
impl ::core::fmt::Debug for MI_BooleanA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_BooleanAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_BooleanAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_BooleanAField {}
impl ::core::fmt::Debug for MI_BooleanAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_BooleanField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_BooleanField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_BooleanField {}
impl ::core::fmt::Debug for MI_BooleanField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_BooleanField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_CallbackMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_CallbackMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_CallbackMode").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_CancellationReason {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_CancellationReason {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_CancellationReason").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_Char16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Char16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Char16A {}
impl ::core::fmt::Debug for MI_Char16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Char16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Char16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Char16AField {}
impl ::core::fmt::Debug for MI_Char16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Char16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Char16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Char16Field {}
impl ::core::fmt::Debug for MI_Char16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Char16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Class {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Class {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.classDecl == other.classDecl && self.namespaceName == other.namespaceName && self.serverName == other.serverName && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_Class {}
impl ::core::fmt::Debug for MI_Class {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Class").field("ft", &self.ft).field("classDecl", &self.classDecl).field("namespaceName", &self.namespaceName).field("serverName", &self.serverName).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for MI_ClassDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ClassDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers && self.properties == other.properties && self.numProperties == other.numProperties && self.size == other.size && self.superClass == other.superClass && self.superClassDecl == other.superClassDecl && self.methods == other.methods && self.numMethods == other.numMethods && self.schema == other.schema && self.providerFT == other.providerFT && self.owningClass == other.owningClass
    }
}
impl ::core::cmp::Eq for MI_ClassDecl {}
impl ::core::fmt::Debug for MI_ClassDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClassDecl")
            .field("flags", &self.flags)
            .field("code", &self.code)
            .field("name", &self.name)
            .field("qualifiers", &self.qualifiers)
            .field("numQualifiers", &self.numQualifiers)
            .field("properties", &self.properties)
            .field("numProperties", &self.numProperties)
            .field("size", &self.size)
            .field("superClass", &self.superClass)
            .field("superClassDecl", &self.superClassDecl)
            .field("methods", &self.methods)
            .field("numMethods", &self.numMethods)
            .field("schema", &self.schema)
            .field("providerFT", &self.providerFT)
            .field("owningClass", &self.owningClass)
            .finish()
    }
}
impl ::core::default::Default for MI_ClassFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ClassFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetClassNameA == other.GetClassNameA && self.GetNameSpace == other.GetNameSpace && self.GetServerName == other.GetServerName && self.GetElementCount == other.GetElementCount && self.GetElement == other.GetElement && self.GetElementAt == other.GetElementAt && self.GetClassQualifierSet == other.GetClassQualifierSet && self.GetMethodCount == other.GetMethodCount && self.GetMethodAt == other.GetMethodAt && self.GetMethod == other.GetMethod && self.GetParentClassName == other.GetParentClassName && self.GetParentClass == other.GetParentClass && self.Delete == other.Delete && self.Clone == other.Clone
    }
}
impl ::core::cmp::Eq for MI_ClassFT {}
impl ::core::fmt::Debug for MI_ClassFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClassFT")
            .field("GetClassNameA", &self.GetClassNameA)
            .field("GetNameSpace", &self.GetNameSpace)
            .field("GetServerName", &self.GetServerName)
            .field("GetElementCount", &self.GetElementCount)
            .field("GetElement", &self.GetElement)
            .field("GetElementAt", &self.GetElementAt)
            .field("GetClassQualifierSet", &self.GetClassQualifierSet)
            .field("GetMethodCount", &self.GetMethodCount)
            .field("GetMethodAt", &self.GetMethodAt)
            .field("GetMethod", &self.GetMethod)
            .field("GetParentClassName", &self.GetParentClassName)
            .field("GetParentClass", &self.GetParentClass)
            .field("Delete", &self.Delete)
            .field("Clone", &self.Clone)
            .finish()
    }
}
impl ::core::default::Default for MI_ClientFT_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ClientFT_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.applicationFT == other.applicationFT && self.sessionFT == other.sessionFT && self.operationFT == other.operationFT && self.hostedProviderFT == other.hostedProviderFT && self.serializerFT == other.serializerFT && self.deserializerFT == other.deserializerFT && self.subscribeDeliveryOptionsFT == other.subscribeDeliveryOptionsFT && self.destinationOptionsFT == other.destinationOptionsFT && self.operationOptionsFT == other.operationOptionsFT && self.utilitiesFT == other.utilitiesFT
    }
}
impl ::core::cmp::Eq for MI_ClientFT_V1 {}
impl ::core::fmt::Debug for MI_ClientFT_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ClientFT_V1")
            .field("applicationFT", &self.applicationFT)
            .field("sessionFT", &self.sessionFT)
            .field("operationFT", &self.operationFT)
            .field("hostedProviderFT", &self.hostedProviderFT)
            .field("serializerFT", &self.serializerFT)
            .field("deserializerFT", &self.deserializerFT)
            .field("subscribeDeliveryOptionsFT", &self.subscribeDeliveryOptionsFT)
            .field("destinationOptionsFT", &self.destinationOptionsFT)
            .field("operationOptionsFT", &self.operationOptionsFT)
            .field("utilitiesFT", &self.utilitiesFT)
            .finish()
    }
}
impl ::core::default::Default for MI_ConstBooleanA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstBooleanA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanA {}
impl ::core::fmt::Debug for MI_ConstBooleanA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstBooleanAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstBooleanAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanAField {}
impl ::core::fmt::Debug for MI_ConstBooleanAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstBooleanField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstBooleanField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstBooleanField {}
impl ::core::fmt::Debug for MI_ConstBooleanField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstBooleanField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstChar16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstChar16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstChar16A {}
impl ::core::fmt::Debug for MI_ConstChar16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstChar16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstChar16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstChar16AField {}
impl ::core::fmt::Debug for MI_ConstChar16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstChar16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstChar16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstChar16Field {}
impl ::core::fmt::Debug for MI_ConstChar16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstChar16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstDatetimeA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeA {}
impl ::core::fmt::Debug for MI_ConstDatetimeA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstDatetimeA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstDatetimeAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstDatetimeAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstDatetimeAField {}
impl ::core::fmt::Debug for MI_ConstDatetimeAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstDatetimeAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstDatetimeField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MI_ConstInstanceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstInstanceA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceA {}
impl ::core::fmt::Debug for MI_ConstInstanceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstInstanceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstInstanceAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceAField {}
impl ::core::fmt::Debug for MI_ConstInstanceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstInstanceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstInstanceField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstInstanceField {}
impl ::core::fmt::Debug for MI_ConstInstanceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstInstanceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstReal32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReal32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstReal32A {}
impl ::core::fmt::Debug for MI_ConstReal32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstReal32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReal32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReal32AField {}
impl ::core::fmt::Debug for MI_ConstReal32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstReal32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReal32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReal32Field {}
impl ::core::fmt::Debug for MI_ConstReal32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstReal64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReal64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstReal64A {}
impl ::core::fmt::Debug for MI_ConstReal64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstReal64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReal64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReal64AField {}
impl ::core::fmt::Debug for MI_ConstReal64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstReal64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReal64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReal64Field {}
impl ::core::fmt::Debug for MI_ConstReal64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReal64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstReferenceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReferenceA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceA {}
impl ::core::fmt::Debug for MI_ConstReferenceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstReferenceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReferenceAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceAField {}
impl ::core::fmt::Debug for MI_ConstReferenceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstReferenceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstReferenceField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstReferenceField {}
impl ::core::fmt::Debug for MI_ConstReferenceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstReferenceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstSint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstSint16A {}
impl ::core::fmt::Debug for MI_ConstSint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstSint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint16AField {}
impl ::core::fmt::Debug for MI_ConstSint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstSint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint16Field {}
impl ::core::fmt::Debug for MI_ConstSint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstSint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstSint32A {}
impl ::core::fmt::Debug for MI_ConstSint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstSint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint32AField {}
impl ::core::fmt::Debug for MI_ConstSint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstSint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint32Field {}
impl ::core::fmt::Debug for MI_ConstSint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstSint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstSint64A {}
impl ::core::fmt::Debug for MI_ConstSint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstSint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint64AField {}
impl ::core::fmt::Debug for MI_ConstSint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstSint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint64Field {}
impl ::core::fmt::Debug for MI_ConstSint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstSint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint8A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstSint8A {}
impl ::core::fmt::Debug for MI_ConstSint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstSint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint8AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint8AField {}
impl ::core::fmt::Debug for MI_ConstSint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstSint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstSint8Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstSint8Field {}
impl ::core::fmt::Debug for MI_ConstSint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstSint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstStringA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstStringA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstStringA {}
impl ::core::fmt::Debug for MI_ConstStringA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstStringAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstStringAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstStringAField {}
impl ::core::fmt::Debug for MI_ConstStringAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstStringField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstStringField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstStringField {}
impl ::core::fmt::Debug for MI_ConstStringField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstStringField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstUint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstUint16A {}
impl ::core::fmt::Debug for MI_ConstUint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstUint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint16AField {}
impl ::core::fmt::Debug for MI_ConstUint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstUint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint16Field {}
impl ::core::fmt::Debug for MI_ConstUint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstUint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstUint32A {}
impl ::core::fmt::Debug for MI_ConstUint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstUint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint32AField {}
impl ::core::fmt::Debug for MI_ConstUint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstUint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint32Field {}
impl ::core::fmt::Debug for MI_ConstUint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstUint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstUint64A {}
impl ::core::fmt::Debug for MI_ConstUint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstUint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint64AField {}
impl ::core::fmt::Debug for MI_ConstUint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstUint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint64Field {}
impl ::core::fmt::Debug for MI_ConstUint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstUint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint8A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ConstUint8A {}
impl ::core::fmt::Debug for MI_ConstUint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ConstUint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint8AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint8AField {}
impl ::core::fmt::Debug for MI_ConstUint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ConstUint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ConstUint8Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ConstUint8Field {}
impl ::core::fmt::Debug for MI_ConstUint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ConstUint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Context {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Context {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_Context {}
impl ::core::fmt::Debug for MI_Context {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Context").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for MI_ContextFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ContextFT {
    fn eq(&self, other: &Self) -> bool {
        self.PostResult == other.PostResult
            && self.PostInstance == other.PostInstance
            && self.PostIndication == other.PostIndication
            && self.ConstructInstance == other.ConstructInstance
            && self.ConstructParameters == other.ConstructParameters
            && self.NewInstance == other.NewInstance
            && self.NewDynamicInstance == other.NewDynamicInstance
            && self.NewParameters == other.NewParameters
            && self.Canceled == other.Canceled
            && self.GetLocale == other.GetLocale
            && self.RegisterCancel == other.RegisterCancel
            && self.RequestUnload == other.RequestUnload
            && self.RefuseUnload == other.RefuseUnload
            && self.GetLocalSession == other.GetLocalSession
            && self.SetStringOption == other.SetStringOption
            && self.GetStringOption == other.GetStringOption
            && self.GetNumberOption == other.GetNumberOption
            && self.GetCustomOption == other.GetCustomOption
            && self.GetCustomOptionCount == other.GetCustomOptionCount
            && self.GetCustomOptionAt == other.GetCustomOptionAt
            && self.WriteMessage == other.WriteMessage
            && self.WriteProgress == other.WriteProgress
            && self.WriteStreamParameter == other.WriteStreamParameter
            && self.WriteCimError == other.WriteCimError
            && self.PromptUser == other.PromptUser
            && self.ShouldProcess == other.ShouldProcess
            && self.ShouldContinue == other.ShouldContinue
            && self.PostError == other.PostError
            && self.PostCimError == other.PostCimError
            && self.WriteError == other.WriteError
    }
}
impl ::core::cmp::Eq for MI_ContextFT {}
impl ::core::fmt::Debug for MI_ContextFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ContextFT")
            .field("PostResult", &self.PostResult)
            .field("PostInstance", &self.PostInstance)
            .field("PostIndication", &self.PostIndication)
            .field("ConstructInstance", &self.ConstructInstance)
            .field("ConstructParameters", &self.ConstructParameters)
            .field("NewInstance", &self.NewInstance)
            .field("NewDynamicInstance", &self.NewDynamicInstance)
            .field("NewParameters", &self.NewParameters)
            .field("Canceled", &self.Canceled)
            .field("GetLocale", &self.GetLocale)
            .field("RegisterCancel", &self.RegisterCancel)
            .field("RequestUnload", &self.RequestUnload)
            .field("RefuseUnload", &self.RefuseUnload)
            .field("GetLocalSession", &self.GetLocalSession)
            .field("SetStringOption", &self.SetStringOption)
            .field("GetStringOption", &self.GetStringOption)
            .field("GetNumberOption", &self.GetNumberOption)
            .field("GetCustomOption", &self.GetCustomOption)
            .field("GetCustomOptionCount", &self.GetCustomOptionCount)
            .field("GetCustomOptionAt", &self.GetCustomOptionAt)
            .field("WriteMessage", &self.WriteMessage)
            .field("WriteProgress", &self.WriteProgress)
            .field("WriteStreamParameter", &self.WriteStreamParameter)
            .field("WriteCimError", &self.WriteCimError)
            .field("PromptUser", &self.PromptUser)
            .field("ShouldProcess", &self.ShouldProcess)
            .field("ShouldContinue", &self.ShouldContinue)
            .field("PostError", &self.PostError)
            .field("PostCimError", &self.PostCimError)
            .field("WriteError", &self.WriteError)
            .finish()
    }
}
impl ::core::default::Default for MI_Datetime {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MI_DatetimeA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_DatetimeA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_DatetimeA {}
impl ::core::fmt::Debug for MI_DatetimeA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DatetimeA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_DatetimeAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_DatetimeAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_DatetimeAField {}
impl ::core::fmt::Debug for MI_DatetimeAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DatetimeAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_DatetimeField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MI_Deserializer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Deserializer {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2
    }
}
impl ::core::cmp::Eq for MI_Deserializer {}
impl ::core::fmt::Debug for MI_Deserializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Deserializer").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).finish()
    }
}
impl ::core::default::Default for MI_DeserializerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_DeserializerFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.DeserializeClass == other.DeserializeClass && self.Class_GetClassName == other.Class_GetClassName && self.Class_GetParentClassName == other.Class_GetParentClassName && self.DeserializeInstance == other.DeserializeInstance && self.Instance_GetClassName == other.Instance_GetClassName
    }
}
impl ::core::cmp::Eq for MI_DeserializerFT {}
impl ::core::fmt::Debug for MI_DeserializerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DeserializerFT").field("Close", &self.Close).field("DeserializeClass", &self.DeserializeClass).field("Class_GetClassName", &self.Class_GetClassName).field("Class_GetParentClassName", &self.Class_GetParentClassName).field("DeserializeInstance", &self.DeserializeInstance).field("Instance_GetClassName", &self.Instance_GetClassName).finish()
    }
}
impl ::core::default::Default for MI_DestinationOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_DestinationOptions {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_DestinationOptions {}
impl ::core::fmt::Debug for MI_DestinationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DestinationOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_DestinationOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_DestinationOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        self.Delete == other.Delete && self.SetString == other.SetString && self.SetNumber == other.SetNumber && self.AddCredentials == other.AddCredentials && self.GetString == other.GetString && self.GetNumber == other.GetNumber && self.GetOptionCount == other.GetOptionCount && self.GetOptionAt == other.GetOptionAt && self.GetOption == other.GetOption && self.GetCredentialsCount == other.GetCredentialsCount && self.GetCredentialsAt == other.GetCredentialsAt && self.GetCredentialsPasswordAt == other.GetCredentialsPasswordAt && self.Clone == other.Clone && self.SetInterval == other.SetInterval && self.GetInterval == other.GetInterval
    }
}
impl ::core::cmp::Eq for MI_DestinationOptionsFT {}
impl ::core::fmt::Debug for MI_DestinationOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_DestinationOptionsFT")
            .field("Delete", &self.Delete)
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("AddCredentials", &self.AddCredentials)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetCredentialsCount", &self.GetCredentialsCount)
            .field("GetCredentialsAt", &self.GetCredentialsAt)
            .field("GetCredentialsPasswordAt", &self.GetCredentialsPasswordAt)
            .field("Clone", &self.Clone)
            .field("SetInterval", &self.SetInterval)
            .field("GetInterval", &self.GetInterval)
            .finish()
    }
}
impl ::core::default::Default for MI_DestinationOptions_ImpersonationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_DestinationOptions_ImpersonationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_DestinationOptions_ImpersonationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_ErrorCategory {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_ErrorCategory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_ErrorCategory").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_FeatureDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_FeatureDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers
    }
}
impl ::core::cmp::Eq for MI_FeatureDecl {}
impl ::core::fmt::Debug for MI_FeatureDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_FeatureDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).finish()
    }
}
impl ::core::default::Default for MI_Filter {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Filter {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_Filter {}
impl ::core::fmt::Debug for MI_Filter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Filter").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for MI_FilterFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_FilterFT {
    fn eq(&self, other: &Self) -> bool {
        self.Evaluate == other.Evaluate && self.GetExpression == other.GetExpression
    }
}
impl ::core::cmp::Eq for MI_FilterFT {}
impl ::core::fmt::Debug for MI_FilterFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_FilterFT").field("Evaluate", &self.Evaluate).field("GetExpression", &self.GetExpression).finish()
    }
}
impl ::core::default::Default for MI_HostedProvider {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_HostedProvider {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_HostedProvider {}
impl ::core::fmt::Debug for MI_HostedProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_HostedProvider").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_HostedProviderFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_HostedProviderFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.GetApplication == other.GetApplication
    }
}
impl ::core::cmp::Eq for MI_HostedProviderFT {}
impl ::core::fmt::Debug for MI_HostedProviderFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_HostedProviderFT").field("Close", &self.Close).field("GetApplication", &self.GetApplication).finish()
    }
}
impl ::core::default::Default for MI_Instance {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Instance {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.classDecl == other.classDecl && self.serverName == other.serverName && self.nameSpace == other.nameSpace && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_Instance {}
impl ::core::fmt::Debug for MI_Instance {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Instance").field("ft", &self.ft).field("classDecl", &self.classDecl).field("serverName", &self.serverName).field("nameSpace", &self.nameSpace).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for MI_InstanceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_InstanceA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_InstanceA {}
impl ::core::fmt::Debug for MI_InstanceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_InstanceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_InstanceAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_InstanceAField {}
impl ::core::fmt::Debug for MI_InstanceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_InstanceExFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_InstanceExFT {
    fn eq(&self, other: &Self) -> bool {
        self.parent == other.parent && self.Normalize == other.Normalize
    }
}
impl ::core::cmp::Eq for MI_InstanceExFT {}
impl ::core::fmt::Debug for MI_InstanceExFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceExFT").field("parent", &self.parent).field("Normalize", &self.Normalize).finish()
    }
}
impl ::core::default::Default for MI_InstanceFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_InstanceFT {
    fn eq(&self, other: &Self) -> bool {
        self.Clone == other.Clone && self.Destruct == other.Destruct && self.Delete == other.Delete && self.IsA == other.IsA && self.GetClassNameA == other.GetClassNameA && self.SetNameSpace == other.SetNameSpace && self.GetNameSpace == other.GetNameSpace && self.GetElementCount == other.GetElementCount && self.AddElement == other.AddElement && self.SetElement == other.SetElement && self.SetElementAt == other.SetElementAt && self.GetElement == other.GetElement && self.GetElementAt == other.GetElementAt && self.ClearElement == other.ClearElement && self.ClearElementAt == other.ClearElementAt && self.GetServerName == other.GetServerName && self.SetServerName == other.SetServerName && self.GetClass == other.GetClass
    }
}
impl ::core::cmp::Eq for MI_InstanceFT {}
impl ::core::fmt::Debug for MI_InstanceFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceFT")
            .field("Clone", &self.Clone)
            .field("Destruct", &self.Destruct)
            .field("Delete", &self.Delete)
            .field("IsA", &self.IsA)
            .field("GetClassNameA", &self.GetClassNameA)
            .field("SetNameSpace", &self.SetNameSpace)
            .field("GetNameSpace", &self.GetNameSpace)
            .field("GetElementCount", &self.GetElementCount)
            .field("AddElement", &self.AddElement)
            .field("SetElement", &self.SetElement)
            .field("SetElementAt", &self.SetElementAt)
            .field("GetElement", &self.GetElement)
            .field("GetElementAt", &self.GetElementAt)
            .field("ClearElement", &self.ClearElement)
            .field("ClearElementAt", &self.ClearElementAt)
            .field("GetServerName", &self.GetServerName)
            .field("SetServerName", &self.SetServerName)
            .field("GetClass", &self.GetClass)
            .finish()
    }
}
impl ::core::default::Default for MI_InstanceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_InstanceField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_InstanceField {}
impl ::core::fmt::Debug for MI_InstanceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_InstanceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Interval {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Interval {
    fn eq(&self, other: &Self) -> bool {
        self.days == other.days && self.hours == other.hours && self.minutes == other.minutes && self.seconds == other.seconds && self.microseconds == other.microseconds && self.__padding1 == other.__padding1 && self.__padding2 == other.__padding2 && self.__padding3 == other.__padding3
    }
}
impl ::core::cmp::Eq for MI_Interval {}
impl ::core::fmt::Debug for MI_Interval {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Interval").field("days", &self.days).field("hours", &self.hours).field("minutes", &self.minutes).field("seconds", &self.seconds).field("microseconds", &self.microseconds).field("__padding1", &self.__padding1).field("__padding2", &self.__padding2).field("__padding3", &self.__padding3).finish()
    }
}
impl ::core::default::Default for MI_LocaleType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_LocaleType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_LocaleType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_MethodDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MI_Module {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MI_ObjectDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ObjectDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers && self.properties == other.properties && self.numProperties == other.numProperties && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ObjectDecl {}
impl ::core::fmt::Debug for MI_ObjectDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ObjectDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("properties", &self.properties).field("numProperties", &self.numProperties).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Operation {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Operation {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_Operation {}
impl ::core::fmt::Debug for MI_Operation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Operation").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_OperationCallback_ResponseType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_OperationCallback_ResponseType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_OperationCallback_ResponseType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_OperationCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MI_OperationFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_OperationFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.Cancel == other.Cancel && self.GetSession == other.GetSession && self.GetInstance == other.GetInstance && self.GetIndication == other.GetIndication && self.GetClass == other.GetClass
    }
}
impl ::core::cmp::Eq for MI_OperationFT {}
impl ::core::fmt::Debug for MI_OperationFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationFT").field("Close", &self.Close).field("Cancel", &self.Cancel).field("GetSession", &self.GetSession).field("GetInstance", &self.GetInstance).field("GetIndication", &self.GetIndication).field("GetClass", &self.GetClass).finish()
    }
}
impl ::core::default::Default for MI_OperationOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_OperationOptions {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_OperationOptions {}
impl ::core::fmt::Debug for MI_OperationOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_OperationOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_OperationOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        self.Delete == other.Delete && self.SetString == other.SetString && self.SetNumber == other.SetNumber && self.SetCustomOption == other.SetCustomOption && self.GetString == other.GetString && self.GetNumber == other.GetNumber && self.GetOptionCount == other.GetOptionCount && self.GetOptionAt == other.GetOptionAt && self.GetOption == other.GetOption && self.GetEnabledChannels == other.GetEnabledChannels && self.Clone == other.Clone && self.SetInterval == other.SetInterval && self.GetInterval == other.GetInterval
    }
}
impl ::core::cmp::Eq for MI_OperationOptionsFT {}
impl ::core::fmt::Debug for MI_OperationOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_OperationOptionsFT")
            .field("Delete", &self.Delete)
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("SetCustomOption", &self.SetCustomOption)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetEnabledChannels", &self.GetEnabledChannels)
            .field("Clone", &self.Clone)
            .field("SetInterval", &self.SetInterval)
            .field("GetInterval", &self.GetInterval)
            .finish()
    }
}
impl ::core::default::Default for MI_ParameterDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ParameterDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers && self.r#type == other.r#type && self.className == other.className && self.subscript == other.subscript && self.offset == other.offset
    }
}
impl ::core::cmp::Eq for MI_ParameterDecl {}
impl ::core::fmt::Debug for MI_ParameterDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("type", &self.r#type).field("className", &self.className).field("subscript", &self.subscript).field("offset", &self.offset).finish()
    }
}
impl ::core::default::Default for MI_ParameterSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ParameterSet {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_ParameterSet {}
impl ::core::fmt::Debug for MI_ParameterSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterSet").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_ParameterSetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ParameterSetFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetMethodReturnType == other.GetMethodReturnType && self.GetParameterCount == other.GetParameterCount && self.GetParameterAt == other.GetParameterAt && self.GetParameter == other.GetParameter
    }
}
impl ::core::cmp::Eq for MI_ParameterSetFT {}
impl ::core::fmt::Debug for MI_ParameterSetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ParameterSetFT").field("GetMethodReturnType", &self.GetMethodReturnType).field("GetParameterCount", &self.GetParameterCount).field("GetParameterAt", &self.GetParameterAt).field("GetParameter", &self.GetParameter).finish()
    }
}
impl ::core::default::Default for MI_PromptType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_PromptType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_PromptType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_PropertyDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_PropertyDecl {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.code == other.code && self.name == other.name && self.qualifiers == other.qualifiers && self.numQualifiers == other.numQualifiers && self.r#type == other.r#type && self.className == other.className && self.subscript == other.subscript && self.offset == other.offset && self.origin == other.origin && self.propagator == other.propagator && self.value == other.value
    }
}
impl ::core::cmp::Eq for MI_PropertyDecl {}
impl ::core::fmt::Debug for MI_PropertyDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertyDecl").field("flags", &self.flags).field("code", &self.code).field("name", &self.name).field("qualifiers", &self.qualifiers).field("numQualifiers", &self.numQualifiers).field("type", &self.r#type).field("className", &self.className).field("subscript", &self.subscript).field("offset", &self.offset).field("origin", &self.origin).field("propagator", &self.propagator).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for MI_PropertySet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_PropertySet {
    fn eq(&self, other: &Self) -> bool {
        self.ft == other.ft && self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for MI_PropertySet {}
impl ::core::fmt::Debug for MI_PropertySet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertySet").field("ft", &self.ft).field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for MI_PropertySetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_PropertySetFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetElementCount == other.GetElementCount && self.ContainsElement == other.ContainsElement && self.AddElement == other.AddElement && self.GetElementAt == other.GetElementAt && self.Clear == other.Clear && self.Destruct == other.Destruct && self.Delete == other.Delete && self.Clone == other.Clone
    }
}
impl ::core::cmp::Eq for MI_PropertySetFT {}
impl ::core::fmt::Debug for MI_PropertySetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_PropertySetFT").field("GetElementCount", &self.GetElementCount).field("ContainsElement", &self.ContainsElement).field("AddElement", &self.AddElement).field("GetElementAt", &self.GetElementAt).field("Clear", &self.Clear).field("Destruct", &self.Destruct).field("Delete", &self.Delete).field("Clone", &self.Clone).finish()
    }
}
impl ::core::default::Default for MI_ProviderArchitecture {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_ProviderArchitecture {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_ProviderArchitecture").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_ProviderFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MI_Qualifier {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Qualifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.flavor == other.flavor && self.value == other.value
    }
}
impl ::core::cmp::Eq for MI_Qualifier {}
impl ::core::fmt::Debug for MI_Qualifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Qualifier").field("name", &self.name).field("type", &self.r#type).field("flavor", &self.flavor).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for MI_QualifierDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_QualifierDecl {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.r#type == other.r#type && self.scope == other.scope && self.flavor == other.flavor && self.subscript == other.subscript && self.value == other.value
    }
}
impl ::core::cmp::Eq for MI_QualifierDecl {}
impl ::core::fmt::Debug for MI_QualifierDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierDecl").field("name", &self.name).field("type", &self.r#type).field("scope", &self.scope).field("flavor", &self.flavor).field("subscript", &self.subscript).field("value", &self.value).finish()
    }
}
impl ::core::default::Default for MI_QualifierSet {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_QualifierSet {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_QualifierSet {}
impl ::core::fmt::Debug for MI_QualifierSet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierSet").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_QualifierSetFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_QualifierSetFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetQualifierCount == other.GetQualifierCount && self.GetQualifierAt == other.GetQualifierAt && self.GetQualifier == other.GetQualifier
    }
}
impl ::core::cmp::Eq for MI_QualifierSetFT {}
impl ::core::fmt::Debug for MI_QualifierSetFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_QualifierSetFT").field("GetQualifierCount", &self.GetQualifierCount).field("GetQualifierAt", &self.GetQualifierAt).field("GetQualifier", &self.GetQualifier).finish()
    }
}
impl ::core::default::Default for MI_Real32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Real32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Real32A {}
impl ::core::fmt::Debug for MI_Real32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Real32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Real32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Real32AField {}
impl ::core::fmt::Debug for MI_Real32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Real32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Real32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Real32Field {}
impl ::core::fmt::Debug for MI_Real32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Real64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Real64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Real64A {}
impl ::core::fmt::Debug for MI_Real64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Real64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Real64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Real64AField {}
impl ::core::fmt::Debug for MI_Real64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Real64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Real64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Real64Field {}
impl ::core::fmt::Debug for MI_Real64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Real64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ReferenceA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ReferenceA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_ReferenceA {}
impl ::core::fmt::Debug for MI_ReferenceA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_ReferenceAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ReferenceAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ReferenceAField {}
impl ::core::fmt::Debug for MI_ReferenceAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_ReferenceField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ReferenceField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_ReferenceField {}
impl ::core::fmt::Debug for MI_ReferenceField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ReferenceField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Result {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_Result {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_Result").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_SchemaDecl {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_SchemaDecl {
    fn eq(&self, other: &Self) -> bool {
        self.qualifierDecls == other.qualifierDecls && self.numQualifierDecls == other.numQualifierDecls && self.classDecls == other.classDecls && self.numClassDecls == other.numClassDecls
    }
}
impl ::core::cmp::Eq for MI_SchemaDecl {}
impl ::core::fmt::Debug for MI_SchemaDecl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SchemaDecl").field("qualifierDecls", &self.qualifierDecls).field("numQualifierDecls", &self.numQualifierDecls).field("classDecls", &self.classDecls).field("numClassDecls", &self.numClassDecls).finish()
    }
}
impl ::core::default::Default for MI_Serializer {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Serializer {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2
    }
}
impl ::core::cmp::Eq for MI_Serializer {}
impl ::core::fmt::Debug for MI_Serializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Serializer").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).finish()
    }
}
impl ::core::default::Default for MI_SerializerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_SerializerFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.SerializeClass == other.SerializeClass && self.SerializeInstance == other.SerializeInstance
    }
}
impl ::core::cmp::Eq for MI_SerializerFT {}
impl ::core::fmt::Debug for MI_SerializerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SerializerFT").field("Close", &self.Close).field("SerializeClass", &self.SerializeClass).field("SerializeInstance", &self.SerializeInstance).finish()
    }
}
impl ::core::default::Default for MI_Server {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Server {
    fn eq(&self, other: &Self) -> bool {
        self.serverFT == other.serverFT && self.contextFT == other.contextFT && self.instanceFT == other.instanceFT && self.propertySetFT == other.propertySetFT && self.filterFT == other.filterFT
    }
}
impl ::core::cmp::Eq for MI_Server {}
impl ::core::fmt::Debug for MI_Server {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Server").field("serverFT", &self.serverFT).field("contextFT", &self.contextFT).field("instanceFT", &self.instanceFT).field("propertySetFT", &self.propertySetFT).field("filterFT", &self.filterFT).finish()
    }
}
impl ::core::default::Default for MI_ServerFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_ServerFT {
    fn eq(&self, other: &Self) -> bool {
        self.GetVersion == other.GetVersion && self.GetSystemName == other.GetSystemName
    }
}
impl ::core::cmp::Eq for MI_ServerFT {}
impl ::core::fmt::Debug for MI_ServerFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_ServerFT").field("GetVersion", &self.GetVersion).field("GetSystemName", &self.GetSystemName).finish()
    }
}
impl ::core::default::Default for MI_Session {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Session {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_Session {}
impl ::core::fmt::Debug for MI_Session {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Session").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_SessionCallbacks {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_SessionCallbacks {
    fn eq(&self, other: &Self) -> bool {
        self.callbackContext == other.callbackContext && self.writeMessage == other.writeMessage && self.writeError == other.writeError
    }
}
impl ::core::cmp::Eq for MI_SessionCallbacks {}
impl ::core::fmt::Debug for MI_SessionCallbacks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SessionCallbacks").field("callbackContext", &self.callbackContext).field("writeMessage", &self.writeMessage).field("writeError", &self.writeError).finish()
    }
}
impl ::core::default::Default for MI_SessionFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_SessionFT {
    fn eq(&self, other: &Self) -> bool {
        self.Close == other.Close && self.GetApplication == other.GetApplication && self.GetInstance == other.GetInstance && self.ModifyInstance == other.ModifyInstance && self.CreateInstance == other.CreateInstance && self.DeleteInstance == other.DeleteInstance && self.Invoke == other.Invoke && self.EnumerateInstances == other.EnumerateInstances && self.QueryInstances == other.QueryInstances && self.AssociatorInstances == other.AssociatorInstances && self.ReferenceInstances == other.ReferenceInstances && self.Subscribe == other.Subscribe && self.GetClass == other.GetClass && self.EnumerateClasses == other.EnumerateClasses && self.TestConnection == other.TestConnection
    }
}
impl ::core::cmp::Eq for MI_SessionFT {}
impl ::core::fmt::Debug for MI_SessionFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SessionFT")
            .field("Close", &self.Close)
            .field("GetApplication", &self.GetApplication)
            .field("GetInstance", &self.GetInstance)
            .field("ModifyInstance", &self.ModifyInstance)
            .field("CreateInstance", &self.CreateInstance)
            .field("DeleteInstance", &self.DeleteInstance)
            .field("Invoke", &self.Invoke)
            .field("EnumerateInstances", &self.EnumerateInstances)
            .field("QueryInstances", &self.QueryInstances)
            .field("AssociatorInstances", &self.AssociatorInstances)
            .field("ReferenceInstances", &self.ReferenceInstances)
            .field("Subscribe", &self.Subscribe)
            .field("GetClass", &self.GetClass)
            .field("EnumerateClasses", &self.EnumerateClasses)
            .field("TestConnection", &self.TestConnection)
            .finish()
    }
}
impl ::core::default::Default for MI_Sint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Sint16A {}
impl ::core::fmt::Debug for MI_Sint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Sint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint16AField {}
impl ::core::fmt::Debug for MI_Sint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Sint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint16Field {}
impl ::core::fmt::Debug for MI_Sint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Sint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Sint32A {}
impl ::core::fmt::Debug for MI_Sint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Sint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint32AField {}
impl ::core::fmt::Debug for MI_Sint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Sint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint32Field {}
impl ::core::fmt::Debug for MI_Sint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Sint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Sint64A {}
impl ::core::fmt::Debug for MI_Sint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Sint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint64AField {}
impl ::core::fmt::Debug for MI_Sint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Sint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint64Field {}
impl ::core::fmt::Debug for MI_Sint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Sint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint8A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Sint8A {}
impl ::core::fmt::Debug for MI_Sint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Sint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint8AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint8AField {}
impl ::core::fmt::Debug for MI_Sint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Sint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Sint8Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Sint8Field {}
impl ::core::fmt::Debug for MI_Sint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Sint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_StringA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_StringA {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_StringA {}
impl ::core::fmt::Debug for MI_StringA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringA").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_StringAField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_StringAField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_StringAField {}
impl ::core::fmt::Debug for MI_StringAField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringAField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_StringField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_StringField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_StringField {}
impl ::core::fmt::Debug for MI_StringField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_StringField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_SubscriptionDeliveryOptions {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_SubscriptionDeliveryOptions {
    fn eq(&self, other: &Self) -> bool {
        self.reserved1 == other.reserved1 && self.reserved2 == other.reserved2 && self.ft == other.ft
    }
}
impl ::core::cmp::Eq for MI_SubscriptionDeliveryOptions {}
impl ::core::fmt::Debug for MI_SubscriptionDeliveryOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SubscriptionDeliveryOptions").field("reserved1", &self.reserved1).field("reserved2", &self.reserved2).field("ft", &self.ft).finish()
    }
}
impl ::core::default::Default for MI_SubscriptionDeliveryOptionsFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_SubscriptionDeliveryOptionsFT {
    fn eq(&self, other: &Self) -> bool {
        self.SetString == other.SetString && self.SetNumber == other.SetNumber && self.SetDateTime == other.SetDateTime && self.SetInterval == other.SetInterval && self.AddCredentials == other.AddCredentials && self.Delete == other.Delete && self.GetString == other.GetString && self.GetNumber == other.GetNumber && self.GetDateTime == other.GetDateTime && self.GetInterval == other.GetInterval && self.GetOptionCount == other.GetOptionCount && self.GetOptionAt == other.GetOptionAt && self.GetOption == other.GetOption && self.GetCredentialsCount == other.GetCredentialsCount && self.GetCredentialsAt == other.GetCredentialsAt && self.GetCredentialsPasswordAt == other.GetCredentialsPasswordAt && self.Clone == other.Clone
    }
}
impl ::core::cmp::Eq for MI_SubscriptionDeliveryOptionsFT {}
impl ::core::fmt::Debug for MI_SubscriptionDeliveryOptionsFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_SubscriptionDeliveryOptionsFT")
            .field("SetString", &self.SetString)
            .field("SetNumber", &self.SetNumber)
            .field("SetDateTime", &self.SetDateTime)
            .field("SetInterval", &self.SetInterval)
            .field("AddCredentials", &self.AddCredentials)
            .field("Delete", &self.Delete)
            .field("GetString", &self.GetString)
            .field("GetNumber", &self.GetNumber)
            .field("GetDateTime", &self.GetDateTime)
            .field("GetInterval", &self.GetInterval)
            .field("GetOptionCount", &self.GetOptionCount)
            .field("GetOptionAt", &self.GetOptionAt)
            .field("GetOption", &self.GetOption)
            .field("GetCredentialsCount", &self.GetCredentialsCount)
            .field("GetCredentialsAt", &self.GetCredentialsAt)
            .field("GetCredentialsPasswordAt", &self.GetCredentialsPasswordAt)
            .field("Clone", &self.Clone)
            .finish()
    }
}
impl ::core::default::Default for MI_SubscriptionDeliveryType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_SubscriptionDeliveryType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_SubscriptionDeliveryType").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_Timestamp {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month && self.day == other.day && self.hour == other.hour && self.minute == other.minute && self.second == other.second && self.microseconds == other.microseconds && self.utc == other.utc
    }
}
impl ::core::cmp::Eq for MI_Timestamp {}
impl ::core::fmt::Debug for MI_Timestamp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Timestamp").field("year", &self.year).field("month", &self.month).field("day", &self.day).field("hour", &self.hour).field("minute", &self.minute).field("second", &self.second).field("microseconds", &self.microseconds).field("utc", &self.utc).finish()
    }
}
impl ::core::default::Default for MI_Type {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for MI_Type {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MI_Type").field(&self.0).finish()
    }
}
impl ::core::default::Default for MI_Uint16A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint16A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Uint16A {}
impl ::core::fmt::Debug for MI_Uint16A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Uint16AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint16AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint16AField {}
impl ::core::fmt::Debug for MI_Uint16AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Uint16Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint16Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint16Field {}
impl ::core::fmt::Debug for MI_Uint16Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint16Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Uint32A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint32A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Uint32A {}
impl ::core::fmt::Debug for MI_Uint32A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Uint32AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint32AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint32AField {}
impl ::core::fmt::Debug for MI_Uint32AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Uint32Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint32Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint32Field {}
impl ::core::fmt::Debug for MI_Uint32Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint32Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Uint64A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint64A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Uint64A {}
impl ::core::fmt::Debug for MI_Uint64A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Uint64AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint64AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint64AField {}
impl ::core::fmt::Debug for MI_Uint64AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Uint64Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint64Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint64Field {}
impl ::core::fmt::Debug for MI_Uint64Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint64Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Uint8A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint8A {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.size == other.size
    }
}
impl ::core::cmp::Eq for MI_Uint8A {}
impl ::core::fmt::Debug for MI_Uint8A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8A").field("data", &self.data).field("size", &self.size).finish()
    }
}
impl ::core::default::Default for MI_Uint8AField {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint8AField {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint8AField {}
impl ::core::fmt::Debug for MI_Uint8AField {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8AField").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_Uint8Field {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_Uint8Field {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.exists == other.exists && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for MI_Uint8Field {}
impl ::core::fmt::Debug for MI_Uint8Field {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_Uint8Field").field("value", &self.value).field("exists", &self.exists).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for MI_UserCredentials {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for MI_UsernamePasswordCreds {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_UsernamePasswordCreds {
    fn eq(&self, other: &Self) -> bool {
        self.domain == other.domain && self.username == other.username && self.password == other.password
    }
}
impl ::core::cmp::Eq for MI_UsernamePasswordCreds {}
impl ::core::fmt::Debug for MI_UsernamePasswordCreds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_UsernamePasswordCreds").field("domain", &self.domain).field("username", &self.username).field("password", &self.password).finish()
    }
}
impl ::core::default::Default for MI_UtilitiesFT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for MI_UtilitiesFT {
    fn eq(&self, other: &Self) -> bool {
        self.MapErrorToMiErrorCategory == other.MapErrorToMiErrorCategory && self.CimErrorFromErrorCode == other.CimErrorFromErrorCode
    }
}
impl ::core::cmp::Eq for MI_UtilitiesFT {}
impl ::core::fmt::Debug for MI_UtilitiesFT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MI_UtilitiesFT").field("MapErrorToMiErrorCategory", &self.MapErrorToMiErrorCategory).field("CimErrorFromErrorCode", &self.CimErrorFromErrorCode).finish()
    }
}
impl ::core::default::Default for MI_Value {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemAnalysisMatrix {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemAnalysisMatrix {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uMatrixType == other.m_uMatrixType && self.m_pszProperty == other.m_pszProperty && self.m_uPropertyType == other.m_uPropertyType && self.m_uEntries == other.m_uEntries && self.m_pValues == other.m_pValues && self.m_pbTruthTable == other.m_pbTruthTable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemAnalysisMatrix {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SWbemAnalysisMatrix {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAnalysisMatrix").field("m_uVersion", &self.m_uVersion).field("m_uMatrixType", &self.m_uMatrixType).field("m_pszProperty", &self.m_pszProperty).field("m_uPropertyType", &self.m_uPropertyType).field("m_uEntries", &self.m_uEntries).field("m_pValues", &self.m_pValues).field("m_pbTruthTable", &self.m_pbTruthTable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemAnalysisMatrixList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemAnalysisMatrixList {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uMatrixType == other.m_uMatrixType && self.m_uNumMatrices == other.m_uNumMatrices && self.m_pMatrices == other.m_pMatrices
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemAnalysisMatrixList {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SWbemAnalysisMatrixList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAnalysisMatrixList").field("m_uVersion", &self.m_uVersion).field("m_uMatrixType", &self.m_uMatrixType).field("m_uNumMatrices", &self.m_uNumMatrices).field("m_pMatrices", &self.m_pMatrices).finish()
    }
}
impl ::core::default::Default for SWbemAssocQueryInf {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SWbemAssocQueryInf {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uAnalysisType == other.m_uAnalysisType && self.m_uFeatureMask == other.m_uFeatureMask && self.m_pPath == other.m_pPath && self.m_pszPath == other.m_pszPath && self.m_pszQueryText == other.m_pszQueryText && self.m_pszResultClass == other.m_pszResultClass && self.m_pszAssocClass == other.m_pszAssocClass && self.m_pszRole == other.m_pszRole && self.m_pszResultRole == other.m_pszResultRole && self.m_pszRequiredQualifier == other.m_pszRequiredQualifier && self.m_pszRequiredAssocQualifier == other.m_pszRequiredAssocQualifier
    }
}
impl ::core::cmp::Eq for SWbemAssocQueryInf {}
impl ::core::fmt::Debug for SWbemAssocQueryInf {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemAssocQueryInf")
            .field("m_uVersion", &self.m_uVersion)
            .field("m_uAnalysisType", &self.m_uAnalysisType)
            .field("m_uFeatureMask", &self.m_uFeatureMask)
            .field("m_pPath", &self.m_pPath)
            .field("m_pszPath", &self.m_pszPath)
            .field("m_pszQueryText", &self.m_pszQueryText)
            .field("m_pszResultClass", &self.m_pszResultClass)
            .field("m_pszAssocClass", &self.m_pszAssocClass)
            .field("m_pszRole", &self.m_pszRole)
            .field("m_pszResultRole", &self.m_pszResultRole)
            .field("m_pszRequiredQualifier", &self.m_pszRequiredQualifier)
            .field("m_pszRequiredAssocQualifier", &self.m_pszRequiredAssocQualifier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemQueryQualifiedName {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemQueryQualifiedName {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uTokenType == other.m_uTokenType && self.m_uNameListSize == other.m_uNameListSize && self.m_ppszNameList == other.m_ppszNameList && self.m_bArraysUsed == other.m_bArraysUsed && self.m_pbArrayElUsed == other.m_pbArrayElUsed && self.m_puArrayIndex == other.m_puArrayIndex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemQueryQualifiedName {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SWbemQueryQualifiedName {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemQueryQualifiedName").field("m_uVersion", &self.m_uVersion).field("m_uTokenType", &self.m_uTokenType).field("m_uNameListSize", &self.m_uNameListSize).field("m_ppszNameList", &self.m_ppszNameList).field("m_bArraysUsed", &self.m_bArraysUsed).field("m_pbArrayElUsed", &self.m_pbArrayElUsed).field("m_puArrayIndex", &self.m_puArrayIndex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemRpnConst {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemRpnEncodedQuery {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SWbemRpnEncodedQuery {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion
            && self.m_uTokenType == other.m_uTokenType
            && self.m_uParsedFeatureMask == other.m_uParsedFeatureMask
            && self.m_uDetectedArraySize == other.m_uDetectedArraySize
            && self.m_puDetectedFeatures == other.m_puDetectedFeatures
            && self.m_uSelectListSize == other.m_uSelectListSize
            && self.m_ppSelectList == other.m_ppSelectList
            && self.m_uFromTargetType == other.m_uFromTargetType
            && self.m_pszOptionalFromPath == other.m_pszOptionalFromPath
            && self.m_uFromListSize == other.m_uFromListSize
            && self.m_ppszFromList == other.m_ppszFromList
            && self.m_uWhereClauseSize == other.m_uWhereClauseSize
            && self.m_ppRpnWhereClause == other.m_ppRpnWhereClause
            && self.m_dblWithinPolling == other.m_dblWithinPolling
            && self.m_dblWithinWindow == other.m_dblWithinWindow
            && self.m_uOrderByListSize == other.m_uOrderByListSize
            && self.m_ppszOrderByList == other.m_ppszOrderByList
            && self.m_uOrderDirectionEl == other.m_uOrderDirectionEl
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SWbemRpnEncodedQuery {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SWbemRpnEncodedQuery {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemRpnEncodedQuery")
            .field("m_uVersion", &self.m_uVersion)
            .field("m_uTokenType", &self.m_uTokenType)
            .field("m_uParsedFeatureMask", &self.m_uParsedFeatureMask)
            .field("m_uDetectedArraySize", &self.m_uDetectedArraySize)
            .field("m_puDetectedFeatures", &self.m_puDetectedFeatures)
            .field("m_uSelectListSize", &self.m_uSelectListSize)
            .field("m_ppSelectList", &self.m_ppSelectList)
            .field("m_uFromTargetType", &self.m_uFromTargetType)
            .field("m_pszOptionalFromPath", &self.m_pszOptionalFromPath)
            .field("m_uFromListSize", &self.m_uFromListSize)
            .field("m_ppszFromList", &self.m_ppszFromList)
            .field("m_uWhereClauseSize", &self.m_uWhereClauseSize)
            .field("m_ppRpnWhereClause", &self.m_ppRpnWhereClause)
            .field("m_dblWithinPolling", &self.m_dblWithinPolling)
            .field("m_dblWithinWindow", &self.m_dblWithinWindow)
            .field("m_uOrderByListSize", &self.m_uOrderByListSize)
            .field("m_ppszOrderByList", &self.m_ppszOrderByList)
            .field("m_uOrderDirectionEl", &self.m_uOrderDirectionEl)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SWbemRpnQueryToken {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for SWbemRpnTokenList {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for SWbemRpnTokenList {
    fn eq(&self, other: &Self) -> bool {
        self.m_uVersion == other.m_uVersion && self.m_uTokenType == other.m_uTokenType && self.m_uNumTokens == other.m_uNumTokens
    }
}
impl ::core::cmp::Eq for SWbemRpnTokenList {}
impl ::core::fmt::Debug for SWbemRpnTokenList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SWbemRpnTokenList").field("m_uVersion", &self.m_uVersion).field("m_uTokenType", &self.m_uTokenType).field("m_uNumTokens", &self.m_uNumTokens).finish()
    }
}
impl ::core::default::Default for WBEMSTATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEMSTATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEMSTATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEMSTATUS_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEMSTATUS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEMSTATUS_FORMAT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_BACKUP_RESTORE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_BACKUP_RESTORE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_BACKUP_RESTORE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_BATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_BATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_BATCH_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_CHANGE_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_CHANGE_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CHANGE_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_COMPARISON_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_COMPARISON_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_COMPARISON_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_COMPILER_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_COMPILER_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_COMPILER_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_COMPILE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WBEM_COMPILE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.lPhaseError == other.lPhaseError && self.hRes == other.hRes && self.ObjectNum == other.ObjectNum && self.FirstLine == other.FirstLine && self.LastLine == other.LastLine && self.dwOutFlags == other.dwOutFlags
    }
}
impl ::core::cmp::Eq for WBEM_COMPILE_STATUS_INFO {}
impl ::core::fmt::Debug for WBEM_COMPILE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WBEM_COMPILE_STATUS_INFO").field("lPhaseError", &self.lPhaseError).field("hRes", &self.hRes).field("ObjectNum", &self.ObjectNum).field("FirstLine", &self.FirstLine).field("LastLine", &self.LastLine).field("dwOutFlags", &self.dwOutFlags).finish()
    }
}
impl ::core::default::Default for WBEM_CONDITION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_CONDITION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CONDITION_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_CONNECT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_CONNECT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_CONNECT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_EXTRA_RETURN_CODES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_EXTRA_RETURN_CODES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_EXTRA_RETURN_CODES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_FLAVOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_FLAVOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_FLAVOR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_GENERIC_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_GENERIC_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GENERIC_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WBEM_GENERIC_FLAG_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WBEM_GENERIC_FLAG_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WBEM_GENERIC_FLAG_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for WBEM_GENUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_GENUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GENUS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_GET_KEY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_GET_KEY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GET_KEY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_GET_TEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_GET_TEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_GET_TEXT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_INFORMATION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_INFORMATION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_INFORMATION_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_LIMITATION_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_LIMITATION_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LIMITATION_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_LIMITS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_LIMITS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LIMITS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_LOCKING_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_LOCKING_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LOCKING_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_LOGIN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_LOGIN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_LOGIN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_PATH_CREATE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_PATH_CREATE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PATH_CREATE_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_PATH_STATUS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_PATH_STATUS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PATH_STATUS_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_PROVIDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_PROVIDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PROVIDER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_PROVIDER_REQUIREMENTS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_PROVIDER_REQUIREMENTS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_QUERY_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_QUERY_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_QUERY_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_REFRESHER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_REFRESHER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_REFRESHER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_SECURITY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_SECURITY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_SECURITY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_SHUTDOWN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_SHUTDOWN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_SHUTDOWN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_STATUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_STATUS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_TEXT_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_TEXT_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_TEXT_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WBEM_UNSECAPP_FLAG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WBEM_UNSECAPP_FLAG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WBEM_UNSECAPP_FLAG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMIQ_ANALYSIS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMIQ_ANALYSIS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_ANALYSIS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMIQ_ASSOCQ_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMIQ_ASSOCQ_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_ASSOCQ_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMIQ_LANGUAGE_FEATURES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMIQ_LANGUAGE_FEATURES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_LANGUAGE_FEATURES").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMIQ_RPNF_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMIQ_RPNF_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_RPNF_FEATURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMIQ_RPN_TOKEN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMIQ_RPN_TOKEN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMIQ_RPN_TOKEN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for WMI_OBJ_TEXT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WMI_OBJ_TEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WMI_OBJ_TEXT").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemAuthenticationLevelEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemAuthenticationLevelEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemAuthenticationLevelEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemChangeFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemChangeFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemChangeFlagEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemCimtypeEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemCimtypeEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemCimtypeEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemComparisonFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemComparisonFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemComparisonFlagEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemConnectOptionsEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemConnectOptionsEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemConnectOptionsEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemErrorEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemErrorEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemErrorEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemFlagEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemImpersonationLevelEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemImpersonationLevelEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemImpersonationLevelEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemObjectTextFormatEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemObjectTextFormatEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemObjectTextFormatEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemPrivilegeEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemPrivilegeEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemPrivilegeEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemQueryFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemQueryFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemQueryFlagEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemTextFlagEnum {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemTextFlagEnum {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemTextFlagEnum").field(&self.0).finish()
    }
}
impl ::core::default::Default for WbemTimeout {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WbemTimeout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WbemTimeout").field(&self.0).finish()
    }
}
