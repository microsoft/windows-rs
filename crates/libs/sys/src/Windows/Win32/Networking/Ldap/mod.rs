#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_Networking_Ldap'*"]
    pub fn LdapGetLastError() -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap'*"]
    pub fn LdapMapErrorToWin32(ldaperror: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LdapUTF8ToUnicode(lpsrcstr: super::super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::super::Foundation::PWSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LdapUnicodeToUTF8(lpsrcstr: super::super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::super::Foundation::PSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_alloc_t(options: i32) -> *mut berelement;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvdup(pberval: *mut LDAP_BERVAL) -> *mut LDAP_BERVAL;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvecfree(pberval: *mut *mut LDAP_BERVAL);
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvfree(bv: *mut LDAP_BERVAL);
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_first_element(pberelement: *mut berelement, plen: *mut u32, ppopaque: *mut *mut super::super::Foundation::CHAR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_flatten(pberelement: *mut berelement, pberval: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_free(pberelement: *mut berelement, fbuf: i32);
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_init(pberval: *mut LDAP_BERVAL) -> *mut berelement;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_next_element(pberelement: *mut berelement, plen: *mut u32, opaque: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_peek_tag(pberelement: *mut berelement, plen: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_printf(pberelement: *mut berelement, fmt: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_scanf(pberelement: *mut berelement, fmt: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_skip_tag(pberelement: *mut berelement, plen: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_open(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_openA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_openW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_abandon(ld: *mut ldap, msgid: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_addA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_addW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bindA(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bindW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, cred: super::super::Foundation::PWSTR, method: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, cred: super::super::Foundation::PWSTR, method: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_check_filterA(ld: *mut ldap, searchfilter: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_check_filterW(ld: *mut ldap, searchfilter: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_cleanup(hinstance: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_close_extended_op(ld: *mut ldap, messagenumber: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compareA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compareW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_conn_from_msg(primaryconn: *mut ldap, res: *mut LDAPMessage) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_connect(ld: *mut ldap, timeout: *mut LDAP_TIMEVAL) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_free(control: *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_freeA(controls: *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_freeW(control: *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_free(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_freeA(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_freeW(control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_entries(ld: *mut ldap, res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_references(ld: *mut ldap, res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_values(vals: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_valuesA(vals: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_valuesW(vals: *const super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_values_len(vals: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_control(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_controlA(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_controlW(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_control(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_vlv_controlA(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolA) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_vlv_controlW(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolW) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_deleteA(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_deleteW(ld: *mut ldap, dn: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_s(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufn(dn: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufnA(dn: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufnW(dn: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_encode_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, control: *mut ldapcontrolA, criticality: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_encode_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, control: *mut ldapcontrolW, criticality: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2string(err: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2stringA(err: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2stringW(err: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_element(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PSTR, destlength: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_elementA(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PSTR, destlength: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_elementW(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PWSTR, destlength: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dn(dn: super::super::Foundation::PSTR, notypes: u32) -> *mut super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dnA(dn: super::super::Foundation::PSTR, notypes: u32) -> *mut super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dnW(dn: super::super::Foundation::PWSTR, notypes: u32) -> *mut super::super::Foundation::PWSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation(ld: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operationA(ld: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operationW(ld: *mut ldap, oid: super::super::Foundation::PWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation_sA(externalhandle: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, returnedoid: *mut super::super::Foundation::PSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation_sW(externalhandle: *mut ldap, oid: super::super::Foundation::PWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, returnedoid: *mut super::super::Foundation::PWSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_entry(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_reference(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controls(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controlsA(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controlsW(controls: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dn(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dnA(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dnW(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_next_page(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, pagesize: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_next_page_s(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, timeout: *mut LDAP_TIMEVAL, pagesize: u32, totalcount: *mut u32, results: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_option(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_optionW(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_paged_count(externalhandle: *mut ldap, searchblock: *mut ldapsearch, totalcount: *mut u32, results: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_valuesA(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_valuesW(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PWSTR) -> *mut super::super::Foundation::PWSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_len(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_lenA(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_lenW(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PWSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_init(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_initA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_initW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfree(block: super::super::Foundation::PSTR);
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfreeA(block: super::super::Foundation::PSTR);
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfreeW(block: super::super::Foundation::PWSTR);
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modifyA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modifyW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2A(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2W(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_s(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_sA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_sW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdnA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdnW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_s(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_sA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_sW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_msgfree(res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_entry(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_reference(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_open(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_openA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_openW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_extended_resultA(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut super::super::Foundation::PSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_extended_resultW(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut super::super::Foundation::PWSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_control(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_controlA(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_controlW(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolW, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_reference(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_referenceA(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_referenceW(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_result(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PSTR, errormessage: *mut super::super::Foundation::PSTR, referrals: *mut *mut super::super::Foundation::PSTR, servercontrols: *mut *mut *mut ldapcontrolA, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_resultA(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PSTR, errormessage: *mut super::super::Foundation::PSTR, referrals: *mut *mut *mut i8, servercontrols: *mut *mut *mut ldapcontrolA, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_resultW(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PWSTR, errormessage: *mut super::super::Foundation::PWSTR, referrals: *mut *mut *mut u16, servercontrols: *mut *mut *mut ldapcontrolW, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_control(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, result: *mut u32, attribute: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_vlv_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_vlv_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_perror(ld: *mut ldap, msg: super::super::Foundation::PSTR);
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, newrdn: super::super::Foundation::PWSTR, newparent: super::super::Foundation::PWSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, newrdn: super::super::Foundation::PWSTR, newparent: super::super::Foundation::PWSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_result(ld: *mut ldap, msgid: u32, all: u32, timeout: *const LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_result2error(ld: *mut ldap, res: *mut LDAPMessage, freeit: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bindA(externalhandle: *mut ldap, distname: super::super::Foundation::PSTR, authmechanism: super::super::Foundation::PSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, messagenumber: *mut i32) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bindW(externalhandle: *mut ldap, distname: super::super::Foundation::PWSTR, authmechanism: super::super::Foundation::PWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, messagenumber: *mut i32) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bind_sA(externalhandle: *mut ldap, distname: super::super::Foundation::PSTR, authmechanism: super::super::Foundation::PSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, serverdata: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bind_sW(externalhandle: *mut ldap, distname: super::super::Foundation::PWSTR, authmechanism: super::super::Foundation::PWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, serverdata: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_searchA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_searchW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_abandon_page(externalhandle: *mut ldap, searchblock: *mut ldapsearch) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_extA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_extW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_s(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_sA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_sW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_page(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PSTR, attributelist: *mut *mut i8, attributesonly: u32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyA) -> *mut ldapsearch;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_pageA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PSTR, attributelist: *const *const i8, attributesonly: u32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyA) -> *mut ldapsearch;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_pageW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PWSTR, attributelist: *const *const u16, attributesonly: u32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyW) -> *mut ldapsearch;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_s(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_sA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_sW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_st(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_stA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_stW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap'*"]
    pub fn ldap_set_dbg_flags(newflags: u32) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_dbg_routine(debugprintroutine: DBGPRINT);
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_option(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_optionW(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bindA(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bindW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinit(hostname: super::super::Foundation::PSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinitA(hostname: super::super::Foundation::PSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinitW(hostname: super::super::Foundation::PWSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_start_tls_sA(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_start_tls_sW(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_startup(version: *mut ldap_version_info, instance: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_stop_tls_s(externalhandle: *mut ldap) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dn(ufn: super::super::Foundation::PSTR, pdn: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dnA(ufn: super::super::Foundation::PSTR, pdn: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dnW(ufn: super::super::Foundation::PWSTR, pdn: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_unbind(ld: *mut ldap) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_unbind_s(ld: *mut ldap) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_free(vals: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_freeA(vals: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_freeW(vals: *const super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_free_len(vals: *mut *mut LDAP_BERVAL) -> u32;
}
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type DBGPRINT = ::core::option::Option<unsafe extern "system" fn(format: super::super::Foundation::PSTR) -> u32>;
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type DEREFERENCECONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut ldap, connectiontodereference: *mut ldap) -> u32>;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LAPI_MAJOR_VER1: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LAPI_MINOR_VER1: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LBER_DEFAULT: i32 = -1i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LBER_ERROR: i32 = -1i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LBER_TRANSLATE_STRINGS: u32 = 4u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LBER_USE_DER: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LBER_USE_INDEFINITE_LEN: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAPAPIFeatureInfoA {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: super::super::Foundation::PSTR,
    pub ldapaif_version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAPAPIFeatureInfoA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAPAPIFeatureInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAPAPIFeatureInfoW {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: super::super::Foundation::PWSTR,
    pub ldapaif_version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAPAPIFeatureInfoW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAPAPIFeatureInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_ABANDON_CMD: i32 = 80i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_ADD_CMD: i32 = 104i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_API_FEATURE_VIRTUAL_LIST_VIEW: u32 = 1001u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_API_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_API_VERSION: u32 = 2004u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_AUTH_OTHERKIND: i32 = 134i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_AUTH_SASL: i32 = 131i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_AUTH_SIMPLE: i32 = 128i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAP_BERVAL {
    pub bv_len: u32,
    pub bv_val: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAP_BERVAL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAP_BERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_BIND_CMD: i32 = 96i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_CHASE_EXTERNAL_REFERRALS: u32 = 64u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_CHASE_SUBORDINATE_REFERRALS: u32 = 32u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_COMPARE_CMD: i32 = 110i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DELETE_CMD: i32 = 74i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DEREF_ALWAYS: u32 = 3u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DEREF_FINDING: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DEREF_NEVER: u32 = 0u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DEREF_SEARCHING: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DIRSYNC_ANCESTORS_FIRST_ORDER: u32 = 2048u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DIRSYNC_INCREMENTAL_VALUES: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DIRSYNC_OBJECT_SECURITY: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DIRSYNC_PUBLIC_DATA_ONLY: u32 = 8192u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DIRSYNC_ROPAS_DATA_ONLY: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_EXTENDED_CMD: i32 = 119i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FEATURE_INFO_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_AND: u32 = 160u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_APPROX: u32 = 168u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_EQUALITY: u32 = 163u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_EXTENSIBLE: u32 = 169u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_GE: u32 = 165u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_LE: u32 = 166u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_NOT: u32 = 162u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_OR: u32 = 161u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_PRESENT: u32 = 135u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_SUBSTRINGS: u32 = 164u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_GC_PORT: u32 = 3268u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_INVALID_CMD: u32 = 255u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_INVALID_RES: u32 = 255u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MODIFY_CMD: i32 = 102i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MODRDN_CMD: i32 = 108i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MOD_ADD: u32 = 0u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MOD_BVALUES: u32 = 128u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MOD_DELETE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MOD_REPLACE: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MSG_ALL: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MSG_ONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MSG_RECEIVED: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NO_LIMIT: u32 = 0u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_API_FEATURE_INFO: u32 = 21u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_API_INFO: u32 = 0u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_AREC_EXCLUSIVE: u32 = 152u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_AUTO_RECONNECT: u32 = 145u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_CACHE_ENABLE: u32 = 15u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_CACHE_FN_PTRS: u32 = 13u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_CACHE_STRATEGY: u32 = 14u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_CHASE_REFERRALS: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_CLIENT_CERTIFICATE: u32 = 128u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_DEREF: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_DESC: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_DNS: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_DNSDOMAIN_NAME: u32 = 59u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_ENCRYPT: u32 = 150u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_ERROR_NUMBER: u32 = 49u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_ERROR_STRING: u32 = 50u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_FAST_CONCURRENT_BIND: u32 = 65u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_GETDSNAME_FLAGS: u32 = 61u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_HOST_NAME: u32 = 48u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_HOST_REACHABLE: u32 = 62u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_IO_FN_PTRS: u32 = 11u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_PING_KEEP_ALIVE: u32 = 54u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_PING_LIMIT: u32 = 56u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_PING_WAIT_TIME: u32 = 55u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_PROMPT_CREDENTIALS: u32 = 63u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_PROTOCOL_VERSION: u32 = 17u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_REBIND_ARG: u32 = 7u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_REBIND_FN: u32 = 6u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_REFERRALS: u32 = 8u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_REFERRAL_CALLBACK: u32 = 112u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_REFERRAL_HOP_LIMIT: u32 = 16u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_REF_DEREF_CONN_PER_MSG: u32 = 148u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_RESTART: u32 = 9u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_RETURN_REFS: u32 = 4u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_ROOTDSE_CACHE: u32 = 154u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SASL_METHOD: u32 = 151u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SCH_FLAGS: u32 = 67u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SECURITY_CONTEXT: u32 = 153u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SEND_TIMEOUT: u32 = 66u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SERVER_CERTIFICATE: u32 = 129u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SERVER_ERROR: u32 = 51u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SERVER_EXT_ERROR: u32 = 52u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SIGN: u32 = 149u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SIZELIMIT: u32 = 3u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SOCKET_BIND_ADDRESSES: u32 = 68u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SSL: u32 = 10u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SSL_INFO: u32 = 147u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_SSPI_FLAGS: u32 = 146u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_TCP_KEEPALIVE: u32 = 64u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_THREAD_FN_PTRS: u32 = 5u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_TIMELIMIT: u32 = 4u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_TLS: u32 = 10u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_TLS_INFO: u32 = 147u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPT_VERSION: u32 = 17u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_POLICYHINT_APPLY_FULLPWDPOLICY: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_PORT: u32 = 389u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_ADD: i32 = 105i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_ANY: i32 = -1i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_BIND: i32 = 97i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_COMPARE: i32 = 111i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_DELETE: i32 = 107i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_EXTENDED: i32 = 120i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_MODIFY: i32 = 103i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_MODRDN: i32 = 109i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_REFERRAL: i32 = 115i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_SEARCH_ENTRY: i32 = 100i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_SEARCH_RESULT: i32 = 101i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RES_SESSION: i32 = 114i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub type LDAP_RETCODE = i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SUCCESS: LDAP_RETCODE = 0i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OPERATIONS_ERROR: LDAP_RETCODE = 1i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_PROTOCOL_ERROR: LDAP_RETCODE = 2i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_TIMELIMIT_EXCEEDED: LDAP_RETCODE = 3i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SIZELIMIT_EXCEEDED: LDAP_RETCODE = 4i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_COMPARE_FALSE: LDAP_RETCODE = 5i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_COMPARE_TRUE: LDAP_RETCODE = 6i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_AUTH_METHOD_NOT_SUPPORTED: LDAP_RETCODE = 7i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_STRONG_AUTH_REQUIRED: LDAP_RETCODE = 8i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_REFERRAL_V2: LDAP_RETCODE = 9i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_PARTIAL_RESULTS: LDAP_RETCODE = 9i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_REFERRAL: LDAP_RETCODE = 10i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_ADMIN_LIMIT_EXCEEDED: LDAP_RETCODE = 11i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_UNAVAILABLE_CRIT_EXTENSION: LDAP_RETCODE = 12i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_CONFIDENTIALITY_REQUIRED: LDAP_RETCODE = 13i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SASL_BIND_IN_PROGRESS: LDAP_RETCODE = 14i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NO_SUCH_ATTRIBUTE: LDAP_RETCODE = 16i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_UNDEFINED_TYPE: LDAP_RETCODE = 17i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_INAPPROPRIATE_MATCHING: LDAP_RETCODE = 18i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_CONSTRAINT_VIOLATION: LDAP_RETCODE = 19i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_ATTRIBUTE_OR_VALUE_EXISTS: LDAP_RETCODE = 20i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_INVALID_SYNTAX: LDAP_RETCODE = 21i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NO_SUCH_OBJECT: LDAP_RETCODE = 32i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_ALIAS_PROBLEM: LDAP_RETCODE = 33i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_INVALID_DN_SYNTAX: LDAP_RETCODE = 34i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_IS_LEAF: LDAP_RETCODE = 35i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_ALIAS_DEREF_PROBLEM: LDAP_RETCODE = 36i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_INAPPROPRIATE_AUTH: LDAP_RETCODE = 48i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_INVALID_CREDENTIALS: LDAP_RETCODE = 49i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_INSUFFICIENT_RIGHTS: LDAP_RETCODE = 50i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_BUSY: LDAP_RETCODE = 51i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_UNAVAILABLE: LDAP_RETCODE = 52i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_UNWILLING_TO_PERFORM: LDAP_RETCODE = 53i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_LOOP_DETECT: LDAP_RETCODE = 54i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SORT_CONTROL_MISSING: LDAP_RETCODE = 60i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OFFSET_RANGE_ERROR: LDAP_RETCODE = 61i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NAMING_VIOLATION: LDAP_RETCODE = 64i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OBJECT_CLASS_VIOLATION: LDAP_RETCODE = 65i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NOT_ALLOWED_ON_NONLEAF: LDAP_RETCODE = 66i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NOT_ALLOWED_ON_RDN: LDAP_RETCODE = 67i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_ALREADY_EXISTS: LDAP_RETCODE = 68i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NO_OBJECT_CLASS_MODS: LDAP_RETCODE = 69i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_RESULTS_TOO_LARGE: LDAP_RETCODE = 70i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_AFFECTS_MULTIPLE_DSAS: LDAP_RETCODE = 71i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VIRTUAL_LIST_VIEW_ERROR: LDAP_RETCODE = 76i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_OTHER: LDAP_RETCODE = 80i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SERVER_DOWN: LDAP_RETCODE = 81i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_LOCAL_ERROR: LDAP_RETCODE = 82i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_ENCODING_ERROR: LDAP_RETCODE = 83i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_DECODING_ERROR: LDAP_RETCODE = 84i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_TIMEOUT: LDAP_RETCODE = 85i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_AUTH_UNKNOWN: LDAP_RETCODE = 86i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_FILTER_ERROR: LDAP_RETCODE = 87i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_USER_CANCELLED: LDAP_RETCODE = 88i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_PARAM_ERROR: LDAP_RETCODE = 89i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NO_MEMORY: LDAP_RETCODE = 90i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_CONNECT_ERROR: LDAP_RETCODE = 91i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NOT_SUPPORTED: LDAP_RETCODE = 92i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_NO_RESULTS_RETURNED: LDAP_RETCODE = 94i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_CONTROL_NOT_FOUND: LDAP_RETCODE = 93i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_MORE_RESULTS_TO_RETURN: LDAP_RETCODE = 95i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_CLIENT_LOOP: LDAP_RETCODE = 96i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_REFERRAL_LIMIT_EXCEEDED: LDAP_RETCODE = 97i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SCOPE_BASE: u32 = 0u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SCOPE_ONELEVEL: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SCOPE_SUBTREE: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SEARCH_CMD: i32 = 99i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SESSION_CMD: i32 = 113i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SSL_GC_PORT: u32 = 3269u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SSL_PORT: u32 = 636u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SUBSTRING_ANY: i32 = 129i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SUBSTRING_FINAL: i32 = 130i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_SUBSTRING_INITIAL: i32 = 128i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
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
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_UNBIND_CMD: i32 = 66i32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_UNICODE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VENDOR_VERSION: u32 = 510u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VERSION: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VERSION1: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VERSION2: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VERSION3: u32 = 3u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VERSION_MAX: u32 = 3u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VERSION_MIN: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const LDAP_VLVINFO_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type NOTIFYOFNEWCONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut ldap, referralfromconnection: *mut ldap, newdn: super::super::Foundation::PWSTR, hostname: super::super::Foundation::PSTR, newconnection: *mut ldap, portnumber: u32, secauthidentity: *mut ::core::ffi::c_void, currentuser: *mut ::core::ffi::c_void, errorcodefrombind: u32) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation', 'Win32_Security_Authentication_Identity', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub type QUERYCLIENTCERT = ::core::option::Option<unsafe extern "system" fn(connection: *mut ldap, trusted_cas: *mut super::super::Security::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertificate: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type QUERYFORCONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut ldap, referralfromconnection: *mut ldap, newdn: super::super::Foundation::PWSTR, hostname: super::super::Foundation::PSTR, portnumber: u32, secauthidentity: *mut ::core::ffi::c_void, currentusertoken: *mut ::core::ffi::c_void, connectiontouse: *mut *mut ldap) -> u32>;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const SERVER_SEARCH_FLAG_DOMAIN_SCOPE: u32 = 1u32;
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
pub const SERVER_SEARCH_FLAG_PHANTOM_ROOT: u32 = 2u32;
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation', 'Win32_Security_Cryptography'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type VERIFYSERVERCERT = ::core::option::Option<unsafe extern "system" fn(connection: *mut ldap, pservercert: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN>;
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct berelement {
    pub opaque: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for berelement {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for berelement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldap {
    pub ld_sb: ldap_0,
    pub ld_host: super::super::Foundation::PSTR,
    pub ld_version: u32,
    pub ld_lberoptions: u8,
    pub ld_deref: u32,
    pub ld_timelimit: u32,
    pub ld_sizelimit: u32,
    pub ld_errno: u32,
    pub ld_matched: super::super::Foundation::PSTR,
    pub ld_error: super::super::Foundation::PSTR,
    pub ld_msgid: u32,
    pub Reserved3: [u8; 25],
    pub ld_cldaptries: u32,
    pub ld_cldaptimeout: u32,
    pub ld_refhoplimit: u32,
    pub ld_options: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldap {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldap {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldap_0 {
    pub sb_sd: usize,
    pub Reserved1: [u8; 41],
    pub sb_naddr: usize,
    pub Reserved2: [u8; 24],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldap_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldap_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap'*"]
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
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapapiinfoA {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut *mut i8,
    pub ldapai_vendor_name: super::super::Foundation::PSTR,
    pub ldapai_vendor_version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapapiinfoA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapapiinfoA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapapiinfoW {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut super::super::Foundation::PWSTR,
    pub ldapai_vendor_name: super::super::Foundation::PWSTR,
    pub ldapai_vendor_version: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapapiinfoW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapapiinfoW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapcontrolA {
    pub ldctl_oid: super::super::Foundation::PSTR,
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
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapcontrolW {
    pub ldctl_oid: super::super::Foundation::PWSTR,
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
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapmodA {
    pub mod_op: u32,
    pub mod_type: super::super::Foundation::PSTR,
    pub mod_vals: ldapmodA_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapmodA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapmodA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union ldapmodA_0 {
    pub modv_strvals: *mut super::super::Foundation::PSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapmodA_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapmodA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapmodW {
    pub mod_op: u32,
    pub mod_type: super::super::Foundation::PWSTR,
    pub mod_vals: ldapmodW_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapmodW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapmodW {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub union ldapmodW_0 {
    pub modv_strvals: *mut super::super::Foundation::PWSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapmodW_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapmodW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct ldapsearch(pub u8);
#[repr(C)]
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapsortkeyA {
    pub sk_attrtype: super::super::Foundation::PSTR,
    pub sk_matchruleoid: super::super::Foundation::PSTR,
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
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ldapsortkeyW {
    pub sk_attrtype: super::super::Foundation::PWSTR,
    pub sk_matchruleoid: super::super::Foundation::PWSTR,
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
#[doc = "*Required features: 'Win32_Networking_Ldap', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ldapvlvinfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ldapvlvinfo {
    fn clone(&self) -> Self {
        *self
    }
}
