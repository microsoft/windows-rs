#[inline]
pub unsafe fn DevCloseObjectQuery(hdevquery: HDEVQUERY) {
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevCloseObjectQuery(hdevquery : HDEVQUERY));
    unsafe { DevCloseObjectQuery(hdevquery) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevCreateObjectQuery(objecttype: DEV_OBJECT_TYPE, queryflags: u32, prequestedproperties: Option<&[super::Properties::DEVPROPCOMPKEY]>, pfilter: Option<&[DEVPROP_FILTER_EXPRESSION]>, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVQUERY> {
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevCreateObjectQuery(objecttype : DEV_OBJECT_TYPE, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount : u32, pfilter : *const DEVPROP_FILTER_EXPRESSION, pcallback : PDEV_QUERY_RESULT_CALLBACK, pcontext : *const core::ffi::c_void, phdevquery : *mut HDEVQUERY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DevCreateObjectQuery(objecttype, queryflags, prequestedproperties.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prequestedproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())), pfilter.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pfilter.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcallback, pcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevCreateObjectQueryEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, prequestedproperties: Option<&[super::Properties::DEVPROPCOMPKEY]>, pfilter: Option<&[DEVPROP_FILTER_EXPRESSION]>, pextendedparameters: Option<&[DEV_QUERY_PARAMETER]>, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVQUERY> {
    windows_core::link!("api-ms-win-devices-query-l1-1-1.dll" "system" fn DevCreateObjectQueryEx(objecttype : DEV_OBJECT_TYPE, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount : u32, pfilter : *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount : u32, pextendedparameters : *const DEV_QUERY_PARAMETER, pcallback : PDEV_QUERY_RESULT_CALLBACK, pcontext : *const core::ffi::c_void, phdevquery : *mut HDEVQUERY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DevCreateObjectQueryEx(
            objecttype,
            queryflags,
            prequestedproperties.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(prequestedproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pfilter.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pfilter.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pextendedparameters.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pextendedparameters.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pcallback,
            pcontext.unwrap_or(core::mem::zeroed()) as _,
            &mut result__,
        )
        .map(|| result__)
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevCreateObjectQueryFromId<P1>(objecttype: DEV_OBJECT_TYPE, pszobjectid: P1, queryflags: u32, prequestedproperties: Option<&[super::Properties::DEVPROPCOMPKEY]>, pfilter: Option<&[DEVPROP_FILTER_EXPRESSION]>, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVQUERY>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevCreateObjectQueryFromId(objecttype : DEV_OBJECT_TYPE, pszobjectid : windows_core::PCWSTR, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount : u32, pfilter : *const DEVPROP_FILTER_EXPRESSION, pcallback : PDEV_QUERY_RESULT_CALLBACK, pcontext : *const core::ffi::c_void, phdevquery : *mut HDEVQUERY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DevCreateObjectQueryFromId(objecttype, pszobjectid.param().abi(), queryflags, prequestedproperties.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prequestedproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())), pfilter.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pfilter.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcallback, pcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIdEx<P1>(objecttype: DEV_OBJECT_TYPE, pszobjectid: P1, queryflags: u32, prequestedproperties: Option<&[super::Properties::DEVPROPCOMPKEY]>, pfilter: Option<&[DEVPROP_FILTER_EXPRESSION]>, pextendedparameters: Option<&[DEV_QUERY_PARAMETER]>, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVQUERY>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-devices-query-l1-1-1.dll" "system" fn DevCreateObjectQueryFromIdEx(objecttype : DEV_OBJECT_TYPE, pszobjectid : windows_core::PCWSTR, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount : u32, pfilter : *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount : u32, pextendedparameters : *const DEV_QUERY_PARAMETER, pcallback : PDEV_QUERY_RESULT_CALLBACK, pcontext : *const core::ffi::c_void, phdevquery : *mut HDEVQUERY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DevCreateObjectQueryFromIdEx(
            objecttype,
            pszobjectid.param().abi(),
            queryflags,
            prequestedproperties.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(prequestedproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pfilter.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pfilter.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pextendedparameters.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pextendedparameters.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pcallback,
            pcontext.unwrap_or(core::mem::zeroed()) as _,
            &mut result__,
        )
        .map(|| result__)
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIds<P1>(objecttype: DEV_OBJECT_TYPE, pszzobjectids: P1, queryflags: u32, prequestedproperties: Option<&[super::Properties::DEVPROPCOMPKEY]>, pfilter: Option<&[DEVPROP_FILTER_EXPRESSION]>, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVQUERY>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevCreateObjectQueryFromIds(objecttype : DEV_OBJECT_TYPE, pszzobjectids : windows_core::PCWSTR, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount : u32, pfilter : *const DEVPROP_FILTER_EXPRESSION, pcallback : PDEV_QUERY_RESULT_CALLBACK, pcontext : *const core::ffi::c_void, phdevquery : *mut HDEVQUERY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DevCreateObjectQueryFromIds(objecttype, pszzobjectids.param().abi(), queryflags, prequestedproperties.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prequestedproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())), pfilter.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pfilter.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcallback, pcontext.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIdsEx<P1>(objecttype: DEV_OBJECT_TYPE, pszzobjectids: P1, queryflags: u32, prequestedproperties: Option<&[super::Properties::DEVPROPCOMPKEY]>, pfilter: Option<&[DEVPROP_FILTER_EXPRESSION]>, pextendedparameters: Option<&[DEV_QUERY_PARAMETER]>, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: Option<*const core::ffi::c_void>) -> windows_core::Result<HDEVQUERY>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-devices-query-l1-1-1.dll" "system" fn DevCreateObjectQueryFromIdsEx(objecttype : DEV_OBJECT_TYPE, pszzobjectids : windows_core::PCWSTR, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount : u32, pfilter : *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount : u32, pextendedparameters : *const DEV_QUERY_PARAMETER, pcallback : PDEV_QUERY_RESULT_CALLBACK, pcontext : *const core::ffi::c_void, phdevquery : *mut HDEVQUERY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        DevCreateObjectQueryFromIdsEx(
            objecttype,
            pszzobjectids.param().abi(),
            queryflags,
            prequestedproperties.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(prequestedproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pfilter.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pfilter.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pextendedparameters.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pextendedparameters.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pcallback,
            pcontext.unwrap_or(core::mem::zeroed()) as _,
            &mut result__,
        )
        .map(|| result__)
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevFindProperty<P2>(pkey: *const super::super::Foundation::DEVPROPKEY, store: super::Properties::DEVPROPSTORE, pszlocalename: P2, pproperties: Option<&[super::Properties::DEVPROPERTY]>) -> *mut super::Properties::DEVPROPERTY
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevFindProperty(pkey : *const super::super::Foundation::DEVPROPKEY, store : super::Properties::DEVPROPSTORE, pszlocalename : windows_core::PCWSTR, cproperties : u32, pproperties : *const super::Properties::DEVPROPERTY) -> *mut super::Properties::DEVPROPERTY);
    unsafe { DevFindProperty(pkey, store, pszlocalename.param().abi(), pproperties.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pproperties.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevFreeObjectProperties(pproperties: &[super::Properties::DEVPROPERTY]) {
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevFreeObjectProperties(cpropertycount : u32, pproperties : *const super::Properties::DEVPROPERTY));
    unsafe { DevFreeObjectProperties(pproperties.len().try_into().unwrap(), core::mem::transmute(pproperties.as_ptr())) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevFreeObjects(pobjects: &[DEV_OBJECT]) {
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevFreeObjects(cobjectcount : u32, pobjects : *const DEV_OBJECT));
    unsafe { DevFreeObjects(pobjects.len().try_into().unwrap(), core::mem::transmute(pobjects.as_ptr())) }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevGetObjectProperties<P1>(objecttype: DEV_OBJECT_TYPE, pszobjectid: P1, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevGetObjectProperties(objecttype : DEV_OBJECT_TYPE, pszobjectid : windows_core::PCWSTR, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, pcpropertycount : *mut u32, ppproperties : *mut *mut super::Properties::DEVPROPERTY) -> windows_core::HRESULT);
    unsafe { DevGetObjectProperties(objecttype, pszobjectid.param().abi(), queryflags, prequestedproperties.len().try_into().unwrap(), core::mem::transmute(prequestedproperties.as_ptr()), pcpropertycount as _, ppproperties as _).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevGetObjectPropertiesEx<P1>(objecttype: DEV_OBJECT_TYPE, pszobjectid: P1, queryflags: u32, prequestedproperties: &[super::Properties::DEVPROPCOMPKEY], pextendedparameters: Option<&[DEV_QUERY_PARAMETER]>, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> windows_core::Result<()>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-devices-query-l1-1-1.dll" "system" fn DevGetObjectPropertiesEx(objecttype : DEV_OBJECT_TYPE, pszobjectid : windows_core::PCWSTR, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cextendedparametercount : u32, pextendedparameters : *const DEV_QUERY_PARAMETER, pcpropertycount : *mut u32, ppproperties : *mut *mut super::Properties::DEVPROPERTY) -> windows_core::HRESULT);
    unsafe { DevGetObjectPropertiesEx(objecttype, pszobjectid.param().abi(), queryflags, prequestedproperties.len().try_into().unwrap(), core::mem::transmute(prequestedproperties.as_ptr()), pextendedparameters.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pextendedparameters.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcpropertycount as _, ppproperties as _).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevGetObjects(objecttype: DEV_OBJECT_TYPE, queryflags: u32, prequestedproperties: Option<&[super::Properties::DEVPROPCOMPKEY]>, pfilter: Option<&[DEVPROP_FILTER_EXPRESSION]>, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-devices-query-l1-1-0.dll" "system" fn DevGetObjects(objecttype : DEV_OBJECT_TYPE, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount : u32, pfilter : *const DEVPROP_FILTER_EXPRESSION, pcobjectcount : *mut u32, ppobjects : *mut *mut DEV_OBJECT) -> windows_core::HRESULT);
    unsafe { DevGetObjects(objecttype, queryflags, prequestedproperties.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(prequestedproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())), pfilter.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pfilter.map_or(core::ptr::null(), |slice| slice.as_ptr())), pcobjectcount as _, ppobjects as _).ok() }
}
#[cfg(feature = "Win32_Devices_Properties")]
#[inline]
pub unsafe fn DevGetObjectsEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, prequestedproperties: Option<&[super::Properties::DEVPROPCOMPKEY]>, pfilter: Option<&[DEVPROP_FILTER_EXPRESSION]>, pextendedparameters: Option<&[DEV_QUERY_PARAMETER]>, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> windows_core::Result<()> {
    windows_core::link!("api-ms-win-devices-query-l1-1-1.dll" "system" fn DevGetObjectsEx(objecttype : DEV_OBJECT_TYPE, queryflags : u32, crequestedproperties : u32, prequestedproperties : *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount : u32, pfilter : *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount : u32, pextendedparameters : *const DEV_QUERY_PARAMETER, pcobjectcount : *mut u32, ppobjects : *mut *mut DEV_OBJECT) -> windows_core::HRESULT);
    unsafe {
        DevGetObjectsEx(
            objecttype,
            queryflags,
            prequestedproperties.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(prequestedproperties.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pfilter.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pfilter.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pextendedparameters.map_or(0, |slice| slice.len().try_into().unwrap()),
            core::mem::transmute(pextendedparameters.map_or(core::ptr::null(), |slice| slice.as_ptr())),
            pcobjectcount as _,
            ppobjects as _,
        )
        .ok()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVPROP_FILTER_EXPRESSION {
    pub Operator: DEVPROP_OPERATOR,
    pub Property: super::Properties::DEVPROPERTY,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEVPROP_OPERATOR(pub u32);
impl DEVPROP_OPERATOR {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for DEVPROP_OPERATOR {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for DEVPROP_OPERATOR {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for DEVPROP_OPERATOR {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0);
    }
}
impl core::ops::BitAndAssign for DEVPROP_OPERATOR {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0);
    }
}
impl core::ops::Not for DEVPROP_OPERATOR {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const DEVPROP_OPERATOR_AND_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(2097152);
pub const DEVPROP_OPERATOR_AND_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(1048576);
pub const DEVPROP_OPERATOR_ARRAY_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(268435456);
pub const DEVPROP_OPERATOR_BEGINS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(9);
pub const DEVPROP_OPERATOR_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131081);
pub const DEVPROP_OPERATOR_BITWISE_AND: DEVPROP_OPERATOR = DEVPROP_OPERATOR(7);
pub const DEVPROP_OPERATOR_BITWISE_OR: DEVPROP_OPERATOR = DEVPROP_OPERATOR(8);
pub const DEVPROP_OPERATOR_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(11);
pub const DEVPROP_OPERATOR_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131083);
pub const DEVPROP_OPERATOR_ENDS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(10);
pub const DEVPROP_OPERATOR_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131082);
pub const DEVPROP_OPERATOR_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(2);
pub const DEVPROP_OPERATOR_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131074);
pub const DEVPROP_OPERATOR_EXISTS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(1);
pub const DEVPROP_OPERATOR_GREATER_THAN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(3);
pub const DEVPROP_OPERATOR_GREATER_THAN_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(5);
pub const DEVPROP_OPERATOR_LESS_THAN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4);
pub const DEVPROP_OPERATOR_LESS_THAN_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(6);
pub const DEVPROP_OPERATOR_LIST_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4096);
pub const DEVPROP_OPERATOR_LIST_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(135168);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(8192);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(139264);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(16384);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(147456);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(12288);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(143360);
pub const DEVPROP_OPERATOR_MASK_ARRAY: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4026531840);
pub const DEVPROP_OPERATOR_MASK_EVAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4095);
pub const DEVPROP_OPERATOR_MASK_LIST: DEVPROP_OPERATOR = DEVPROP_OPERATOR(61440);
pub const DEVPROP_OPERATOR_MASK_LOGICAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(267386880);
pub const DEVPROP_OPERATOR_MASK_MODIFIER: DEVPROP_OPERATOR = DEVPROP_OPERATOR(983040);
pub const DEVPROP_OPERATOR_MASK_NOT_LOGICAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4027580415);
pub const DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131072);
pub const DEVPROP_OPERATOR_MODIFIER_NOT: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65536);
pub const DEVPROP_OPERATOR_NONE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(0);
pub const DEVPROP_OPERATOR_NOT_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(6291456);
pub const DEVPROP_OPERATOR_NOT_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65538);
pub const DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(196610);
pub const DEVPROP_OPERATOR_NOT_EXISTS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65537);
pub const DEVPROP_OPERATOR_NOT_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(5242880);
pub const DEVPROP_OPERATOR_OR_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4194304);
pub const DEVPROP_OPERATOR_OR_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(3145728);
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEV_OBJECT {
    pub ObjectType: DEV_OBJECT_TYPE,
    pub pszObjectId: windows_core::PCWSTR,
    pub cPropertyCount: u32,
    pub pProperties: *const super::Properties::DEVPROPERTY,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEV_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEV_OBJECT_TYPE(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEV_QUERY_FLAGS(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEV_QUERY_PARAMETER {
    pub Key: super::super::Foundation::DEVPROPKEY,
    pub Type: super::Properties::DEVPROPTYPE,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEV_QUERY_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEV_QUERY_RESULT_ACTION(pub i32);
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy)]
pub struct DEV_QUERY_RESULT_ACTION_DATA {
    pub Action: DEV_QUERY_RESULT_ACTION,
    pub Data: DEV_QUERY_RESULT_ACTION_DATA_0,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEV_QUERY_RESULT_ACTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Devices_Properties")]
#[derive(Clone, Copy)]
pub union DEV_QUERY_RESULT_ACTION_DATA_0 {
    pub State: DEV_QUERY_STATE,
    pub DeviceObject: DEV_OBJECT,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl Default for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DEV_QUERY_STATE(pub i32);
pub const DevObjectTypeAEP: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(5);
pub const DevObjectTypeAEPContainer: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(6);
pub const DevObjectTypeAEPProtocol: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(12);
pub const DevObjectTypeAEPService: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(10);
pub const DevObjectTypeDevice: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(3);
pub const DevObjectTypeDeviceContainer: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(2);
pub const DevObjectTypeDeviceContainerDisplay: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(9);
pub const DevObjectTypeDeviceInstallerClass: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(7);
pub const DevObjectTypeDeviceInterface: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(1);
pub const DevObjectTypeDeviceInterfaceClass: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(4);
pub const DevObjectTypeDeviceInterfaceDisplay: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(8);
pub const DevObjectTypeDevicePanel: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(11);
pub const DevObjectTypeUnknown: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(0);
pub const DevQueryFlagAllProperties: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(2);
pub const DevQueryFlagAsyncClose: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(8);
pub const DevQueryFlagLocalize: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(4);
pub const DevQueryFlagNone: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(0);
pub const DevQueryFlagUpdateResults: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(1);
pub const DevQueryResultAdd: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(1);
pub const DevQueryResultRemove: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(3);
pub const DevQueryResultStateChange: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(0);
pub const DevQueryResultUpdate: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(2);
pub const DevQueryStateAborted: DEV_QUERY_STATE = DEV_QUERY_STATE(2);
pub const DevQueryStateClosed: DEV_QUERY_STATE = DEV_QUERY_STATE(3);
pub const DevQueryStateEnumCompleted: DEV_QUERY_STATE = DEV_QUERY_STATE(1);
pub const DevQueryStateInitialized: DEV_QUERY_STATE = DEV_QUERY_STATE(0);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HDEVQUERY(pub *mut core::ffi::c_void);
impl HDEVQUERY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HDEVQUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
pub type PDEV_QUERY_RESULT_CALLBACK = Option<unsafe extern "system" fn(hdevquery: HDEVQUERY, pcontext: *const core::ffi::c_void, pactiondata: *const DEV_QUERY_RESULT_ACTION_DATA)>;
