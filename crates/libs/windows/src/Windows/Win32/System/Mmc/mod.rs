#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct AppEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl AppEvents {}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(AppEvents, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for AppEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for AppEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for AppEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for AppEvents {
    type Vtable = AppEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for AppEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for AppEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfc7a4252_78ac_4532_8c5a_563cfe138863);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct AppEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Column(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Column {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Width(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Width)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetWidth(&self, width: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWidth)(::windows::core::Interface::as_raw(self), width).ok()
    }
    pub unsafe fn DisplayPosition(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).DisplayPosition)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetDisplayPosition(&self, index: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDisplayPosition)(::windows::core::Interface::as_raw(self), index).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Hidden(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Hidden)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetHidden<P0>(&self, hidden: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetHidden)(::windows::core::Interface::as_raw(self), hidden.into_param().abi()).ok()
    }
    pub unsafe fn SetAsSortColumn(&self, sortorder: _ColumnSortOrder) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAsSortColumn)(::windows::core::Interface::as_raw(self), sortorder).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSortColumn(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsSortColumn)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Column, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Column {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Column {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Column").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Column {
    type Vtable = Column_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Column {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Column {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfd1c5f63_2b16_4d06_9ab3_f45350b940ab);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Column_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Width: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: *mut i32) -> ::windows::core::HRESULT,
    pub SetWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, width: i32) -> ::windows::core::HRESULT,
    pub DisplayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayposition: *mut i32) -> ::windows::core::HRESULT,
    pub SetDisplayPosition: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Hidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hidden: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Hidden: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetHidden: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hidden: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetHidden: usize,
    pub SetAsSortColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sortorder: _ColumnSortOrder) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSortColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issortcolumn: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSortColumn: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Columns(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Columns {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<Column> {
        let mut result__ = ::windows::core::zeroed::<Column>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Columns, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Columns {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Columns {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Columns {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Columns").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Columns {
    type Vtable = Columns_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Columns {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Columns {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x383d4d97_fc44_478b_b139_6323dc48611c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Columns_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, column: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ContextMenu(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ContextMenu {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_Item(&self, indexorpath: super::Com::VARIANT) -> ::windows::core::Result<MenuItem> {
        let mut result__ = ::windows::core::zeroed::<MenuItem>();
        (::windows::core::Interface::vtable(self).get_Item)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(indexorpath), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ContextMenu, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ContextMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ContextMenu {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ContextMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ContextMenu").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ContextMenu {
    type Vtable = ContextMenu_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ContextMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ContextMenu {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdab39ce0_25e6_4e07_8362_ba9c95706545);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ContextMenu_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, indexorpath: super::Com::VARIANT, menuitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Document(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Document {
    pub unsafe fn Save(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Save)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SaveAs<P0>(&self, filename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SaveAs)(::windows::core::Interface::as_raw(self), filename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Close<P0>(&self, savechanges: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self), savechanges.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Views(&self) -> ::windows::core::Result<Views> {
        let mut result__ = ::windows::core::zeroed::<Views>();
        (::windows::core::Interface::vtable(self).Views)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SnapIns(&self) -> ::windows::core::Result<SnapIns> {
        let mut result__ = ::windows::core::zeroed::<SnapIns>();
        (::windows::core::Interface::vtable(self).SnapIns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActiveView(&self) -> ::windows::core::Result<View> {
        let mut result__ = ::windows::core::zeroed::<View>();
        (::windows::core::Interface::vtable(self).ActiveView)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetName)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
    pub unsafe fn Location(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Location)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsSaved(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsSaved)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Mode(&self) -> ::windows::core::Result<_DocumentMode> {
        let mut result__ = ::windows::core::zeroed::<_DocumentMode>();
        (::windows::core::Interface::vtable(self).Mode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetMode(&self, mode: _DocumentMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn RootNode(&self) -> ::windows::core::Result<Node> {
        let mut result__ = ::windows::core::zeroed::<Node>();
        (::windows::core::Interface::vtable(self).RootNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ScopeNamespace(&self) -> ::windows::core::Result<ScopeNamespace> {
        let mut result__ = ::windows::core::zeroed::<ScopeNamespace>();
        (::windows::core::Interface::vtable(self).ScopeNamespace)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateProperties(&self) -> ::windows::core::Result<Properties> {
        let mut result__ = ::windows::core::zeroed::<Properties>();
        (::windows::core::Interface::vtable(self).CreateProperties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Application(&self) -> ::windows::core::Result<_Application> {
        let mut result__ = ::windows::core::zeroed::<_Application>();
        (::windows::core::Interface::vtable(self).Application)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Document, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Document {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Document {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Document").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Document {
    type Vtable = Document_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Document {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Document {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x225120d6_1e0f_40a3_93fe_1079e6a8017b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Document_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Save: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SaveAs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, savechanges: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Close: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Views: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, views: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Views: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SnapIns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapins: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SnapIns: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ActiveView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActiveView: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Location: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, location: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsSaved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, issaved: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsSaved: usize,
    pub Mode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut _DocumentMode) -> ::windows::core::HRESULT,
    pub SetMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: _DocumentMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub RootNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    RootNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ScopeNamespace: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenamespace: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ScopeNamespace: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateProperties: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Application: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Application: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Extension(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Extension {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Vendor(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Vendor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Extensions(&self) -> ::windows::core::Result<Extensions> {
        let mut result__ = ::windows::core::zeroed::<Extensions>();
        (::windows::core::Interface::vtable(self).Extensions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SnapinCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).SnapinCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableAllExtensions<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).EnableAllExtensions)(::windows::core::Interface::as_raw(self), enable.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enable<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Enable)(::windows::core::Interface::as_raw(self), enable.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Extension, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Extension {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Extension {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Extension {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Extension").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Extension {
    type Vtable = Extension_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Extension {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Extension {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad4d6ca6_912f_409b_a26e_7fd234aef542);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Extension_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Vendor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendor: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Extensions: usize,
    pub SnapinCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapinclsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableAllExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableAllExtensions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enable: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Extensions(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Extensions {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<Extension> {
        let mut result__ = ::windows::core::zeroed::<Extension>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Extensions, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Extensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Extensions {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Extensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Extensions").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Extensions {
    type Vtable = Extensions_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Extensions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Extensions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82dbea43_8ca4_44bc_a2ca_d18741059ec8);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Extensions_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, extension: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Frame(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Frame {
    pub unsafe fn Maximize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Maximize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Minimize(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Minimize)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Top(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Top)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetTop(&self, top: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTop)(::windows::core::Interface::as_raw(self), top).ok()
    }
    pub unsafe fn Bottom(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Bottom)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetBottom(&self, bottom: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetBottom)(::windows::core::Interface::as_raw(self), bottom).ok()
    }
    pub unsafe fn Left(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Left)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetLeft(&self, left: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLeft)(::windows::core::Interface::as_raw(self), left).ok()
    }
    pub unsafe fn Right(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Right)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetRight(&self, right: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRight)(::windows::core::Interface::as_raw(self), right).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Frame, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Frame {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Frame {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Frame").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Frame {
    type Vtable = Frame_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Frame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Frame {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe5e2d970_5bb3_4306_8804_b0968a31c8e6);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Frame_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Maximize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Minimize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Top: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: *mut i32) -> ::windows::core::HRESULT,
    pub SetTop: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, top: i32) -> ::windows::core::HRESULT,
    pub Bottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: *mut i32) -> ::windows::core::HRESULT,
    pub SetBottom: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bottom: i32) -> ::windows::core::HRESULT,
    pub Left: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: *mut i32) -> ::windows::core::HRESULT,
    pub SetLeft: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, left: i32) -> ::windows::core::HRESULT,
    pub Right: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: *mut i32) -> ::windows::core::HRESULT,
    pub SetRight: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, right: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IColumnData(::windows::core::IUnknown);
impl IColumnData {
    pub unsafe fn SetColumnConfigData(&self, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColumnConfigData)(::windows::core::Interface::as_raw(self), pcolid, pcolsetdata).ok()
    }
    pub unsafe fn GetColumnConfigData(&self, pcolid: *const SColumnSetID) -> ::windows::core::Result<*mut MMC_COLUMN_SET_DATA> {
        let mut result__ = ::windows::core::zeroed::<*mut MMC_COLUMN_SET_DATA>();
        (::windows::core::Interface::vtable(self).GetColumnConfigData)(::windows::core::Interface::as_raw(self), pcolid, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetColumnSortData(&self, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColumnSortData)(::windows::core::Interface::as_raw(self), pcolid, pcolsortdata).ok()
    }
    pub unsafe fn GetColumnSortData(&self, pcolid: *const SColumnSetID) -> ::windows::core::Result<*mut MMC_SORT_SET_DATA> {
        let mut result__ = ::windows::core::zeroed::<*mut MMC_SORT_SET_DATA>();
        (::windows::core::Interface::vtable(self).GetColumnSortData)(::windows::core::Interface::as_raw(self), pcolid, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IColumnData, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IColumnData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IColumnData {}
impl ::core::fmt::Debug for IColumnData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IColumnData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IColumnData {
    type Vtable = IColumnData_Vtbl;
}
impl ::core::clone::Clone for IColumnData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IColumnData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x547c1354_024d_11d3_a707_00c04f8ef4cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IColumnData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetColumnConfigData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsetdata: *const MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT,
    pub GetColumnConfigData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsetdata: *mut *mut MMC_COLUMN_SET_DATA) -> ::windows::core::HRESULT,
    pub SetColumnSortData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, pcolsortdata: *const MMC_SORT_SET_DATA) -> ::windows::core::HRESULT,
    pub GetColumnSortData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolid: *const SColumnSetID, ppcolsortdata: *mut *mut MMC_SORT_SET_DATA) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IComponent(::windows::core::IUnknown);
impl IComponent {
    pub unsafe fn Initialize<P0>(&self, lpconsole: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IConsole>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), lpconsole.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<P0, P1, P2>(&self, lpdataobject: P0, event: MMC_NOTIFY_TYPE, arg: P1, param3: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).Notify)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi(), event, arg.into_param().abi(), param3.into_param().abi()).ok()
    }
    pub unsafe fn Destroy(&self, cookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Destroy)(::windows::core::Interface::as_raw(self), cookie).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).QueryDataObject)(::windows::core::Interface::as_raw(self), cookie, r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResultViewType(&self, cookie: isize, ppviewtype: *mut ::windows::core::PWSTR, pviewoptions: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResultViewType)(::windows::core::Interface::as_raw(self), cookie, ppviewtype, pviewoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(&self, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayInfo)(::windows::core::Interface::as_raw(self), presultdataitem).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<P0, P1>(&self, lpdataobjecta: P0, lpdataobjectb: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).CompareObjects)(::windows::core::Interface::as_raw(self), lpdataobjecta.into_param().abi(), lpdataobjectb.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IComponent, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IComponent {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponent {}
impl ::core::fmt::Debug for IComponent {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponent").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponent {
    type Vtable = IComponent_Vtbl;
}
impl ::core::clone::Clone for IComponent {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IComponent {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb2_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponent_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpconsole: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Notify: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDataObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDataObject: usize,
    pub GetResultViewType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, ppviewtype: *mut ::windows::core::PWSTR, pviewoptions: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobjecta: *mut ::core::ffi::c_void, lpdataobjectb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareObjects: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IComponent2(::windows::core::IUnknown);
impl IComponent2 {
    pub unsafe fn Initialize<P0>(&self, lpconsole: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IConsole>,
    {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), lpconsole.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<P0, P1, P2>(&self, lpdataobject: P0, event: MMC_NOTIFY_TYPE, arg: P1, param3: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.Notify)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi(), event, arg.into_param().abi(), param3.into_param().abi()).ok()
    }
    pub unsafe fn Destroy(&self, cookie: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Destroy)(::windows::core::Interface::as_raw(self), cookie).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).base__.QueryDataObject)(::windows::core::Interface::as_raw(self), cookie, r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResultViewType(&self, cookie: isize, ppviewtype: *mut ::windows::core::PWSTR, pviewoptions: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetResultViewType)(::windows::core::Interface::as_raw(self), cookie, ppviewtype, pviewoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(&self, presultdataitem: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDisplayInfo)(::windows::core::Interface::as_raw(self), presultdataitem).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<P0, P1>(&self, lpdataobjecta: P0, lpdataobjectb: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).base__.CompareObjects)(::windows::core::Interface::as_raw(self), lpdataobjecta.into_param().abi(), lpdataobjectb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).QueryDispatch)(::windows::core::Interface::as_raw(self), cookie, r#type, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetResultViewType2(&self, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResultViewType2)(::windows::core::Interface::as_raw(self), cookie, presultviewtype).ok()
    }
    pub unsafe fn RestoreResultView(&self, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RestoreResultView)(::windows::core::Interface::as_raw(self), cookie, presultviewtype).ok()
    }
}
::windows::imp::interface_hierarchy!(IComponent2, ::windows::core::IUnknown, IComponent);
impl ::core::cmp::PartialEq for IComponent2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponent2 {}
impl ::core::fmt::Debug for IComponent2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponent2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponent2 {
    type Vtable = IComponent2_Vtbl;
}
impl ::core::clone::Clone for IComponent2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IComponent2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x79a2d615_4a10_4ed4_8c65_8633f9335095);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponent2_Vtbl {
    pub base__: IComponent_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatch: usize,
    pub GetResultViewType2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *mut RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT,
    pub RestoreResultView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, presultviewtype: *const RESULT_VIEW_TYPE_INFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IComponentData(::windows::core::IUnknown);
impl IComponentData {
    pub unsafe fn Initialize<P0>(&self, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), punknown.into_param().abi()).ok()
    }
    pub unsafe fn CreateComponent(&self) -> ::windows::core::Result<IComponent> {
        let mut result__ = ::windows::core::zeroed::<IComponent>();
        (::windows::core::Interface::vtable(self).CreateComponent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<P0, P1, P2>(&self, lpdataobject: P0, event: MMC_NOTIFY_TYPE, arg: P1, param3: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).Notify)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi(), event, arg.into_param().abi(), param3.into_param().abi()).ok()
    }
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Destroy)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).QueryDataObject)(::windows::core::Interface::as_raw(self), cookie, r#type, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(&self, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDisplayInfo)(::windows::core::Interface::as_raw(self), pscopedataitem).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<P0, P1>(&self, lpdataobjecta: P0, lpdataobjectb: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).CompareObjects)(::windows::core::Interface::as_raw(self), lpdataobjecta.into_param().abi(), lpdataobjectb.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IComponentData, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IComponentData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentData {}
impl ::core::fmt::Debug for IComponentData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponentData {
    type Vtable = IComponentData_Vtbl;
}
impl ::core::clone::Clone for IComponentData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IComponentData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x955ab28a_5218_11d0_a985_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CreateComponent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcomponent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Notify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param3: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Notify: usize,
    pub Destroy: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDataObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdataobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDataObject: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetDisplayInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetDisplayInfo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CompareObjects: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobjecta: *mut ::core::ffi::c_void, lpdataobjectb: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CompareObjects: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IComponentData2(::windows::core::IUnknown);
impl IComponentData2 {
    pub unsafe fn Initialize<P0>(&self, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).base__.Initialize)(::windows::core::Interface::as_raw(self), punknown.into_param().abi()).ok()
    }
    pub unsafe fn CreateComponent(&self) -> ::windows::core::Result<IComponent> {
        let mut result__ = ::windows::core::zeroed::<IComponent>();
        (::windows::core::Interface::vtable(self).base__.CreateComponent)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Notify<P0, P1, P2>(&self, lpdataobject: P0, event: MMC_NOTIFY_TYPE, arg: P1, param3: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
        P2: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.Notify)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi(), event, arg.into_param().abi(), param3.into_param().abi()).ok()
    }
    pub unsafe fn Destroy(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Destroy)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDataObject(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDataObject> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDataObject>();
        (::windows::core::Interface::vtable(self).base__.QueryDataObject)(::windows::core::Interface::as_raw(self), cookie, r#type, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDisplayInfo(&self, pscopedataitem: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetDisplayInfo)(::windows::core::Interface::as_raw(self), pscopedataitem).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CompareObjects<P0, P1>(&self, lpdataobjecta: P0, lpdataobjectb: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).base__.CompareObjects)(::windows::core::Interface::as_raw(self), lpdataobjecta.into_param().abi(), lpdataobjectb.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryDispatch(&self, cookie: isize, r#type: DATA_OBJECT_TYPES) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).QueryDispatch)(::windows::core::Interface::as_raw(self), cookie, r#type, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IComponentData2, ::windows::core::IUnknown, IComponentData);
impl ::core::cmp::PartialEq for IComponentData2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IComponentData2 {}
impl ::core::fmt::Debug for IComponentData2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IComponentData2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IComponentData2 {
    type Vtable = IComponentData2_Vtbl;
}
impl ::core::clone::Clone for IComponentData2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IComponentData2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcca0f2d2_82de_41b5_bf47_3b2076273d5c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IComponentData2_Vtbl {
    pub base__: IComponentData_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryDispatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cookie: isize, r#type: DATA_OBJECT_TYPES, ppdispatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryDispatch: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IConsole(::windows::core::IUnknown);
impl IConsole {
    pub unsafe fn SetHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IHeaderCtrl>,
    {
        (::windows::core::Interface::vtable(self).SetHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn SetToolbar<P0>(&self, ptoolbar: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IToolbar>,
    {
        (::windows::core::Interface::vtable(self).SetToolbar)(::windows::core::Interface::as_raw(self), ptoolbar.into_param().abi()).ok()
    }
    pub unsafe fn QueryResultView(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).QueryResultView)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__ = ::windows::core::zeroed::<IImageList>();
        (::windows::core::Interface::vtable(self).QueryScopeImageList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryResultImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__ = ::windows::core::zeroed::<IImageList>();
        (::windows::core::Interface::vtable(self).QueryResultImageList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<P0, P1>(&self, lpdataobject: P0, data: P1, hint: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).UpdateAllViews)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi(), data.into_param().abi(), hint).ok()
    }
    pub unsafe fn MessageBox<P0, P1>(&self, lpsztext: P0, lpsztitle: P1, fustyle: u32) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).MessageBox)(::windows::core::Interface::as_raw(self), lpsztext.into_param().abi(), lpsztitle.into_param().abi(), fustyle, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::core::Result<IConsoleVerb> {
        let mut result__ = ::windows::core::zeroed::<IConsoleVerb>();
        (::windows::core::Interface::vtable(self).QueryConsoleVerb)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SelectScopeItem)(::windows::core::Interface::as_raw(self), hscopeitem).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).GetMainWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NewWindow)(::windows::core::Interface::as_raw(self), hscopeitem, loptions).ok()
    }
}
::windows::imp::interface_hierarchy!(IConsole, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IConsole {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsole {}
impl ::core::fmt::Debug for IConsole {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsole").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsole {
    type Vtable = IConsole_Vtbl;
}
impl ::core::clone::Clone for IConsole {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConsole {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb1_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetHeader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pheader: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetToolbar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ptoolbar: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryResultView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryScopeImageList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimagelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub QueryResultImageList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppimagelist: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub UpdateAllViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void, data: super::super::Foundation::LPARAM, hint: isize) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    UpdateAllViews: usize,
    pub MessageBox: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpsztext: ::windows::core::PCWSTR, lpsztitle: ::windows::core::PCWSTR, fustyle: u32, piretval: *mut i32) -> ::windows::core::HRESULT,
    pub QueryConsoleVerb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconsoleverb: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SelectScopeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetMainWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phwnd: *mut super::super::Foundation::HWND) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetMainWindow: usize,
    pub NewWindow: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize, loptions: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IConsole2(::windows::core::IUnknown);
impl IConsole2 {
    pub unsafe fn SetHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IHeaderCtrl>,
    {
        (::windows::core::Interface::vtable(self).base__.SetHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn SetToolbar<P0>(&self, ptoolbar: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IToolbar>,
    {
        (::windows::core::Interface::vtable(self).base__.SetToolbar)(::windows::core::Interface::as_raw(self), ptoolbar.into_param().abi()).ok()
    }
    pub unsafe fn QueryResultView(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.QueryResultView)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__ = ::windows::core::zeroed::<IImageList>();
        (::windows::core::Interface::vtable(self).base__.QueryScopeImageList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryResultImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__ = ::windows::core::zeroed::<IImageList>();
        (::windows::core::Interface::vtable(self).base__.QueryResultImageList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<P0, P1>(&self, lpdataobject: P0, data: P1, hint: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.UpdateAllViews)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi(), data.into_param().abi(), hint).ok()
    }
    pub unsafe fn MessageBox<P0, P1>(&self, lpsztext: P0, lpsztitle: P1, fustyle: u32) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.MessageBox)(::windows::core::Interface::as_raw(self), lpsztext.into_param().abi(), lpsztitle.into_param().abi(), fustyle, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::core::Result<IConsoleVerb> {
        let mut result__ = ::windows::core::zeroed::<IConsoleVerb>();
        (::windows::core::Interface::vtable(self).base__.QueryConsoleVerb)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SelectScopeItem)(::windows::core::Interface::as_raw(self), hscopeitem).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.GetMainWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.NewWindow)(::windows::core::Interface::as_raw(self), hscopeitem, loptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Expand<P0>(&self, hitem: isize, bexpand: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Expand)(::windows::core::Interface::as_raw(self), hitem, bexpand.into_param().abi()).ok()
    }
    pub unsafe fn IsTaskpadViewPreferred(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsTaskpadViewPreferred)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStatusText<P0>(&self, pszstatustext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStatusText)(::windows::core::Interface::as_raw(self), pszstatustext.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IConsole2, ::windows::core::IUnknown, IConsole);
impl ::core::cmp::PartialEq for IConsole2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsole2 {}
impl ::core::fmt::Debug for IConsole2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsole2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsole2 {
    type Vtable = IConsole2_Vtbl;
}
impl ::core::clone::Clone for IConsole2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConsole2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x103d842a_aa63_11d1_a7e1_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole2_Vtbl {
    pub base__: IConsole_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, bexpand: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Expand: usize,
    pub IsTaskpadViewPreferred: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStatusText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszstatustext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IConsole3(::windows::core::IUnknown);
impl IConsole3 {
    pub unsafe fn SetHeader<P0>(&self, pheader: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IHeaderCtrl>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetHeader)(::windows::core::Interface::as_raw(self), pheader.into_param().abi()).ok()
    }
    pub unsafe fn SetToolbar<P0>(&self, ptoolbar: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IToolbar>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.SetToolbar)(::windows::core::Interface::as_raw(self), ptoolbar.into_param().abi()).ok()
    }
    pub unsafe fn QueryResultView(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).base__.base__.QueryResultView)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryScopeImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__ = ::windows::core::zeroed::<IImageList>();
        (::windows::core::Interface::vtable(self).base__.base__.QueryScopeImageList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryResultImageList(&self) -> ::windows::core::Result<IImageList> {
        let mut result__ = ::windows::core::zeroed::<IImageList>();
        (::windows::core::Interface::vtable(self).base__.base__.QueryResultImageList)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdateAllViews<P0, P1>(&self, lpdataobject: P0, data: P1, hint: isize) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.UpdateAllViews)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi(), data.into_param().abi(), hint).ok()
    }
    pub unsafe fn MessageBox<P0, P1>(&self, lpsztext: P0, lpsztitle: P1, fustyle: u32) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.base__.MessageBox)(::windows::core::Interface::as_raw(self), lpsztext.into_param().abi(), lpsztitle.into_param().abi(), fustyle, &mut result__).from_abi(result__)
    }
    pub unsafe fn QueryConsoleVerb(&self) -> ::windows::core::Result<IConsoleVerb> {
        let mut result__ = ::windows::core::zeroed::<IConsoleVerb>();
        (::windows::core::Interface::vtable(self).base__.base__.QueryConsoleVerb)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SelectScopeItem(&self, hscopeitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SelectScopeItem)(::windows::core::Interface::as_raw(self), hscopeitem).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMainWindow(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::HWND>();
        (::windows::core::Interface::vtable(self).base__.base__.GetMainWindow)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn NewWindow(&self, hscopeitem: isize, loptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.NewWindow)(::windows::core::Interface::as_raw(self), hscopeitem, loptions).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Expand<P0>(&self, hitem: isize, bexpand: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Expand)(::windows::core::Interface::as_raw(self), hitem, bexpand.into_param().abi()).ok()
    }
    pub unsafe fn IsTaskpadViewPreferred(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsTaskpadViewPreferred)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStatusText<P0>(&self, pszstatustext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetStatusText)(::windows::core::Interface::as_raw(self), pszstatustext.into_param().abi()).ok()
    }
    pub unsafe fn RenameScopeItem(&self, hscopeitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RenameScopeItem)(::windows::core::Interface::as_raw(self), hscopeitem).ok()
    }
}
::windows::imp::interface_hierarchy!(IConsole3, ::windows::core::IUnknown, IConsole, IConsole2);
impl ::core::cmp::PartialEq for IConsole3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsole3 {}
impl ::core::fmt::Debug for IConsole3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsole3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsole3 {
    type Vtable = IConsole3_Vtbl;
}
impl ::core::clone::Clone for IConsole3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConsole3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f85efdb_d0e1_498c_8d4a_d010dfdd404f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsole3_Vtbl {
    pub base__: IConsole2_Vtbl,
    pub RenameScopeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hscopeitem: isize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IConsoleNameSpace(::windows::core::IUnknown);
impl IConsoleNameSpace {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn DeleteItem(&self, hitem: isize, fdeletethis: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteItem)(::windows::core::Interface::as_raw(self), hitem, fdeletethis).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn GetChildItem(&self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChildItem)(::windows::core::Interface::as_raw(self), item, pitemchild, pcookie).ok()
    }
    pub unsafe fn GetNextItem(&self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextItem)(::windows::core::Interface::as_raw(self), item, pitemnext, pcookie).ok()
    }
    pub unsafe fn GetParentItem(&self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetParentItem)(::windows::core::Interface::as_raw(self), item, pitemparent, pcookie).ok()
    }
}
::windows::imp::interface_hierarchy!(IConsoleNameSpace, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IConsoleNameSpace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsoleNameSpace {}
impl ::core::fmt::Debug for IConsoleNameSpace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsoleNameSpace").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsoleNameSpace {
    type Vtable = IConsoleNameSpace_Vtbl;
}
impl ::core::clone::Clone for IConsoleNameSpace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConsoleNameSpace {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbedeb620_f24d_11cf_8afc_00aa003ca9f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleNameSpace_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InsertItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InsertItem: usize,
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, fdeletethis: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut SCOPEDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItem: usize,
    pub GetChildItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
    pub GetNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
    pub GetParentItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IConsoleNameSpace2(::windows::core::IUnknown);
impl IConsoleNameSpace2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.InsertItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn DeleteItem(&self, hitem: isize, fdeletethis: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteItem)(::windows::core::Interface::as_raw(self), hitem, fdeletethis).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut SCOPEDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn GetChildItem(&self, item: isize, pitemchild: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetChildItem)(::windows::core::Interface::as_raw(self), item, pitemchild, pcookie).ok()
    }
    pub unsafe fn GetNextItem(&self, item: isize, pitemnext: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNextItem)(::windows::core::Interface::as_raw(self), item, pitemnext, pcookie).ok()
    }
    pub unsafe fn GetParentItem(&self, item: isize, pitemparent: *mut isize, pcookie: *mut isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetParentItem)(::windows::core::Interface::as_raw(self), item, pitemparent, pcookie).ok()
    }
    pub unsafe fn Expand(&self, hitem: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Expand)(::windows::core::Interface::as_raw(self), hitem).ok()
    }
    pub unsafe fn AddExtension(&self, hitem: isize, lpclsid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddExtension)(::windows::core::Interface::as_raw(self), hitem, lpclsid).ok()
    }
}
::windows::imp::interface_hierarchy!(IConsoleNameSpace2, ::windows::core::IUnknown, IConsoleNameSpace);
impl ::core::cmp::PartialEq for IConsoleNameSpace2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsoleNameSpace2 {}
impl ::core::fmt::Debug for IConsoleNameSpace2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsoleNameSpace2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsoleNameSpace2 {
    type Vtable = IConsoleNameSpace2_Vtbl;
}
impl ::core::clone::Clone for IConsoleNameSpace2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConsoleNameSpace2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x255f18cc_65db_11d1_a7dc_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleNameSpace2_Vtbl {
    pub base__: IConsoleNameSpace_Vtbl,
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize) -> ::windows::core::HRESULT,
    pub AddExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, lpclsid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IConsolePower(::windows::core::IUnknown);
impl IConsolePower {
    pub unsafe fn SetExecutionState(&self, dwadd: u32, dwremove: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetExecutionState)(::windows::core::Interface::as_raw(self), dwadd, dwremove).ok()
    }
    pub unsafe fn ResetIdleTimer(&self, dwflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetIdleTimer)(::windows::core::Interface::as_raw(self), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(IConsolePower, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IConsolePower {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsolePower {}
impl ::core::fmt::Debug for IConsolePower {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsolePower").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsolePower {
    type Vtable = IConsolePower_Vtbl;
}
impl ::core::clone::Clone for IConsolePower {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConsolePower {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1cfbdd0e_62ca_49ce_a3af_dbb2de61b068);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsolePower_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetExecutionState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwadd: u32, dwremove: u32) -> ::windows::core::HRESULT,
    pub ResetIdleTimer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IConsolePowerSink(::windows::core::IUnknown);
impl IConsolePowerSink {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OnPowerBroadcast<P0>(&self, nevent: u32, lparam: P0) -> ::windows::core::Result<super::super::Foundation::LRESULT>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::LRESULT>();
        (::windows::core::Interface::vtable(self).OnPowerBroadcast)(::windows::core::Interface::as_raw(self), nevent, lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IConsolePowerSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IConsolePowerSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsolePowerSink {}
impl ::core::fmt::Debug for IConsolePowerSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsolePowerSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsolePowerSink {
    type Vtable = IConsolePowerSink_Vtbl;
}
impl ::core::clone::Clone for IConsolePowerSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConsolePowerSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3333759f_fe4f_4975_b143_fec0a5dd6d65);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsolePowerSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub OnPowerBroadcast: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nevent: u32, lparam: super::super::Foundation::LPARAM, plreturn: *mut super::super::Foundation::LRESULT) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OnPowerBroadcast: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IConsoleVerb(::windows::core::IUnknown);
impl IConsoleVerb {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetVerbState(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetVerbState)(::windows::core::Interface::as_raw(self), ecmdid, nstate, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetVerbState<P0>(&self, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetVerbState)(::windows::core::Interface::as_raw(self), ecmdid, nstate, bstate.into_param().abi()).ok()
    }
    pub unsafe fn SetDefaultVerb(&self, ecmdid: MMC_CONSOLE_VERB) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDefaultVerb)(::windows::core::Interface::as_raw(self), ecmdid).ok()
    }
    pub unsafe fn GetDefaultVerb(&self) -> ::windows::core::Result<MMC_CONSOLE_VERB> {
        let mut result__ = ::windows::core::zeroed::<MMC_CONSOLE_VERB>();
        (::windows::core::Interface::vtable(self).GetDefaultVerb)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IConsoleVerb, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IConsoleVerb {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConsoleVerb {}
impl ::core::fmt::Debug for IConsoleVerb {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConsoleVerb").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConsoleVerb {
    type Vtable = IConsoleVerb_Vtbl;
}
impl ::core::clone::Clone for IConsoleVerb {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IConsoleVerb {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe49f7a60_74af_11d0_a286_00c04fd8fe93);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConsoleVerb_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetVerbState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetVerbState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetVerbState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetVerbState: usize,
    pub SetDefaultVerb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ecmdid: MMC_CONSOLE_VERB) -> ::windows::core::HRESULT,
    pub GetDefaultVerb: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pecmdid: *mut MMC_CONSOLE_VERB) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IContextMenuCallback(::windows::core::IUnknown);
impl IContextMenuCallback {
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddItem)(::windows::core::Interface::as_raw(self), pitem).ok()
    }
}
::windows::imp::interface_hierarchy!(IContextMenuCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IContextMenuCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenuCallback {}
impl ::core::fmt::Debug for IContextMenuCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenuCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextMenuCallback {
    type Vtable = IContextMenuCallback_Vtbl;
}
impl ::core::clone::Clone for IContextMenuCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContextMenuCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb7_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IContextMenuCallback2(::windows::core::IUnknown);
impl IContextMenuCallback2 {
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddItem)(::windows::core::Interface::as_raw(self), pitem).ok()
    }
}
::windows::imp::interface_hierarchy!(IContextMenuCallback2, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IContextMenuCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenuCallback2 {}
impl ::core::fmt::Debug for IContextMenuCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenuCallback2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextMenuCallback2 {
    type Vtable = IContextMenuCallback2_Vtbl;
}
impl ::core::clone::Clone for IContextMenuCallback2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContextMenuCallback2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe178bc0e_2ed0_4b5e_8097_42c9087e8b33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuCallback2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: *const CONTEXTMENUITEM2) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IContextMenuProvider(::windows::core::IUnknown);
impl IContextMenuProvider {
    pub unsafe fn AddItem(&self, pitem: *const CONTEXTMENUITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.AddItem)(::windows::core::Interface::as_raw(self), pitem).ok()
    }
    pub unsafe fn EmptyMenuList(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EmptyMenuList)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPrimaryExtensionItems<P0, P1>(&self, piextension: P0, pidataobject: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).AddPrimaryExtensionItems)(::windows::core::Interface::as_raw(self), piextension.into_param().abi(), pidataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddThirdPartyExtensionItems<P0>(&self, pidataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).AddThirdPartyExtensionItems)(::windows::core::Interface::as_raw(self), pidataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ShowContextMenu<P0>(&self, hwndparent: P0, xpos: i32, ypos: i32) -> ::windows::core::Result<i32>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::HWND>,
    {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).ShowContextMenu)(::windows::core::Interface::as_raw(self), hwndparent.into_param().abi(), xpos, ypos, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IContextMenuProvider, ::windows::core::IUnknown, IContextMenuCallback);
impl ::core::cmp::PartialEq for IContextMenuProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IContextMenuProvider {}
impl ::core::fmt::Debug for IContextMenuProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IContextMenuProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IContextMenuProvider {
    type Vtable = IContextMenuProvider_Vtbl;
}
impl ::core::clone::Clone for IContextMenuProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IContextMenuProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb6_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IContextMenuProvider_Vtbl {
    pub base__: IContextMenuCallback_Vtbl,
    pub EmptyMenuList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPrimaryExtensionItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piextension: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPrimaryExtensionItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddThirdPartyExtensionItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddThirdPartyExtensionItems: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ShowContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, xpos: i32, ypos: i32, plselected: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ShowContextMenu: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IControlbar(::windows::core::IUnknown);
impl IControlbar {
    pub unsafe fn Create<P0>(&self, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::windows::core::IntoParam<IExtendControlbar>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self).Create)(::windows::core::Interface::as_raw(self), ntype, pextendcontrolbar.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Attach<P0>(&self, ntype: MMC_CONTROL_TYPE, lpunknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Attach)(::windows::core::Interface::as_raw(self), ntype, lpunknown.into_param().abi()).ok()
    }
    pub unsafe fn Detach<P0>(&self, lpunknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
    {
        (::windows::core::Interface::vtable(self).Detach)(::windows::core::Interface::as_raw(self), lpunknown.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IControlbar, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IControlbar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IControlbar {}
impl ::core::fmt::Debug for IControlbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IControlbar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IControlbar {
    type Vtable = IControlbar_Vtbl;
}
impl ::core::clone::Clone for IControlbar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IControlbar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x69fb811e_6c1c_11d0_a2cb_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IControlbar_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Create: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, pextendcontrolbar: *mut ::core::ffi::c_void, ppunknown: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ntype: MMC_CONTROL_TYPE, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Detach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IDisplayHelp(::windows::core::IUnknown);
impl IDisplayHelp {
    pub unsafe fn ShowTopic<P0>(&self, pszhelptopic: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).ShowTopic)(::windows::core::Interface::as_raw(self), pszhelptopic.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IDisplayHelp, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDisplayHelp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDisplayHelp {}
impl ::core::fmt::Debug for IDisplayHelp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDisplayHelp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDisplayHelp {
    type Vtable = IDisplayHelp_Vtbl;
}
impl ::core::clone::Clone for IDisplayHelp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDisplayHelp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc593830_b926_11d1_8063_0000f875a9ce);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDisplayHelp_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ShowTopic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszhelptopic: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IEnumTASK(::windows::core::IUnknown);
impl IEnumTASK {
    pub unsafe fn Next(&self, rgelt: &mut [MMC_TASK], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumTASK> {
        let mut result__ = ::windows::core::zeroed::<IEnumTASK>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumTASK, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumTASK {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumTASK {}
impl ::core::fmt::Debug for IEnumTASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumTASK").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumTASK {
    type Vtable = IEnumTASK_Vtbl;
}
impl ::core::clone::Clone for IEnumTASK {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumTASK {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x338698b1_5a02_11d1_9fec_00600832db4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumTASK_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut MMC_TASK, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IExtendContextMenu(::windows::core::IUnknown);
impl IExtendContextMenu {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddMenuItems<P0, P1>(&self, pidataobject: P0, picallback: P1, pinsertionallowed: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<IContextMenuCallback>,
    {
        (::windows::core::Interface::vtable(self).AddMenuItems)(::windows::core::Interface::as_raw(self), pidataobject.into_param().abi(), picallback.into_param().abi(), pinsertionallowed).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Command<P0>(&self, lcommandid: i32, pidataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).Command)(::windows::core::Interface::as_raw(self), lcommandid, pidataobject.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IExtendContextMenu, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IExtendContextMenu {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendContextMenu {}
impl ::core::fmt::Debug for IExtendContextMenu {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendContextMenu").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendContextMenu {
    type Vtable = IExtendContextMenu_Vtbl;
}
impl ::core::clone::Clone for IExtendContextMenu {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IExtendContextMenu {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f3b7a4f_cfac_11cf_b8e3_00c04fd8d5b0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendContextMenu_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddMenuItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidataobject: *mut ::core::ffi::c_void, picallback: *mut ::core::ffi::c_void, pinsertionallowed: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddMenuItems: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Command: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lcommandid: i32, pidataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Command: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IExtendControlbar(::windows::core::IUnknown);
impl IExtendControlbar {
    pub unsafe fn SetControlbar<P0>(&self, pcontrolbar: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IControlbar>,
    {
        (::windows::core::Interface::vtable(self).SetControlbar)(::windows::core::Interface::as_raw(self), pcontrolbar.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ControlbarNotify<P0, P1>(&self, event: MMC_NOTIFY_TYPE, arg: P0, param2: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
        P1: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).ControlbarNotify)(::windows::core::Interface::as_raw(self), event, arg.into_param().abi(), param2.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IExtendControlbar, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IExtendControlbar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendControlbar {}
impl ::core::fmt::Debug for IExtendControlbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendControlbar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendControlbar {
    type Vtable = IExtendControlbar_Vtbl;
}
impl ::core::clone::Clone for IExtendControlbar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IExtendControlbar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49506520_6f40_11d0_a98b_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendControlbar_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetControlbar: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontrolbar: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ControlbarNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, event: MMC_NOTIFY_TYPE, arg: super::super::Foundation::LPARAM, param2: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ControlbarNotify: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IExtendPropertySheet(::windows::core::IUnknown);
impl IExtendPropertySheet {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyPages<P0, P1>(&self, lpprovider: P0, handle: isize, lpidataobject: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPropertySheetCallback>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).CreatePropertyPages)(::windows::core::Interface::as_raw(self), lpprovider.into_param().abi(), handle, lpidataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryPagesFor<P0>(&self, lpdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).QueryPagesFor)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IExtendPropertySheet, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IExtendPropertySheet {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendPropertySheet {}
impl ::core::fmt::Debug for IExtendPropertySheet {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendPropertySheet").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendPropertySheet {
    type Vtable = IExtendPropertySheet_Vtbl;
}
impl ::core::clone::Clone for IExtendPropertySheet {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IExtendPropertySheet {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85de64dc_ef21_11cf_a285_00c04fd8dbe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendPropertySheet_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertyPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpprovider: *mut ::core::ffi::c_void, handle: isize, lpidataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertyPages: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub QueryPagesFor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    QueryPagesFor: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IExtendPropertySheet2(::windows::core::IUnknown);
impl IExtendPropertySheet2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertyPages<P0, P1>(&self, lpprovider: P0, handle: isize, lpidataobject: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IPropertySheetCallback>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).base__.CreatePropertyPages)(::windows::core::Interface::as_raw(self), lpprovider.into_param().abi(), handle, lpidataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn QueryPagesFor<P0>(&self, lpdataobject: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).base__.QueryPagesFor)(::windows::core::Interface::as_raw(self), lpdataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub unsafe fn GetWatermarks<P0>(&self, lpidataobject: P0, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).GetWatermarks)(::windows::core::Interface::as_raw(self), lpidataobject.into_param().abi(), lphwatermark, lphheader, lphpalette, bstretch).ok()
    }
}
::windows::imp::interface_hierarchy!(IExtendPropertySheet2, ::windows::core::IUnknown, IExtendPropertySheet);
impl ::core::cmp::PartialEq for IExtendPropertySheet2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendPropertySheet2 {}
impl ::core::fmt::Debug for IExtendPropertySheet2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendPropertySheet2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendPropertySheet2 {
    type Vtable = IExtendPropertySheet2_Vtbl;
}
impl ::core::clone::Clone for IExtendPropertySheet2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IExtendPropertySheet2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7a87232_4a51_11d1_a7ea_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendPropertySheet2_Vtbl {
    pub base__: IExtendPropertySheet_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com"))]
    pub GetWatermarks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpidataobject: *mut ::core::ffi::c_void, lphwatermark: *mut super::super::Graphics::Gdi::HBITMAP, lphheader: *mut super::super::Graphics::Gdi::HBITMAP, lphpalette: *mut super::super::Graphics::Gdi::HPALETTE, bstretch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com")))]
    GetWatermarks: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IExtendTaskPad(::windows::core::IUnknown);
impl IExtendTaskPad {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn TaskNotify<P0>(&self, pdo: P0, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).TaskNotify)(::windows::core::Interface::as_raw(self), pdo.into_param().abi(), arg, param2).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EnumTasks<P0, P1>(&self, pdo: P0, sztaskgroup: P1) -> ::windows::core::Result<IEnumTASK>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<IEnumTASK>();
        (::windows::core::Interface::vtable(self).EnumTasks)(::windows::core::Interface::as_raw(self), pdo.into_param().abi(), sztaskgroup.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetTitle<P0>(&self, pszgroup: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetTitle)(::windows::core::Interface::as_raw(self), pszgroup.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetDescriptiveText<P0>(&self, pszgroup: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetDescriptiveText)(::windows::core::Interface::as_raw(self), pszgroup.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetBackground<P0>(&self, pszgroup: P0) -> ::windows::core::Result<MMC_TASK_DISPLAY_OBJECT>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<MMC_TASK_DISPLAY_OBJECT>();
        (::windows::core::Interface::vtable(self).GetBackground)(::windows::core::Interface::as_raw(self), pszgroup.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetListPadInfo<P0>(&self, pszgroup: P0) -> ::windows::core::Result<MMC_LISTPAD_INFO>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<MMC_LISTPAD_INFO>();
        (::windows::core::Interface::vtable(self).GetListPadInfo)(::windows::core::Interface::as_raw(self), pszgroup.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IExtendTaskPad, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IExtendTaskPad {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendTaskPad {}
impl ::core::fmt::Debug for IExtendTaskPad {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendTaskPad").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendTaskPad {
    type Vtable = IExtendTaskPad_Vtbl;
}
impl ::core::clone::Clone for IExtendTaskPad {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IExtendTaskPad {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8dee6511_554d_11d1_9fea_00600832db4a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendTaskPad_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub TaskNotify: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdo: *mut ::core::ffi::c_void, arg: *const super::Com::VARIANT, param2: *const super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    TaskNotify: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EnumTasks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdo: *mut ::core::ffi::c_void, sztaskgroup: ::windows::core::PCWSTR, ppenumtask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EnumTasks: usize,
    pub GetTitle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgroup: ::windows::core::PCWSTR, psztitle: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetDescriptiveText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgroup: ::windows::core::PCWSTR, pszdescriptivetext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetBackground: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgroup: ::windows::core::PCWSTR, ptdo: *mut MMC_TASK_DISPLAY_OBJECT) -> ::windows::core::HRESULT,
    pub GetListPadInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszgroup: ::windows::core::PCWSTR, lplistpadinfo: *mut MMC_LISTPAD_INFO) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IExtendView(::windows::core::IUnknown);
impl IExtendView {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetViews<P0, P1>(&self, pdataobject: P0, pviewextensioncallback: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<IViewExtensionCallback>,
    {
        (::windows::core::Interface::vtable(self).GetViews)(::windows::core::Interface::as_raw(self), pdataobject.into_param().abi(), pviewextensioncallback.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IExtendView, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IExtendView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IExtendView {}
impl ::core::fmt::Debug for IExtendView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IExtendView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IExtendView {
    type Vtable = IExtendView_Vtbl;
}
impl ::core::clone::Clone for IExtendView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IExtendView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x89995cee_d2ed_4c0e_ae5e_df7e76f3fa53);
}
#[repr(C)]
#[doc(hidden)]
pub struct IExtendView_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetViews: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, pviewextensioncallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetViews: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IHeaderCtrl(::windows::core::IUnknown);
impl IHeaderCtrl {
    pub unsafe fn InsertColumn<P0>(&self, ncol: i32, title: P0, nformat: i32, nwidth: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).InsertColumn)(::windows::core::Interface::as_raw(self), ncol, title.into_param().abi(), nformat, nwidth).ok()
    }
    pub unsafe fn DeleteColumn(&self, ncol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteColumn)(::windows::core::Interface::as_raw(self), ncol).ok()
    }
    pub unsafe fn SetColumnText<P0>(&self, ncol: i32, title: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetColumnText)(::windows::core::Interface::as_raw(self), ncol, title.into_param().abi()).ok()
    }
    pub unsafe fn GetColumnText(&self, ncol: i32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetColumnText)(::windows::core::Interface::as_raw(self), ncol, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColumnWidth)(::windows::core::Interface::as_raw(self), ncol, nwidth).ok()
    }
    pub unsafe fn GetColumnWidth(&self, ncol: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetColumnWidth)(::windows::core::Interface::as_raw(self), ncol, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IHeaderCtrl, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IHeaderCtrl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHeaderCtrl {}
impl ::core::fmt::Debug for IHeaderCtrl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHeaderCtrl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHeaderCtrl {
    type Vtable = IHeaderCtrl_Vtbl;
}
impl ::core::clone::Clone for IHeaderCtrl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHeaderCtrl {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb3_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderCtrl_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub InsertColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, title: ::windows::core::PCWSTR, nformat: i32, nwidth: i32) -> ::windows::core::HRESULT,
    pub DeleteColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32) -> ::windows::core::HRESULT,
    pub SetColumnText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, title: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetColumnText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, ptext: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub SetColumnWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, nwidth: i32) -> ::windows::core::HRESULT,
    pub GetColumnWidth: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncol: i32, pwidth: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IHeaderCtrl2(::windows::core::IUnknown);
impl IHeaderCtrl2 {
    pub unsafe fn InsertColumn<P0>(&self, ncol: i32, title: P0, nformat: i32, nwidth: i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.InsertColumn)(::windows::core::Interface::as_raw(self), ncol, title.into_param().abi(), nformat, nwidth).ok()
    }
    pub unsafe fn DeleteColumn(&self, ncol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteColumn)(::windows::core::Interface::as_raw(self), ncol).ok()
    }
    pub unsafe fn SetColumnText<P0>(&self, ncol: i32, title: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetColumnText)(::windows::core::Interface::as_raw(self), ncol, title.into_param().abi()).ok()
    }
    pub unsafe fn GetColumnText(&self, ncol: i32) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetColumnText)(::windows::core::Interface::as_raw(self), ncol, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetColumnWidth(&self, ncol: i32, nwidth: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetColumnWidth)(::windows::core::Interface::as_raw(self), ncol, nwidth).ok()
    }
    pub unsafe fn GetColumnWidth(&self, ncol: i32) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.GetColumnWidth)(::windows::core::Interface::as_raw(self), ncol, &mut result__).from_abi(result__)
    }
    pub unsafe fn SetChangeTimeOut(&self, utimeout: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetChangeTimeOut)(::windows::core::Interface::as_raw(self), utimeout).ok()
    }
    pub unsafe fn SetColumnFilter(&self, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetColumnFilter)(::windows::core::Interface::as_raw(self), ncolumn, dwtype, pfilterdata).ok()
    }
    pub unsafe fn GetColumnFilter(&self, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumnFilter)(::windows::core::Interface::as_raw(self), ncolumn, pdwtype, pfilterdata).ok()
    }
}
::windows::imp::interface_hierarchy!(IHeaderCtrl2, ::windows::core::IUnknown, IHeaderCtrl);
impl ::core::cmp::PartialEq for IHeaderCtrl2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IHeaderCtrl2 {}
impl ::core::fmt::Debug for IHeaderCtrl2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IHeaderCtrl2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IHeaderCtrl2 {
    type Vtable = IHeaderCtrl2_Vtbl;
}
impl ::core::clone::Clone for IHeaderCtrl2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IHeaderCtrl2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9757abb8_1b32_11d1_a7ce_00c04fd8d565);
}
#[repr(C)]
#[doc(hidden)]
pub struct IHeaderCtrl2_Vtbl {
    pub base__: IHeaderCtrl_Vtbl,
    pub SetChangeTimeOut: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, utimeout: u32) -> ::windows::core::HRESULT,
    pub SetColumnFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: u32, dwtype: u32, pfilterdata: *const MMC_FILTERDATA) -> ::windows::core::HRESULT,
    pub GetColumnFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: u32, pdwtype: *mut u32, pfilterdata: *mut MMC_FILTERDATA) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IImageList(::windows::core::IUnknown);
impl IImageList {
    pub unsafe fn ImageListSetIcon(&self, picon: *const isize, nloc: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImageListSetIcon)(::windows::core::Interface::as_raw(self), picon, nloc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImageListSetStrip<P0>(&self, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::COLORREF>,
    {
        (::windows::core::Interface::vtable(self).ImageListSetStrip)(::windows::core::Interface::as_raw(self), pbmapsm, pbmaplg, nstartloc, cmask.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IImageList, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IImageList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IImageList {}
impl ::core::fmt::Debug for IImageList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IImageList").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IImageList {
    type Vtable = IImageList_Vtbl;
}
impl ::core::clone::Clone for IImageList {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IImageList {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb8_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IImageList_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ImageListSetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, picon: *const isize, nloc: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ImageListSetStrip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbmapsm: *const isize, pbmaplg: *const isize, nstartloc: i32, cmask: super::super::Foundation::COLORREF) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImageListSetStrip: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IMMCVersionInfo(::windows::core::IUnknown);
impl IMMCVersionInfo {
    pub unsafe fn GetMMCVersion(&self, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMMCVersion)(::windows::core::Interface::as_raw(self), pversionmajor, pversionminor).ok()
    }
}
::windows::imp::interface_hierarchy!(IMMCVersionInfo, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMMCVersionInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMMCVersionInfo {}
impl ::core::fmt::Debug for IMMCVersionInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMMCVersionInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMMCVersionInfo {
    type Vtable = IMMCVersionInfo_Vtbl;
}
impl ::core::clone::Clone for IMMCVersionInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMMCVersionInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa8d2c5fe_cdcb_4b9d_bde5_a27343ff54bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMMCVersionInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMMCVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversionmajor: *mut i32, pversionminor: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IMenuButton(::windows::core::IUnknown);
impl IMenuButton {
    pub unsafe fn AddButton<P0, P1>(&self, idcommand: i32, lpbuttontext: P0, lptooltiptext: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddButton)(::windows::core::Interface::as_raw(self), idcommand, lpbuttontext.into_param().abi(), lptooltiptext.into_param().abi()).ok()
    }
    pub unsafe fn SetButton<P0, P1>(&self, idcommand: i32, lpbuttontext: P0, lptooltiptext: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetButton)(::windows::core::Interface::as_raw(self), idcommand, lpbuttontext.into_param().abi(), lptooltiptext.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButtonState<P0>(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetButtonState)(::windows::core::Interface::as_raw(self), idcommand, nstate, bstate.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IMenuButton, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMenuButton {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMenuButton {}
impl ::core::fmt::Debug for IMenuButton {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMenuButton").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMenuButton {
    type Vtable = IMenuButton_Vtbl;
}
impl ::core::clone::Clone for IMenuButton {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMenuButton {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x951ed750_d080_11d0_b197_000000000000);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMenuButton_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddButton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: ::windows::core::PCWSTR, lptooltiptext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetButton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, lpbuttontext: ::windows::core::PCWSTR, lptooltiptext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetButtonState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetButtonState: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IMessageView(::windows::core::IUnknown);
impl IMessageView {
    pub unsafe fn SetTitleText<P0>(&self, psztitletext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetTitleText)(::windows::core::Interface::as_raw(self), psztitletext.into_param().abi()).ok()
    }
    pub unsafe fn SetBodyText<P0>(&self, pszbodytext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetBodyText)(::windows::core::Interface::as_raw(self), pszbodytext.into_param().abi()).ok()
    }
    pub unsafe fn SetIcon(&self, id: IconIdentifier) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetIcon)(::windows::core::Interface::as_raw(self), id).ok()
    }
    pub unsafe fn Clear(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Clear)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IMessageView, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IMessageView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMessageView {}
impl ::core::fmt::Debug for IMessageView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMessageView").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMessageView {
    type Vtable = IMessageView_Vtbl;
}
impl ::core::clone::Clone for IMessageView {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IMessageView {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80f94174_fccc_11d2_b991_00c04f8ecd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMessageView_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetTitleText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psztitletext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetBodyText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszbodytext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetIcon: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: IconIdentifier) -> ::windows::core::HRESULT,
    pub Clear: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct INodeProperties(::windows::core::IUnknown);
impl INodeProperties {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetProperty<P0, P1>(&self, pdataobject: P0, szpropertyname: P1) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<super::Com::IDataObject>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).GetProperty)(::windows::core::Interface::as_raw(self), pdataobject.into_param().abi(), szpropertyname.into_param().abi(), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(INodeProperties, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for INodeProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for INodeProperties {}
impl ::core::fmt::Debug for INodeProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("INodeProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for INodeProperties {
    type Vtable = INodeProperties_Vtbl;
}
impl ::core::clone::Clone for INodeProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for INodeProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x15bc4d24_a522_4406_aa55_0749537a6865);
}
#[repr(C)]
#[doc(hidden)]
pub struct INodeProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataobject: *mut ::core::ffi::c_void, szpropertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, pbstrproperty: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetProperty: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IPropertySheetCallback(::windows::core::IUnknown);
impl IPropertySheetCallback {
    #[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn AddPage<P0>(&self, hpage: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::UI::Controls::HPROPSHEETPAGE>,
    {
        (::windows::core::Interface::vtable(self).AddPage)(::windows::core::Interface::as_raw(self), hpage.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Controls\"`*"]
    #[cfg(feature = "Win32_UI_Controls")]
    pub unsafe fn RemovePage<P0>(&self, hpage: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::UI::Controls::HPROPSHEETPAGE>,
    {
        (::windows::core::Interface::vtable(self).RemovePage)(::windows::core::Interface::as_raw(self), hpage.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertySheetCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertySheetCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySheetCallback {}
impl ::core::fmt::Debug for IPropertySheetCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySheetCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertySheetCallback {
    type Vtable = IPropertySheetCallback_Vtbl;
}
impl ::core::clone::Clone for IPropertySheetCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertySheetCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85de64dd_ef21_11cf_a285_00c04fd8dbe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySheetCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_UI_Controls")]
    pub AddPage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    AddPage: usize,
    #[cfg(feature = "Win32_UI_Controls")]
    pub RemovePage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hpage: super::super::UI::Controls::HPROPSHEETPAGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Controls"))]
    RemovePage: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IPropertySheetProvider(::windows::core::IUnknown);
impl IPropertySheetProvider {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePropertySheet<P0, P1>(&self, title: P0, r#type: u8, cookie: isize, pidataobjectm: P1, dwoptions: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).CreatePropertySheet)(::windows::core::Interface::as_raw(self), title.into_param().abi(), r#type, cookie, pidataobjectm.into_param().abi(), dwoptions).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn FindPropertySheet<P0, P1>(&self, hitem: isize, lpcomponent: P0, lpdataobject: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IComponent>,
        P1: ::windows::core::IntoParam<super::Com::IDataObject>,
    {
        (::windows::core::Interface::vtable(self).FindPropertySheet)(::windows::core::Interface::as_raw(self), hitem, lpcomponent.into_param().abi(), lpdataobject.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddPrimaryPages<P0, P1, P2, P3>(&self, lpunknown: P0, bcreatehandle: P1, hnotifywindow: P2, bscopepane: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::IUnknown>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
        P2: ::windows::core::IntoParam<super::super::Foundation::HWND>,
        P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).AddPrimaryPages)(::windows::core::Interface::as_raw(self), lpunknown.into_param().abi(), bcreatehandle.into_param().abi(), hnotifywindow.into_param().abi(), bscopepane.into_param().abi()).ok()
    }
    pub unsafe fn AddExtensionPages(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddExtensionPages)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Show(&self, window: isize, page: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self), window, page).ok()
    }
}
::windows::imp::interface_hierarchy!(IPropertySheetProvider, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IPropertySheetProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPropertySheetProvider {}
impl ::core::fmt::Debug for IPropertySheetProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPropertySheetProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPropertySheetProvider {
    type Vtable = IPropertySheetProvider_Vtbl;
}
impl ::core::clone::Clone for IPropertySheetProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IPropertySheetProvider {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x85de64de_ef21_11cf_a285_00c04fd8dbe6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPropertySheetProvider_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePropertySheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, title: ::windows::core::PCWSTR, r#type: u8, cookie: isize, pidataobjectm: *mut ::core::ffi::c_void, dwoptions: u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePropertySheet: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub FindPropertySheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hitem: isize, lpcomponent: *mut ::core::ffi::c_void, lpdataobject: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    FindPropertySheet: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AddPrimaryPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpunknown: *mut ::core::ffi::c_void, bcreatehandle: super::super::Foundation::BOOL, hnotifywindow: super::super::Foundation::HWND, bscopepane: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddPrimaryPages: usize,
    pub AddExtensionPages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, window: isize, page: i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IRequiredExtensions(::windows::core::IUnknown);
impl IRequiredExtensions {
    pub unsafe fn EnableAllExtensions(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableAllExtensions)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetFirstExtension(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetFirstExtension)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNextExtension(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::GUID>();
        (::windows::core::Interface::vtable(self).GetNextExtension)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IRequiredExtensions, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IRequiredExtensions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRequiredExtensions {}
impl ::core::fmt::Debug for IRequiredExtensions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRequiredExtensions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRequiredExtensions {
    type Vtable = IRequiredExtensions_Vtbl;
}
impl ::core::clone::Clone for IRequiredExtensions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IRequiredExtensions {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x72782d7a_a4a0_11d1_af0f_00c04fb6dd2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRequiredExtensions_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub EnableAllExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFirstExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetNextExtension: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextclsid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IResultData(::windows::core::IUnknown);
impl IResultData {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn DeleteItem(&self, itemid: isize, ncol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteItem)(::windows::core::Interface::as_raw(self), itemid, ncol).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByLParam<P0>(&self, lparam: P0) -> ::windows::core::Result<isize>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<isize>();
        (::windows::core::Interface::vtable(self).FindItemByLParam)(::windows::core::Interface::as_raw(self), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteAllRsltItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAllRsltItems)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetNextItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn ModifyItemState(&self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyItemState)(::windows::core::Interface::as_raw(self), nindex, itemid, uadd, uremove).ok()
    }
    pub unsafe fn ModifyViewStyle(&self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ModifyViewStyle)(::windows::core::Interface::as_raw(self), add, remove).ok()
    }
    pub unsafe fn SetViewMode(&self, lviewmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetViewMode)(::windows::core::Interface::as_raw(self), lviewmode).ok()
    }
    pub unsafe fn GetViewMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).GetViewMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UpdateItem(&self, itemid: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UpdateItem)(::windows::core::Interface::as_raw(self), itemid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Sort<P0>(&self, ncolumn: i32, dwsortoptions: u32, luserparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).Sort)(::windows::core::Interface::as_raw(self), ncolumn, dwsortoptions, luserparam.into_param().abi()).ok()
    }
    pub unsafe fn SetDescBarText<P0>(&self, desctext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).SetDescBarText)(::windows::core::Interface::as_raw(self), desctext.into_param().abi()).ok()
    }
    pub unsafe fn SetItemCount(&self, nitemcount: i32, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetItemCount)(::windows::core::Interface::as_raw(self), nitemcount, dwoptions).ok()
    }
}
::windows::imp::interface_hierarchy!(IResultData, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IResultData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultData {}
impl ::core::fmt::Debug for IResultData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultData {
    type Vtable = IResultData_Vtbl;
}
impl ::core::clone::Clone for IResultData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IResultData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x31da5fa0_e0eb_11cf_9f21_00aa003ca9f6);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub InsertItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InsertItem: usize,
    pub DeleteItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: isize, ncol: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub FindItemByLParam: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lparam: super::super::Foundation::LPARAM, pitemid: *mut isize) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FindItemByLParam: usize,
    pub DeleteAllRsltItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetItem: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetNextItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *mut RESULTDATAITEM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetNextItem: usize,
    pub ModifyItemState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::HRESULT,
    pub ModifyViewStyle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::HRESULT,
    pub SetViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lviewmode: i32) -> ::windows::core::HRESULT,
    pub GetViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lviewmode: *mut i32) -> ::windows::core::HRESULT,
    pub UpdateItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Sort: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Sort: usize,
    pub SetDescBarText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, desctext: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub SetItemCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nitemcount: i32, dwoptions: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IResultData2(::windows::core::IUnknown);
impl IResultData2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InsertItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.InsertItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn DeleteItem(&self, itemid: isize, ncol: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteItem)(::windows::core::Interface::as_raw(self), itemid, ncol).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FindItemByLParam<P0>(&self, lparam: P0) -> ::windows::core::Result<isize>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        let mut result__ = ::windows::core::zeroed::<isize>();
        (::windows::core::Interface::vtable(self).base__.FindItemByLParam)(::windows::core::Interface::as_raw(self), lparam.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteAllRsltItems(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.DeleteAllRsltItems)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetItem(&self, item: *const RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetNextItem(&self, item: *mut RESULTDATAITEM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetNextItem)(::windows::core::Interface::as_raw(self), item).ok()
    }
    pub unsafe fn ModifyItemState(&self, nindex: i32, itemid: isize, uadd: u32, uremove: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ModifyItemState)(::windows::core::Interface::as_raw(self), nindex, itemid, uadd, uremove).ok()
    }
    pub unsafe fn ModifyViewStyle(&self, add: MMC_RESULT_VIEW_STYLE, remove: MMC_RESULT_VIEW_STYLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ModifyViewStyle)(::windows::core::Interface::as_raw(self), add, remove).ok()
    }
    pub unsafe fn SetViewMode(&self, lviewmode: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetViewMode)(::windows::core::Interface::as_raw(self), lviewmode).ok()
    }
    pub unsafe fn GetViewMode(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).base__.GetViewMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn UpdateItem(&self, itemid: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.UpdateItem)(::windows::core::Interface::as_raw(self), itemid).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Sort<P0>(&self, ncolumn: i32, dwsortoptions: u32, luserparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).base__.Sort)(::windows::core::Interface::as_raw(self), ncolumn, dwsortoptions, luserparam.into_param().abi()).ok()
    }
    pub unsafe fn SetDescBarText<P0>(&self, desctext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).base__.SetDescBarText)(::windows::core::Interface::as_raw(self), desctext.into_param().abi()).ok()
    }
    pub unsafe fn SetItemCount(&self, nitemcount: i32, dwoptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetItemCount)(::windows::core::Interface::as_raw(self), nitemcount, dwoptions).ok()
    }
    pub unsafe fn RenameResultItem(&self, itemid: isize) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RenameResultItem)(::windows::core::Interface::as_raw(self), itemid).ok()
    }
}
::windows::imp::interface_hierarchy!(IResultData2, ::windows::core::IUnknown, IResultData);
impl ::core::cmp::PartialEq for IResultData2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultData2 {}
impl ::core::fmt::Debug for IResultData2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultData2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultData2 {
    type Vtable = IResultData2_Vtbl;
}
impl ::core::clone::Clone for IResultData2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IResultData2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f36e0eb_a7f1_4a81_be5a_9247f7de4b1b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultData2_Vtbl {
    pub base__: IResultData_Vtbl,
    pub RenameResultItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, itemid: isize) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IResultDataCompare(::windows::core::IUnknown);
impl IResultDataCompare {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare<P0>(&self, luserparam: P0, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).Compare)(::windows::core::Interface::as_raw(self), luserparam.into_param().abi(), cookiea, cookieb, pnresult).ok()
    }
}
::windows::imp::interface_hierarchy!(IResultDataCompare, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IResultDataCompare {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultDataCompare {}
impl ::core::fmt::Debug for IResultDataCompare {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultDataCompare").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultDataCompare {
    type Vtable = IResultDataCompare_Vtbl;
}
impl ::core::clone::Clone for IResultDataCompare {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IResultDataCompare {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe8315a52_7a1a_11d0_a2d2_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultDataCompare_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, luserparam: super::super::Foundation::LPARAM, cookiea: isize, cookieb: isize, pnresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Compare: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IResultDataCompareEx(::windows::core::IUnknown);
impl IResultDataCompareEx {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Compare(&self, prdc: *const RDCOMPARE) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Compare)(::windows::core::Interface::as_raw(self), prdc, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IResultDataCompareEx, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IResultDataCompareEx {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultDataCompareEx {}
impl ::core::fmt::Debug for IResultDataCompareEx {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultDataCompareEx").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultDataCompareEx {
    type Vtable = IResultDataCompareEx_Vtbl;
}
impl ::core::clone::Clone for IResultDataCompareEx {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IResultDataCompareEx {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x96933476_0251_11d3_aeb0_00c04f8ecd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultDataCompareEx_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Compare: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prdc: *const RDCOMPARE, pnresult: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Compare: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IResultOwnerData(::windows::core::IUnknown);
impl IResultOwnerData {
    pub unsafe fn FindItem(&self, pfindinfo: *const RESULTFINDINFO) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).FindItem)(::windows::core::Interface::as_raw(self), pfindinfo, &mut result__).from_abi(result__)
    }
    pub unsafe fn CacheHint(&self, nstartindex: i32, nendindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CacheHint)(::windows::core::Interface::as_raw(self), nstartindex, nendindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SortItems<P0>(&self, ncolumn: i32, dwsortoptions: u32, luserparam: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::LPARAM>,
    {
        (::windows::core::Interface::vtable(self).SortItems)(::windows::core::Interface::as_raw(self), ncolumn, dwsortoptions, luserparam.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IResultOwnerData, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IResultOwnerData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IResultOwnerData {}
impl ::core::fmt::Debug for IResultOwnerData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IResultOwnerData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IResultOwnerData {
    type Vtable = IResultOwnerData_Vtbl;
}
impl ::core::clone::Clone for IResultOwnerData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IResultOwnerData {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cb396d8_ea83_11d0_aef1_00c04fb6dd2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IResultOwnerData_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub FindItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfindinfo: *const RESULTFINDINFO, pnfoundindex: *mut i32) -> ::windows::core::HRESULT,
    pub CacheHint: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nstartindex: i32, nendindex: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SortItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ncolumn: i32, dwsortoptions: u32, luserparam: super::super::Foundation::LPARAM) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SortItems: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct ISnapinAbout(::windows::core::IUnknown);
impl ISnapinAbout {
    pub unsafe fn GetSnapinDescription(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSnapinDescription)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetProvider(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetProvider)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetSnapinVersion(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSnapinVersion)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_WindowsAndMessaging\"`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub unsafe fn GetSnapinImage(&self) -> ::windows::core::Result<super::super::UI::WindowsAndMessaging::HICON> {
        let mut result__ = ::windows::core::zeroed::<super::super::UI::WindowsAndMessaging::HICON>();
        (::windows::core::Interface::vtable(self).GetSnapinImage)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetStaticFolderImage(&self, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStaticFolderImage)(::windows::core::Interface::as_raw(self), hsmallimage, hsmallimageopen, hlargeimage, cmask).ok()
    }
}
::windows::imp::interface_hierarchy!(ISnapinAbout, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISnapinAbout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinAbout {}
impl ::core::fmt::Debug for ISnapinAbout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinAbout").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinAbout {
    type Vtable = ISnapinAbout_Vtbl;
}
impl ::core::clone::Clone for ISnapinAbout {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISnapinAbout {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1245208c_a151_11d0_a7d7_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinAbout_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetSnapinDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpdescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSnapinVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpversion: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub GetSnapinImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, happicon: *mut super::super::UI::WindowsAndMessaging::HICON) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_WindowsAndMessaging"))]
    GetSnapinImage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub GetStaticFolderImage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hsmallimage: *mut super::super::Graphics::Gdi::HBITMAP, hsmallimageopen: *mut super::super::Graphics::Gdi::HBITMAP, hlargeimage: *mut super::super::Graphics::Gdi::HBITMAP, cmask: *mut super::super::Foundation::COLORREF) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    GetStaticFolderImage: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct ISnapinHelp(::windows::core::IUnknown);
impl ISnapinHelp {
    pub unsafe fn GetHelpTopic(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetHelpTopic)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISnapinHelp, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISnapinHelp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinHelp {}
impl ::core::fmt::Debug for ISnapinHelp {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinHelp").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinHelp {
    type Vtable = ISnapinHelp_Vtbl;
}
impl ::core::clone::Clone for ISnapinHelp {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISnapinHelp {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa6b15ace_df59_11d0_a7dd_00c04fd909dd);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinHelp_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetHelpTopic: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcompiledhelpfile: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct ISnapinHelp2(::windows::core::IUnknown);
impl ISnapinHelp2 {
    pub unsafe fn GetHelpTopic(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).base__.GetHelpTopic)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetLinkedTopics(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetLinkedTopics)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(ISnapinHelp2, ::windows::core::IUnknown, ISnapinHelp);
impl ::core::cmp::PartialEq for ISnapinHelp2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinHelp2 {}
impl ::core::fmt::Debug for ISnapinHelp2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinHelp2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinHelp2 {
    type Vtable = ISnapinHelp2_Vtbl;
}
impl ::core::clone::Clone for ISnapinHelp2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISnapinHelp2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4861a010_20f9_11d2_a510_00c04fb6dd2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinHelp2_Vtbl {
    pub base__: ISnapinHelp_Vtbl,
    pub GetLinkedTopics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lpcompiledhelpfiles: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct ISnapinProperties(::windows::core::IUnknown);
impl ISnapinProperties {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Initialize<P0>(&self, pproperties: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Properties>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pproperties.into_param().abi()).ok()
    }
    pub unsafe fn QueryPropertyNames<P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<ISnapinPropertiesCallback>,
    {
        (::windows::core::Interface::vtable(self).QueryPropertyNames)(::windows::core::Interface::as_raw(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PropertiesChanged(&self, pproperties: &[MMC_SNAPIN_PROPERTY]) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PropertiesChanged)(::windows::core::Interface::as_raw(self), pproperties.len() as _, ::core::mem::transmute(pproperties.as_ptr())).ok()
    }
}
::windows::imp::interface_hierarchy!(ISnapinProperties, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISnapinProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinProperties {}
impl ::core::fmt::Debug for ISnapinProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinProperties {
    type Vtable = ISnapinProperties_Vtbl;
}
impl ::core::clone::Clone for ISnapinProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISnapinProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7889da9_4a02_4837_bf89_1a6f2a021010);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproperties: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Initialize: usize,
    pub QueryPropertyNames: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PropertiesChanged: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cproperties: i32, pproperties: *const MMC_SNAPIN_PROPERTY) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PropertiesChanged: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct ISnapinPropertiesCallback(::windows::core::IUnknown);
impl ISnapinPropertiesCallback {
    pub unsafe fn AddPropertyName<P0>(&self, pszpropname: P0, dwflags: u32) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).AddPropertyName)(::windows::core::Interface::as_raw(self), pszpropname.into_param().abi(), dwflags).ok()
    }
}
::windows::imp::interface_hierarchy!(ISnapinPropertiesCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for ISnapinPropertiesCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISnapinPropertiesCallback {}
impl ::core::fmt::Debug for ISnapinPropertiesCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISnapinPropertiesCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISnapinPropertiesCallback {
    type Vtable = ISnapinPropertiesCallback_Vtbl;
}
impl ::core::clone::Clone for ISnapinPropertiesCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for ISnapinPropertiesCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa50fa2e5_7e61_45eb_a8d4_9a07b3e851a8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISnapinPropertiesCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddPropertyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszpropname: ::windows::core::PCWSTR, dwflags: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IStringTable(::windows::core::IUnknown);
impl IStringTable {
    pub unsafe fn AddString<P0>(&self, pszadd: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).AddString)(::windows::core::Interface::as_raw(self), pszadd.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetString(&self, stringid: u32, lpbuffer: &mut [u16], pcchout: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetString)(::windows::core::Interface::as_raw(self), stringid, lpbuffer.len() as _, ::core::mem::transmute(lpbuffer.as_ptr()), pcchout).ok()
    }
    pub unsafe fn GetStringLength(&self, stringid: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).GetStringLength)(::windows::core::Interface::as_raw(self), stringid, &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteString(&self, stringid: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteString)(::windows::core::Interface::as_raw(self), stringid).ok()
    }
    pub unsafe fn DeleteAllStrings(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteAllStrings)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn FindString<P0>(&self, pszfind: P0) -> ::windows::core::Result<u32>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<u32>();
        (::windows::core::Interface::vtable(self).FindString)(::windows::core::Interface::as_raw(self), pszfind.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Enumerate(&self) -> ::windows::core::Result<super::Com::IEnumString> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IEnumString>();
        (::windows::core::Interface::vtable(self).Enumerate)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IStringTable, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IStringTable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStringTable {}
impl ::core::fmt::Debug for IStringTable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStringTable").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IStringTable {
    type Vtable = IStringTable_Vtbl;
}
impl ::core::clone::Clone for IStringTable {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IStringTable {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde40b7a4_0f65_11d2_8e25_00c04f8ecd78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStringTable_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszadd: ::windows::core::PCWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT,
    pub GetString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stringid: u32, cchbuffer: u32, lpbuffer: ::windows::core::PWSTR, pcchout: *mut u32) -> ::windows::core::HRESULT,
    pub GetStringLength: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stringid: u32, pcchstring: *mut u32) -> ::windows::core::HRESULT,
    pub DeleteString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stringid: u32) -> ::windows::core::HRESULT,
    pub DeleteAllStrings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindString: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszfind: ::windows::core::PCWSTR, pstringid: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Enumerate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Enumerate: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IToolbar(::windows::core::IUnknown);
impl IToolbar {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn AddBitmap<P0, P1>(&self, nimages: i32, hbmp: P0, cxsize: i32, cysize: i32, crmask: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Graphics::Gdi::HBITMAP>,
        P1: ::windows::core::IntoParam<super::super::Foundation::COLORREF>,
    {
        (::windows::core::Interface::vtable(self).AddBitmap)(::windows::core::Interface::as_raw(self), nimages, hbmp.into_param().abi(), cxsize, cysize, crmask.into_param().abi()).ok()
    }
    pub unsafe fn AddButtons(&self, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddButtons)(::windows::core::Interface::as_raw(self), nbuttons, lpbuttons).ok()
    }
    pub unsafe fn InsertButton(&self, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).InsertButton)(::windows::core::Interface::as_raw(self), nindex, lpbutton).ok()
    }
    pub unsafe fn DeleteButton(&self, nindex: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteButton)(::windows::core::Interface::as_raw(self), nindex).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetButtonState(&self, idcommand: i32, nstate: MMC_BUTTON_STATE) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).GetButtonState)(::windows::core::Interface::as_raw(self), idcommand, nstate, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetButtonState<P0>(&self, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetButtonState)(::windows::core::Interface::as_raw(self), idcommand, nstate, bstate.into_param().abi()).ok()
    }
}
::windows::imp::interface_hierarchy!(IToolbar, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IToolbar {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IToolbar {}
impl ::core::fmt::Debug for IToolbar {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IToolbar").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IToolbar {
    type Vtable = IToolbar_Vtbl;
}
impl ::core::clone::Clone for IToolbar {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IToolbar {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43136eb9_d36c_11cf_adbc_00aa00a80033);
}
#[repr(C)]
#[doc(hidden)]
pub struct IToolbar_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub AddBitmap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nimages: i32, hbmp: super::super::Graphics::Gdi::HBITMAP, cxsize: i32, cysize: i32, crmask: super::super::Foundation::COLORREF) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi")))]
    AddBitmap: usize,
    pub AddButtons: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nbuttons: i32, lpbuttons: *const MMCBUTTON) -> ::windows::core::HRESULT,
    pub InsertButton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32, lpbutton: *const MMCBUTTON) -> ::windows::core::HRESULT,
    pub DeleteButton: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetButtonState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, pstate: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetButtonState: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetButtonState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, idcommand: i32, nstate: MMC_BUTTON_STATE, bstate: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetButtonState: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
pub struct IViewExtensionCallback(::windows::core::IUnknown);
impl IViewExtensionCallback {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddView(&self, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddView)(::windows::core::Interface::as_raw(self), pextviewdata).ok()
    }
}
::windows::imp::interface_hierarchy!(IViewExtensionCallback, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IViewExtensionCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IViewExtensionCallback {}
impl ::core::fmt::Debug for IViewExtensionCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IViewExtensionCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IViewExtensionCallback {
    type Vtable = IViewExtensionCallback_Vtbl;
}
impl ::core::clone::Clone for IViewExtensionCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IViewExtensionCallback {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34dd928a_7599_41e5_9f5e_d6bc3062c2da);
}
#[repr(C)]
#[doc(hidden)]
pub struct IViewExtensionCallback_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pextviewdata: *const MMC_EXT_VIEW_DATA) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddView: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct MenuItem(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl MenuItem {
    pub unsafe fn DisplayName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).DisplayName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LanguageIndependentName(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).LanguageIndependentName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Path(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Path)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn LanguageIndependentPath(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).LanguageIndependentPath)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Execute(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Execute)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Enabled)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(MenuItem, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MenuItem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MenuItem {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MenuItem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MenuItem").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for MenuItem {
    type Vtable = MenuItem_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MenuItem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for MenuItem {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0178fad1_b361_4b27_96ad_67c57ebf2e1d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct MenuItem_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub DisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, displayname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LanguageIndependentName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageindependentname: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Path: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, path: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub LanguageIndependentPath: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, languageindependentpath: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Execute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Node(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Node {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn get_Property<P0>(&self, propertyname: P0) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_Property)(::windows::core::Interface::as_raw(self), propertyname.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Bookmark(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Bookmark)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsScopeNode(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsScopeNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Nodetype(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Nodetype)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Node, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Node {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Node {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Node").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Node {
    type Vtable = Node_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Node {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Node {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf81ed800_7839_4447_945d_8e15da59ca55);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Node_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub get_Property: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::std::mem::MaybeUninit<::windows::core::BSTR>, propertyvalue: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Bookmark: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bookmark: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsScopeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isscopenode: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsScopeNode: usize,
    pub Nodetype: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodetype: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Nodes(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Nodes {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<Node> {
        let mut result__ = ::windows::core::zeroed::<Node>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Nodes, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Nodes {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Nodes {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Nodes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Nodes").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Nodes {
    type Vtable = Nodes_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Nodes {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Nodes {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x313b01df_b22f_4d42_b1b8_483cdcf51d35);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Nodes_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, node: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Properties(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Properties {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item<P0>(&self, name: P0) -> ::windows::core::Result<Property>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<Property>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), name.into_param().abi(), &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Remove<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), name.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Properties, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Properties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Properties {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Properties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Properties").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Properties {
    type Vtable = Properties_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Properties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Properties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2886abc2_a425_42b2_91c6_e25c0e04581c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Properties_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>, property: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Property(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Property {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::Com::VARIANT> {
        let mut result__ = ::windows::core::zeroed::<super::Com::VARIANT>();
        (::windows::core::Interface::vtable(self).Value)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, value: super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(value)).ok()
    }
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Property, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Property {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Property {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Property").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Property {
    type Vtable = Property_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Property {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Property {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4600c3a5_e301_41d8_b6d0_ef2e4212e0ca);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Property_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: *mut super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, value: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SetValue: usize,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ScopeNamespace(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ScopeNamespace {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetParent<P0>(&self, node: P0) -> ::windows::core::Result<Node>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        let mut result__ = ::windows::core::zeroed::<Node>();
        (::windows::core::Interface::vtable(self).GetParent)(::windows::core::Interface::as_raw(self), node.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetChild<P0>(&self, node: P0) -> ::windows::core::Result<Node>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        let mut result__ = ::windows::core::zeroed::<Node>();
        (::windows::core::Interface::vtable(self).GetChild)(::windows::core::Interface::as_raw(self), node.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetNext<P0>(&self, node: P0) -> ::windows::core::Result<Node>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        let mut result__ = ::windows::core::zeroed::<Node>();
        (::windows::core::Interface::vtable(self).GetNext)(::windows::core::Interface::as_raw(self), node.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetRoot(&self) -> ::windows::core::Result<Node> {
        let mut result__ = ::windows::core::zeroed::<Node>();
        (::windows::core::Interface::vtable(self).GetRoot)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Expand<P0>(&self, node: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        (::windows::core::Interface::vtable(self).Expand)(::windows::core::Interface::as_raw(self), node.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(ScopeNamespace, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ScopeNamespace {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ScopeNamespace {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ScopeNamespace {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ScopeNamespace").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ScopeNamespace {
    type Vtable = ScopeNamespace_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ScopeNamespace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for ScopeNamespace {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xebbb48dc_1a3b_4d86_b786_c21b28389012);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ScopeNamespace_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub GetParent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, parent: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetParent: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetChild: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, child: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetChild: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, next: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetNext: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetRoot: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, root: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetRoot: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Expand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Expand: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct SnapIn(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl SnapIn {
    pub unsafe fn Name(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Name)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Vendor(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Vendor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Version(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Version)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Extensions(&self) -> ::windows::core::Result<Extensions> {
        let mut result__ = ::windows::core::zeroed::<Extensions>();
        (::windows::core::Interface::vtable(self).Extensions)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SnapinCLSID(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).SnapinCLSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Properties(&self) -> ::windows::core::Result<Properties> {
        let mut result__ = ::windows::core::zeroed::<Properties>();
        (::windows::core::Interface::vtable(self).Properties)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnableAllExtensions<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).EnableAllExtensions)(::windows::core::Interface::as_raw(self), enable.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(SnapIn, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SnapIn {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SnapIn {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SnapIn {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapIn").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for SnapIn {
    type Vtable = SnapIn_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SnapIn {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for SnapIn {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3be910f6_3459_49c6_a1bb_41e6be9df3ea);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct SnapIn_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Vendor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, vendor: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Version: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Extensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, extensions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Extensions: usize,
    pub SnapinCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapinclsid: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Properties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, properties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Properties: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EnableAllExtensions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enable: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EnableAllExtensions: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct SnapIns(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl SnapIns {
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<SnapIn> {
        let mut result__ = ::windows::core::zeroed::<SnapIn>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<P0>(&self, snapinnameorclsid: P0, parentsnapin: super::Com::VARIANT, properties: super::Com::VARIANT) -> ::windows::core::Result<SnapIn>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        let mut result__ = ::windows::core::zeroed::<SnapIn>();
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), snapinnameorclsid.into_param().abi(), ::core::mem::transmute(parentsnapin), ::core::mem::transmute(properties), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Remove<P0>(&self, snapin: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<SnapIn>,
    {
        (::windows::core::Interface::vtable(self).Remove)(::windows::core::Interface::as_raw(self), snapin.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(SnapIns, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SnapIns {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SnapIns {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SnapIns {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SnapIns").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for SnapIns {
    type Vtable = SnapIns_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SnapIns {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for SnapIns {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2ef3de1d_b12a_49d1_92c5_0b00798768f1);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct SnapIns_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, snapin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapinnameorclsid: ::std::mem::MaybeUninit<::windows::core::BSTR>, parentsnapin: super::Com::VARIANT, properties: super::Com::VARIANT, snapin: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct View(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl View {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ActiveScopeNode(&self) -> ::windows::core::Result<Node> {
        let mut result__ = ::windows::core::zeroed::<Node>();
        (::windows::core::Interface::vtable(self).ActiveScopeNode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SetActiveScopeNode<P0>(&self, node: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        (::windows::core::Interface::vtable(self).SetActiveScopeNode)(::windows::core::Interface::as_raw(self), node.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Selection(&self) -> ::windows::core::Result<Nodes> {
        let mut result__ = ::windows::core::zeroed::<Nodes>();
        (::windows::core::Interface::vtable(self).Selection)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ListItems(&self) -> ::windows::core::Result<Nodes> {
        let mut result__ = ::windows::core::zeroed::<Nodes>();
        (::windows::core::Interface::vtable(self).ListItems)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn SnapinScopeObject(&self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).SnapinScopeObject)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scopenode), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SnapinSelectionObject(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).SnapinSelectionObject)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Is<P0>(&self, view: P0) -> ::windows::core::Result<super::super::Foundation::VARIANT_BOOL>
    where
        P0: ::windows::core::IntoParam<View>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::VARIANT_BOOL>();
        (::windows::core::Interface::vtable(self).Is)(::windows::core::Interface::as_raw(self), view.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Document(&self) -> ::windows::core::Result<Document> {
        let mut result__ = ::windows::core::zeroed::<Document>();
        (::windows::core::Interface::vtable(self).Document)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SelectAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SelectAll)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Select<P0>(&self, node: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        (::windows::core::Interface::vtable(self).Select)(::windows::core::Interface::as_raw(self), node.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Deselect<P0>(&self, node: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        (::windows::core::Interface::vtable(self).Deselect)(::windows::core::Interface::as_raw(self), node.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn IsSelected<P0>(&self, node: P0) -> ::windows::core::Result<super::super::Foundation::BOOL>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).IsSelected)(::windows::core::Interface::as_raw(self), node.into_param().abi(), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DisplayScopeNodePropertySheet(&self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplayScopeNodePropertySheet)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scopenode)).ok()
    }
    pub unsafe fn DisplaySelectionPropertySheet(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DisplaySelectionPropertySheet)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn CopyScopeNode(&self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopyScopeNode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scopenode)).ok()
    }
    pub unsafe fn CopySelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).CopySelection)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn DeleteScopeNode(&self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteScopeNode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scopenode)).ok()
    }
    pub unsafe fn DeleteSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteSelection)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RenameScopeNode<P0>(&self, newname: P0, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).RenameScopeNode)(::windows::core::Interface::as_raw(self), newname.into_param().abi(), ::core::mem::transmute(scopenode)).ok()
    }
    pub unsafe fn RenameSelectedItem<P0>(&self, newname: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).RenameSelectedItem)(::windows::core::Interface::as_raw(self), newname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn get_ScopeNodeContextMenu(&self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<ContextMenu> {
        let mut result__ = ::windows::core::zeroed::<ContextMenu>();
        (::windows::core::Interface::vtable(self).get_ScopeNodeContextMenu)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scopenode), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn SelectionContextMenu(&self) -> ::windows::core::Result<ContextMenu> {
        let mut result__ = ::windows::core::zeroed::<ContextMenu>();
        (::windows::core::Interface::vtable(self).SelectionContextMenu)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn RefreshScopeNode(&self, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshScopeNode)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(scopenode)).ok()
    }
    pub unsafe fn RefreshSelection(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RefreshSelection)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn ExecuteSelectionMenuItem<P0>(&self, menuitempath: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ExecuteSelectionMenuItem)(::windows::core::Interface::as_raw(self), menuitempath.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn ExecuteScopeNodeMenuItem<P0>(&self, menuitempath: P0, scopenode: super::Com::VARIANT) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ExecuteScopeNodeMenuItem)(::windows::core::Interface::as_raw(self), menuitempath.into_param().abi(), ::core::mem::transmute(scopenode)).ok()
    }
    pub unsafe fn ExecuteShellCommand<P0, P1, P2, P3>(&self, command: P0, directory: P1, parameters: P2, windowstate: P3) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
        P1: ::windows::core::IntoParam<::windows::core::BSTR>,
        P2: ::windows::core::IntoParam<::windows::core::BSTR>,
        P3: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ExecuteShellCommand)(::windows::core::Interface::as_raw(self), command.into_param().abi(), directory.into_param().abi(), parameters.into_param().abi(), windowstate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Frame(&self) -> ::windows::core::Result<Frame> {
        let mut result__ = ::windows::core::zeroed::<Frame>();
        (::windows::core::Interface::vtable(self).Frame)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Close)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ScopeTreeVisible(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).ScopeTreeVisible)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetScopeTreeVisible<P0>(&self, visible: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetScopeTreeVisible)(::windows::core::Interface::as_raw(self), visible.into_param().abi()).ok()
    }
    pub unsafe fn Back(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Back)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Forward(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Forward)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetStatusBarText<P0>(&self, statusbartext: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).SetStatusBarText)(::windows::core::Interface::as_raw(self), statusbartext.into_param().abi()).ok()
    }
    pub unsafe fn Memento(&self) -> ::windows::core::Result<::windows::core::BSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).Memento)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn ViewMemento<P0>(&self, memento: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ViewMemento)(::windows::core::Interface::as_raw(self), memento.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Columns(&self) -> ::windows::core::Result<Columns> {
        let mut result__ = ::windows::core::zeroed::<Columns>();
        (::windows::core::Interface::vtable(self).Columns)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn get_CellContents<P0>(&self, node: P0, column: i32) -> ::windows::core::Result<::windows::core::BSTR>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        let mut result__ = ::windows::core::zeroed::<::windows::core::BSTR>();
        (::windows::core::Interface::vtable(self).get_CellContents)(::windows::core::Interface::as_raw(self), node.into_param().abi(), column, &mut result__).from_abi(result__)
    }
    pub unsafe fn ExportList<P0>(&self, file: P0, exportoptions: _ExportListOptions) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).ExportList)(::windows::core::Interface::as_raw(self), file.into_param().abi(), exportoptions).ok()
    }
    pub unsafe fn ListViewMode(&self) -> ::windows::core::Result<_ListViewMode> {
        let mut result__ = ::windows::core::zeroed::<_ListViewMode>();
        (::windows::core::Interface::vtable(self).ListViewMode)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn SetListViewMode(&self, mode: _ListViewMode) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetListViewMode)(::windows::core::Interface::as_raw(self), mode).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ControlObject(&self) -> ::windows::core::Result<super::Com::IDispatch> {
        let mut result__ = ::windows::core::zeroed::<super::Com::IDispatch>();
        (::windows::core::Interface::vtable(self).ControlObject)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(View, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for View {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for View {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for View {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("View").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for View {
    type Vtable = View_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for View {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for View {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6efc2da2_b38c_457e_9abb_ed2d189b8c38);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct View_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ActiveScopeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ActiveScopeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SetActiveScopeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SetActiveScopeNode: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Selection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Selection: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub ListItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, nodes: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ListItems: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub SnapinScopeObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: super::Com::VARIANT, scopenodeobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    SnapinScopeObject: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SnapinSelectionObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, selectionobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SnapinSelectionObject: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Is: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, thesame: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Is: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Document: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Document: usize,
    pub SelectAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Select: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Select: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Deselect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Deselect: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub IsSelected: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, isselected: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    IsSelected: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DisplayScopeNodePropertySheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DisplayScopeNodePropertySheet: usize,
    pub DisplaySelectionPropertySheet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub CopyScopeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    CopyScopeNode: usize,
    pub CopySelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub DeleteScopeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    DeleteScopeNode: usize,
    pub DeleteSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RenameScopeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newname: ::std::mem::MaybeUninit<::windows::core::BSTR>, scopenode: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RenameScopeNode: usize,
    pub RenameSelectedItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, newname: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub get_ScopeNodeContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: super::Com::VARIANT, contextmenu: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    get_ScopeNodeContextMenu: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub SelectionContextMenu: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contextmenu: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    SelectionContextMenu: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub RefreshScopeNode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scopenode: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    RefreshScopeNode: usize,
    pub RefreshSelection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExecuteSelectionMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, menuitempath: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub ExecuteScopeNodeMenuItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, menuitempath: ::std::mem::MaybeUninit<::windows::core::BSTR>, scopenode: super::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    ExecuteScopeNodeMenuItem: usize,
    pub ExecuteShellCommand: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, command: ::std::mem::MaybeUninit<::windows::core::BSTR>, directory: ::std::mem::MaybeUninit<::windows::core::BSTR>, parameters: ::std::mem::MaybeUninit<::windows::core::BSTR>, windowstate: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Frame: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ScopeTreeVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ScopeTreeVisible: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetScopeTreeVisible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetScopeTreeVisible: usize,
    pub Back: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Forward: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetStatusBarText: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, statusbartext: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub Memento: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memento: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    pub ViewMemento: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, memento: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Columns: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, columns: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Columns: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub get_CellContents: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, column: i32, cellcontents: *mut ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    get_CellContents: usize,
    pub ExportList: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: ::std::mem::MaybeUninit<::windows::core::BSTR>, exportoptions: _ExportListOptions) -> ::windows::core::HRESULT,
    pub ListViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: *mut _ListViewMode) -> ::windows::core::HRESULT,
    pub SetListViewMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mode: _ListViewMode) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub ControlObject: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, control: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ControlObject: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct Views(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl Views {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Item(&self, index: i32) -> ::windows::core::Result<View> {
        let mut result__ = ::windows::core::zeroed::<View>();
        (::windows::core::Interface::vtable(self).Item)(::windows::core::Interface::as_raw(self), index, &mut result__).from_abi(result__)
    }
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).Count)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Add<P0>(&self, node: P0, viewoptions: _ViewOptions) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Node>,
    {
        (::windows::core::Interface::vtable(self).Add)(::windows::core::Interface::as_raw(self), node.into_param().abi(), viewoptions).ok()
    }
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::IUnknown>();
        (::windows::core::Interface::vtable(self)._NewEnum)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(Views, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for Views {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for Views {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for Views {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("Views").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for Views {
    type Vtable = Views_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for Views {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for Views {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6b8c29d_a1ff_4d72_aab0_e381e9b9338d);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct Views_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, index: i32, view: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Item: usize,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, node: *mut ::core::ffi::c_void, viewoptions: _ViewOptions) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Add: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, retval: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _AppEvents(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _AppEvents {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnQuit<P0>(&self, application: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<_Application>,
    {
        (::windows::core::Interface::vtable(self).OnQuit)(::windows::core::Interface::as_raw(self), application.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn OnDocumentOpen<P0, P1>(&self, document: P0, new: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Document>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).OnDocumentOpen)(::windows::core::Interface::as_raw(self), document.into_param().abi(), new.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnDocumentClose<P0>(&self, document: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Document>,
    {
        (::windows::core::Interface::vtable(self).OnDocumentClose)(::windows::core::Interface::as_raw(self), document.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSnapInAdded<P0, P1>(&self, document: P0, snapin: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Document>,
        P1: ::windows::core::IntoParam<SnapIn>,
    {
        (::windows::core::Interface::vtable(self).OnSnapInAdded)(::windows::core::Interface::as_raw(self), document.into_param().abi(), snapin.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSnapInRemoved<P0, P1>(&self, document: P0, snapin: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<Document>,
        P1: ::windows::core::IntoParam<SnapIn>,
    {
        (::windows::core::Interface::vtable(self).OnSnapInRemoved)(::windows::core::Interface::as_raw(self), document.into_param().abi(), snapin.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnNewView<P0>(&self, view: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<View>,
    {
        (::windows::core::Interface::vtable(self).OnNewView)(::windows::core::Interface::as_raw(self), view.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnViewClose<P0>(&self, view: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<View>,
    {
        (::windows::core::Interface::vtable(self).OnViewClose)(::windows::core::Interface::as_raw(self), view.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnViewChange<P0, P1>(&self, view: P0, newownernode: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<View>,
        P1: ::windows::core::IntoParam<Node>,
    {
        (::windows::core::Interface::vtable(self).OnViewChange)(::windows::core::Interface::as_raw(self), view.into_param().abi(), newownernode.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnSelectionChange<P0, P1>(&self, view: P0, newnodes: P1) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<View>,
        P1: ::windows::core::IntoParam<Nodes>,
    {
        (::windows::core::Interface::vtable(self).OnSelectionChange)(::windows::core::Interface::as_raw(self), view.into_param().abi(), newnodes.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnContextMenuExecuted<P0>(&self, menuitem: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<MenuItem>,
    {
        (::windows::core::Interface::vtable(self).OnContextMenuExecuted)(::windows::core::Interface::as_raw(self), menuitem.into_param().abi()).ok()
    }
    pub unsafe fn OnToolbarButtonClicked(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnToolbarButtonClicked)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn OnListUpdated<P0>(&self, view: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<View>,
    {
        (::windows::core::Interface::vtable(self).OnListUpdated)(::windows::core::Interface::as_raw(self), view.into_param().abi()).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(_AppEvents, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _AppEvents {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _AppEvents {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _AppEvents {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_AppEvents").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _AppEvents {
    type Vtable = _AppEvents_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _AppEvents {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for _AppEvents {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde46cbdd_53f5_4635_af54_4fe71e923d3f);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _AppEvents_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub OnQuit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnQuit: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub OnDocumentOpen: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, new: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    OnDocumentOpen: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnDocumentClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnDocumentClose: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSnapInAdded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSnapInAdded: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSnapInRemoved: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut ::core::ffi::c_void, snapin: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSnapInRemoved: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnNewView: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnNewView: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnViewClose: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnViewClose: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnViewChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, newownernode: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnViewChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnSelectionChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void, newnodes: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnSelectionChange: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub OnContextMenuExecuted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, menuitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnContextMenuExecuted: usize,
    pub OnToolbarButtonClicked: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub OnListUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, view: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    OnListUpdated: usize,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _Application(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _Application {
    pub unsafe fn Help(&self) {
        (::windows::core::Interface::vtable(self).Help)(::windows::core::Interface::as_raw(self))
    }
    pub unsafe fn Quit(&self) {
        (::windows::core::Interface::vtable(self).Quit)(::windows::core::Interface::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Document(&self) -> ::windows::core::Result<Document> {
        let mut result__ = ::windows::core::zeroed::<Document>();
        (::windows::core::Interface::vtable(self).Document)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Load<P0>(&self, filename: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::BSTR>,
    {
        (::windows::core::Interface::vtable(self).Load)(::windows::core::Interface::as_raw(self), filename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Frame(&self) -> ::windows::core::Result<Frame> {
        let mut result__ = ::windows::core::zeroed::<Frame>();
        (::windows::core::Interface::vtable(self).Frame)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Visible(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).Visible)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn Show(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Show)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Hide(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Hide)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn UserControl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::windows::core::zeroed::<super::super::Foundation::BOOL>();
        (::windows::core::Interface::vtable(self).UserControl)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetUserControl<P0>(&self, usercontrol: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetUserControl)(::windows::core::Interface::as_raw(self), usercontrol.into_param().abi()).ok()
    }
    pub unsafe fn VersionMajor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).VersionMajor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn VersionMinor(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::windows::core::zeroed::<i32>();
        (::windows::core::Interface::vtable(self).VersionMinor)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(_Application, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _Application {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _Application {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _Application {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_Application").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _Application {
    type Vtable = _Application_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _Application {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for _Application {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa3afb9cc_b653_4741_86ab_f0470ec1384c);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _Application_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    pub Help: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    pub Quit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void),
    #[cfg(feature = "Win32_System_Com")]
    pub Document: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, document: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Document: usize,
    pub Load: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::std::mem::MaybeUninit<::windows::core::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub Frame: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, frame: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Frame: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Visible: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, visible: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Visible: usize,
    pub Show: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Hide: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub UserControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usercontrol: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    UserControl: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetUserControl: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, usercontrol: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetUserControl: usize,
    pub VersionMajor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, versionmajor: *mut i32) -> ::windows::core::HRESULT,
    pub VersionMinor: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, versionminor: *mut i32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct _EventConnector(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl _EventConnector {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn ConnectTo<P0>(&self, application: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<_Application>,
    {
        (::windows::core::Interface::vtable(self).ConnectTo)(::windows::core::Interface::as_raw(self), application.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
::windows::imp::interface_hierarchy!(_EventConnector, ::windows::core::IUnknown, super::Com::IDispatch);
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for _EventConnector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for _EventConnector {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for _EventConnector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_EventConnector").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for _EventConnector {
    type Vtable = _EventConnector_Vtbl;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for _EventConnector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::ComInterface for _EventConnector {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0bccd30_de44_4528_8403_a05a6a1cc8ea);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct _EventConnector_Vtbl {
    pub base__: super::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub ConnectTo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    ConnectTo: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const AUTO_WIDTH: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const AppEventsDHTMLConnector: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xade6444b_c91f_4e37_92a4_5bb430a33340);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Application: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x49b2791a_b1ae_4c90_9b8e_e860ba07f889);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ConsolePower: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf0285374_dff1_11d3_b433_00c04f8ecd78);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const HDI_HIDDEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const HIDE_COLUMN: i32 = -4i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ILSIF_LEAVE_LARGE_ICON: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ILSIF_LEAVE_SMALL_ICON: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_AUTO: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_NOICON: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_NOPARAM: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_NOPTR: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_UPDATE_NOINVALIDATEALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_UPDATE_NOSCROLL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_FILTERED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_ICON: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_LIST: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_REPORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCLV_VIEWSTYLE_SMALLICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCVersionInfo: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd6fedb1d_cf21_4bd9_af3b_c5468e9c6684);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_DEFAULT_OPERATION_COPY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_IMAGECALLBACK: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ITEM_OVERLAY_STATE_MASK: u32 = 3840u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ITEM_OVERLAY_STATE_SHIFT: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ITEM_STATE_MASK: u32 = 255u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_MULTI_SELECT_COOKIE: i32 = -2i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NODEID_SLOW_RETRIEVAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_CUSTOMTITLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NOACTIONPANE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NOPERSIST: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NOSCOPEPANE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_NOTOOLBARS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NW_OPTION_SHORTTITLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROP_CHANGEAFFECTSUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROP_MODIFIABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROP_PERSIST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROP_REMOVABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PSO_HASHELP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PSO_NEWWIZARDTYPE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PSO_NOAPPLYNOW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PSO_NO_PROPTITLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VER: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_CREATENEW: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_FILTERED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_LEXICAL_SORT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_MULTISELECT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_NOLISTVIEWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_OWNERDATALIST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_WINDOW_COOKIE: i32 = -3i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDCI_ScopeItem: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_IMAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_INDENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_INDEX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_PARAM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_STATE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RDI_STR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RFI_PARTIAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RFI_WRAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RSI_DESCENDING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RSI_NOSORTICON: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_HTML_OPTIONS_NOLISTVIEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_HTML_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_ALLOWPASTE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_EXCLUDE_SCOPE_ITEMS_FROM_LIST: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_FILTERED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_LEXICAL_SORT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_MULTISELECT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_OWNERDATALIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_LIST_OPTIONS_USEFONTLINKING: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_MISC_OPTIONS_NOLISTVIEWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_OCX_OPTIONS_CACHE_OCX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_OCX_OPTIONS_NOLISTVIEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const RVTI_OCX_OPTIONS_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_CHILDREN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_FIRST: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_IMAGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_NEXT: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_OPENIMAGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_PARAM: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_PARENT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_PREVIOUS: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_STATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SDI_STR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SPECIAL_COOKIE_MAX: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SPECIAL_COOKIE_MIN: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SPECIAL_DOBJ_MAX: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SPECIAL_DOBJ_MIN: i32 = -10i32;
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CCM_COMMANDID_MASK_CONSTANTS(pub u32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_COMMANDID_MASK_RESERVED: CCM_COMMANDID_MASK_CONSTANTS = CCM_COMMANDID_MASK_CONSTANTS(4294901760u32);
impl ::core::marker::Copy for CCM_COMMANDID_MASK_CONSTANTS {}
impl ::core::clone::Clone for CCM_COMMANDID_MASK_CONSTANTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CCM_COMMANDID_MASK_CONSTANTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CCM_COMMANDID_MASK_CONSTANTS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CCM_COMMANDID_MASK_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CCM_COMMANDID_MASK_CONSTANTS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CCM_INSERTIONALLOWED(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONALLOWED_TOP: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONALLOWED_NEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONALLOWED_TASK: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(4i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONALLOWED_VIEW: CCM_INSERTIONALLOWED = CCM_INSERTIONALLOWED(8i32);
impl ::core::marker::Copy for CCM_INSERTIONALLOWED {}
impl ::core::clone::Clone for CCM_INSERTIONALLOWED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CCM_INSERTIONALLOWED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CCM_INSERTIONALLOWED {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CCM_INSERTIONALLOWED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CCM_INSERTIONALLOWED").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CCM_INSERTIONPOINTID(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_SPECIAL: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-65536i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_SHARED: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-2147483648i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_CREATE_PRIMARY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(1073741824i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_ADD_PRIMARY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(536870912i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_ADD_3RDPARTY: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(268435456i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_RESERVED: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(268369920i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_MASK_FLAGINDEX: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(31i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_TOP: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612736i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_NEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612735i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_TASK: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612734i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_VIEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612733i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_PRIMARY_HELP: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1610612732i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_3RDPARTY_NEW: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1879048191i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_3RDPARTY_TASK: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-1879048190i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_INSERTIONPOINTID_ROOT_MENU: CCM_INSERTIONPOINTID = CCM_INSERTIONPOINTID(-2147483648i32);
impl ::core::marker::Copy for CCM_INSERTIONPOINTID {}
impl ::core::clone::Clone for CCM_INSERTIONPOINTID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CCM_INSERTIONPOINTID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CCM_INSERTIONPOINTID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CCM_INSERTIONPOINTID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CCM_INSERTIONPOINTID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CCM_SPECIAL(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_SEPARATOR: CCM_SPECIAL = CCM_SPECIAL(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_SUBMENU: CCM_SPECIAL = CCM_SPECIAL(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_DEFAULT_ITEM: CCM_SPECIAL = CCM_SPECIAL(4i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_INSERTION_POINT: CCM_SPECIAL = CCM_SPECIAL(8i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCM_SPECIAL_TESTONLY: CCM_SPECIAL = CCM_SPECIAL(16i32);
impl ::core::marker::Copy for CCM_SPECIAL {}
impl ::core::clone::Clone for CCM_SPECIAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CCM_SPECIAL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CCM_SPECIAL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CCM_SPECIAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CCM_SPECIAL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DATA_OBJECT_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCT_SCOPE: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32768i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCT_RESULT: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32769i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCT_SNAPIN_MANAGER: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(32770i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CCT_UNINITIALIZED: DATA_OBJECT_TYPES = DATA_OBJECT_TYPES(65535i32);
impl ::core::marker::Copy for DATA_OBJECT_TYPES {}
impl ::core::clone::Clone for DATA_OBJECT_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DATA_OBJECT_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DATA_OBJECT_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DATA_OBJECT_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DATA_OBJECT_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IconIdentifier(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_None: IconIdentifier = IconIdentifier(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Error: IconIdentifier = IconIdentifier(32513i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Question: IconIdentifier = IconIdentifier(32514i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Warning: IconIdentifier = IconIdentifier(32515i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Information: IconIdentifier = IconIdentifier(32516i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_First: IconIdentifier = IconIdentifier(32513i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const Icon_Last: IconIdentifier = IconIdentifier(32516i32);
impl ::core::marker::Copy for IconIdentifier {}
impl ::core::clone::Clone for IconIdentifier {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IconIdentifier {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for IconIdentifier {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for IconIdentifier {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IconIdentifier").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_ACTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ACTION_UNINITIALIZED: MMC_ACTION_TYPE = MMC_ACTION_TYPE(-1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ACTION_ID: MMC_ACTION_TYPE = MMC_ACTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ACTION_LINK: MMC_ACTION_TYPE = MMC_ACTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ACTION_SCRIPT: MMC_ACTION_TYPE = MMC_ACTION_TYPE(2i32);
impl ::core::marker::Copy for MMC_ACTION_TYPE {}
impl ::core::clone::Clone for MMC_ACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_ACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_ACTION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_ACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_ACTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_BUTTON_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ENABLED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const CHECKED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const HIDDEN: MMC_BUTTON_STATE = MMC_BUTTON_STATE(4i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const INDETERMINATE: MMC_BUTTON_STATE = MMC_BUTTON_STATE(8i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const BUTTONPRESSED: MMC_BUTTON_STATE = MMC_BUTTON_STATE(16i32);
impl ::core::marker::Copy for MMC_BUTTON_STATE {}
impl ::core::clone::Clone for MMC_BUTTON_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_BUTTON_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_BUTTON_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_BUTTON_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_BUTTON_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_CONSOLE_VERB(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_NONE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_OPEN: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32768i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_COPY: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32769i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_PASTE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32770i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_DELETE: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32771i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_PROPERTIES: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32772i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_RENAME: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32773i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_REFRESH: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32774i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_PRINT: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32775i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_CUT: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32776i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_MAX: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32777i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_FIRST: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32768i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VERB_LAST: MMC_CONSOLE_VERB = MMC_CONSOLE_VERB(32776i32);
impl ::core::marker::Copy for MMC_CONSOLE_VERB {}
impl ::core::clone::Clone for MMC_CONSOLE_VERB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_CONSOLE_VERB {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_CONSOLE_VERB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_CONSOLE_VERB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_CONSOLE_VERB").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_CONTROL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const TOOLBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MENUBUTTON: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const COMBOBOXBAR: MMC_CONTROL_TYPE = MMC_CONTROL_TYPE(2i32);
impl ::core::marker::Copy for MMC_CONTROL_TYPE {}
impl ::core::clone::Clone for MMC_CONTROL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_CONTROL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_CONTROL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_CONTROL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_CONTROL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_FILTER_CHANGE_CODE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MFCC_DISABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MFCC_ENABLE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MFCC_VALUE_CHANGE: MMC_FILTER_CHANGE_CODE = MMC_FILTER_CHANGE_CODE(2i32);
impl ::core::marker::Copy for MMC_FILTER_CHANGE_CODE {}
impl ::core::clone::Clone for MMC_FILTER_CHANGE_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_FILTER_CHANGE_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_FILTER_CHANGE_CODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_FILTER_CHANGE_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_FILTER_CHANGE_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_FILTER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_STRING_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_INT_FILTER: MMC_FILTER_TYPE = MMC_FILTER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_FILTER_NOVALUE: MMC_FILTER_TYPE = MMC_FILTER_TYPE(32768i32);
impl ::core::marker::Copy for MMC_FILTER_TYPE {}
impl ::core::clone::Clone for MMC_FILTER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_FILTER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_FILTER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_MENU_COMMAND_IDS(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCC_STANDARD_VIEW_SELECT: MMC_MENU_COMMAND_IDS = MMC_MENU_COMMAND_IDS(-1i32);
impl ::core::marker::Copy for MMC_MENU_COMMAND_IDS {}
impl ::core::clone::Clone for MMC_MENU_COMMAND_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_MENU_COMMAND_IDS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_MENU_COMMAND_IDS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_MENU_COMMAND_IDS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_MENU_COMMAND_IDS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_NOTIFY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_ACTIVATE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32769i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_ADD_IMAGES: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32770i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_BTN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32771i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32772i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_COLUMN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32773i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CONTEXTMENU: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32774i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CUTORMOVE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32775i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_DBLCLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32776i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_DELETE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32777i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_DESELECT_ALL: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32778i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_EXPAND: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32779i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_HELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32780i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_MENU_BTNCLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32781i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_MINIMIZED: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32782i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_PASTE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32783i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_PROPERTY_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32784i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_QUERY_PASTE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32785i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_REFRESH: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32786i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_REMOVE_CHILDREN: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32787i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_RENAME: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32788i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_SELECT: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32789i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_SHOW: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32790i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_VIEW_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32791i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_SNAPINHELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32792i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CONTEXTHELP: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32793i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_INITOCX: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32794i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_FILTER_CHANGE: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32795i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_FILTERBTN_CLICK: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32796i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_RESTORE_VIEW: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32797i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_PRINT: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32798i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_PRELOAD: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32799i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_LISTPAD: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32800i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_EXPANDSYNC: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32801i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_COLUMNS_CHANGED: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32802i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMCN_CANPASTE_OUTOFPROC: MMC_NOTIFY_TYPE = MMC_NOTIFY_TYPE(32803i32);
impl ::core::marker::Copy for MMC_NOTIFY_TYPE {}
impl ::core::clone::Clone for MMC_NOTIFY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_NOTIFY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_NOTIFY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_NOTIFY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_NOTIFY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_PROPERTY_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROPACT_DELETING: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROPACT_CHANGING: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_PROPACT_INITIALIZED: MMC_PROPERTY_ACTION = MMC_PROPERTY_ACTION(3i32);
impl ::core::marker::Copy for MMC_PROPERTY_ACTION {}
impl ::core::clone::Clone for MMC_PROPERTY_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_PROPERTY_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_PROPERTY_ACTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_PROPERTY_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_PROPERTY_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_RESULT_VIEW_STYLE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SINGLESEL: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SHOWSELALWAYS: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_NOSORTHEADER: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(4i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_ENSUREFOCUSVISIBLE: MMC_RESULT_VIEW_STYLE = MMC_RESULT_VIEW_STYLE(8i32);
impl ::core::marker::Copy for MMC_RESULT_VIEW_STYLE {}
impl ::core::clone::Clone for MMC_RESULT_VIEW_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_RESULT_VIEW_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_RESULT_VIEW_STYLE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_RESULT_VIEW_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_RESULT_VIEW_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_SCOPE_ITEM_STATE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SCOPE_ITEM_STATE_NORMAL: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SCOPE_ITEM_STATE_BOLD: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_SCOPE_ITEM_STATE_EXPANDEDONCE: MMC_SCOPE_ITEM_STATE = MMC_SCOPE_ITEM_STATE(3i32);
impl ::core::marker::Copy for MMC_SCOPE_ITEM_STATE {}
impl ::core::clone::Clone for MMC_SCOPE_ITEM_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_SCOPE_ITEM_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_SCOPE_ITEM_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_SCOPE_ITEM_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_SCOPE_ITEM_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_TASK_DISPLAY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_UNINITIALIZED: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_TYPE_SYMBOL: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_TYPE_VANILLA_GIF: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_TYPE_CHOCOLATE_GIF: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_TASK_DISPLAY_TYPE_BITMAP: MMC_TASK_DISPLAY_TYPE = MMC_TASK_DISPLAY_TYPE(4i32);
impl ::core::marker::Copy for MMC_TASK_DISPLAY_TYPE {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_TASK_DISPLAY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_TASK_DISPLAY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_TASK_DISPLAY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_TASK_DISPLAY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MMC_VIEW_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_TYPE_LIST: MMC_VIEW_TYPE = MMC_VIEW_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_TYPE_HTML: MMC_VIEW_TYPE = MMC_VIEW_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const MMC_VIEW_TYPE_OCX: MMC_VIEW_TYPE = MMC_VIEW_TYPE(2i32);
impl ::core::marker::Copy for MMC_VIEW_TYPE {}
impl ::core::clone::Clone for MMC_VIEW_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MMC_VIEW_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for MMC_VIEW_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for MMC_VIEW_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MMC_VIEW_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _ColumnSortOrder(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SortOrder_Ascending: _ColumnSortOrder = _ColumnSortOrder(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const SortOrder_Descending: _ColumnSortOrder = _ColumnSortOrder(1i32);
impl ::core::marker::Copy for _ColumnSortOrder {}
impl ::core::clone::Clone for _ColumnSortOrder {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _ColumnSortOrder {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for _ColumnSortOrder {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _ColumnSortOrder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ColumnSortOrder").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _DocumentMode(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const DocumentMode_Author: _DocumentMode = _DocumentMode(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const DocumentMode_User: _DocumentMode = _DocumentMode(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const DocumentMode_User_MDI: _DocumentMode = _DocumentMode(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const DocumentMode_User_SDI: _DocumentMode = _DocumentMode(3i32);
impl ::core::marker::Copy for _DocumentMode {}
impl ::core::clone::Clone for _DocumentMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _DocumentMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for _DocumentMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _DocumentMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_DocumentMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _ExportListOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ExportListOptions_Default: _ExportListOptions = _ExportListOptions(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ExportListOptions_Unicode: _ExportListOptions = _ExportListOptions(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ExportListOptions_TabDelimited: _ExportListOptions = _ExportListOptions(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ExportListOptions_SelectedItemsOnly: _ExportListOptions = _ExportListOptions(4i32);
impl ::core::marker::Copy for _ExportListOptions {}
impl ::core::clone::Clone for _ExportListOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _ExportListOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for _ExportListOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _ExportListOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ExportListOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _ListViewMode(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_Small_Icons: _ListViewMode = _ListViewMode(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_Large_Icons: _ListViewMode = _ListViewMode(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_List: _ListViewMode = _ListViewMode(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_Detail: _ListViewMode = _ListViewMode(3i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ListMode_Filtered: _ListViewMode = _ListViewMode(4i32);
impl ::core::marker::Copy for _ListViewMode {}
impl ::core::clone::Clone for _ListViewMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _ListViewMode {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for _ListViewMode {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _ListViewMode {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ListViewMode").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct _ViewOptions(pub i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_Default: _ViewOptions = _ViewOptions(0i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_ScopeTreeHidden: _ViewOptions = _ViewOptions(1i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_NoToolBars: _ViewOptions = _ViewOptions(2i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_NotPersistable: _ViewOptions = _ViewOptions(4i32);
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub const ViewOption_ActionPaneHidden: _ViewOptions = _ViewOptions(8i32);
impl ::core::marker::Copy for _ViewOptions {}
impl ::core::clone::Clone for _ViewOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for _ViewOptions {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for _ViewOptions {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for _ViewOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("_ViewOptions").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct CONTEXTMENUITEM {
    pub strName: ::windows::core::PWSTR,
    pub strStatusBarText: ::windows::core::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
}
impl ::core::marker::Copy for CONTEXTMENUITEM {}
impl ::core::clone::Clone for CONTEXTMENUITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONTEXTMENUITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTEXTMENUITEM").field("strName", &self.strName).field("strStatusBarText", &self.strStatusBarText).field("lCommandID", &self.lCommandID).field("lInsertionPointID", &self.lInsertionPointID).field("fFlags", &self.fFlags).field("fSpecialFlags", &self.fSpecialFlags).finish()
    }
}
impl ::windows::core::TypeKind for CONTEXTMENUITEM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CONTEXTMENUITEM {
    fn eq(&self, other: &Self) -> bool {
        self.strName == other.strName && self.strStatusBarText == other.strStatusBarText && self.lCommandID == other.lCommandID && self.lInsertionPointID == other.lInsertionPointID && self.fFlags == other.fFlags && self.fSpecialFlags == other.fSpecialFlags
    }
}
impl ::core::cmp::Eq for CONTEXTMENUITEM {}
impl ::core::default::Default for CONTEXTMENUITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct CONTEXTMENUITEM2 {
    pub strName: ::windows::core::PWSTR,
    pub strStatusBarText: ::windows::core::PWSTR,
    pub lCommandID: i32,
    pub lInsertionPointID: i32,
    pub fFlags: i32,
    pub fSpecialFlags: i32,
    pub strLanguageIndependentName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for CONTEXTMENUITEM2 {}
impl ::core::clone::Clone for CONTEXTMENUITEM2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CONTEXTMENUITEM2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONTEXTMENUITEM2").field("strName", &self.strName).field("strStatusBarText", &self.strStatusBarText).field("lCommandID", &self.lCommandID).field("lInsertionPointID", &self.lInsertionPointID).field("fFlags", &self.fFlags).field("fSpecialFlags", &self.fSpecialFlags).field("strLanguageIndependentName", &self.strLanguageIndependentName).finish()
    }
}
impl ::windows::core::TypeKind for CONTEXTMENUITEM2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for CONTEXTMENUITEM2 {
    fn eq(&self, other: &Self) -> bool {
        self.strName == other.strName && self.strStatusBarText == other.strStatusBarText && self.lCommandID == other.lCommandID && self.lInsertionPointID == other.lInsertionPointID && self.fFlags == other.fFlags && self.fSpecialFlags == other.fSpecialFlags && self.strLanguageIndependentName == other.strLanguageIndependentName
    }
}
impl ::core::cmp::Eq for CONTEXTMENUITEM2 {}
impl ::core::default::Default for CONTEXTMENUITEM2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MENUBUTTONDATA {
    pub idCommand: i32,
    pub x: i32,
    pub y: i32,
}
impl ::core::marker::Copy for MENUBUTTONDATA {}
impl ::core::clone::Clone for MENUBUTTONDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MENUBUTTONDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MENUBUTTONDATA").field("idCommand", &self.idCommand).field("x", &self.x).field("y", &self.y).finish()
    }
}
impl ::windows::core::TypeKind for MENUBUTTONDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MENUBUTTONDATA {
    fn eq(&self, other: &Self) -> bool {
        self.idCommand == other.idCommand && self.x == other.x && self.y == other.y
    }
}
impl ::core::cmp::Eq for MENUBUTTONDATA {}
impl ::core::default::Default for MENUBUTTONDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMCBUTTON {
    pub nBitmap: i32,
    pub idCommand: i32,
    pub fsState: u8,
    pub fsType: u8,
    pub lpButtonText: ::windows::core::PWSTR,
    pub lpTooltipText: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MMCBUTTON {}
impl ::core::clone::Clone for MMCBUTTON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMCBUTTON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMCBUTTON").field("nBitmap", &self.nBitmap).field("idCommand", &self.idCommand).field("fsState", &self.fsState).field("fsType", &self.fsType).field("lpButtonText", &self.lpButtonText).field("lpTooltipText", &self.lpTooltipText).finish()
    }
}
impl ::windows::core::TypeKind for MMCBUTTON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMCBUTTON {
    fn eq(&self, other: &Self) -> bool {
        self.nBitmap == other.nBitmap && self.idCommand == other.idCommand && self.fsState == other.fsState && self.fsType == other.fsType && self.lpButtonText == other.lpButtonText && self.lpTooltipText == other.lpTooltipText
    }
}
impl ::core::cmp::Eq for MMCBUTTON {}
impl ::core::default::Default for MMCBUTTON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_COLUMN_DATA {
    pub nColIndex: i32,
    pub dwFlags: u32,
    pub nWidth: i32,
    pub ulReserved: usize,
}
impl ::core::marker::Copy for MMC_COLUMN_DATA {}
impl ::core::clone::Clone for MMC_COLUMN_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_COLUMN_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_COLUMN_DATA").field("nColIndex", &self.nColIndex).field("dwFlags", &self.dwFlags).field("nWidth", &self.nWidth).field("ulReserved", &self.ulReserved).finish()
    }
}
impl ::windows::core::TypeKind for MMC_COLUMN_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_COLUMN_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.nColIndex == other.nColIndex && self.dwFlags == other.dwFlags && self.nWidth == other.nWidth && self.ulReserved == other.ulReserved
    }
}
impl ::core::cmp::Eq for MMC_COLUMN_DATA {}
impl ::core::default::Default for MMC_COLUMN_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_COLUMN_SET_DATA {
    pub cbSize: i32,
    pub nNumCols: i32,
    pub pColData: *mut MMC_COLUMN_DATA,
}
impl ::core::marker::Copy for MMC_COLUMN_SET_DATA {}
impl ::core::clone::Clone for MMC_COLUMN_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_COLUMN_SET_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_COLUMN_SET_DATA").field("cbSize", &self.cbSize).field("nNumCols", &self.nNumCols).field("pColData", &self.pColData).finish()
    }
}
impl ::windows::core::TypeKind for MMC_COLUMN_SET_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_COLUMN_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.nNumCols == other.nNumCols && self.pColData == other.pColData
    }
}
impl ::core::cmp::Eq for MMC_COLUMN_SET_DATA {}
impl ::core::default::Default for MMC_COLUMN_SET_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXPANDSYNC_STRUCT {
    pub bHandled: super::super::Foundation::BOOL,
    pub bExpanding: super::super::Foundation::BOOL,
    pub hItem: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_EXPANDSYNC_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_EXPANDSYNC_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_EXPANDSYNC_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_EXPANDSYNC_STRUCT").field("bHandled", &self.bHandled).field("bExpanding", &self.bExpanding).field("hItem", &self.hItem).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for MMC_EXPANDSYNC_STRUCT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_EXPANDSYNC_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.bHandled == other.bHandled && self.bExpanding == other.bExpanding && self.hItem == other.hItem
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_EXPANDSYNC_STRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_EXPANDSYNC_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MMC_EXT_VIEW_DATA {
    pub viewID: ::windows::core::GUID,
    pub pszURL: ::windows::core::PCWSTR,
    pub pszViewTitle: ::windows::core::PCWSTR,
    pub pszTooltipText: ::windows::core::PCWSTR,
    pub bReplacesDefaultView: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MMC_EXT_VIEW_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MMC_EXT_VIEW_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MMC_EXT_VIEW_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_EXT_VIEW_DATA").field("viewID", &self.viewID).field("pszURL", &self.pszURL).field("pszViewTitle", &self.pszViewTitle).field("pszTooltipText", &self.pszTooltipText).field("bReplacesDefaultView", &self.bReplacesDefaultView).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for MMC_EXT_VIEW_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MMC_EXT_VIEW_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.viewID == other.viewID && self.pszURL == other.pszURL && self.pszViewTitle == other.pszViewTitle && self.pszTooltipText == other.pszTooltipText && self.bReplacesDefaultView == other.bReplacesDefaultView
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MMC_EXT_VIEW_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MMC_EXT_VIEW_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_FILTERDATA {
    pub pszText: ::windows::core::PWSTR,
    pub cchTextMax: i32,
    pub lValue: i32,
}
impl ::core::marker::Copy for MMC_FILTERDATA {}
impl ::core::clone::Clone for MMC_FILTERDATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_FILTERDATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_FILTERDATA").field("pszText", &self.pszText).field("cchTextMax", &self.cchTextMax).field("lValue", &self.lValue).finish()
    }
}
impl ::windows::core::TypeKind for MMC_FILTERDATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_FILTERDATA {
    fn eq(&self, other: &Self) -> bool {
        self.pszText == other.pszText && self.cchTextMax == other.cchTextMax && self.lValue == other.lValue
    }
}
impl ::core::cmp::Eq for MMC_FILTERDATA {}
impl ::core::default::Default for MMC_FILTERDATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_LISTPAD_INFO {
    pub szTitle: ::windows::core::PWSTR,
    pub szButtonText: ::windows::core::PWSTR,
    pub nCommandID: isize,
}
impl ::core::marker::Copy for MMC_LISTPAD_INFO {}
impl ::core::clone::Clone for MMC_LISTPAD_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_LISTPAD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_LISTPAD_INFO").field("szTitle", &self.szTitle).field("szButtonText", &self.szButtonText).field("nCommandID", &self.nCommandID).finish()
    }
}
impl ::windows::core::TypeKind for MMC_LISTPAD_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_LISTPAD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.szTitle == other.szTitle && self.szButtonText == other.szButtonText && self.nCommandID == other.nCommandID
    }
}
impl ::core::cmp::Eq for MMC_LISTPAD_INFO {}
impl ::core::default::Default for MMC_LISTPAD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_RESTORE_VIEW {
    pub dwSize: u32,
    pub cookie: isize,
    pub pViewType: ::windows::core::PWSTR,
    pub lViewOptions: i32,
}
impl ::core::marker::Copy for MMC_RESTORE_VIEW {}
impl ::core::clone::Clone for MMC_RESTORE_VIEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_RESTORE_VIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_RESTORE_VIEW").field("dwSize", &self.dwSize).field("cookie", &self.cookie).field("pViewType", &self.pViewType).field("lViewOptions", &self.lViewOptions).finish()
    }
}
impl ::windows::core::TypeKind for MMC_RESTORE_VIEW {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_RESTORE_VIEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.cookie == other.cookie && self.pViewType == other.pViewType && self.lViewOptions == other.lViewOptions
    }
}
impl ::core::cmp::Eq for MMC_RESTORE_VIEW {}
impl ::core::default::Default for MMC_RESTORE_VIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub struct MMC_SNAPIN_PROPERTY {
    pub pszPropName: ::windows::core::PCWSTR,
    pub varValue: super::Com::VARIANT,
    pub eAction: MMC_PROPERTY_ACTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::clone::Clone for MMC_SNAPIN_PROPERTY {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::TypeKind for MMC_SNAPIN_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::core::default::Default for MMC_SNAPIN_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_SORT_DATA {
    pub nColIndex: i32,
    pub dwSortOptions: u32,
    pub ulReserved: usize,
}
impl ::core::marker::Copy for MMC_SORT_DATA {}
impl ::core::clone::Clone for MMC_SORT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_SORT_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_SORT_DATA").field("nColIndex", &self.nColIndex).field("dwSortOptions", &self.dwSortOptions).field("ulReserved", &self.ulReserved).finish()
    }
}
impl ::windows::core::TypeKind for MMC_SORT_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_SORT_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.nColIndex == other.nColIndex && self.dwSortOptions == other.dwSortOptions && self.ulReserved == other.ulReserved
    }
}
impl ::core::cmp::Eq for MMC_SORT_DATA {}
impl ::core::default::Default for MMC_SORT_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_SORT_SET_DATA {
    pub cbSize: i32,
    pub nNumItems: i32,
    pub pSortData: *mut MMC_SORT_DATA,
}
impl ::core::marker::Copy for MMC_SORT_SET_DATA {}
impl ::core::clone::Clone for MMC_SORT_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_SORT_SET_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_SORT_SET_DATA").field("cbSize", &self.cbSize).field("nNumItems", &self.nNumItems).field("pSortData", &self.pSortData).finish()
    }
}
impl ::windows::core::TypeKind for MMC_SORT_SET_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_SORT_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.nNumItems == other.nNumItems && self.pSortData == other.pSortData
    }
}
impl ::core::cmp::Eq for MMC_SORT_SET_DATA {}
impl ::core::default::Default for MMC_SORT_SET_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_TASK {
    pub sDisplayObject: MMC_TASK_DISPLAY_OBJECT,
    pub szText: ::windows::core::PWSTR,
    pub szHelpString: ::windows::core::PWSTR,
    pub eActionType: MMC_ACTION_TYPE,
    pub Anonymous: MMC_TASK_0,
}
impl ::core::marker::Copy for MMC_TASK {}
impl ::core::clone::Clone for MMC_TASK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MMC_TASK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MMC_TASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub union MMC_TASK_0 {
    pub nCommandID: isize,
    pub szActionURL: ::windows::core::PWSTR,
    pub szScript: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MMC_TASK_0 {}
impl ::core::clone::Clone for MMC_TASK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MMC_TASK_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MMC_TASK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_TASK_DISPLAY_BITMAP {
    pub szMouseOverBitmap: ::windows::core::PWSTR,
    pub szMouseOffBitmap: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MMC_TASK_DISPLAY_BITMAP {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_BITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_TASK_DISPLAY_BITMAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_TASK_DISPLAY_BITMAP").field("szMouseOverBitmap", &self.szMouseOverBitmap).field("szMouseOffBitmap", &self.szMouseOffBitmap).finish()
    }
}
impl ::windows::core::TypeKind for MMC_TASK_DISPLAY_BITMAP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_TASK_DISPLAY_BITMAP {
    fn eq(&self, other: &Self) -> bool {
        self.szMouseOverBitmap == other.szMouseOverBitmap && self.szMouseOffBitmap == other.szMouseOffBitmap
    }
}
impl ::core::cmp::Eq for MMC_TASK_DISPLAY_BITMAP {}
impl ::core::default::Default for MMC_TASK_DISPLAY_BITMAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_TASK_DISPLAY_OBJECT {
    pub eDisplayType: MMC_TASK_DISPLAY_TYPE,
    pub Anonymous: MMC_TASK_DISPLAY_OBJECT_0,
}
impl ::core::marker::Copy for MMC_TASK_DISPLAY_OBJECT {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MMC_TASK_DISPLAY_OBJECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MMC_TASK_DISPLAY_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub union MMC_TASK_DISPLAY_OBJECT_0 {
    pub uBitmap: MMC_TASK_DISPLAY_BITMAP,
    pub uSymbol: MMC_TASK_DISPLAY_SYMBOL,
}
impl ::core::marker::Copy for MMC_TASK_DISPLAY_OBJECT_0 {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_OBJECT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for MMC_TASK_DISPLAY_OBJECT_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for MMC_TASK_DISPLAY_OBJECT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_TASK_DISPLAY_SYMBOL {
    pub szFontFamilyName: ::windows::core::PWSTR,
    pub szURLtoEOT: ::windows::core::PWSTR,
    pub szSymbolString: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MMC_TASK_DISPLAY_SYMBOL {}
impl ::core::clone::Clone for MMC_TASK_DISPLAY_SYMBOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_TASK_DISPLAY_SYMBOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_TASK_DISPLAY_SYMBOL").field("szFontFamilyName", &self.szFontFamilyName).field("szURLtoEOT", &self.szURLtoEOT).field("szSymbolString", &self.szSymbolString).finish()
    }
}
impl ::windows::core::TypeKind for MMC_TASK_DISPLAY_SYMBOL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_TASK_DISPLAY_SYMBOL {
    fn eq(&self, other: &Self) -> bool {
        self.szFontFamilyName == other.szFontFamilyName && self.szURLtoEOT == other.szURLtoEOT && self.szSymbolString == other.szSymbolString
    }
}
impl ::core::cmp::Eq for MMC_TASK_DISPLAY_SYMBOL {}
impl ::core::default::Default for MMC_TASK_DISPLAY_SYMBOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct MMC_VISIBLE_COLUMNS {
    pub nVisibleColumns: i32,
    pub rgVisibleCols: [i32; 1],
}
impl ::core::marker::Copy for MMC_VISIBLE_COLUMNS {}
impl ::core::clone::Clone for MMC_VISIBLE_COLUMNS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MMC_VISIBLE_COLUMNS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MMC_VISIBLE_COLUMNS").field("nVisibleColumns", &self.nVisibleColumns).field("rgVisibleCols", &self.rgVisibleCols).finish()
    }
}
impl ::windows::core::TypeKind for MMC_VISIBLE_COLUMNS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for MMC_VISIBLE_COLUMNS {
    fn eq(&self, other: &Self) -> bool {
        self.nVisibleColumns == other.nVisibleColumns && self.rgVisibleCols == other.rgVisibleCols
    }
}
impl ::core::cmp::Eq for MMC_VISIBLE_COLUMNS {}
impl ::core::default::Default for MMC_VISIBLE_COLUMNS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RDCOMPARE {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub nColumn: i32,
    pub lUserParam: super::super::Foundation::LPARAM,
    pub prdch1: *mut RDITEMHDR,
    pub prdch2: *mut RDITEMHDR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RDCOMPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RDCOMPARE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RDCOMPARE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RDCOMPARE").field("cbSize", &self.cbSize).field("dwFlags", &self.dwFlags).field("nColumn", &self.nColumn).field("lUserParam", &self.lUserParam).field("prdch1", &self.prdch1).field("prdch2", &self.prdch2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RDCOMPARE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RDCOMPARE {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.dwFlags == other.dwFlags && self.nColumn == other.nColumn && self.lUserParam == other.lUserParam && self.prdch1 == other.prdch1 && self.prdch2 == other.prdch2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RDCOMPARE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RDCOMPARE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RDITEMHDR {
    pub dwFlags: u32,
    pub cookie: isize,
    pub lpReserved: super::super::Foundation::LPARAM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RDITEMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RDITEMHDR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RDITEMHDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RDITEMHDR").field("dwFlags", &self.dwFlags).field("cookie", &self.cookie).field("lpReserved", &self.lpReserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RDITEMHDR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RDITEMHDR {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.cookie == other.cookie && self.lpReserved == other.lpReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RDITEMHDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RDITEMHDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RESULTDATAITEM {
    pub mask: u32,
    pub bScopeItem: super::super::Foundation::BOOL,
    pub itemID: isize,
    pub nIndex: i32,
    pub nCol: i32,
    pub str: ::windows::core::PWSTR,
    pub nImage: i32,
    pub nState: u32,
    pub lParam: super::super::Foundation::LPARAM,
    pub iIndent: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RESULTDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RESULTDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RESULTDATAITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESULTDATAITEM").field("mask", &self.mask).field("bScopeItem", &self.bScopeItem).field("itemID", &self.itemID).field("nIndex", &self.nIndex).field("nCol", &self.nCol).field("str", &self.str).field("nImage", &self.nImage).field("nState", &self.nState).field("lParam", &self.lParam).field("iIndent", &self.iIndent).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for RESULTDATAITEM {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RESULTDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.bScopeItem == other.bScopeItem && self.itemID == other.itemID && self.nIndex == other.nIndex && self.nCol == other.nCol && self.str == other.str && self.nImage == other.nImage && self.nState == other.nState && self.lParam == other.lParam && self.iIndent == other.iIndent
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RESULTDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RESULTDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct RESULTFINDINFO {
    pub psz: ::windows::core::PWSTR,
    pub nStart: i32,
    pub dwOptions: u32,
}
impl ::core::marker::Copy for RESULTFINDINFO {}
impl ::core::clone::Clone for RESULTFINDINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESULTFINDINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESULTFINDINFO").field("psz", &self.psz).field("nStart", &self.nStart).field("dwOptions", &self.dwOptions).finish()
    }
}
impl ::windows::core::TypeKind for RESULTFINDINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RESULTFINDINFO {
    fn eq(&self, other: &Self) -> bool {
        self.psz == other.psz && self.nStart == other.nStart && self.dwOptions == other.dwOptions
    }
}
impl ::core::cmp::Eq for RESULTFINDINFO {}
impl ::core::default::Default for RESULTFINDINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct RESULT_VIEW_TYPE_INFO {
    pub pstrPersistableViewDescription: ::windows::core::PWSTR,
    pub eViewType: MMC_VIEW_TYPE,
    pub dwMiscOptions: u32,
    pub Anonymous: RESULT_VIEW_TYPE_INFO_0,
}
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for RESULT_VIEW_TYPE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RESULT_VIEW_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub union RESULT_VIEW_TYPE_INFO_0 {
    pub dwListOptions: u32,
    pub Anonymous1: RESULT_VIEW_TYPE_INFO_0_0,
    pub Anonymous2: ::std::mem::ManuallyDrop<RESULT_VIEW_TYPE_INFO_0_1>,
}
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::windows::core::TypeKind for RESULT_VIEW_TYPE_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for RESULT_VIEW_TYPE_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct RESULT_VIEW_TYPE_INFO_0_0 {
    pub dwHTMLOptions: u32,
    pub pstrURL: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for RESULT_VIEW_TYPE_INFO_0_0 {}
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RESULT_VIEW_TYPE_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESULT_VIEW_TYPE_INFO_0_0").field("dwHTMLOptions", &self.dwHTMLOptions).field("pstrURL", &self.pstrURL).finish()
    }
}
impl ::windows::core::TypeKind for RESULT_VIEW_TYPE_INFO_0_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RESULT_VIEW_TYPE_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwHTMLOptions == other.dwHTMLOptions && self.pstrURL == other.pstrURL
    }
}
impl ::core::cmp::Eq for RESULT_VIEW_TYPE_INFO_0_0 {}
impl ::core::default::Default for RESULT_VIEW_TYPE_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct RESULT_VIEW_TYPE_INFO_0_1 {
    pub dwOCXOptions: u32,
    pub pUnkControl: ::std::mem::ManuallyDrop<::core::option::Option<::windows::core::IUnknown>>,
}
impl ::core::clone::Clone for RESULT_VIEW_TYPE_INFO_0_1 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
impl ::core::fmt::Debug for RESULT_VIEW_TYPE_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RESULT_VIEW_TYPE_INFO_0_1").field("dwOCXOptions", &self.dwOCXOptions).field("pUnkControl", &self.pUnkControl).finish()
    }
}
impl ::windows::core::TypeKind for RESULT_VIEW_TYPE_INFO_0_1 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for RESULT_VIEW_TYPE_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwOCXOptions == other.dwOCXOptions && self.pUnkControl == other.pUnkControl
    }
}
impl ::core::cmp::Eq for RESULT_VIEW_TYPE_INFO_0_1 {}
impl ::core::default::Default for RESULT_VIEW_TYPE_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SCOPEDATAITEM {
    pub mask: u32,
    pub displayname: ::windows::core::PWSTR,
    pub nImage: i32,
    pub nOpenImage: i32,
    pub nState: u32,
    pub cChildren: i32,
    pub lParam: super::super::Foundation::LPARAM,
    pub relativeID: isize,
    pub ID: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SCOPEDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SCOPEDATAITEM {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SCOPEDATAITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCOPEDATAITEM").field("mask", &self.mask).field("displayname", &self.displayname).field("nImage", &self.nImage).field("nOpenImage", &self.nOpenImage).field("nState", &self.nState).field("cChildren", &self.cChildren).field("lParam", &self.lParam).field("relativeID", &self.relativeID).field("ID", &self.ID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for SCOPEDATAITEM {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SCOPEDATAITEM {
    fn eq(&self, other: &Self) -> bool {
        self.mask == other.mask && self.displayname == other.displayname && self.nImage == other.nImage && self.nOpenImage == other.nOpenImage && self.nState == other.nState && self.cChildren == other.cChildren && self.lParam == other.lParam && self.relativeID == other.relativeID && self.ID == other.ID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SCOPEDATAITEM {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SCOPEDATAITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct SColumnSetID {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SColumnSetID {}
impl ::core::clone::Clone for SColumnSetID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SColumnSetID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SColumnSetID").field("dwFlags", &self.dwFlags).field("cBytes", &self.cBytes).field("id", &self.id).finish()
    }
}
impl ::windows::core::TypeKind for SColumnSetID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SColumnSetID {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.cBytes == other.cBytes && self.id == other.id
    }
}
impl ::core::cmp::Eq for SColumnSetID {}
impl ::core::default::Default for SColumnSetID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct SMMCDataObjects {
    pub count: u32,
    pub lpDataObject: [::std::mem::ManuallyDrop<::core::option::Option<super::Com::IDataObject>>; 1usize],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for SMMCDataObjects {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for SMMCDataObjects {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMMCDataObjects").field("count", &self.count).field("lpDataObject", &self.lpDataObject).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::TypeKind for SMMCDataObjects {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for SMMCDataObjects {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.lpDataObject == other.lpDataObject
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for SMMCDataObjects {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for SMMCDataObjects {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct SMMCObjectTypes {
    pub count: u32,
    pub guid: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for SMMCObjectTypes {}
impl ::core::clone::Clone for SMMCObjectTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SMMCObjectTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SMMCObjectTypes").field("count", &self.count).field("guid", &self.guid).finish()
    }
}
impl ::windows::core::TypeKind for SMMCObjectTypes {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SMMCObjectTypes {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.guid == other.guid
    }
}
impl ::core::cmp::Eq for SMMCObjectTypes {}
impl ::core::default::Default for SMMCObjectTypes {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct SNodeID {
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SNodeID {}
impl ::core::clone::Clone for SNodeID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SNodeID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SNodeID").field("cBytes", &self.cBytes).field("id", &self.id).finish()
    }
}
impl ::windows::core::TypeKind for SNodeID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SNodeID {
    fn eq(&self, other: &Self) -> bool {
        self.cBytes == other.cBytes && self.id == other.id
    }
}
impl ::core::cmp::Eq for SNodeID {}
impl ::core::default::Default for SNodeID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Mmc\"`*"]
pub struct SNodeID2 {
    pub dwFlags: u32,
    pub cBytes: u32,
    pub id: [u8; 1],
}
impl ::core::marker::Copy for SNodeID2 {}
impl ::core::clone::Clone for SNodeID2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SNodeID2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SNodeID2").field("dwFlags", &self.dwFlags).field("cBytes", &self.cBytes).field("id", &self.id).finish()
    }
}
impl ::windows::core::TypeKind for SNodeID2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for SNodeID2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwFlags == other.dwFlags && self.cBytes == other.cBytes && self.id == other.id
    }
}
impl ::core::cmp::Eq for SNodeID2 {}
impl ::core::default::Default for SNodeID2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
