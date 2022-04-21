#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn LdapGetLastError() -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn LdapMapErrorToWin32(ldaperror: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn LdapUTF8ToUnicode(lpsrcstr: ::windows_sys::core::PCSTR, cchsrc: i32, lpdeststr: ::windows_sys::core::PWSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn LdapUnicodeToUTF8(lpsrcstr: ::windows_sys::core::PCWSTR, cchsrc: i32, lpdeststr: ::windows_sys::core::PSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_alloc_t(options: i32) -> *mut berelement;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_bvdup(pberval: *mut LDAP_BERVAL) -> *mut LDAP_BERVAL;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_bvecfree(pberval: *mut *mut LDAP_BERVAL);
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_bvfree(bv: *mut LDAP_BERVAL);
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_first_element(pberelement: *mut berelement, plen: *mut u32, ppopaque: *mut *mut super::super::Foundation::CHAR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_flatten(pberelement: *mut berelement, pberval: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_free(pberelement: *mut berelement, fbuf: i32);
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_init(pberval: *mut LDAP_BERVAL) -> *mut berelement;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_next_element(pberelement: *mut berelement, plen: *mut u32, opaque: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_peek_tag(pberelement: *mut berelement, plen: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_printf(pberelement: *mut berelement, fmt: ::windows_sys::core::PCSTR) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_scanf(pberelement: *mut berelement, fmt: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ber_skip_tag(pberelement: *mut berelement, plen: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn cldap_open(hostname: ::windows_sys::core::PCSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn cldap_openA(hostname: ::windows_sys::core::PCSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn cldap_openW(hostname: ::windows_sys::core::PCWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_abandon(ld: *mut ldap, msgid: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_add(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_addA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_addW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, attrs: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_extA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_extW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_add_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_add_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_add_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, attrs: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_bind(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, cred: ::windows_sys::core::PCSTR, method: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_bindA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, cred: ::windows_sys::core::PCSTR, method: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_bindW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, cred: ::windows_sys::core::PCWSTR, method: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_bind_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, cred: ::windows_sys::core::PCSTR, method: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_bind_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, cred: ::windows_sys::core::PCSTR, method: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_bind_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, cred: ::windows_sys::core::PCWSTR, method: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_check_filterA(ld: *mut ldap, searchfilter: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_check_filterW(ld: *mut ldap, searchfilter: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_cleanup(hinstance: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_close_extended_op(ld: *mut ldap, messagenumber: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_compare(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attr: ::windows_sys::core::PCSTR, value: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_compareA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attr: ::windows_sys::core::PCSTR, value: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_compareW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, attr: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attr: ::windows_sys::core::PCSTR, value: ::windows_sys::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_extA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attr: ::windows_sys::core::PCSTR, value: ::windows_sys::core::PCSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_extW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, attr: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attr: ::windows_sys::core::PCSTR, value: ::windows_sys::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attr: ::windows_sys::core::PCSTR, value: ::windows_sys::core::PCSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, attr: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_compare_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attr: ::windows_sys::core::PCSTR, value: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_compare_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, attr: ::windows_sys::core::PCSTR, value: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_compare_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, attr: ::windows_sys::core::PCWSTR, value: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_conn_from_msg(primaryconn: *mut ldap, res: *mut LDAPMessage) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_connect(ld: *mut ldap, timeout: *mut LDAP_TIMEVAL) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_free(control: *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_freeA(controls: *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_freeW(control: *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_free(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_freeA(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_freeW(control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_entries(ld: *mut ldap, res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_references(ld: *mut ldap, res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_count_values(vals: *const ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_count_valuesA(vals: *const ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_count_valuesW(vals: *const ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_count_values_len(vals: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_control(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_controlA(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_controlW(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_control(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_vlv_controlA(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolA) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_vlv_controlW(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolW) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_delete(ld: *mut ldap, dn: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_deleteA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_deleteW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_extA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_extW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_delete_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_delete_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_delete_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_dn2ufn(dn: ::windows_sys::core::PCSTR) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_dn2ufnA(dn: ::windows_sys::core::PCSTR) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_dn2ufnW(dn: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_encode_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, control: *mut ldapcontrolA, criticality: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_encode_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, control: *mut ldapcontrolW, criticality: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_err2string(err: u32) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_err2stringA(err: u32) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_err2stringW(err: u32) -> ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_escape_filter_element(sourcefilterelement: ::windows_sys::core::PCSTR, sourcelength: u32, destfilterelement: ::windows_sys::core::PSTR, destlength: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_escape_filter_elementA(sourcefilterelement: ::windows_sys::core::PCSTR, sourcelength: u32, destfilterelement: ::windows_sys::core::PSTR, destlength: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_escape_filter_elementW(sourcefilterelement: ::windows_sys::core::PCSTR, sourcelength: u32, destfilterelement: ::windows_sys::core::PWSTR, destlength: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_explode_dn(dn: ::windows_sys::core::PCSTR, notypes: u32) -> *mut ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_explode_dnA(dn: ::windows_sys::core::PCSTR, notypes: u32) -> *mut ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_explode_dnW(dn: ::windows_sys::core::PCWSTR, notypes: u32) -> *mut ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation(ld: *mut ldap, oid: ::windows_sys::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operationA(ld: *mut ldap, oid: ::windows_sys::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operationW(ld: *mut ldap, oid: ::windows_sys::core::PCWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation_sA(externalhandle: *mut ldap, oid: ::windows_sys::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, returnedoid: *mut ::windows_sys::core::PSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation_sW(externalhandle: *mut ldap, oid: ::windows_sys::core::PCWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, returnedoid: *mut ::windows_sys::core::PWSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_entry(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_reference(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controls(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controlsA(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controlsW(controls: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dn(ld: *mut ldap, entry: *mut LDAPMessage) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dnA(ld: *mut ldap, entry: *mut LDAPMessage) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dnW(ld: *mut ldap, entry: *mut LDAPMessage) -> ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_get_next_page(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, pagesize: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_next_page_s(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, timeout: *mut LDAP_TIMEVAL, pagesize: u32, totalcount: *mut u32, results: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_get_option(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_get_optionW(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_paged_count(externalhandle: *mut ldap, searchblock: *mut ldapsearch, totalcount: *mut u32, results: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values(ld: *mut ldap, entry: *mut LDAPMessage, attr: ::windows_sys::core::PCSTR) -> *mut ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_valuesA(ld: *mut ldap, entry: *mut LDAPMessage, attr: ::windows_sys::core::PCSTR) -> *mut ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_valuesW(ld: *mut ldap, entry: *mut LDAPMessage, attr: ::windows_sys::core::PCWSTR) -> *mut ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_len(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: ::windows_sys::core::PCSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_lenA(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: ::windows_sys::core::PCSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_lenW(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: ::windows_sys::core::PCWSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_init(hostname: ::windows_sys::core::PCSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_initA(hostname: ::windows_sys::core::PCSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_initW(hostname: ::windows_sys::core::PCWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_memfree(block: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_memfreeA(block: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_memfreeW(block: ::windows_sys::core::PCWSTR);
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modify(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modifyA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modifyW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, mods: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_extA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_extW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modify_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modify_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modify_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, mods: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, newdistinguishedname: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn2(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, newdistinguishedname: ::windows_sys::core::PCSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn2A(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, newdistinguishedname: ::windows_sys::core::PCSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn2W(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCWSTR, newdistinguishedname: ::windows_sys::core::PCWSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn2_s(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, newdistinguishedname: ::windows_sys::core::PCSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn2_sA(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, newdistinguishedname: ::windows_sys::core::PCSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn2_sW(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCWSTR, newdistinguishedname: ::windows_sys::core::PCWSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdnA(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, newdistinguishedname: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdnW(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCWSTR, newdistinguishedname: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn_s(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, newdistinguishedname: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn_sA(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, newdistinguishedname: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_modrdn_sW(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCWSTR, newdistinguishedname: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_msgfree(res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> ::windows_sys::core::PSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> ::windows_sys::core::PWSTR;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_entry(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_reference(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_open(hostname: ::windows_sys::core::PCSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_openA(hostname: ::windows_sys::core::PCSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_openW(hostname: ::windows_sys::core::PCWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_extended_resultA(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut ::windows_sys::core::PSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_extended_resultW(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut ::windows_sys::core::PWSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_control(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_controlA(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_controlW(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolW, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_reference(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_referenceA(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_referenceW(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_result(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut ::windows_sys::core::PSTR, errormessage: *mut ::windows_sys::core::PSTR, referrals: *mut *mut ::windows_sys::core::PSTR, servercontrols: *mut *mut *mut ldapcontrolA, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_resultA(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut ::windows_sys::core::PSTR, errormessage: *mut ::windows_sys::core::PSTR, referrals: *mut *mut *mut i8, servercontrols: *mut *mut *mut ldapcontrolA, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_resultW(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut ::windows_sys::core::PWSTR, errormessage: *mut ::windows_sys::core::PWSTR, referrals: *mut *mut *mut u16, servercontrols: *mut *mut *mut ldapcontrolW, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_control(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, result: *mut u32, attribute: *mut ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_vlv_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_vlv_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_perror(ld: *mut ldap, msg: ::windows_sys::core::PCSTR);
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, newrdn: ::windows_sys::core::PCSTR, newparent: ::windows_sys::core::PCSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_extA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, newrdn: ::windows_sys::core::PCSTR, newparent: ::windows_sys::core::PCSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_extW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, newrdn: ::windows_sys::core::PCWSTR, newparent: ::windows_sys::core::PCWSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, newrdn: ::windows_sys::core::PCSTR, newparent: ::windows_sys::core::PCSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, newrdn: ::windows_sys::core::PCSTR, newparent: ::windows_sys::core::PCSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, newrdn: ::windows_sys::core::PCWSTR, newparent: ::windows_sys::core::PCWSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_result(ld: *mut ldap, msgid: u32, all: u32, timeout: *const LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_result2error(ld: *mut ldap, res: *mut LDAPMessage, freeit: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bindA(externalhandle: *mut ldap, distname: ::windows_sys::core::PCSTR, authmechanism: ::windows_sys::core::PCSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, messagenumber: *mut i32) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bindW(externalhandle: *mut ldap, distname: ::windows_sys::core::PCWSTR, authmechanism: ::windows_sys::core::PCWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, messagenumber: *mut i32) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bind_sA(externalhandle: *mut ldap, distname: ::windows_sys::core::PCSTR, authmechanism: ::windows_sys::core::PCSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, serverdata: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bind_sW(externalhandle: *mut ldap, distname: ::windows_sys::core::PCWSTR, authmechanism: ::windows_sys::core::PCWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, serverdata: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_search(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_searchA(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_searchW(ld: *mut ldap, base: ::windows_sys::core::PCWSTR, scope: u32, filter: ::windows_sys::core::PCWSTR, attrs: *const *const u16, attrsonly: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_search_abandon_page(externalhandle: *mut ldap, searchblock: *mut ldapsearch) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_extA(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_extW(ld: *mut ldap, base: ::windows_sys::core::PCWSTR, scope: u32, filter: ::windows_sys::core::PCWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_s(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_sA(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_sW(ld: *mut ldap, base: ::windows_sys::core::PCWSTR, scope: u32, filter: ::windows_sys::core::PCWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_page(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, scopeofsearch: u32, searchfilter: ::windows_sys::core::PCSTR, attributelist: *mut *mut i8, attributesonly: u32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyA) -> *mut ldapsearch;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_pageA(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCSTR, scopeofsearch: u32, searchfilter: ::windows_sys::core::PCSTR, attributelist: *const *const i8, attributesonly: u32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyA) -> *mut ldapsearch;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_pageW(externalhandle: *mut ldap, distinguishedname: ::windows_sys::core::PCWSTR, scopeofsearch: u32, searchfilter: ::windows_sys::core::PCWSTR, attributelist: *const *const u16, attributesonly: u32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyW) -> *mut ldapsearch;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_s(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_sA(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_sW(ld: *mut ldap, base: ::windows_sys::core::PCWSTR, scope: u32, filter: ::windows_sys::core::PCWSTR, attrs: *const *const u16, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_st(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_stA(ld: *mut ldap, base: ::windows_sys::core::PCSTR, scope: u32, filter: ::windows_sys::core::PCSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_stW(ld: *mut ldap, base: ::windows_sys::core::PCWSTR, scope: u32, filter: ::windows_sys::core::PCWSTR, attrs: *const *const u16, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_set_dbg_flags(newflags: u32) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_set_dbg_routine(debugprintroutine: DBGPRINT);
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_set_option(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_set_optionW(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_simple_bind(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, passwd: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_simple_bindA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, passwd: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_simple_bindW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, passwd: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_simple_bind_s(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, passwd: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_simple_bind_sA(ld: *mut ldap, dn: ::windows_sys::core::PCSTR, passwd: ::windows_sys::core::PCSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_simple_bind_sW(ld: *mut ldap, dn: ::windows_sys::core::PCWSTR, passwd: ::windows_sys::core::PCWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_sslinit(hostname: ::windows_sys::core::PCSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_sslinitA(hostname: ::windows_sys::core::PCSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_sslinitW(hostname: ::windows_sys::core::PCWSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_start_tls_sA(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_start_tls_sW(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_startup(version: *mut ldap_version_info, instance: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_stop_tls_s(externalhandle: *mut ldap) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_ufn2dn(ufn: ::windows_sys::core::PCSTR, pdn: *mut ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_ufn2dnA(ufn: ::windows_sys::core::PCSTR, pdn: *mut ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_ufn2dnW(ufn: ::windows_sys::core::PCWSTR, pdn: *mut ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_unbind(ld: *mut ldap) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_unbind_s(ld: *mut ldap) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_value_free(vals: *const ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_value_freeA(vals: *const ::windows_sys::core::PSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_value_freeW(vals: *const ::windows_sys::core::PWSTR) -> u32;
    #[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
    pub fn ldap_value_free_len(vals: *mut *mut LDAP_BERVAL) -> u32;
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub type DBGPRINT = ::core::option::Option<unsafe extern "system" fn(format: ::windows_sys::core::PCSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub type DEREFERENCECONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut ldap, connectiontodereference: *mut ldap) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LAPI_MAJOR_VER1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LAPI_MINOR_VER1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LBER_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LBER_ERROR: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LBER_TRANSLATE_STRINGS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LBER_USE_DER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LBER_USE_INDEFINITE_LEN: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPAPIFeatureInfoA {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: ::windows_sys::core::PSTR,
    pub ldapaif_version: i32,
}
impl ::core::marker::Copy for LDAPAPIFeatureInfoA {}
impl ::core::clone::Clone for LDAPAPIFeatureInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPAPIFeatureInfoW {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: ::windows_sys::core::PWSTR,
    pub ldapaif_version: i32,
}
impl ::core::marker::Copy for LDAPAPIFeatureInfoW {}
impl ::core::clone::Clone for LDAPAPIFeatureInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAPMessage {
    pub lm_msgid: u32,
    pub lm_msgtype: u32,
    pub lm_ber: *mut ::core::ffi::c_void,
    pub lm_chain: *mut LDAPMessage,
    pub lm_next: *mut LDAPMessage,
    pub lm_time: u32,
    pub Connection: *mut ldap,
    pub Request: *mut ::core::ffi::c_void,
    pub lm_returncode: u32,
    pub lm_referral: u16,
    pub lm_chased: super::super::Foundation::BOOLEAN,
    pub lm_eom: super::super::Foundation::BOOLEAN,
    pub ConnectionReferenced: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAPMessage {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAPMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ABANDON_CMD: i32 = 80i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ADD_CMD: i32 = 104i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_API_FEATURE_VIRTUAL_LIST_VIEW: u32 = 1001u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_API_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_API_VERSION: u32 = 2004u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AUTH_OTHERKIND: i32 = 134i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AUTH_SASL: i32 = 131i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AUTH_SIMPLE: i32 = 128i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAP_BERVAL {
    pub bv_len: u32,
    pub bv_val: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for LDAP_BERVAL {}
impl ::core::clone::Clone for LDAP_BERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_BIND_CMD: i32 = 96i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_ADAM_OID: &str = "1.2.840.113556.1.4.1851";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_ADAM_OID_W: &str = "1.2.840.113556.1.4.1851";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_LDAP_INTEG_OID: &str = "1.2.840.113556.1.4.1791";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_LDAP_INTEG_OID_W: &str = "1.2.840.113556.1.4.1791";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_OID: &str = "1.2.840.113556.1.4.800";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_OID_W: &str = "1.2.840.113556.1.4.800";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_PARTIAL_SECRETS_OID: &str = "1.2.840.113556.1.4.1920";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_PARTIAL_SECRETS_OID_W: &str = "1.2.840.113556.1.4.1920";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_V51_OID: &str = "1.2.840.113556.1.4.1670";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_V51_OID_W: &str = "1.2.840.113556.1.4.1670";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_V60_OID: &str = "1.2.840.113556.1.4.1935";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_V60_OID_W: &str = "1.2.840.113556.1.4.1935";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_V61_OID: &str = "1.2.840.113556.1.4.1935";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_V61_OID_W: &str = "1.2.840.113556.1.4.1935";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_V61_R2_OID: &str = "1.2.840.113556.1.4.2080";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_V61_R2_OID_W: &str = "1.2.840.113556.1.4.2080";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_W8_OID: &str = "1.2.840.113556.1.4.2237";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CAP_ACTIVE_DIRECTORY_W8_OID_W: &str = "1.2.840.113556.1.4.2237";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CHASE_EXTERNAL_REFERRALS: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CHASE_SUBORDINATE_REFERRALS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_COMPARE_CMD: i32 = 110i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONTROL_REFERRALS: &str = "1.2.840.113556.1.4.616";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONTROL_REFERRALS_W: &str = "1.2.840.113556.1.4.616";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONTROL_VLVREQUEST: &str = "2.16.840.1.113730.3.4.9";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONTROL_VLVREQUEST_W: &str = "2.16.840.1.113730.3.4.9";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONTROL_VLVRESPONSE: &str = "2.16.840.1.113730.3.4.10";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONTROL_VLVRESPONSE_W: &str = "2.16.840.1.113730.3.4.10";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DELETE_CMD: i32 = 74i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DEREF_ALWAYS: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DEREF_FINDING: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DEREF_NEVER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DEREF_SEARCHING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DIRSYNC_ANCESTORS_FIRST_ORDER: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DIRSYNC_INCREMENTAL_VALUES: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DIRSYNC_OBJECT_SECURITY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DIRSYNC_PUBLIC_DATA_ONLY: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DIRSYNC_ROPAS_DATA_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_EXTENDED_CMD: i32 = 119i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FEATURE_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_AND: u32 = 160u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_APPROX: u32 = 168u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_EQUALITY: u32 = 163u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_EXTENSIBLE: u32 = 169u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_GE: u32 = 165u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_LE: u32 = 166u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_NOT: u32 = 162u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_OR: u32 = 161u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_PRESENT: u32 = 135u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_SUBSTRINGS: u32 = 164u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_GC_PORT: u32 = 3268u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INVALID_CMD: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INVALID_RES: u32 = 255u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MATCHING_RULE_BIT_AND: &str = "1.2.840.113556.1.4.803";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MATCHING_RULE_BIT_AND_W: &str = "1.2.840.113556.1.4.803";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MATCHING_RULE_BIT_OR: &str = "1.2.840.113556.1.4.804";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MATCHING_RULE_BIT_OR_W: &str = "1.2.840.113556.1.4.804";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MATCHING_RULE_DN_BINARY_COMPLEX: &str = "1.2.840.113556.1.4.2253";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MATCHING_RULE_DN_BINARY_COMPLEX_W: &str = "1.2.840.113556.1.4.2253";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MATCHING_RULE_TRANSITIVE_EVALUATION: &str = "1.2.840.113556.1.4.1941";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MATCHING_RULE_TRANSITIVE_EVALUATION_W: &str = "1.2.840.113556.1.4.1941";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MODIFY_CMD: i32 = 102i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MODRDN_CMD: i32 = 108i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MOD_ADD: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MOD_BVALUES: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MOD_DELETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MOD_REPLACE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MSG_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MSG_ONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MSG_RECEIVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_LIMIT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_ABANDON_REPL: &str = "abandonReplication";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_ABANDON_REPL_W: &str = "abandonReplication";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_BECOME_DOM_MASTER: &str = "becomeDomainMaster";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_BECOME_DOM_MASTER_W: &str = "becomeDomainMaster";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_BECOME_PDC: &str = "becomePdc";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_BECOME_PDC_W: &str = "becomePdc";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_BECOME_RID_MASTER: &str = "becomeRidMaster";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_BECOME_RID_MASTER_W: &str = "becomeRidMaster";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_BECOME_SCHEMA_MASTER: &str = "becomeSchemaMaster";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_BECOME_SCHEMA_MASTER_W: &str = "becomeSchemaMaster";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_CONFIG_NAMING_CONTEXT: &str = "configurationNamingContext";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_CONFIG_NAMING_CONTEXT_W: &str = "configurationNamingContext";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_CURRENT_TIME: &str = "currentTime";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_CURRENT_TIME_W: &str = "currentTime";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_DEFAULT_NAMING_CONTEXT: &str = "defaultNamingContext";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_DEFAULT_NAMING_CONTEXT_W: &str = "defaultNamingContext";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_DNS_HOST_NAME: &str = "dnsHostName";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_DNS_HOST_NAME_W: &str = "dnsHostName";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_DO_GARBAGE_COLLECTION: &str = "doGarbageCollection";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_DO_GARBAGE_COLLECTION_W: &str = "doGarbageCollection";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_DS_SERVICE_NAME: &str = "dsServiceName";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_DS_SERVICE_NAME_W: &str = "dsServiceName";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_FIXUP_INHERITANCE: &str = "fixupInheritance";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_FIXUP_INHERITANCE_W: &str = "fixupInheritance";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_HIGHEST_COMMITTED_USN: &str = "highestCommitedUSN";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_HIGHEST_COMMITTED_USN_W: &str = "highestCommitedUSN";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_INVALIDATE_RID_POOL: &str = "invalidateRidPool";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_INVALIDATE_RID_POOL_W: &str = "invalidateRidPool";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_LDAP_SERVICE_NAME: &str = "ldapServiceName";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_LDAP_SERVICE_NAME_W: &str = "ldapServiceName";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_NAMING_CONTEXTS: &str = "namingContexts";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_NAMING_CONTEXTS_W: &str = "namingContexts";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_RECALC_HIERARCHY: &str = "recalcHierarchy";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_RECALC_HIERARCHY_W: &str = "recalcHierarchy";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_ROOT_DOMAIN_NAMING_CONTEXT: &str = "rootDomainNamingContext";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_ROOT_DOMAIN_NAMING_CONTEXT_W: &str = "rootDomainNamingContext";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SCHEMA_NAMING_CONTEXT: &str = "schemaNamingContext";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SCHEMA_NAMING_CONTEXT_W: &str = "schemaNamingContext";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SCHEMA_UPDATE_NOW: &str = "schemaUpdateNow";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SCHEMA_UPDATE_NOW_W: &str = "schemaUpdateNow";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SERVER_NAME: &str = "serverName";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SERVER_NAME_W: &str = "serverName";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUBSCHEMA_SUBENTRY: &str = "subschemaSubentry";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUBSCHEMA_SUBENTRY_W: &str = "subschemaSubentry";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_CAPABILITIES: &str = "supportedCapabilities";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_CAPABILITIES_W: &str = "supportedCapabilities";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_CONTROL: &str = "supportedControl";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_CONTROL_W: &str = "supportedControl";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_LDAP_POLICIES: &str = "supportedLDAPPolicies";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_LDAP_POLICIES_W: &str = "supportedLDAPPolicies";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_LDAP_VERSION: &str = "supportedLDAPVersion";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_LDAP_VERSION_W: &str = "supportedLDAPVersion";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_SASL_MECHANISM: &str = "supportedSASLMechanisms";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPATT_SUPPORTED_SASL_MECHANISM_W: &str = "supportedSASLMechanisms";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_API_FEATURE_INFO: u32 = 21u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_API_INFO: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_AREC_EXCLUSIVE: u32 = 152u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_AUTO_RECONNECT: u32 = 145u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_CACHE_ENABLE: u32 = 15u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_CACHE_FN_PTRS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_CACHE_STRATEGY: u32 = 14u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_CHASE_REFERRALS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_CLIENT_CERTIFICATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_DEREF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_DESC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_DNS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_DNSDOMAIN_NAME: u32 = 59u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_ENCRYPT: u32 = 150u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_ERROR_NUMBER: u32 = 49u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_ERROR_STRING: u32 = 50u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_FAST_CONCURRENT_BIND: u32 = 65u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_GETDSNAME_FLAGS: u32 = 61u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_HOST_NAME: u32 = 48u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_HOST_REACHABLE: u32 = 62u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_IO_FN_PTRS: u32 = 11u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_PING_KEEP_ALIVE: u32 = 54u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_PING_LIMIT: u32 = 56u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_PING_WAIT_TIME: u32 = 55u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_PROMPT_CREDENTIALS: u32 = 63u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_PROTOCOL_VERSION: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_REBIND_ARG: u32 = 7u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_REBIND_FN: u32 = 6u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_REFERRALS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_REFERRAL_CALLBACK: u32 = 112u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_REFERRAL_HOP_LIMIT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_REF_DEREF_CONN_PER_MSG: u32 = 148u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_RESTART: u32 = 9u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_RETURN_REFS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_ROOTDSE_CACHE: u32 = 154u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SASL_METHOD: u32 = 151u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SCH_FLAGS: u32 = 67u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SECURITY_CONTEXT: u32 = 153u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SEND_TIMEOUT: u32 = 66u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SERVER_CERTIFICATE: u32 = 129u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SERVER_ERROR: u32 = 51u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SERVER_EXT_ERROR: u32 = 52u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SIGN: u32 = 149u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SIZELIMIT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SOCKET_BIND_ADDRESSES: u32 = 68u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SSL: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SSL_INFO: u32 = 147u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_SSPI_FLAGS: u32 = 146u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_TCP_KEEPALIVE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_THREAD_FN_PTRS: u32 = 5u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_TIMELIMIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_TLS: u32 = 10u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_TLS_INFO: u32 = 147u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPT_VERSION: u32 = 17u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PAGED_RESULT_OID_STRING: &str = "1.2.840.113556.1.4.319";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PAGED_RESULT_OID_STRING_W: &str = "1.2.840.113556.1.4.319";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_POLICYHINT_APPLY_FULLPWDPOLICY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PORT: u32 = 389u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAP_REFERRAL_CALLBACK {
    pub SizeOfCallbacks: u32,
    pub QueryForConnection: QUERYFORCONNECTION,
    pub NotifyRoutine: NOTIFYOFNEWCONNECTION,
    pub DereferenceRoutine: DEREFERENCECONNECTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAP_REFERRAL_CALLBACK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAP_REFERRAL_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_ADD: i32 = 105i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_ANY: i32 = -1i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_BIND: i32 = 97i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_COMPARE: i32 = 111i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_DELETE: i32 = 107i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_EXTENDED: i32 = 120i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_MODIFY: i32 = 103i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_MODRDN: i32 = 109i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_REFERRAL: i32 = 115i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_SEARCH_ENTRY: i32 = 100i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_SEARCH_RESULT: i32 = 101i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RES_SESSION: i32 = 114i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub type LDAP_RETCODE = i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SUCCESS: LDAP_RETCODE = 0i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPERATIONS_ERROR: LDAP_RETCODE = 1i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PROTOCOL_ERROR: LDAP_RETCODE = 2i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_TIMELIMIT_EXCEEDED: LDAP_RETCODE = 3i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SIZELIMIT_EXCEEDED: LDAP_RETCODE = 4i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_COMPARE_FALSE: LDAP_RETCODE = 5i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_COMPARE_TRUE: LDAP_RETCODE = 6i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AUTH_METHOD_NOT_SUPPORTED: LDAP_RETCODE = 7i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_STRONG_AUTH_REQUIRED: LDAP_RETCODE = 8i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_REFERRAL_V2: LDAP_RETCODE = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PARTIAL_RESULTS: LDAP_RETCODE = 9i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_REFERRAL: LDAP_RETCODE = 10i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ADMIN_LIMIT_EXCEEDED: LDAP_RETCODE = 11i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNAVAILABLE_CRIT_EXTENSION: LDAP_RETCODE = 12i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONFIDENTIALITY_REQUIRED: LDAP_RETCODE = 13i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SASL_BIND_IN_PROGRESS: LDAP_RETCODE = 14i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_SUCH_ATTRIBUTE: LDAP_RETCODE = 16i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNDEFINED_TYPE: LDAP_RETCODE = 17i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INAPPROPRIATE_MATCHING: LDAP_RETCODE = 18i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONSTRAINT_VIOLATION: LDAP_RETCODE = 19i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ATTRIBUTE_OR_VALUE_EXISTS: LDAP_RETCODE = 20i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INVALID_SYNTAX: LDAP_RETCODE = 21i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_SUCH_OBJECT: LDAP_RETCODE = 32i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ALIAS_PROBLEM: LDAP_RETCODE = 33i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INVALID_DN_SYNTAX: LDAP_RETCODE = 34i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_IS_LEAF: LDAP_RETCODE = 35i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ALIAS_DEREF_PROBLEM: LDAP_RETCODE = 36i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INAPPROPRIATE_AUTH: LDAP_RETCODE = 48i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INVALID_CREDENTIALS: LDAP_RETCODE = 49i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INSUFFICIENT_RIGHTS: LDAP_RETCODE = 50i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_BUSY: LDAP_RETCODE = 51i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNAVAILABLE: LDAP_RETCODE = 52i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNWILLING_TO_PERFORM: LDAP_RETCODE = 53i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_LOOP_DETECT: LDAP_RETCODE = 54i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SORT_CONTROL_MISSING: LDAP_RETCODE = 60i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OFFSET_RANGE_ERROR: LDAP_RETCODE = 61i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NAMING_VIOLATION: LDAP_RETCODE = 64i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OBJECT_CLASS_VIOLATION: LDAP_RETCODE = 65i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NOT_ALLOWED_ON_NONLEAF: LDAP_RETCODE = 66i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NOT_ALLOWED_ON_RDN: LDAP_RETCODE = 67i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ALREADY_EXISTS: LDAP_RETCODE = 68i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_OBJECT_CLASS_MODS: LDAP_RETCODE = 69i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RESULTS_TOO_LARGE: LDAP_RETCODE = 70i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AFFECTS_MULTIPLE_DSAS: LDAP_RETCODE = 71i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VIRTUAL_LIST_VIEW_ERROR: LDAP_RETCODE = 76i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OTHER: LDAP_RETCODE = 80i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DOWN: LDAP_RETCODE = 81i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_LOCAL_ERROR: LDAP_RETCODE = 82i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ENCODING_ERROR: LDAP_RETCODE = 83i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DECODING_ERROR: LDAP_RETCODE = 84i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_TIMEOUT: LDAP_RETCODE = 85i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AUTH_UNKNOWN: LDAP_RETCODE = 86i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_ERROR: LDAP_RETCODE = 87i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_USER_CANCELLED: LDAP_RETCODE = 88i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PARAM_ERROR: LDAP_RETCODE = 89i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_MEMORY: LDAP_RETCODE = 90i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONNECT_ERROR: LDAP_RETCODE = 91i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NOT_SUPPORTED: LDAP_RETCODE = 92i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_RESULTS_RETURNED: LDAP_RETCODE = 94i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONTROL_NOT_FOUND: LDAP_RETCODE = 93i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MORE_RESULTS_TO_RETURN: LDAP_RETCODE = 95i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CLIENT_LOOP: LDAP_RETCODE = 96i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_REFERRAL_LIMIT_EXCEEDED: LDAP_RETCODE = 97i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SCOPE_BASE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SCOPE_ONELEVEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SCOPE_SUBTREE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SEARCH_CMD: i32 = 99i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SEARCH_HINT_INDEX_ONLY_OID: &str = "1.2.840.113556.1.4.2207";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SEARCH_HINT_INDEX_ONLY_OID_W: &str = "1.2.840.113556.1.4.2207";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SEARCH_HINT_REQUIRED_INDEX_OID: &str = "1.2.840.113556.1.4.2306";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SEARCH_HINT_REQUIRED_INDEX_OID_W: &str = "1.2.840.113556.1.4.2306";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SEARCH_HINT_SOFT_SIZE_LIMIT_OID: &str = "1.2.840.113556.1.4.2210";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SEARCH_HINT_SOFT_SIZE_LIMIT_OID_W: &str = "1.2.840.113556.1.4.2210";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_ASQ_OID: &str = "1.2.840.113556.1.4.1504";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_ASQ_OID_W: &str = "1.2.840.113556.1.4.1504";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_BATCH_REQUEST_OID: &str = "1.2.840.113556.1.4.2212";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_BATCH_REQUEST_OID_W: &str = "1.2.840.113556.1.4.2212";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_BYPASS_QUOTA_OID: &str = "1.2.840.113556.1.4.2256";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_BYPASS_QUOTA_OID_W: &str = "1.2.840.113556.1.4.2256";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_CROSSDOM_MOVE_TARGET_OID: &str = "1.2.840.113556.1.4.521";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_CROSSDOM_MOVE_TARGET_OID_W: &str = "1.2.840.113556.1.4.521";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DIRSYNC_EX_OID: &str = "1.2.840.113556.1.4.2090";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DIRSYNC_EX_OID_W: &str = "1.2.840.113556.1.4.2090";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DIRSYNC_OID: &str = "1.2.840.113556.1.4.841";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DIRSYNC_OID_W: &str = "1.2.840.113556.1.4.841";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DN_INPUT_OID: &str = "1.2.840.113556.1.4.2026";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DN_INPUT_OID_W: &str = "1.2.840.113556.1.4.2026";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DOMAIN_SCOPE_OID: &str = "1.2.840.113556.1.4.1339";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DOMAIN_SCOPE_OID_W: &str = "1.2.840.113556.1.4.1339";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_EXPECTED_ENTRY_COUNT_OID: &str = "1.2.840.113556.1.4.2211";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_EXPECTED_ENTRY_COUNT_OID_W: &str = "1.2.840.113556.1.4.2211";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_EXTENDED_DN_OID: &str = "1.2.840.113556.1.4.529";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_EXTENDED_DN_OID_W: &str = "1.2.840.113556.1.4.529";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_FAST_BIND_OID: &str = "1.2.840.113556.1.4.1781";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_FAST_BIND_OID_W: &str = "1.2.840.113556.1.4.1781";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_FORCE_UPDATE_OID: &str = "1.2.840.113556.1.4.1974";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_FORCE_UPDATE_OID_W: &str = "1.2.840.113556.1.4.1974";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_GET_STATS_OID: &str = "1.2.840.113556.1.4.970";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_GET_STATS_OID_W: &str = "1.2.840.113556.1.4.970";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_LAZY_COMMIT_OID: &str = "1.2.840.113556.1.4.619";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_LAZY_COMMIT_OID_W: &str = "1.2.840.113556.1.4.619";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_LINK_TTL_OID: &str = "1.2.840.113556.1.4.2309";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_LINK_TTL_OID_W: &str = "1.2.840.113556.1.4.2309";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_NOTIFICATION_OID: &str = "1.2.840.113556.1.4.528";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_NOTIFICATION_OID_W: &str = "1.2.840.113556.1.4.528";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_PERMISSIVE_MODIFY_OID: &str = "1.2.840.113556.1.4.1413";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_PERMISSIVE_MODIFY_OID_W: &str = "1.2.840.113556.1.4.1413";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_POLICY_HINTS_DEPRECATED_OID: &str = "1.2.840.113556.1.4.2066";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_POLICY_HINTS_DEPRECATED_OID_W: &str = "1.2.840.113556.1.4.2066";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_POLICY_HINTS_OID: &str = "1.2.840.113556.1.4.2239";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_POLICY_HINTS_OID_W: &str = "1.2.840.113556.1.4.2239";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_QUOTA_CONTROL_OID: &str = "1.2.840.113556.1.4.1852";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_QUOTA_CONTROL_OID_W: &str = "1.2.840.113556.1.4.1852";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_RANGE_OPTION_OID: &str = "1.2.840.113556.1.4.802";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_RANGE_OPTION_OID_W: &str = "1.2.840.113556.1.4.802";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_RANGE_RETRIEVAL_NOERR_OID: &str = "1.2.840.113556.1.4.1948";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_RANGE_RETRIEVAL_NOERR_OID_W: &str = "1.2.840.113556.1.4.1948";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_RESP_SORT_OID: &str = "1.2.840.113556.1.4.474";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_RESP_SORT_OID_W: &str = "1.2.840.113556.1.4.474";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SD_FLAGS_OID: &str = "1.2.840.113556.1.4.801";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SD_FLAGS_OID_W: &str = "1.2.840.113556.1.4.801";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SEARCH_HINTS_OID: &str = "1.2.840.113556.1.4.2206";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SEARCH_HINTS_OID_W: &str = "1.2.840.113556.1.4.2206";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SEARCH_OPTIONS_OID: &str = "1.2.840.113556.1.4.1340";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SEARCH_OPTIONS_OID_W: &str = "1.2.840.113556.1.4.1340";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SET_OWNER_OID: &str = "1.2.840.113556.1.4.2255";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SET_OWNER_OID_W: &str = "1.2.840.113556.1.4.2255";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SHOW_DEACTIVATED_LINK_OID: &str = "1.2.840.113556.1.4.2065";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SHOW_DEACTIVATED_LINK_OID_W: &str = "1.2.840.113556.1.4.2065";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SHOW_DELETED_OID: &str = "1.2.840.113556.1.4.417";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SHOW_DELETED_OID_W: &str = "1.2.840.113556.1.4.417";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SHOW_RECYCLED_OID: &str = "1.2.840.113556.1.4.2064";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SHOW_RECYCLED_OID_W: &str = "1.2.840.113556.1.4.2064";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SHUTDOWN_NOTIFY_OID: &str = "1.2.840.113556.1.4.1907";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SHUTDOWN_NOTIFY_OID_W: &str = "1.2.840.113556.1.4.1907";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SORT_OID: &str = "1.2.840.113556.1.4.473";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_SORT_OID_W: &str = "1.2.840.113556.1.4.473";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_TREE_DELETE_EX_OID: &str = "1.2.840.113556.1.4.2204";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_TREE_DELETE_EX_OID_W: &str = "1.2.840.113556.1.4.2204";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_TREE_DELETE_OID: &str = "1.2.840.113556.1.4.805";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_TREE_DELETE_OID_W: &str = "1.2.840.113556.1.4.805";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_UPDATE_STATS_OID: &str = "1.2.840.113556.1.4.2205";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_UPDATE_STATS_OID_W: &str = "1.2.840.113556.1.4.2205";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_VERIFY_NAME_OID: &str = "1.2.840.113556.1.4.1338";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_VERIFY_NAME_OID_W: &str = "1.2.840.113556.1.4.1338";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_WHO_AM_I_OID: &str = "1.3.6.1.4.1.4203.1.11.3";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_WHO_AM_I_OID_W: &str = "1.3.6.1.4.1.4203.1.11.3";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SESSION_CMD: i32 = 113i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SSL_GC_PORT: u32 = 3269u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SSL_PORT: u32 = 636u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_START_TLS_OID: &str = "1.3.6.1.4.1.1466.20037";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_START_TLS_OID_W: &str = "1.3.6.1.4.1.1466.20037";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SUBSTRING_ANY: i32 = 129i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SUBSTRING_FINAL: i32 = 130i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SUBSTRING_INITIAL: i32 = 128i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAP_TIMEVAL {
    pub tv_sec: i32,
    pub tv_usec: i32,
}
impl ::core::marker::Copy for LDAP_TIMEVAL {}
impl ::core::clone::Clone for LDAP_TIMEVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_TTL_EXTENDED_OP_OID: &str = "1.3.6.1.4.1.1466.101.119.1";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_TTL_EXTENDED_OP_OID_W: &str = "1.3.6.1.4.1.1466.101.119.1";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNBIND_CMD: i32 = 66i32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNICODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UPDATE_STATS_INVOCATIONID_OID: &str = "1.2.840.113556.1.4.2209";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UPDATE_STATS_INVOCATIONID_OID_W: &str = "1.2.840.113556.1.4.2209";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UPDATE_STATS_USN_OID: &str = "1.2.840.113556.1.4.2208";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UPDATE_STATS_USN_OID_W: &str = "1.2.840.113556.1.4.2208";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VENDOR_NAME: &str = "Microsoft Corporation.";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VENDOR_NAME_W: &str = "Microsoft Corporation.";
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VENDOR_VERSION: u32 = 510u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VERSION1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VERSION2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VERSION3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VERSION_MAX: u32 = 3u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VERSION_MIN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VLVINFO_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type NOTIFYOFNEWCONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut ldap, referralfromconnection: *mut ldap, newdn: ::windows_sys::core::PCWSTR, hostname: ::windows_sys::core::PCSTR, newconnection: *mut ldap, portnumber: u32, secauthidentity: *mut ::core::ffi::c_void, currentuser: *mut ::core::ffi::c_void, errorcodefrombind: u32) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub type QUERYCLIENTCERT = ::core::option::Option<unsafe extern "system" fn(connection: *mut ldap, trusted_cas: *mut super::super::Security::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertificate: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub type QUERYFORCONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut ldap, referralfromconnection: *mut ldap, newdn: ::windows_sys::core::PCWSTR, hostname: ::windows_sys::core::PCSTR, portnumber: u32, secauthidentity: *mut ::core::ffi::c_void, currentusertoken: *mut ::core::ffi::c_void, connectiontouse: *mut *mut ldap) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const SERVER_SEARCH_FLAG_DOMAIN_SCOPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const SERVER_SEARCH_FLAG_PHANTOM_ROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type VERIFYSERVERCERT = ::core::option::Option<unsafe extern "system" fn(connection: *mut ldap, pservercert: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct berelement {
    pub opaque: ::windows_sys::core::PSTR,
}
impl ::core::marker::Copy for berelement {}
impl ::core::clone::Clone for berelement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct ldap {
    pub ld_sb: ldap_0,
    pub ld_host: ::windows_sys::core::PSTR,
    pub ld_version: u32,
    pub ld_lberoptions: u8,
    pub ld_deref: u32,
    pub ld_timelimit: u32,
    pub ld_sizelimit: u32,
    pub ld_errno: u32,
    pub ld_matched: ::windows_sys::core::PSTR,
    pub ld_error: ::windows_sys::core::PSTR,
    pub ld_msgid: u32,
    pub Reserved3: [u8; 25],
    pub ld_cldaptries: u32,
    pub ld_cldaptimeout: u32,
    pub ld_refhoplimit: u32,
    pub ld_options: u32,
}
impl ::core::marker::Copy for ldap {}
impl ::core::clone::Clone for ldap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct ldap_0 {
    pub sb_sd: usize,
    pub Reserved1: [u8; 41],
    pub sb_naddr: usize,
    pub Reserved2: [u8; 24],
}
impl ::core::marker::Copy for ldap_0 {}
impl ::core::clone::Clone for ldap_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct ldap_version_info {
    pub lv_size: u32,
    pub lv_major: u32,
    pub lv_minor: u32,
}
impl ::core::marker::Copy for ldap_version_info {}
impl ::core::clone::Clone for ldap_version_info {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct ldapapiinfoA {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut *mut i8,
    pub ldapai_vendor_name: ::windows_sys::core::PSTR,
    pub ldapai_vendor_version: i32,
}
impl ::core::marker::Copy for ldapapiinfoA {}
impl ::core::clone::Clone for ldapapiinfoA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct ldapapiinfoW {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut ::windows_sys::core::PWSTR,
    pub ldapai_vendor_name: ::windows_sys::core::PWSTR,
    pub ldapai_vendor_version: i32,
}
impl ::core::marker::Copy for ldapapiinfoW {}
impl ::core::clone::Clone for ldapapiinfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapcontrolA {
    pub ldctl_oid: ::windows_sys::core::PSTR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapcontrolA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapcontrolA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapcontrolW {
    pub ldctl_oid: ::windows_sys::core::PWSTR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapcontrolW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapcontrolW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct ldapmodA {
    pub mod_op: u32,
    pub mod_type: ::windows_sys::core::PSTR,
    pub mod_vals: ldapmodA_0,
}
impl ::core::marker::Copy for ldapmodA {}
impl ::core::clone::Clone for ldapmodA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub union ldapmodA_0 {
    pub modv_strvals: *mut ::windows_sys::core::PSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
impl ::core::marker::Copy for ldapmodA_0 {}
impl ::core::clone::Clone for ldapmodA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct ldapmodW {
    pub mod_op: u32,
    pub mod_type: ::windows_sys::core::PWSTR,
    pub mod_vals: ldapmodW_0,
}
impl ::core::marker::Copy for ldapmodW {}
impl ::core::clone::Clone for ldapmodW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub union ldapmodW_0 {
    pub modv_strvals: *mut ::windows_sys::core::PWSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
impl ::core::marker::Copy for ldapmodW_0 {}
impl ::core::clone::Clone for ldapmodW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ldapsearch(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapsortkeyA {
    pub sk_attrtype: ::windows_sys::core::PSTR,
    pub sk_matchruleoid: ::windows_sys::core::PSTR,
    pub sk_reverseorder: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapsortkeyA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapsortkeyA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapsortkeyW {
    pub sk_attrtype: ::windows_sys::core::PWSTR,
    pub sk_matchruleoid: ::windows_sys::core::PWSTR,
    pub sk_reverseorder: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapsortkeyW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapsortkeyW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct ldapvlvinfo {
    pub ldvlv_version: i32,
    pub ldvlv_before_count: u32,
    pub ldvlv_after_count: u32,
    pub ldvlv_offset: u32,
    pub ldvlv_count: u32,
    pub ldvlv_attrvalue: *mut LDAP_BERVAL,
    pub ldvlv_context: *mut LDAP_BERVAL,
    pub ldvlv_extradata: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for ldapvlvinfo {}
impl ::core::clone::Clone for ldapvlvinfo {
    fn clone(&self) -> Self {
        *self
    }
}
