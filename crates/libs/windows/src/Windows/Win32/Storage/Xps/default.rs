impl ::core::default::Default for DEVICE_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DEVICE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DEVICE_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::default::Default for DOCINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOCINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpszDocName == other.lpszDocName && self.lpszOutput == other.lpszOutput && self.lpszDatatype == other.lpszDatatype && self.fwType == other.fwType
    }
}
impl ::core::cmp::Eq for DOCINFOA {}
impl ::core::fmt::Debug for DOCINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCINFOA").field("cbSize", &self.cbSize).field("lpszDocName", &self.lpszDocName).field("lpszOutput", &self.lpszOutput).field("lpszDatatype", &self.lpszDatatype).field("fwType", &self.fwType).finish()
    }
}
impl ::core::default::Default for DOCINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DOCINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.lpszDocName == other.lpszDocName && self.lpszOutput == other.lpszOutput && self.lpszDatatype == other.lpszDatatype && self.fwType == other.fwType
    }
}
impl ::core::cmp::Eq for DOCINFOW {}
impl ::core::fmt::Debug for DOCINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOCINFOW").field("cbSize", &self.cbSize).field("lpszDocName", &self.lpszDocName).field("lpszOutput", &self.lpszOutput).field("lpszDatatype", &self.lpszDatatype).field("fwType", &self.fwType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DRAWPATRECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DRAWPATRECT {
    fn eq(&self, other: &Self) -> bool {
        self.ptPosition == other.ptPosition && self.ptSize == other.ptSize && self.wStyle == other.wStyle && self.wPattern == other.wPattern
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DRAWPATRECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DRAWPATRECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DRAWPATRECT").field("ptPosition", &self.ptPosition).field("ptSize", &self.ptSize).field("wStyle", &self.wStyle).field("wPattern", &self.wPattern).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsDocumentPackageTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsDocumentPackageTarget {}
impl ::core::fmt::Debug for IXpsDocumentPackageTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsDocumentPackageTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsDocumentPackageTarget3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsDocumentPackageTarget3D {}
impl ::core::fmt::Debug for IXpsDocumentPackageTarget3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsDocumentPackageTarget3D").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMBrush {}
impl ::core::fmt::Debug for IXpsOMBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMBrush").field(&self.0).finish()
    }
}
impl IXpsOMBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXpsOMCanvas {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMCanvas {}
impl ::core::fmt::Debug for IXpsOMCanvas {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMCanvas").field(&self.0).finish()
    }
}
impl IXpsOMCanvas {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLocal)(::windows::core::Vtable::as_raw(self), matrixtransform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometryLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGeometry>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetClipGeometryLocal)(::windows::core::Vtable::as_raw(self), clipgeometry.into().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometryLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetClipGeometryLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrush)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrushLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMBrush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOpacityMaskBrushLocal)(::windows::core::Vtable::as_raw(self), opacitymaskbrush.into().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrushLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOpacityMaskBrushLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), ishyperlink.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHyperlinkNavigateUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHyperlinkNavigateUri)(::windows::core::Vtable::as_raw(self), hyperlinkuri.into().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLanguage)(::windows::core::Vtable::as_raw(self), language.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMColorProfileResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMColorProfileResource {}
impl ::core::fmt::Debug for IXpsOMColorProfileResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMColorProfileResource").field(&self.0).finish()
    }
}
impl IXpsOMColorProfileResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMColorProfileResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMColorProfileResourceCollection {}
impl ::core::fmt::Debug for IXpsOMColorProfileResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMColorProfileResourceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMCoreProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMCoreProperties {}
impl ::core::fmt::Debug for IXpsOMCoreProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMCoreProperties").field(&self.0).finish()
    }
}
impl IXpsOMCoreProperties {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMDashCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDashCollection {}
impl ::core::fmt::Debug for IXpsOMDashCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDashCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMDictionary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDictionary {}
impl ::core::fmt::Debug for IXpsOMDictionary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDictionary").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMDocument {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDocument {}
impl ::core::fmt::Debug for IXpsOMDocument {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDocument").field(&self.0).finish()
    }
}
impl IXpsOMDocument {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMDocumentCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDocumentCollection {}
impl ::core::fmt::Debug for IXpsOMDocumentCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDocumentCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMDocumentSequence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDocumentSequence {}
impl ::core::fmt::Debug for IXpsOMDocumentSequence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDocumentSequence").field(&self.0).finish()
    }
}
impl IXpsOMDocumentSequence {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMDocumentStructureResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMDocumentStructureResource {}
impl ::core::fmt::Debug for IXpsOMDocumentStructureResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMDocumentStructureResource").field(&self.0).finish()
    }
}
impl IXpsOMDocumentStructureResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMFontResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMFontResource {}
impl ::core::fmt::Debug for IXpsOMFontResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMFontResource").field(&self.0).finish()
    }
}
impl IXpsOMFontResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMFontResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMFontResourceCollection {}
impl ::core::fmt::Debug for IXpsOMFontResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMFontResourceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMGeometry {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGeometry {}
impl ::core::fmt::Debug for IXpsOMGeometry {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGeometry").field(&self.0).finish()
    }
}
impl IXpsOMGeometry {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXpsOMGeometryFigure {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGeometryFigure {}
impl ::core::fmt::Debug for IXpsOMGeometryFigure {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGeometryFigure").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMGeometryFigureCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGeometryFigureCollection {}
impl ::core::fmt::Debug for IXpsOMGeometryFigureCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGeometryFigureCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMGlyphs {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGlyphs {}
impl ::core::fmt::Debug for IXpsOMGlyphs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGlyphs").field(&self.0).finish()
    }
}
impl IXpsOMGlyphs {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLocal)(::windows::core::Vtable::as_raw(self), matrixtransform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometryLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGeometry>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetClipGeometryLocal)(::windows::core::Vtable::as_raw(self), clipgeometry.into().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometryLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetClipGeometryLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrush)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrushLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMBrush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOpacityMaskBrushLocal)(::windows::core::Vtable::as_raw(self), opacitymaskbrush.into().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrushLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOpacityMaskBrushLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), ishyperlink.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHyperlinkNavigateUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHyperlinkNavigateUri)(::windows::core::Vtable::as_raw(self), hyperlinkuri.into().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLanguage)(::windows::core::Vtable::as_raw(self), language.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMGlyphsEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGlyphsEditor {}
impl ::core::fmt::Debug for IXpsOMGlyphsEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGlyphsEditor").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGradientBrush {}
impl ::core::fmt::Debug for IXpsOMGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGradientBrush").field(&self.0).finish()
    }
}
impl IXpsOMGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMGradientStop {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGradientStop {}
impl ::core::fmt::Debug for IXpsOMGradientStop {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGradientStop").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMGradientStopCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMGradientStopCollection {}
impl ::core::fmt::Debug for IXpsOMGradientStopCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMGradientStopCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMImageBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMImageBrush {}
impl ::core::fmt::Debug for IXpsOMImageBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMImageBrush").field(&self.0).finish()
    }
}
impl IXpsOMImageBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLocal)(::windows::core::Vtable::as_raw(self), transform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetViewbox)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetViewbox)(::windows::core::Vtable::as_raw(self), viewbox).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetViewport)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetViewport)(::windows::core::Vtable::as_raw(self), viewport).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTileMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTileMode)(::windows::core::Vtable::as_raw(self), tilemode).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMImageResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMImageResource {}
impl ::core::fmt::Debug for IXpsOMImageResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMImageResource").field(&self.0).finish()
    }
}
impl IXpsOMImageResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMImageResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMImageResourceCollection {}
impl ::core::fmt::Debug for IXpsOMImageResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMImageResourceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMLinearGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMLinearGradientBrush {}
impl ::core::fmt::Debug for IXpsOMLinearGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMLinearGradientBrush").field(&self.0).finish()
    }
}
impl IXpsOMLinearGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGradientStops)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLocal)(::windows::core::Vtable::as_raw(self), transform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSpreadMethod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSpreadMethod)(::windows::core::Vtable::as_raw(self), spreadmethod).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetColorInterpolationMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetColorInterpolationMode)(::windows::core::Vtable::as_raw(self), colorinterpolationmode).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMMatrixTransform {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMMatrixTransform {}
impl ::core::fmt::Debug for IXpsOMMatrixTransform {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMMatrixTransform").field(&self.0).finish()
    }
}
impl IXpsOMMatrixTransform {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXpsOMNameCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMNameCollection {}
impl ::core::fmt::Debug for IXpsOMNameCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMNameCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMObjectFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMObjectFactory {}
impl ::core::fmt::Debug for IXpsOMObjectFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMObjectFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMObjectFactory1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMObjectFactory1 {}
impl ::core::fmt::Debug for IXpsOMObjectFactory1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMObjectFactory1").field(&self.0).finish()
    }
}
impl IXpsOMObjectFactory1 {
    pub unsafe fn CreatePackage(&self) -> ::windows::core::Result<IXpsOMPackage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePackage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreatePackageFromFile<P0, P1>(&self, filename: P0, reuseobjects: P1) -> ::windows::core::Result<IXpsOMPackage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePackageFromFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageFromStream<P0, P1>(&self, stream: P0, reuseobjects: P1) -> ::windows::core::Result<IXpsOMPackage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePackageFromStream)(::windows::core::Vtable::as_raw(self), stream.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateStoryFragmentsResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMStoryFragmentsResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStoryFragmentsResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentStructureResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMDocumentStructureResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDocumentStructureResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateSignatureBlockResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMSignatureBlockResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSignatureBlockResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResource<P0, P1>(&self, dictionary: P0, parturi: P1) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDictionary>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRemoteDictionaryResource)(::windows::core::Vtable::as_raw(self), dictionary.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateRemoteDictionaryResourceFromStream<P0, P1, P2>(&self, dictionarymarkupstream: P0, dictionaryparturi: P1, resources: P2) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMPartResources>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRemoteDictionaryResourceFromStream)(::windows::core::Vtable::as_raw(self), dictionarymarkupstream.into().abi(), dictionaryparturi.into().abi(), resources.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePartResources(&self) -> ::windows::core::Result<IXpsOMPartResources> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePartResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocumentSequence<P0>(&self, parturi: P0) -> ::windows::core::Result<IXpsOMDocumentSequence>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDocumentSequence)(::windows::core::Vtable::as_raw(self), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateDocument<P0>(&self, parturi: P0) -> ::windows::core::Result<IXpsOMDocument>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDocument)(::windows::core::Vtable::as_raw(self), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePageReference(&self, advisorypagedimensions: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePageReference)(::windows::core::Vtable::as_raw(self), advisorypagedimensions, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePage<P0, P1>(&self, pagedimensions: *const XPS_SIZE, language: P0, parturi: P1) -> ::windows::core::Result<IXpsOMPage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePage)(::windows::core::Vtable::as_raw(self), pagedimensions, language.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePageFromStream<P0, P1, P2, P3>(&self, pagemarkupstream: P0, parturi: P1, resources: P2, reuseobjects: P3) -> ::windows::core::Result<IXpsOMPage>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMPartResources>>,
        P3: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePageFromStream)(::windows::core::Vtable::as_raw(self), pagemarkupstream.into().abi(), parturi.into().abi(), resources.into().abi(), reuseobjects.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCanvas(&self) -> ::windows::core::Result<IXpsOMCanvas> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCanvas)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateGlyphs<P0>(&self, fontresource: P0) -> ::windows::core::Result<IXpsOMGlyphs>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMFontResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGlyphs)(::windows::core::Vtable::as_raw(self), fontresource.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePath(&self) -> ::windows::core::Result<IXpsOMPath> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePath)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateGeometryFigure(&self, startpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMGeometryFigure> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGeometryFigure)(::windows::core::Vtable::as_raw(self), startpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateMatrixTransform(&self, matrix: *const XPS_MATRIX) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateMatrixTransform)(::windows::core::Vtable::as_raw(self), matrix, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateSolidColorBrush<P0>(&self, color: *const XPS_COLOR, colorprofile: P0) -> ::windows::core::Result<IXpsOMSolidColorBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSolidColorBrush)(::windows::core::Vtable::as_raw(self), color, colorprofile.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateColorProfileResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMColorProfileResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateColorProfileResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateImageBrush<P0>(&self, image: P0, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMImageBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateImageBrush)(::windows::core::Vtable::as_raw(self), image.into().abi(), viewbox, viewport, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateVisualBrush(&self, viewbox: *const XPS_RECT, viewport: *const XPS_RECT) -> ::windows::core::Result<IXpsOMVisualBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateVisualBrush)(::windows::core::Vtable::as_raw(self), viewbox, viewport, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateImageResource<P0, P1>(&self, acquiredstream: P0, contenttype: XPS_IMAGE_TYPE, parturi: P1) -> ::windows::core::Result<IXpsOMImageResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateImageResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), contenttype, parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePrintTicketResource<P0, P1>(&self, acquiredstream: P0, parturi: P1) -> ::windows::core::Result<IXpsOMPrintTicketResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePrintTicketResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateFontResource<P0, P1, P2>(&self, acquiredstream: P0, fontembedding: XPS_FONT_EMBEDDING, parturi: P1, isobfsourcestream: P2) -> ::windows::core::Result<IXpsOMFontResource>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P2: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFontResource)(::windows::core::Vtable::as_raw(self), acquiredstream.into().abi(), fontembedding, parturi.into().abi(), isobfsourcestream.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateGradientStop<P0>(&self, color: *const XPS_COLOR, colorprofile: P0, offset: f32) -> ::windows::core::Result<IXpsOMGradientStop>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMColorProfileResource>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGradientStop)(::windows::core::Vtable::as_raw(self), color, colorprofile.into().abi(), offset, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateLinearGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, startpoint: *const XPS_POINT, endpoint: *const XPS_POINT) -> ::windows::core::Result<IXpsOMLinearGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLinearGradientBrush)(::windows::core::Vtable::as_raw(self), gradstop1.into().abi(), gradstop2.into().abi(), startpoint, endpoint, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateRadialGradientBrush<P0, P1>(&self, gradstop1: P0, gradstop2: P1, centerpoint: *const XPS_POINT, gradientorigin: *const XPS_POINT, radiisizes: *const XPS_SIZE) -> ::windows::core::Result<IXpsOMRadialGradientBrush>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMGradientStop>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRadialGradientBrush)(::windows::core::Vtable::as_raw(self), gradstop1.into().abi(), gradstop2.into().abi(), centerpoint, gradientorigin, radiisizes, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreateCoreProperties<P0>(&self, parturi: P0) -> ::windows::core::Result<IXpsOMCoreProperties>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCoreProperties)(::windows::core::Vtable::as_raw(self), parturi.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDictionary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreatePartUriCollection(&self) -> ::windows::core::Result<IXpsOMPartUriCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePartUriCollection)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnFile<P0, P1, P2, P3, P4, P5, P6>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> ::windows::core::Result<IXpsOMPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMCoreProperties>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
        P5: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P6: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePackageWriterOnFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into(), interleaving, documentsequencepartname.into().abi(), coreproperties.into().abi(), packagethumbnail.into().abi(), documentsequenceprintticket.into().abi(), discardcontrolpartname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriterOnStream<P0, P1, P2, P3, P4, P5, P6>(&self, outputstream: P0, optimizemarkupsize: P1, interleaving: XPS_INTERLEAVING, documentsequencepartname: P2, coreproperties: P3, packagethumbnail: P4, documentsequenceprintticket: P5, discardcontrolpartname: P6) -> ::windows::core::Result<IXpsOMPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
        P2: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMCoreProperties>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
        P5: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P6: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePackageWriterOnStream)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), optimizemarkupsize.into(), interleaving, documentsequencepartname.into().abi(), coreproperties.into().abi(), packagethumbnail.into().abi(), documentsequenceprintticket.into().abi(), discardcontrolpartname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePartUri<P0>(&self, uri: P0) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePartUri)(::windows::core::Vtable::as_raw(self), uri.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateReadOnlyStreamOnFile<P0>(&self, filename: P0) -> ::windows::core::Result<super::super::System::Com::IStream>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateReadOnlyStreamOnFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackage {}
impl ::core::fmt::Debug for IXpsOMPackage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackage").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackage1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackage1 {}
impl ::core::fmt::Debug for IXpsOMPackage1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackage1").field(&self.0).finish()
    }
}
impl IXpsOMPackage1 {
    pub unsafe fn GetDocumentSequence(&self) -> ::windows::core::Result<IXpsOMDocumentSequence> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDocumentSequence)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDocumentSequence<P0>(&self, documentsequence: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDocumentSequence>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDocumentSequence)(::windows::core::Vtable::as_raw(self), documentsequence.into().abi()).ok()
    }
    pub unsafe fn GetCoreProperties(&self) -> ::windows::core::Result<IXpsOMCoreProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCoreProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetCoreProperties<P0>(&self, coreproperties: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMCoreProperties>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetCoreProperties)(::windows::core::Vtable::as_raw(self), coreproperties.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetDiscardControlPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDiscardControlPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetDiscardControlPartName<P0>(&self, discardcontrolparturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDiscardControlPartName)(::windows::core::Vtable::as_raw(self), discardcontrolparturi.into().abi()).ok()
    }
    pub unsafe fn GetThumbnailResource(&self) -> ::windows::core::Result<IXpsOMImageResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetThumbnailResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetThumbnailResource<P0>(&self, imageresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetThumbnailResource)(::windows::core::Vtable::as_raw(self), imageresource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn WriteToFile<P0, P1>(&self, filename: P0, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flagsandattributes: u32, optimizemarkupsize: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.WriteToFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), securityattributes, flagsandattributes, optimizemarkupsize.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn WriteToStream<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.WriteToStream)(::windows::core::Vtable::as_raw(self), stream.into().abi(), optimizemarkupsize.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackageTarget {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackageTarget {}
impl ::core::fmt::Debug for IXpsOMPackageTarget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackageTarget").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackageWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackageWriter {}
impl ::core::fmt::Debug for IXpsOMPackageWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackageWriter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPackageWriter3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPackageWriter3D {}
impl ::core::fmt::Debug for IXpsOMPackageWriter3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPackageWriter3D").field(&self.0).finish()
    }
}
impl IXpsOMPackageWriter3D {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn StartNewDocument<P0, P1, P2, P3, P4>(&self, documentpartname: P0, documentprintticket: P1, documentstructure: P2, signatureblockresources: P3, restrictedfonts: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMDocumentStructureResource>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMSignatureBlockResourceCollection>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMPartUriCollection>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StartNewDocument)(::windows::core::Vtable::as_raw(self), documentpartname.into().abi(), documentprintticket.into().abi(), documentstructure.into().abi(), signatureblockresources.into().abi(), restrictedfonts.into().abi()).ok()
    }
    pub unsafe fn AddPage<P0, P1, P2, P3, P4>(&self, page: P0, advisorypagedimensions: *const XPS_SIZE, discardableresourceparts: P1, storyfragments: P2, pageprintticket: P3, pagethumbnail: P4) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMPage>>,
        P1: ::std::convert::Into<::windows::core::InParam<IXpsOMPartUriCollection>>,
        P2: ::std::convert::Into<::windows::core::InParam<IXpsOMStoryFragmentsResource>>,
        P3: ::std::convert::Into<::windows::core::InParam<IXpsOMPrintTicketResource>>,
        P4: ::std::convert::Into<::windows::core::InParam<IXpsOMImageResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddPage)(::windows::core::Vtable::as_raw(self), page.into().abi(), advisorypagedimensions, discardableresourceparts.into().abi(), storyfragments.into().abi(), pageprintticket.into().abi(), pagethumbnail.into().abi()).ok()
    }
    pub unsafe fn AddResource<P0>(&self, resource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddResource)(::windows::core::Vtable::as_raw(self), resource.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsClosed(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IsClosed)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXpsOMPage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPage {}
impl ::core::fmt::Debug for IXpsOMPage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPage").field(&self.0).finish()
    }
}
impl IXpsOMPage {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPage1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPage1 {}
impl ::core::fmt::Debug for IXpsOMPage1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPage1").field(&self.0).finish()
    }
}
impl IXpsOMPage1 {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<IXpsOMPageReference> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVisuals(&self) -> ::windows::core::Result<IXpsOMVisualCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVisuals)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageDimensions(&self) -> ::windows::core::Result<XPS_SIZE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPageDimensions)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetPageDimensions(&self, pagedimensions: *const XPS_SIZE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPageDimensions)(::windows::core::Vtable::as_raw(self), pagedimensions).ok()
    }
    pub unsafe fn GetContentBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContentBox)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetContentBox(&self, contentbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetContentBox)(::windows::core::Vtable::as_raw(self), contentbox).ok()
    }
    pub unsafe fn GetBleedBox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBleedBox)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBleedBox(&self, bleedbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBleedBox)(::windows::core::Vtable::as_raw(self), bleedbox).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLanguage)(::windows::core::Vtable::as_raw(self), language.into().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlinktarget: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), ishyperlinktarget.into()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDictionary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDictionaryLocal(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDictionaryLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDictionaryLocal<P0>(&self, resourcedictionary: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDictionary>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDictionaryLocal)(::windows::core::Vtable::as_raw(self), resourcedictionary.into().abi()).ok()
    }
    pub unsafe fn GetDictionaryResource(&self) -> ::windows::core::Result<IXpsOMRemoteDictionaryResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDictionaryResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDictionaryResource<P0>(&self, remotedictionaryresource: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMRemoteDictionaryResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDictionaryResource)(::windows::core::Vtable::as_raw(self), remotedictionaryresource.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Write<P0, P1>(&self, stream: P0, optimizemarkupsize: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::ISequentialStream>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.Write)(::windows::core::Vtable::as_raw(self), stream.into().abi(), optimizemarkupsize.into()).ok()
    }
    pub unsafe fn GenerateUnusedLookupKey(&self, r#type: XPS_OBJECT_TYPE) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GenerateUnusedLookupKey)(::windows::core::Vtable::as_raw(self), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IXpsOMPage> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.Clone)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXpsOMPageReference {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPageReference {}
impl ::core::fmt::Debug for IXpsOMPageReference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPageReference").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPageReferenceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPageReferenceCollection {}
impl ::core::fmt::Debug for IXpsOMPageReferenceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPageReferenceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPart {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPart {}
impl ::core::fmt::Debug for IXpsOMPart {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPart").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPartResources {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPartResources {}
impl ::core::fmt::Debug for IXpsOMPartResources {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPartResources").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPartUriCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPartUriCollection {}
impl ::core::fmt::Debug for IXpsOMPartUriCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPartUriCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPath {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPath {}
impl ::core::fmt::Debug for IXpsOMPath {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPath").field(&self.0).finish()
    }
}
impl IXpsOMPath {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, matrixtransform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLocal)(::windows::core::Vtable::as_raw(self), matrixtransform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetClipGeometry(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometry)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetClipGeometryLocal(&self) -> ::windows::core::Result<IXpsOMGeometry> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometryLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLocal<P0>(&self, clipgeometry: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMGeometry>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetClipGeometryLocal)(::windows::core::Vtable::as_raw(self), clipgeometry.into().abi()).ok()
    }
    pub unsafe fn GetClipGeometryLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetClipGeometryLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetClipGeometryLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetClipGeometryLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetOpacityMaskBrush(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrush)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacityMaskBrushLocal(&self) -> ::windows::core::Result<IXpsOMBrush> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrushLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLocal<P0>(&self, opacitymaskbrush: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMBrush>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOpacityMaskBrushLocal)(::windows::core::Vtable::as_raw(self), opacitymaskbrush.into().abi()).ok()
    }
    pub unsafe fn GetOpacityMaskBrushLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacityMaskBrushLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacityMaskBrushLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetOpacityMaskBrushLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsHyperlinkTarget(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetIsHyperlinkTarget<P0>(&self, ishyperlink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetIsHyperlinkTarget)(::windows::core::Vtable::as_raw(self), ishyperlink.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHyperlinkNavigateUri(&self) -> ::windows::core::Result<super::super::System::Com::IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHyperlinkNavigateUri)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetHyperlinkNavigateUri<P0>(&self, hyperlinkuri: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::System::Com::IUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetHyperlinkNavigateUri)(::windows::core::Vtable::as_raw(self), hyperlinkuri.into().abi()).ok()
    }
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetLanguage<P0>(&self, language: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetLanguage)(::windows::core::Vtable::as_raw(self), language.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMPrintTicketResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMPrintTicketResource {}
impl ::core::fmt::Debug for IXpsOMPrintTicketResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMPrintTicketResource").field(&self.0).finish()
    }
}
impl IXpsOMPrintTicketResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMRadialGradientBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMRadialGradientBrush {}
impl ::core::fmt::Debug for IXpsOMRadialGradientBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMRadialGradientBrush").field(&self.0).finish()
    }
}
impl IXpsOMRadialGradientBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetGradientStops(&self) -> ::windows::core::Result<IXpsOMGradientStopCollection> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGradientStops)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLocal)(::windows::core::Vtable::as_raw(self), transform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetSpreadMethod(&self) -> ::windows::core::Result<XPS_SPREAD_METHOD> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSpreadMethod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSpreadMethod(&self, spreadmethod: XPS_SPREAD_METHOD) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSpreadMethod)(::windows::core::Vtable::as_raw(self), spreadmethod).ok()
    }
    pub unsafe fn GetColorInterpolationMode(&self) -> ::windows::core::Result<XPS_COLOR_INTERPOLATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetColorInterpolationMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetColorInterpolationMode(&self, colorinterpolationmode: XPS_COLOR_INTERPOLATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetColorInterpolationMode)(::windows::core::Vtable::as_raw(self), colorinterpolationmode).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMRemoteDictionaryResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMRemoteDictionaryResource {}
impl ::core::fmt::Debug for IXpsOMRemoteDictionaryResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMRemoteDictionaryResource").field(&self.0).finish()
    }
}
impl IXpsOMRemoteDictionaryResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMRemoteDictionaryResource1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMRemoteDictionaryResource1 {}
impl ::core::fmt::Debug for IXpsOMRemoteDictionaryResource1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMRemoteDictionaryResource1").field(&self.0).finish()
    }
}
impl IXpsOMRemoteDictionaryResource1 {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
    pub unsafe fn GetDictionary(&self) -> ::windows::core::Result<IXpsOMDictionary> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDictionary)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetDictionary<P0>(&self, dictionary: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMDictionary>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDictionary)(::windows::core::Vtable::as_raw(self), dictionary.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMRemoteDictionaryResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMRemoteDictionaryResourceCollection {}
impl ::core::fmt::Debug for IXpsOMRemoteDictionaryResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMRemoteDictionaryResourceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMResource {}
impl ::core::fmt::Debug for IXpsOMResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMResource").field(&self.0).finish()
    }
}
impl IXpsOMResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMShareable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMShareable {}
impl ::core::fmt::Debug for IXpsOMShareable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMShareable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMSignatureBlockResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMSignatureBlockResource {}
impl ::core::fmt::Debug for IXpsOMSignatureBlockResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMSignatureBlockResource").field(&self.0).finish()
    }
}
impl IXpsOMSignatureBlockResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMSignatureBlockResourceCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMSignatureBlockResourceCollection {}
impl ::core::fmt::Debug for IXpsOMSignatureBlockResourceCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMSignatureBlockResourceCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMSolidColorBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMSolidColorBrush {}
impl ::core::fmt::Debug for IXpsOMSolidColorBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMSolidColorBrush").field(&self.0).finish()
    }
}
impl IXpsOMSolidColorBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMStoryFragmentsResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMStoryFragmentsResource {}
impl ::core::fmt::Debug for IXpsOMStoryFragmentsResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMStoryFragmentsResource").field(&self.0).finish()
    }
}
impl IXpsOMStoryFragmentsResource {
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn GetPartName(&self) -> ::windows::core::Result<super::Packaging::Opc::IOpcPartUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPartName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Storage_Packaging_Opc\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Storage_Packaging_Opc", feature = "Win32_System_Com"))]
    pub unsafe fn SetPartName<P0>(&self, parturi: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::Packaging::Opc::IOpcPartUri>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPartName)(::windows::core::Vtable::as_raw(self), parturi.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMThumbnailGenerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMThumbnailGenerator {}
impl ::core::fmt::Debug for IXpsOMThumbnailGenerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMThumbnailGenerator").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsOMTileBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMTileBrush {}
impl ::core::fmt::Debug for IXpsOMTileBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMTileBrush").field(&self.0).finish()
    }
}
impl IXpsOMTileBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMVisual {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMVisual {}
impl ::core::fmt::Debug for IXpsOMVisual {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMVisual").field(&self.0).finish()
    }
}
impl IXpsOMVisual {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IXpsOMVisualBrush {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMVisualBrush {}
impl ::core::fmt::Debug for IXpsOMVisualBrush {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMVisualBrush").field(&self.0).finish()
    }
}
impl IXpsOMVisualBrush {
    pub unsafe fn GetOwner(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetOwner)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetType(&self) -> ::windows::core::Result<XPS_OBJECT_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOpacity(&self) -> ::windows::core::Result<f32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetOpacity)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetOpacity(&self, opacity: f32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetOpacity)(::windows::core::Vtable::as_raw(self), opacity).ok()
    }
    pub unsafe fn GetTransform(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransform)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTransformLocal(&self) -> ::windows::core::Result<IXpsOMMatrixTransform> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLocal)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLocal<P0>(&self, transform: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IXpsOMMatrixTransform>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLocal)(::windows::core::Vtable::as_raw(self), transform.into().abi()).ok()
    }
    pub unsafe fn GetTransformLookup(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTransformLookup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTransformLookup<P0>(&self, key: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetTransformLookup)(::windows::core::Vtable::as_raw(self), key.into().abi()).ok()
    }
    pub unsafe fn GetViewbox(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetViewbox)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetViewbox(&self, viewbox: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetViewbox)(::windows::core::Vtable::as_raw(self), viewbox).ok()
    }
    pub unsafe fn GetViewport(&self) -> ::windows::core::Result<XPS_RECT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetViewport)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetViewport(&self, viewport: *const XPS_RECT) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetViewport)(::windows::core::Vtable::as_raw(self), viewport).ok()
    }
    pub unsafe fn GetTileMode(&self) -> ::windows::core::Result<XPS_TILE_MODE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTileMode)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetTileMode(&self, tilemode: XPS_TILE_MODE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetTileMode)(::windows::core::Vtable::as_raw(self), tilemode).ok()
    }
}
impl ::core::cmp::PartialEq for IXpsOMVisualCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsOMVisualCollection {}
impl ::core::fmt::Debug for IXpsOMVisualCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsOMVisualCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignature {}
impl ::core::fmt::Debug for IXpsSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignature").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureBlock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureBlock {}
impl ::core::fmt::Debug for IXpsSignatureBlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureBlock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureBlockCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureBlockCollection {}
impl ::core::fmt::Debug for IXpsSignatureBlockCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureBlockCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureCollection {}
impl ::core::fmt::Debug for IXpsSignatureCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureManager {}
impl ::core::fmt::Debug for IXpsSignatureManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureManager").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureRequest {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureRequest {}
impl ::core::fmt::Debug for IXpsSignatureRequest {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureRequest").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsSignatureRequestCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSignatureRequestCollection {}
impl ::core::fmt::Debug for IXpsSignatureRequestCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSignatureRequestCollection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IXpsSigningOptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IXpsSigningOptions {}
impl ::core::fmt::Debug for IXpsSigningOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IXpsSigningOptions").field(&self.0).finish()
    }
}
impl ::core::default::Default for PRINT_WINDOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PRINT_WINDOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PRINT_WINDOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for PSFEATURE_CUSTPAPER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSFEATURE_CUSTPAPER {
    fn eq(&self, other: &Self) -> bool {
        self.lOrientation == other.lOrientation && self.lWidth == other.lWidth && self.lHeight == other.lHeight && self.lWidthOffset == other.lWidthOffset && self.lHeightOffset == other.lHeightOffset
    }
}
impl ::core::cmp::Eq for PSFEATURE_CUSTPAPER {}
impl ::core::fmt::Debug for PSFEATURE_CUSTPAPER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSFEATURE_CUSTPAPER").field("lOrientation", &self.lOrientation).field("lWidth", &self.lWidth).field("lHeight", &self.lHeight).field("lWidthOffset", &self.lWidthOffset).field("lHeightOffset", &self.lHeightOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PSFEATURE_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PSFEATURE_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.bPageIndependent == other.bPageIndependent && self.bSetPageDevice == other.bSetPageDevice
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PSFEATURE_OUTPUT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PSFEATURE_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSFEATURE_OUTPUT").field("bPageIndependent", &self.bPageIndependent).field("bSetPageDevice", &self.bSetPageDevice).finish()
    }
}
impl ::core::default::Default for PSINJECTDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for PSINJECTDATA {
    fn eq(&self, other: &Self) -> bool {
        self.DataBytes == other.DataBytes && self.InjectionPoint == other.InjectionPoint && self.PageNumber == other.PageNumber
    }
}
impl ::core::cmp::Eq for PSINJECTDATA {}
impl ::core::fmt::Debug for PSINJECTDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PSINJECTDATA").field("DataBytes", &self.DataBytes).field("InjectionPoint", &self.InjectionPoint).field("PageNumber", &self.PageNumber).finish()
    }
}
impl ::core::default::Default for PSINJECT_POINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for PSINJECT_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PSINJECT_POINT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PSINJECT_POINT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PSINJECT_POINT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PSINJECT_POINT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PSINJECT_POINT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PSINJECT_POINT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for XPS_COLOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for XPS_COLOR_INTERPOLATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_COLOR_INTERPOLATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_COLOR_INTERPOLATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_COLOR_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_COLOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_COLOR_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_DASH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_DASH {
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.gap == other.gap
    }
}
impl ::core::cmp::Eq for XPS_DASH {}
impl ::core::fmt::Debug for XPS_DASH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_DASH").field("length", &self.length).field("gap", &self.gap).finish()
    }
}
impl ::core::default::Default for XPS_DASH_CAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_DASH_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_DASH_CAP").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_DOCUMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_DOCUMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_DOCUMENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_FILL_RULE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_FILL_RULE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_FILL_RULE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_FONT_EMBEDDING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_FONT_EMBEDDING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_FONT_EMBEDDING").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_GLYPH_INDEX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_GLYPH_INDEX {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index && self.advanceWidth == other.advanceWidth && self.horizontalOffset == other.horizontalOffset && self.verticalOffset == other.verticalOffset
    }
}
impl ::core::cmp::Eq for XPS_GLYPH_INDEX {}
impl ::core::fmt::Debug for XPS_GLYPH_INDEX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_GLYPH_INDEX").field("index", &self.index).field("advanceWidth", &self.advanceWidth).field("horizontalOffset", &self.horizontalOffset).field("verticalOffset", &self.verticalOffset).finish()
    }
}
impl ::core::default::Default for XPS_GLYPH_MAPPING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_GLYPH_MAPPING {
    fn eq(&self, other: &Self) -> bool {
        self.unicodeStringStart == other.unicodeStringStart && self.unicodeStringLength == other.unicodeStringLength && self.glyphIndicesStart == other.glyphIndicesStart && self.glyphIndicesLength == other.glyphIndicesLength
    }
}
impl ::core::cmp::Eq for XPS_GLYPH_MAPPING {}
impl ::core::fmt::Debug for XPS_GLYPH_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_GLYPH_MAPPING").field("unicodeStringStart", &self.unicodeStringStart).field("unicodeStringLength", &self.unicodeStringLength).field("glyphIndicesStart", &self.glyphIndicesStart).field("glyphIndicesLength", &self.glyphIndicesLength).finish()
    }
}
impl ::core::default::Default for XPS_IMAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_IMAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_IMAGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_INTERLEAVING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_INTERLEAVING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_INTERLEAVING").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_LINE_CAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_LINE_CAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_LINE_CAP").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_LINE_JOIN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_LINE_JOIN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_LINE_JOIN").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_MATRIX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_MATRIX {
    fn eq(&self, other: &Self) -> bool {
        self.m11 == other.m11 && self.m12 == other.m12 && self.m21 == other.m21 && self.m22 == other.m22 && self.m31 == other.m31 && self.m32 == other.m32
    }
}
impl ::core::cmp::Eq for XPS_MATRIX {}
impl ::core::fmt::Debug for XPS_MATRIX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_MATRIX").field("m11", &self.m11).field("m12", &self.m12).field("m21", &self.m21).field("m22", &self.m22).field("m31", &self.m31).field("m32", &self.m32).finish()
    }
}
impl ::core::default::Default for XPS_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_POINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_POINT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for XPS_POINT {}
impl ::core::fmt::Debug for XPS_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_POINT").field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::core::default::Default for XPS_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for XPS_RECT {}
impl ::core::fmt::Debug for XPS_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_RECT").field("x", &self.x).field("y", &self.y).field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::default::Default for XPS_SEGMENT_STROKE_PATTERN {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_SEGMENT_STROKE_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SEGMENT_STROKE_PATTERN").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_SEGMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_SEGMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SEGMENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_SIGNATURE_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_SIGNATURE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGNATURE_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_SIGN_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_SIGN_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGN_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_SIGN_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_SIGN_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SIGN_POLICY").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for XPS_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}
impl ::core::cmp::Eq for XPS_SIZE {}
impl ::core::fmt::Debug for XPS_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XPS_SIZE").field("width", &self.width).field("height", &self.height).finish()
    }
}
impl ::core::default::Default for XPS_SPREAD_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_SPREAD_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_SPREAD_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_STYLE_SIMULATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_STYLE_SIMULATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_STYLE_SIMULATION").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_THUMBNAIL_SIZE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_THUMBNAIL_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_THUMBNAIL_SIZE").field(&self.0).finish()
    }
}
impl ::core::default::Default for XPS_TILE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for XPS_TILE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XPS_TILE_MODE").field(&self.0).finish()
    }
}
