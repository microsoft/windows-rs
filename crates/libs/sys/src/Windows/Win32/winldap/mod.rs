windows_link::link!("wldap32.dll" "C" fn LdapGetLastError() -> u32);
windows_link::link!("wldap32.dll" "C" fn LdapMapErrorToWin32(ldaperror : u32) -> u32);
windows_link::link!("wldap32.dll" "C" fn LdapUTF8ToUnicode(lpsrcstr : windows_sys::core::PCSTR, cchsrc : i32, lpdeststr : windows_sys::core::PWSTR, cchdest : i32) -> i32);
windows_link::link!("wldap32.dll" "C" fn LdapUnicodeToUTF8(lpsrcstr : windows_sys::core::PCWSTR, cchsrc : i32, lpdeststr : windows_sys::core::PSTR, cchdest : i32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ber_bvfree(bv : *mut LDAP_BERVAL));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn cldap_open(hostname : windows_sys::core::PCSTR, portnumber : u32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn cldap_openA(hostname : windows_sys::core::PCSTR, portnumber : u32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn cldap_openW(hostname : windows_sys::core::PCWSTR, portnumber : u32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_abandon(ld : *mut LDAP, msgid : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attrs : *mut *mut LDAPModA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_addA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attrs : *mut *mut LDAPModA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_addW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, attrs : *mut *mut LDAPModW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_ext(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attrs : *mut *mut LDAPModA, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_extA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attrs : *mut *mut LDAPModA, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_extW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, attrs : *mut *mut LDAPModW, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_ext_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attrs : *mut *mut LDAPModA, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_ext_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attrs : *mut *mut LDAPModA, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_ext_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, attrs : *mut *mut LDAPModW, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attrs : *mut *mut LDAPModA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attrs : *mut *mut LDAPModA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_add_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, attrs : *mut *mut LDAPModW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_bind(ld : *mut LDAP, dn : windows_sys::core::PCSTR, cred : *const i8, method : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_bindA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, cred : *const i8, method : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_bindW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, cred : *const u16, method : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_bind_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, cred : *const i8, method : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_bind_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, cred : *const i8, method : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_bind_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, cred : *const u16, method : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_check_filterA(ld : *mut LDAP, searchfilter : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_check_filterW(ld : *mut LDAP, searchfilter : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_cleanup(hinstance : super::winnt::HANDLE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_close_extended_op(ld : *mut LDAP, messagenumber : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attr : windows_sys::core::PCSTR, value : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compareA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attr : windows_sys::core::PCSTR, value : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compareW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, attr : windows_sys::core::PCWSTR, value : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_ext(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attr : windows_sys::core::PCSTR, value : windows_sys::core::PCSTR, data : *mut LDAP_BERVAL, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_extA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attr : windows_sys::core::PCSTR, value : windows_sys::core::PCSTR, data : *const LDAP_BERVAL, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_extW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, attr : windows_sys::core::PCWSTR, value : windows_sys::core::PCWSTR, data : *const LDAP_BERVAL, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_ext_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attr : windows_sys::core::PCSTR, value : windows_sys::core::PCSTR, data : *mut LDAP_BERVAL, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_ext_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attr : windows_sys::core::PCSTR, value : windows_sys::core::PCSTR, data : *const LDAP_BERVAL, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_ext_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, attr : windows_sys::core::PCWSTR, value : windows_sys::core::PCWSTR, data : *const LDAP_BERVAL, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attr : windows_sys::core::PCSTR, value : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, attr : windows_sys::core::PCSTR, value : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_compare_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, attr : windows_sys::core::PCWSTR, value : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_conn_from_msg(primaryconn : *mut LDAP, res : *mut LDAPMessage) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_connect(ld : *mut LDAP, timeout : *mut LDAP_TIMEVAL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_control_free(control : *mut LDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_control_freeA(controls : *mut LDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_control_freeW(control : *mut LDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_controls_free(controls : *mut *mut LDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_controls_freeA(controls : *mut *mut LDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_controls_freeW(control : *mut *mut LDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_count_entries(ld : *mut LDAP, res : *mut LDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_count_references(ld : *mut LDAP, res : *mut LDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_count_values(vals : *const super::winnt::PCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_count_valuesA(vals : *const super::winnt::PCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_count_valuesW(vals : *const super::winnt::PWCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_count_values_len(vals : *mut *mut LDAP_BERVAL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_create_page_control(externalhandle : *mut LDAP, pagesize : u32, cookie : *mut LDAP_BERVAL, iscritical : u8, control : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_create_page_controlA(externalhandle : *mut LDAP, pagesize : u32, cookie : *mut LDAP_BERVAL, iscritical : u8, control : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_create_page_controlW(externalhandle : *mut LDAP, pagesize : u32, cookie : *mut LDAP_BERVAL, iscritical : u8, control : *mut PLDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_create_sort_control(externalhandle : *mut LDAP, sortkeys : *mut PLDAPSortKeyA, iscritical : u8, control : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_create_sort_controlA(externalhandle : *mut LDAP, sortkeys : *mut PLDAPSortKeyA, iscritical : u8, control : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_create_sort_controlW(externalhandle : *mut LDAP, sortkeys : *mut PLDAPSortKeyW, iscritical : u8, control : *mut PLDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_create_vlv_controlA(externalhandle : *mut LDAP, vlvinfo : *mut LDAPVLVInfo, iscritical : u8, control : *mut PLDAPControlA) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_create_vlv_controlW(externalhandle : *mut LDAP, vlvinfo : *mut LDAPVLVInfo, iscritical : u8, control : *mut PLDAPControlW) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete(ld : *mut LDAP, dn : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_deleteA(ld : *mut LDAP, dn : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_deleteW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_ext(ld : *mut LDAP, dn : windows_sys::core::PCSTR, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_extA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_extW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_ext_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_ext_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_ext_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_delete_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_dn2ufn(dn : windows_sys::core::PCSTR) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_dn2ufnA(dn : windows_sys::core::PCSTR) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_dn2ufnW(dn : windows_sys::core::PCWSTR) -> super::winnt::PWCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_encode_sort_controlA(externalhandle : *mut LDAP, sortkeys : *mut PLDAPSortKeyA, control : *mut LDAPControlA, criticality : bool) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_encode_sort_controlW(externalhandle : *mut LDAP, sortkeys : *mut PLDAPSortKeyW, control : *mut LDAPControlW, criticality : bool) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_err2string(err : u32) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_err2stringA(err : u32) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_err2stringW(err : u32) -> super::winnt::PWCHAR);
windows_link::link!("wldap32.dll" "C" fn ldap_escape_filter_element(sourcefilterelement : *const i8, sourcelength : u32, destfilterelement : *mut i8, destlength : u32) -> u32);
windows_link::link!("wldap32.dll" "C" fn ldap_escape_filter_elementA(sourcefilterelement : *const i8, sourcelength : u32, destfilterelement : *mut i8, destlength : u32) -> u32);
windows_link::link!("wldap32.dll" "C" fn ldap_escape_filter_elementW(sourcefilterelement : *const i8, sourcelength : u32, destfilterelement : *mut u16, destlength : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_explode_dn(dn : windows_sys::core::PCSTR, notypes : u32) -> *mut super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_explode_dnA(dn : windows_sys::core::PCSTR, notypes : u32) -> *mut super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_explode_dnW(dn : windows_sys::core::PCWSTR, notypes : u32) -> *mut super::winnt::PWCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_extended_operation(ld : *mut LDAP, oid : windows_sys::core::PCSTR, data : *mut LDAP_BERVAL, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_extended_operationA(ld : *mut LDAP, oid : windows_sys::core::PCSTR, data : *mut LDAP_BERVAL, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_extended_operationW(ld : *mut LDAP, oid : windows_sys::core::PCWSTR, data : *mut LDAP_BERVAL, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_extended_operation_sA(externalhandle : *mut LDAP, oid : windows_sys::core::PCSTR, data : *mut LDAP_BERVAL, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, returnedoid : *mut super::winnt::PCHAR, returneddata : *mut *mut LDAP_BERVAL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_extended_operation_sW(externalhandle : *mut LDAP, oid : windows_sys::core::PCWSTR, data : *mut LDAP_BERVAL, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW, returnedoid : *mut super::winnt::PWCHAR, returneddata : *mut *mut LDAP_BERVAL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_first_attribute(ld : *mut LDAP, entry : *mut LDAPMessage, ptr : *mut *mut BerElement) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_first_attributeA(ld : *mut LDAP, entry : *mut LDAPMessage, ptr : *mut *mut BerElement) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_first_attributeW(ld : *mut LDAP, entry : *mut LDAPMessage, ptr : *mut *mut BerElement) -> super::winnt::PWCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_first_entry(ld : *mut LDAP, res : *mut LDAPMessage) -> *mut LDAPMessage);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_first_reference(ld : *mut LDAP, res : *mut LDAPMessage) -> *mut LDAPMessage);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_free_controls(controls : *mut *mut LDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_free_controlsA(controls : *mut *mut LDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_free_controlsW(controls : *mut *mut LDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_dn(ld : *mut LDAP, entry : *mut LDAPMessage) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_dnA(ld : *mut LDAP, entry : *mut LDAPMessage) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_dnW(ld : *mut LDAP, entry : *mut LDAPMessage) -> super::winnt::PWCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_next_page(externalhandle : *mut LDAP, searchhandle : *mut ldapsearch, pagesize : u32, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_next_page_s(externalhandle : *mut LDAP, searchhandle : *mut ldapsearch, timeout : *mut LDAP_TIMEVAL, pagesize : u32, totalcount : *mut u32, results : *mut *mut LDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_option(ld : *mut LDAP, option : i32, outvalue : *mut core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_optionW(ld : *mut LDAP, option : i32, outvalue : *mut core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_paged_count(externalhandle : *mut LDAP, searchblock : *mut ldapsearch, totalcount : *mut u32, results : *mut LDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_values(ld : *mut LDAP, entry : *mut LDAPMessage, attr : windows_sys::core::PCSTR) -> *mut super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_valuesA(ld : *mut LDAP, entry : *mut LDAPMessage, attr : windows_sys::core::PCSTR) -> *mut super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_valuesW(ld : *mut LDAP, entry : *mut LDAPMessage, attr : windows_sys::core::PCWSTR) -> *mut super::winnt::PWCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_values_len(externalhandle : *mut LDAP, message : *mut LDAPMessage, attr : windows_sys::core::PCSTR) -> *mut *mut LDAP_BERVAL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_values_lenA(externalhandle : *mut LDAP, message : *mut LDAPMessage, attr : windows_sys::core::PCSTR) -> *mut *mut LDAP_BERVAL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_get_values_lenW(externalhandle : *mut LDAP, message : *mut LDAPMessage, attr : windows_sys::core::PCWSTR) -> *mut *mut LDAP_BERVAL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_init(hostname : windows_sys::core::PCSTR, portnumber : u32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_initA(hostname : windows_sys::core::PCSTR, portnumber : u32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_initW(hostname : windows_sys::core::PCWSTR, portnumber : u32) -> *mut LDAP);
windows_link::link!("wldap32.dll" "C" fn ldap_memfree(block : *const i8));
windows_link::link!("wldap32.dll" "C" fn ldap_memfreeA(block : *const i8));
windows_link::link!("wldap32.dll" "C" fn ldap_memfreeW(block : *const u16));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify(ld : *mut LDAP, dn : windows_sys::core::PCSTR, mods : *mut *mut LDAPModA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modifyA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, mods : *mut *mut LDAPModA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modifyW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, mods : *mut *mut LDAPModW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_ext(ld : *mut LDAP, dn : windows_sys::core::PCSTR, mods : *mut *mut LDAPModA, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_extA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, mods : *mut *mut LDAPModA, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_extW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, mods : *mut *mut LDAPModW, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_ext_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, mods : *mut *mut LDAPModA, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_ext_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, mods : *mut *mut LDAPModA, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_ext_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, mods : *mut *mut LDAPModW, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, mods : *mut *mut LDAPModA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, mods : *mut *mut LDAPModA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modify_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, mods : *mut *mut LDAPModW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, newdistinguishedname : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn2(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, newdistinguishedname : windows_sys::core::PCSTR, deleteoldrdn : i32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn2A(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, newdistinguishedname : windows_sys::core::PCSTR, deleteoldrdn : i32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn2W(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCWSTR, newdistinguishedname : windows_sys::core::PCWSTR, deleteoldrdn : i32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn2_s(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, newdistinguishedname : windows_sys::core::PCSTR, deleteoldrdn : i32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn2_sA(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, newdistinguishedname : windows_sys::core::PCSTR, deleteoldrdn : i32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn2_sW(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCWSTR, newdistinguishedname : windows_sys::core::PCWSTR, deleteoldrdn : i32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdnA(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, newdistinguishedname : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdnW(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCWSTR, newdistinguishedname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn_s(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, newdistinguishedname : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn_sA(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, newdistinguishedname : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_modrdn_sW(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCWSTR, newdistinguishedname : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_msgfree(res : *mut LDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_next_attribute(ld : *mut LDAP, entry : *mut LDAPMessage, ptr : *mut BerElement) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_next_attributeA(ld : *mut LDAP, entry : *mut LDAPMessage, ptr : *mut BerElement) -> super::winnt::PCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_next_attributeW(ld : *mut LDAP, entry : *mut LDAPMessage, ptr : *mut BerElement) -> super::winnt::PWCHAR);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_next_entry(ld : *mut LDAP, entry : *mut LDAPMessage) -> *mut LDAPMessage);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_next_reference(ld : *mut LDAP, entry : *mut LDAPMessage) -> *mut LDAPMessage);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_open(hostname : windows_sys::core::PCSTR, portnumber : u32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_openA(hostname : windows_sys::core::PCSTR, portnumber : u32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_openW(hostname : windows_sys::core::PCWSTR, portnumber : u32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_extended_resultA(connection : *mut LDAP, resultmessage : *mut LDAPMessage, resultoid : *mut windows_sys::core::PSTR, resultdata : *mut *mut LDAP_BERVAL, freeit : bool) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_extended_resultW(connection : *mut LDAP, resultmessage : *mut LDAPMessage, resultoid : *mut windows_sys::core::PWSTR, resultdata : *mut *mut LDAP_BERVAL, freeit : bool) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_page_control(externalhandle : *mut LDAP, servercontrols : *mut PLDAPControlA, totalcount : *mut u32, cookie : *mut *mut LDAP_BERVAL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_page_controlA(externalhandle : *mut LDAP, servercontrols : *mut PLDAPControlA, totalcount : *mut u32, cookie : *mut *mut LDAP_BERVAL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_page_controlW(externalhandle : *mut LDAP, servercontrols : *mut PLDAPControlW, totalcount : *mut u32, cookie : *mut *mut LDAP_BERVAL) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_reference(connection : *mut LDAP, resultmessage : *mut LDAPMessage, referrals : *mut *mut super::winnt::PCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_referenceA(connection : *mut LDAP, resultmessage : *mut LDAPMessage, referrals : *mut *mut super::winnt::PCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_referenceW(connection : *mut LDAP, resultmessage : *mut LDAPMessage, referrals : *mut *mut super::winnt::PWCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_result(connection : *mut LDAP, resultmessage : *mut LDAPMessage, returncode : *mut u32, matcheddns : *mut windows_sys::core::PSTR, errormessage : *mut windows_sys::core::PSTR, referrals : *mut *mut windows_sys::core::PSTR, servercontrols : *mut *mut PLDAPControlA, freeit : bool) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_resultA(connection : *mut LDAP, resultmessage : *mut LDAPMessage, returncode : *mut u32, matcheddns : *mut windows_sys::core::PSTR, errormessage : *mut windows_sys::core::PSTR, referrals : *mut super::winnt::PZPSTR, servercontrols : *mut *mut PLDAPControlA, freeit : bool) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_resultW(connection : *mut LDAP, resultmessage : *mut LDAPMessage, returncode : *mut u32, matcheddns : *mut windows_sys::core::PWSTR, errormessage : *mut windows_sys::core::PWSTR, referrals : *mut super::winnt::PZPWSTR, servercontrols : *mut *mut PLDAPControlW, freeit : bool) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_sort_control(externalhandle : *mut LDAP, control : *mut PLDAPControlA, result : *mut u32, attribute : *mut super::winnt::PCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_sort_controlA(externalhandle : *mut LDAP, control : *mut PLDAPControlA, result : *mut u32, attribute : *mut super::winnt::PCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_sort_controlW(externalhandle : *mut LDAP, control : *mut PLDAPControlW, result : *mut u32, attribute : *mut super::winnt::PWCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_vlv_controlA(externalhandle : *mut LDAP, control : *mut PLDAPControlA, targetpos : *mut u32, listcount : *mut u32, context : *mut PBERVAL, errcode : *mut i32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_parse_vlv_controlW(externalhandle : *mut LDAP, control : *mut PLDAPControlW, targetpos : *mut u32, listcount : *mut u32, context : *mut PBERVAL, errcode : *mut i32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_perror(ld : *mut LDAP, msg : *const i8));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_rename_ext(ld : *mut LDAP, dn : windows_sys::core::PCSTR, newrdn : windows_sys::core::PCSTR, newparent : windows_sys::core::PCSTR, deleteoldrdn : i32, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_rename_extA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, newrdn : windows_sys::core::PCSTR, newparent : windows_sys::core::PCSTR, deleteoldrdn : i32, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_rename_extW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, newrdn : windows_sys::core::PCWSTR, newparent : windows_sys::core::PCWSTR, deleteoldrdn : i32, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_rename_ext_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, newrdn : windows_sys::core::PCSTR, newparent : windows_sys::core::PCSTR, deleteoldrdn : i32, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_rename_ext_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, newrdn : windows_sys::core::PCSTR, newparent : windows_sys::core::PCSTR, deleteoldrdn : i32, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_rename_ext_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, newrdn : windows_sys::core::PCWSTR, newparent : windows_sys::core::PCWSTR, deleteoldrdn : i32, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_result(ld : *mut LDAP, msgid : u32, all : u32, timeout : *const LDAP_TIMEVAL, res : *mut PLDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_result2error(ld : *mut LDAP, res : *mut LDAPMessage, freeit : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_sasl_bindA(externalhandle : *mut LDAP, distname : windows_sys::core::PCSTR, authmechanism : windows_sys::core::PCSTR, cred : *const BERVAL, serverctrls : *mut PLDAPControlA, clientctrls : *mut PLDAPControlA, messagenumber : *mut i32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_sasl_bindW(externalhandle : *mut LDAP, distname : windows_sys::core::PCWSTR, authmechanism : windows_sys::core::PCWSTR, cred : *const BERVAL, serverctrls : *mut PLDAPControlW, clientctrls : *mut PLDAPControlW, messagenumber : *mut i32) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_sasl_bind_sA(externalhandle : *mut LDAP, distname : windows_sys::core::PCSTR, authmechanism : windows_sys::core::PCSTR, cred : *const BERVAL, serverctrls : *mut PLDAPControlA, clientctrls : *mut PLDAPControlA, serverdata : *mut PBERVAL) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_sasl_bind_sW(externalhandle : *mut LDAP, distname : windows_sys::core::PCWSTR, authmechanism : windows_sys::core::PCWSTR, cred : *const BERVAL, serverctrls : *mut PLDAPControlW, clientctrls : *mut PLDAPControlW, serverdata : *mut PBERVAL) -> i32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_searchA(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_searchW(ld : *mut LDAP, base : windows_sys::core::PCWSTR, scope : u32, filter : windows_sys::core::PCWSTR, attrs : *const windows_sys::core::PCWSTR, attrsonly : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_abandon_page(externalhandle : *mut LDAP, searchblock : *mut ldapsearch) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_ext(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32, servercontrols : *const PLDAPControlA, clientcontrols : *const PLDAPControlA, timelimit : u32, sizelimit : u32, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_extA(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32, servercontrols : *const PLDAPControlA, clientcontrols : *const PLDAPControlA, timelimit : u32, sizelimit : u32, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_extW(ld : *mut LDAP, base : windows_sys::core::PCWSTR, scope : u32, filter : windows_sys::core::PCWSTR, attrs : *const windows_sys::core::PCWSTR, attrsonly : u32, servercontrols : *const PLDAPControlW, clientcontrols : *const PLDAPControlW, timelimit : u32, sizelimit : u32, messagenumber : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_ext_s(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32, servercontrols : *const PLDAPControlA, clientcontrols : *const PLDAPControlA, timeout : *mut LDAP_TIMEVAL, sizelimit : u32, res : *mut PLDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_ext_sA(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32, servercontrols : *const PLDAPControlA, clientcontrols : *const PLDAPControlA, timeout : *mut LDAP_TIMEVAL, sizelimit : u32, res : *mut PLDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_ext_sW(ld : *mut LDAP, base : windows_sys::core::PCWSTR, scope : u32, filter : windows_sys::core::PCWSTR, attrs : *const windows_sys::core::PCWSTR, attrsonly : u32, servercontrols : *const PLDAPControlW, clientcontrols : *const PLDAPControlW, timeout : *mut LDAP_TIMEVAL, sizelimit : u32, res : *mut PLDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_init_page(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, scopeofsearch : u32, searchfilter : windows_sys::core::PCSTR, attributelist : *mut windows_sys::core::PSTR, attributesonly : u32, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, pagetimelimit : u32, totalsizelimit : u32, sortkeys : *mut PLDAPSortKeyA) -> PLDAPSearch);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_init_pageA(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCSTR, scopeofsearch : u32, searchfilter : windows_sys::core::PCSTR, attributelist : *const windows_sys::core::PCSTR, attributesonly : u32, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA, pagetimelimit : u32, totalsizelimit : u32, sortkeys : *mut PLDAPSortKeyA) -> PLDAPSearch);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_init_pageW(externalhandle : *mut LDAP, distinguishedname : windows_sys::core::PCWSTR, scopeofsearch : u32, searchfilter : windows_sys::core::PCWSTR, attributelist : *const windows_sys::core::PCWSTR, attributesonly : u32, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW, pagetimelimit : u32, totalsizelimit : u32, sortkeys : *mut PLDAPSortKeyW) -> PLDAPSearch);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_s(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32, res : *mut PLDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_sA(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32, res : *mut *mut LDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_sW(ld : *mut LDAP, base : windows_sys::core::PCWSTR, scope : u32, filter : windows_sys::core::PCWSTR, attrs : *const windows_sys::core::PCWSTR, attrsonly : u32, res : *mut *mut LDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_st(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32, timeout : *mut LDAP_TIMEVAL, res : *mut PLDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_stA(ld : *mut LDAP, base : windows_sys::core::PCSTR, scope : u32, filter : windows_sys::core::PCSTR, attrs : *const windows_sys::core::PCSTR, attrsonly : u32, timeout : *mut LDAP_TIMEVAL, res : *mut PLDAPMessage) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_search_stW(ld : *mut LDAP, base : windows_sys::core::PCWSTR, scope : u32, filter : windows_sys::core::PCWSTR, attrs : *const windows_sys::core::PCWSTR, attrsonly : u32, timeout : *mut LDAP_TIMEVAL, res : *mut PLDAPMessage) -> u32);
windows_link::link!("wldap32.dll" "C" fn ldap_set_dbg_flags(newflags : u32) -> u32);
windows_link::link!("wldap32.dll" "C" fn ldap_set_dbg_routine(debugprintroutine : DBGPRINT));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_set_option(ld : *mut LDAP, option : i32, invalue : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_set_optionW(ld : *mut LDAP, option : i32, invalue : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_simple_bind(ld : *mut LDAP, dn : windows_sys::core::PCSTR, passwd : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_simple_bindA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, passwd : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_simple_bindW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, passwd : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_simple_bind_s(ld : *mut LDAP, dn : windows_sys::core::PCSTR, passwd : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_simple_bind_sA(ld : *mut LDAP, dn : windows_sys::core::PCSTR, passwd : windows_sys::core::PCSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_simple_bind_sW(ld : *mut LDAP, dn : windows_sys::core::PCWSTR, passwd : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_sslinit(hostname : windows_sys::core::PCSTR, portnumber : u32, secure : i32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_sslinitA(hostname : windows_sys::core::PCSTR, portnumber : u32, secure : i32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_sslinitW(hostname : windows_sys::core::PCWSTR, portnumber : u32, secure : i32) -> *mut LDAP);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_start_tls_sA(externalhandle : *mut LDAP, serverreturnvalue : *mut u32, result : *mut *mut LDAPMessage, servercontrols : *mut PLDAPControlA, clientcontrols : *mut PLDAPControlA) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_start_tls_sW(externalhandle : *mut LDAP, serverreturnvalue : *mut u32, result : *mut *mut LDAPMessage, servercontrols : *mut PLDAPControlW, clientcontrols : *mut PLDAPControlW) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_startup(version : *mut LDAP_VERSION_INFO, instance : *mut super::winnt::HANDLE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_stop_tls_s(externalhandle : *mut LDAP) -> bool);
windows_link::link!("wldap32.dll" "C" fn ldap_ufn2dn(ufn : windows_sys::core::PCSTR, pdn : *mut windows_sys::core::PSTR) -> u32);
windows_link::link!("wldap32.dll" "C" fn ldap_ufn2dnA(ufn : windows_sys::core::PCSTR, pdn : *mut windows_sys::core::PSTR) -> u32);
windows_link::link!("wldap32.dll" "C" fn ldap_ufn2dnW(ufn : windows_sys::core::PCWSTR, pdn : *mut windows_sys::core::PWSTR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_unbind(ld : *mut LDAP) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_unbind_s(ld : *mut LDAP) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_value_free(vals : *const super::winnt::PCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_value_freeA(vals : *const super::winnt::PCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_value_freeW(vals : *const super::winnt::PWCHAR) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wldap32.dll" "C" fn ldap_value_free_len(vals : *mut *mut LDAP_BERVAL) -> u32);
#[cfg(feature = "Win32_winnt")]
pub type BERVAL = LDAP_BERVAL;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct BerElement {
    pub opaque: super::winnt::PCHAR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for BerElement {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
pub type BerValue = LDAP_BERVAL;
pub type DBGPRINT = *mut u8;
#[cfg(feature = "Win32_winnt")]
pub type DEREFERENCECONNECTION = Option<unsafe extern "C" fn(primaryconnection: *mut LDAP, connectiontodereference: *mut LDAP) -> u32>;
pub const LAPI_MAJOR_VER1: u32 = 1;
pub const LAPI_MINOR_VER1: u32 = 1;
pub const LBER_TRANSLATE_STRINGS: u32 = 4;
pub const LBER_USE_DER: u32 = 1;
pub const LBER_USE_INDEFINITE_LEN: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAP {
    pub ld_sb: LDAP_0,
    pub ld_host: super::winnt::PCHAR,
    pub ld_version: u32,
    pub ld_lberoptions: u8,
    pub ld_deref: u32,
    pub ld_timelimit: u32,
    pub ld_sizelimit: u32,
    pub ld_errno: u32,
    pub ld_matched: super::winnt::PCHAR,
    pub ld_error: super::winnt::PCHAR,
    pub ld_msgid: u32,
    pub Reserved3: [u8; 25],
    pub ld_cldaptries: u32,
    pub ld_cldaptimeout: u32,
    pub ld_refhoplimit: u32,
    pub ld_options: u32,
    pub ld_anonymousmaxvalrange: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAP_0 {
    pub sb_sd: usize,
    pub Reserved1: [u8; 41],
    pub sb_naddr: usize,
    pub Reserved2: [u8; 24],
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LDAPAPIFeatureInfoA {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: *mut i8,
    pub ldapaif_version: i32,
}
impl Default for LDAPAPIFeatureInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPAPIFeatureInfoW {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: super::winnt::PWCHAR,
    pub ldapaif_version: i32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPAPIFeatureInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LDAPAPIInfoA {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut *mut i8,
    pub ldapai_vendor_name: *mut i8,
    pub ldapai_vendor_version: i32,
}
impl Default for LDAPAPIInfoA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPAPIInfoW {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut super::winnt::PWCHAR,
    pub ldapai_vendor_name: super::winnt::PWCHAR,
    pub ldapai_vendor_version: i32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPAPIInfoW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPControlA {
    pub ldctl_oid: super::winnt::PCHAR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: bool,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPControlA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPControlW {
    pub ldctl_oid: super::winnt::PWCHAR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: bool,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPControlW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPMessage {
    pub lm_msgid: u32,
    pub lm_msgtype: u32,
    pub lm_ber: *mut core::ffi::c_void,
    pub lm_chain: *mut Self,
    pub lm_next: *mut Self,
    pub lm_time: u32,
    pub Connection: PLDAP,
    pub Request: *mut core::ffi::c_void,
    pub lm_returncode: u32,
    pub lm_referral: u16,
    pub lm_chased: bool,
    pub lm_eom: bool,
    pub ConnectionReferenced: bool,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPMessage {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPModA {
    pub mod_op: u32,
    pub mod_type: super::winnt::PCHAR,
    pub mod_vals: LDAPModA_0,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPModA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union LDAPModA_0 {
    pub modv_strvals: *mut super::winnt::PCHAR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPModA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPModW {
    pub mod_op: u32,
    pub mod_type: super::winnt::PWCHAR,
    pub mod_vals: LDAPModW_0,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPModW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union LDAPModW_0 {
    pub modv_strvals: *mut super::winnt::PWCHAR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPModW_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LDAPSearch = ldapsearch;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPSortKeyA {
    pub sk_attrtype: super::winnt::PCHAR,
    pub sk_matchruleoid: super::winnt::PCHAR,
    pub sk_reverseorder: bool,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPSortKeyA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPSortKeyW {
    pub sk_attrtype: super::winnt::PWCHAR,
    pub sk_matchruleoid: super::winnt::PWCHAR,
    pub sk_reverseorder: bool,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPSortKeyW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAPVLVInfo {
    pub ldvlv_version: i32,
    pub ldvlv_before_count: u32,
    pub ldvlv_after_count: u32,
    pub ldvlv_offset: u32,
    pub ldvlv_count: u32,
    pub ldvlv_attrvalue: PBERVAL,
    pub ldvlv_context: PBERVAL,
    pub ldvlv_extradata: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAPVLVInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LDAP_ABANDON_CMD: u32 = 80;
pub const LDAP_ADD_CMD: u32 = 104;
pub const LDAP_ADMIN_LIMIT_EXCEEDED: LDAP_RETCODE = 11;
pub const LDAP_AFFECTS_MULTIPLE_DSAS: LDAP_RETCODE = 71;
pub const LDAP_ALIAS_DEREF_PROBLEM: LDAP_RETCODE = 36;
pub const LDAP_ALIAS_PROBLEM: LDAP_RETCODE = 33;
pub const LDAP_ALREADY_EXISTS: LDAP_RETCODE = 68;
pub const LDAP_API_FEATURE_VIRTUAL_LIST_VIEW: u32 = 1001;
pub const LDAP_API_INFO_VERSION: u32 = 1;
pub const LDAP_API_VERSION: u32 = 2004;
pub const LDAP_ATTRIBUTE_OR_VALUE_EXISTS: LDAP_RETCODE = 20;
pub const LDAP_AUTH_DIGEST: u32 = 16518;
pub const LDAP_AUTH_DPA: u32 = 8326;
pub const LDAP_AUTH_EXTERNAL: u32 = 166;
pub const LDAP_AUTH_METHOD_NOT_SUPPORTED: LDAP_RETCODE = 7;
pub const LDAP_AUTH_MSN: u32 = 2182;
pub const LDAP_AUTH_NEGOTIATE: u32 = 1158;
pub const LDAP_AUTH_NTLM: u32 = 4230;
pub const LDAP_AUTH_OTHERKIND: u32 = 134;
pub const LDAP_AUTH_SASL: u32 = 131;
pub const LDAP_AUTH_SICILY: u32 = 646;
pub const LDAP_AUTH_SIMPLE: u32 = 128;
pub const LDAP_AUTH_SSPI: u32 = 1158;
pub const LDAP_AUTH_UNKNOWN: LDAP_RETCODE = 86;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct LDAP_BERVAL {
    pub bv_len: u32,
    pub bv_val: super::winnt::PCHAR,
}
#[cfg(feature = "Win32_winnt")]
impl Default for LDAP_BERVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LDAP_BIND_CMD: u32 = 96;
pub const LDAP_BUSY: LDAP_RETCODE = 51;
pub const LDAP_CHASE_EXTERNAL_REFERRALS: u32 = 64;
pub const LDAP_CHASE_SUBORDINATE_REFERRALS: u32 = 32;
pub const LDAP_CLIENT_LOOP: LDAP_RETCODE = 96;
pub const LDAP_COMPARE_CMD: u32 = 110;
pub const LDAP_COMPARE_FALSE: LDAP_RETCODE = 5;
pub const LDAP_COMPARE_TRUE: LDAP_RETCODE = 6;
pub const LDAP_CONFIDENTIALITY_REQUIRED: LDAP_RETCODE = 13;
pub const LDAP_CONNECT_ERROR: LDAP_RETCODE = 91;
pub const LDAP_CONSTRAINT_VIOLATION: LDAP_RETCODE = 19;
pub const LDAP_CONTROL_NOT_FOUND: LDAP_RETCODE = 93;
pub const LDAP_CONTROL_REFERRALS: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113556.1.4.616");
pub const LDAP_CONTROL_REFERRALS_W: windows_sys::core::PCWSTR = windows_sys::core::w!("1.2.840.113556.1.4.616");
pub const LDAP_CONTROL_VLVREQUEST: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.3.4.9");
pub const LDAP_CONTROL_VLVREQUEST_W: windows_sys::core::PCWSTR = windows_sys::core::w!("2.16.840.1.113730.3.4.9");
pub const LDAP_CONTROL_VLVRESPONSE: windows_sys::core::PCSTR = windows_sys::core::s!("2.16.840.1.113730.3.4.10");
pub const LDAP_CONTROL_VLVRESPONSE_W: windows_sys::core::PCWSTR = windows_sys::core::w!("2.16.840.1.113730.3.4.10");
pub const LDAP_DECODING_ERROR: LDAP_RETCODE = 84;
pub const LDAP_DELETE_CMD: u32 = 74;
pub const LDAP_DEREF_ALWAYS: u32 = 3;
pub const LDAP_DEREF_FINDING: u32 = 2;
pub const LDAP_DEREF_NEVER: u32 = 0;
pub const LDAP_DEREF_SEARCHING: u32 = 1;
pub const LDAP_ENCODING_ERROR: LDAP_RETCODE = 83;
pub const LDAP_EXTENDED_CMD: u32 = 119;
pub const LDAP_FEATURE_INFO_VERSION: u32 = 1;
pub const LDAP_FILTER_AND: u32 = 160;
pub const LDAP_FILTER_APPROX: u32 = 168;
pub const LDAP_FILTER_EQUALITY: u32 = 163;
pub const LDAP_FILTER_ERROR: LDAP_RETCODE = 87;
pub const LDAP_FILTER_EXTENSIBLE: u32 = 169;
pub const LDAP_FILTER_GE: u32 = 165;
pub const LDAP_FILTER_LE: u32 = 166;
pub const LDAP_FILTER_NOT: u32 = 162;
pub const LDAP_FILTER_OR: u32 = 161;
pub const LDAP_FILTER_PRESENT: u32 = 135;
pub const LDAP_FILTER_SUBSTRINGS: u32 = 164;
pub const LDAP_GC_PORT: u32 = 3268;
pub const LDAP_INAPPROPRIATE_AUTH: LDAP_RETCODE = 48;
pub const LDAP_INAPPROPRIATE_MATCHING: LDAP_RETCODE = 18;
pub const LDAP_INSUFFICIENT_RIGHTS: LDAP_RETCODE = 50;
pub const LDAP_INVALID_CMD: u32 = 255;
pub const LDAP_INVALID_CREDENTIALS: LDAP_RETCODE = 49;
pub const LDAP_INVALID_DN_SYNTAX: LDAP_RETCODE = 34;
pub const LDAP_INVALID_RES: u32 = 255;
pub const LDAP_INVALID_SYNTAX: LDAP_RETCODE = 21;
pub const LDAP_IS_LEAF: LDAP_RETCODE = 35;
pub const LDAP_LOCAL_ERROR: LDAP_RETCODE = 82;
pub const LDAP_LOOP_DETECT: LDAP_RETCODE = 54;
pub const LDAP_MODIFY_CMD: u32 = 102;
pub const LDAP_MODRDN_CMD: u32 = 108;
pub const LDAP_MOD_ADD: u32 = 0;
pub const LDAP_MOD_BVALUES: u32 = 128;
pub const LDAP_MOD_DELETE: u32 = 1;
pub const LDAP_MOD_REPLACE: u32 = 2;
pub const LDAP_MORE_RESULTS_TO_RETURN: LDAP_RETCODE = 95;
pub const LDAP_MSG_ALL: u32 = 1;
pub const LDAP_MSG_ONE: u32 = 0;
pub const LDAP_MSG_RECEIVED: u32 = 2;
pub const LDAP_NAMING_VIOLATION: LDAP_RETCODE = 64;
pub const LDAP_NOT_ALLOWED_ON_NONLEAF: LDAP_RETCODE = 66;
pub const LDAP_NOT_ALLOWED_ON_RDN: LDAP_RETCODE = 67;
pub const LDAP_NOT_SUPPORTED: LDAP_RETCODE = 92;
pub const LDAP_NO_LIMIT: u32 = 0;
pub const LDAP_NO_MEMORY: LDAP_RETCODE = 90;
pub const LDAP_NO_OBJECT_CLASS_MODS: LDAP_RETCODE = 69;
pub const LDAP_NO_RESULTS_RETURNED: LDAP_RETCODE = 94;
pub const LDAP_NO_SUCH_ATTRIBUTE: LDAP_RETCODE = 16;
pub const LDAP_NO_SUCH_OBJECT: LDAP_RETCODE = 32;
pub const LDAP_OBJECT_CLASS_VIOLATION: LDAP_RETCODE = 65;
pub const LDAP_OFFSET_RANGE_ERROR: LDAP_RETCODE = 61;
pub const LDAP_OPERATIONS_ERROR: LDAP_RETCODE = 1;
pub const LDAP_OPT_ANONYMOUS_MAX_VAL_RANGE: u32 = 71;
pub const LDAP_OPT_API_FEATURE_INFO: u32 = 21;
pub const LDAP_OPT_API_INFO: u32 = 0;
pub const LDAP_OPT_AREC_EXCLUSIVE: u32 = 152;
pub const LDAP_OPT_AUTO_RECONNECT: u32 = 145;
pub const LDAP_OPT_CACHE_ENABLE: u32 = 15;
pub const LDAP_OPT_CACHE_FN_PTRS: u32 = 13;
pub const LDAP_OPT_CACHE_STRATEGY: u32 = 14;
pub const LDAP_OPT_CHASE_REFERRALS: u32 = 2;
pub const LDAP_OPT_CLDAP_TIMEOUT: u32 = 69;
pub const LDAP_OPT_CLDAP_TRIES: u32 = 70;
pub const LDAP_OPT_CLIENT_CERTIFICATE: u32 = 128;
pub const LDAP_OPT_DEREF: u32 = 2;
pub const LDAP_OPT_DESC: u32 = 1;
pub const LDAP_OPT_DNS: u32 = 1;
pub const LDAP_OPT_DNSDOMAIN_NAME: u32 = 59;
pub const LDAP_OPT_ENCRYPT: u32 = 150;
pub const LDAP_OPT_ERROR_NUMBER: u32 = 49;
pub const LDAP_OPT_ERROR_STRING: u32 = 50;
pub const LDAP_OPT_FAST_CONCURRENT_BIND: u32 = 65;
pub const LDAP_OPT_GETDSNAME_FLAGS: u32 = 61;
pub const LDAP_OPT_HOST_NAME: u32 = 48;
pub const LDAP_OPT_HOST_REACHABLE: u32 = 62;
pub const LDAP_OPT_IO_FN_PTRS: u32 = 11;
pub const LDAP_OPT_PING_KEEP_ALIVE: u32 = 54;
pub const LDAP_OPT_PING_LIMIT: u32 = 56;
pub const LDAP_OPT_PING_WAIT_TIME: u32 = 55;
pub const LDAP_OPT_PROMPT_CREDENTIALS: u32 = 63;
pub const LDAP_OPT_PROTOCOL_VERSION: u32 = 17;
pub const LDAP_OPT_REBIND_ARG: u32 = 7;
pub const LDAP_OPT_REBIND_FN: u32 = 6;
pub const LDAP_OPT_REFERRALS: u32 = 8;
pub const LDAP_OPT_REFERRAL_CALLBACK: u32 = 112;
pub const LDAP_OPT_REFERRAL_HOP_LIMIT: u32 = 16;
pub const LDAP_OPT_REF_DEREF_CONN_PER_MSG: u32 = 148;
pub const LDAP_OPT_RESTART: u32 = 9;
pub const LDAP_OPT_RETURN_REFS: u32 = 4;
pub const LDAP_OPT_ROOTDSE_CACHE: u32 = 154;
pub const LDAP_OPT_SASL_METHOD: u32 = 151;
pub const LDAP_OPT_SCH_FLAGS: u32 = 67;
pub const LDAP_OPT_SECURITY_CONTEXT: u32 = 153;
pub const LDAP_OPT_SEND_TIMEOUT: u32 = 66;
pub const LDAP_OPT_SERVER_CERTIFICATE: u32 = 129;
pub const LDAP_OPT_SERVER_ERROR: u32 = 51;
pub const LDAP_OPT_SERVER_EXT_ERROR: u32 = 52;
pub const LDAP_OPT_SIGN: u32 = 149;
pub const LDAP_OPT_SIZELIMIT: u32 = 3;
pub const LDAP_OPT_SOCKET_BIND_ADDRESSES: u32 = 68;
pub const LDAP_OPT_SSL: u32 = 10;
pub const LDAP_OPT_SSL_INFO: u32 = 147;
pub const LDAP_OPT_SSPI_FLAGS: u32 = 146;
pub const LDAP_OPT_TCP_KEEPALIVE: u32 = 64;
pub const LDAP_OPT_THREAD_FN_PTRS: u32 = 5;
pub const LDAP_OPT_TIMELIMIT: u32 = 4;
pub const LDAP_OPT_TLS: u32 = 10;
pub const LDAP_OPT_TLS_INFO: u32 = 147;
pub const LDAP_OPT_VERSION: u32 = 17;
pub const LDAP_OTHER: LDAP_RETCODE = 80;
pub const LDAP_PAGED_RESULT_OID_STRING: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113556.1.4.319");
pub const LDAP_PAGED_RESULT_OID_STRING_W: windows_sys::core::PCWSTR = windows_sys::core::w!("1.2.840.113556.1.4.319");
pub const LDAP_PARAM_ERROR: LDAP_RETCODE = 89;
pub const LDAP_PARTIAL_RESULTS: LDAP_RETCODE = 9;
pub const LDAP_PORT: u32 = 389;
pub const LDAP_PROTOCOL_ERROR: LDAP_RETCODE = 2;
pub const LDAP_REFERRAL: LDAP_RETCODE = 10;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct LDAP_REFERRAL_CALLBACK {
    pub SizeOfCallbacks: u32,
    pub QueryForConnection: QUERYFORCONNECTION,
    pub NotifyRoutine: NOTIFYOFNEWCONNECTION,
    pub DereferenceRoutine: DEREFERENCECONNECTION,
}
pub const LDAP_REFERRAL_LIMIT_EXCEEDED: LDAP_RETCODE = 97;
pub const LDAP_REFERRAL_V2: LDAP_RETCODE = 9;
pub const LDAP_RESULTS_TOO_LARGE: LDAP_RETCODE = 70;
pub const LDAP_RES_ADD: u32 = 105;
pub const LDAP_RES_ANY: i32 = -1;
pub const LDAP_RES_BIND: u32 = 97;
pub const LDAP_RES_COMPARE: u32 = 111;
pub const LDAP_RES_DELETE: u32 = 107;
pub const LDAP_RES_EXTENDED: u32 = 120;
pub const LDAP_RES_MODIFY: u32 = 103;
pub const LDAP_RES_MODRDN: u32 = 109;
pub const LDAP_RES_REFERRAL: u32 = 115;
pub const LDAP_RES_SEARCH_ENTRY: u32 = 100;
pub const LDAP_RES_SEARCH_RESULT: u32 = 101;
pub const LDAP_RES_SESSION: u32 = 114;
pub type LDAP_RETCODE = i32;
pub const LDAP_SASL_BIND_IN_PROGRESS: LDAP_RETCODE = 14;
pub const LDAP_SCOPE_BASE: u32 = 0;
pub const LDAP_SCOPE_ONELEVEL: u32 = 1;
pub const LDAP_SCOPE_SUBTREE: u32 = 2;
pub const LDAP_SEARCH_CMD: u32 = 99;
pub const LDAP_SERVER_DOWN: LDAP_RETCODE = 81;
pub const LDAP_SERVER_RESP_SORT_OID: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113556.1.4.474");
pub const LDAP_SERVER_RESP_SORT_OID_W: windows_sys::core::PCWSTR = windows_sys::core::w!("1.2.840.113556.1.4.474");
pub const LDAP_SERVER_SORT_OID: windows_sys::core::PCSTR = windows_sys::core::s!("1.2.840.113556.1.4.473");
pub const LDAP_SERVER_SORT_OID_W: windows_sys::core::PCWSTR = windows_sys::core::w!("1.2.840.113556.1.4.473");
pub const LDAP_SESSION_CMD: u32 = 113;
pub const LDAP_SIZELIMIT_EXCEEDED: LDAP_RETCODE = 4;
pub const LDAP_SORT_CONTROL_MISSING: LDAP_RETCODE = 60;
pub const LDAP_SSL_GC_PORT: u32 = 3269;
pub const LDAP_SSL_PORT: u32 = 636;
pub const LDAP_START_TLS_OID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.1466.20037");
pub const LDAP_START_TLS_OID_W: windows_sys::core::PCWSTR = windows_sys::core::w!("1.3.6.1.4.1.1466.20037");
pub const LDAP_STRONG_AUTH_REQUIRED: LDAP_RETCODE = 8;
pub const LDAP_SUBSTRING_ANY: u32 = 129;
pub const LDAP_SUBSTRING_FINAL: u32 = 130;
pub const LDAP_SUBSTRING_INITIAL: u32 = 128;
pub const LDAP_SUCCESS: LDAP_RETCODE = 0;
pub const LDAP_TIMELIMIT_EXCEEDED: LDAP_RETCODE = 3;
pub const LDAP_TIMEOUT: LDAP_RETCODE = 85;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LDAP_TIMEVAL {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
pub const LDAP_TTL_EXTENDED_OP_OID: windows_sys::core::PCSTR = windows_sys::core::s!("1.3.6.1.4.1.1466.101.119.1");
pub const LDAP_TTL_EXTENDED_OP_OID_W: windows_sys::core::PCWSTR = windows_sys::core::w!("1.3.6.1.4.1.1466.101.119.1");
pub const LDAP_UNAVAILABLE: LDAP_RETCODE = 52;
pub const LDAP_UNAVAILABLE_CRIT_EXTENSION: LDAP_RETCODE = 12;
pub const LDAP_UNBIND_CMD: u32 = 66;
pub const LDAP_UNDEFINED_TYPE: LDAP_RETCODE = 17;
pub const LDAP_UNICODE: u32 = 0;
pub const LDAP_UNWILLING_TO_PERFORM: LDAP_RETCODE = 53;
pub const LDAP_USER_CANCELLED: LDAP_RETCODE = 88;
pub const LDAP_VENDOR_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("Microsoft Corporation.");
pub const LDAP_VENDOR_NAME_W: windows_sys::core::PCWSTR = windows_sys::core::w!("Microsoft Corporation.");
pub const LDAP_VENDOR_VERSION: u32 = 510;
pub const LDAP_VERSION: u32 = 2;
pub const LDAP_VERSION1: u32 = 1;
pub const LDAP_VERSION2: u32 = 2;
pub const LDAP_VERSION3: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct LDAP_VERSION_INFO {
    pub lv_size: u32,
    pub lv_major: u32,
    pub lv_minor: u32,
}
pub const LDAP_VERSION_MAX: u32 = 3;
pub const LDAP_VERSION_MIN: u32 = 2;
pub const LDAP_VIRTUAL_LIST_VIEW_ERROR: LDAP_RETCODE = 76;
pub const LDAP_VLVINFO_VERSION: u32 = 1;
#[cfg(feature = "Win32_winnt")]
pub type NOTIFYOFNEWCONNECTION = Option<unsafe extern "C" fn(primaryconnection: *mut LDAP, referralfromconnection: *mut LDAP, newdn: *mut u16, hostname: *mut i8, newconnection: *mut LDAP, portnumber: u32, secauthidentity: *mut core::ffi::c_void, currentuser: *mut core::ffi::c_void, errorcodefrombind: u32) -> bool>;
#[cfg(feature = "Win32_winnt")]
pub type PBERVAL = *mut LDAP_BERVAL;
#[cfg(feature = "Win32_winnt")]
pub type PLDAP = *mut LDAP;
#[cfg(feature = "Win32_winnt")]
pub type PLDAPControlA = *mut LDAPControlA;
#[cfg(feature = "Win32_winnt")]
pub type PLDAPControlW = *mut LDAPControlW;
#[cfg(feature = "Win32_winnt")]
pub type PLDAPMessage = *mut LDAPMessage;
#[cfg(feature = "Win32_winnt")]
pub type PLDAPModA = *mut LDAPModA;
#[cfg(feature = "Win32_winnt")]
pub type PLDAPModW = *mut LDAPModW;
pub type PLDAPSearch = *mut ldapsearch;
#[cfg(feature = "Win32_winnt")]
pub type PLDAPSortKeyA = *mut LDAPSortKeyA;
#[cfg(feature = "Win32_winnt")]
pub type PLDAPSortKeyW = *mut LDAPSortKeyW;
#[cfg(feature = "Win32_winnt")]
pub type PLDAPVLVInfo = *mut LDAPVLVInfo;
#[cfg(feature = "Win32_winnt")]
pub type PLDAP_BERVAL = *mut LDAP_BERVAL;
#[cfg(feature = "Win32_winnt")]
pub type PLDAP_REFERRAL_CALLBACK = *mut LDAP_REFERRAL_CALLBACK;
pub type PLDAP_TIMEVAL = *mut LDAP_TIMEVAL;
pub type PLDAP_VERSION_INFO = *mut LDAP_VERSION_INFO;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_schannel", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
pub type QUERYCLIENTCERT = Option<unsafe extern "C" fn(connection: *mut LDAP, trusted_cas: *mut super::schannel::SecPkgContext_IssuerListInfoEx, ppcertificate: *mut super::wincrypt::PCCERT_CONTEXT) -> bool>;
#[cfg(feature = "Win32_winnt")]
pub type QUERYFORCONNECTION = Option<unsafe extern "C" fn(primaryconnection: *mut LDAP, referralfromconnection: *mut LDAP, newdn: *mut u16, hostname: *mut i8, portnumber: u32, secauthidentity: *mut core::ffi::c_void, currentusertoken: *mut core::ffi::c_void, connectiontouse: *mut PLDAP) -> u32>;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_wincrypt", feature = "Win32_winnt"))]
pub type VERIFYSERVERCERT = Option<unsafe extern "C" fn(connection: *mut LDAP, pservercert: *mut super::wincrypt::PCCERT_CONTEXT) -> bool>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ldapsearch(pub u8);
