#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_Ldap`*"]
    pub fn LdapGetLastError() -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`*"]
    pub fn LdapMapErrorToWin32(ldaperror: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LdapUTF8ToUnicode(lpsrcstr: super::super::Foundation::PSTR, cchsrc: i32, lpdeststr: super::super::Foundation::PWSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LdapUnicodeToUTF8(lpsrcstr: super::super::Foundation::PWSTR, cchsrc: i32, lpdeststr: super::super::Foundation::PSTR, cchdest: i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_alloc_t(options: i32) -> *mut berelement;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvdup(pberval: *mut LDAP_BERVAL) -> *mut LDAP_BERVAL;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvecfree(pberval: *mut *mut LDAP_BERVAL);
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvfree(bv: *mut LDAP_BERVAL);
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_first_element(pberelement: *mut berelement, plen: *mut u32, ppopaque: *mut *mut super::super::Foundation::CHAR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_flatten(pberelement: *mut berelement, pberval: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_free(pberelement: *mut berelement, fbuf: i32);
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_init(pberval: *mut LDAP_BERVAL) -> *mut berelement;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_next_element(pberelement: *mut berelement, plen: *mut u32, opaque: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_peek_tag(pberelement: *mut berelement, plen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_printf(pberelement: *mut berelement, fmt: super::super::Foundation::PSTR) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_scanf(pberelement: *mut berelement, fmt: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_skip_tag(pberelement: *mut berelement, plen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_open(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_openA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_openW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_abandon(ld: *mut ldap, msgid: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_addA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_addW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attrs: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attrs: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bindA(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bindW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, cred: super::super::Foundation::PWSTR, method: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, cred: super::super::Foundation::PSTR, method: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, cred: super::super::Foundation::PWSTR, method: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_check_filterA(ld: *mut ldap, searchfilter: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_check_filterW(ld: *mut ldap, searchfilter: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_cleanup(hinstance: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_close_extended_op(ld: *mut ldap, messagenumber: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compareA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compareW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, attr: super::super::Foundation::PSTR, value: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, attr: super::super::Foundation::PWSTR, value: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_conn_from_msg(primaryconn: *mut ldap, res: *mut LDAPMessage) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_connect(ld: *mut ldap, timeout: *mut LDAP_TIMEVAL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_free(control: *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_freeA(controls: *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_freeW(control: *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_free(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_freeA(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_freeW(control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_entries(ld: *mut ldap, res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_references(ld: *mut ldap, res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_values(vals: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_valuesA(vals: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_valuesW(vals: *const super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_values_len(vals: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_control(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_controlA(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_controlW(externalhandle: *mut ldap, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_control(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, iscritical: u8, control: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, iscritical: u8, control: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_vlv_controlA(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolA) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_vlv_controlW(externalhandle: *mut ldap, vlvinfo: *mut ldapvlvinfo, iscritical: u8, control: *mut *mut ldapcontrolW) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_deleteA(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_deleteW(ld: *mut ldap, dn: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_s(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufn(dn: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufnA(dn: super::super::Foundation::PSTR) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufnW(dn: super::super::Foundation::PWSTR) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_encode_sort_controlA(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyA, control: *mut ldapcontrolA, criticality: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_encode_sort_controlW(externalhandle: *mut ldap, sortkeys: *mut *mut ldapsortkeyW, control: *mut ldapcontrolW, criticality: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2string(err: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2stringA(err: u32) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2stringW(err: u32) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_element(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PSTR, destlength: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_elementA(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PSTR, destlength: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_elementW(sourcefilterelement: super::super::Foundation::PSTR, sourcelength: u32, destfilterelement: super::super::Foundation::PWSTR, destlength: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dn(dn: super::super::Foundation::PSTR, notypes: u32) -> *mut super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dnA(dn: super::super::Foundation::PSTR, notypes: u32) -> *mut super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dnW(dn: super::super::Foundation::PWSTR, notypes: u32) -> *mut super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation(ld: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operationA(ld: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operationW(ld: *mut ldap, oid: super::super::Foundation::PWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation_sA(externalhandle: *mut ldap, oid: super::super::Foundation::PSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, returnedoid: *mut super::super::Foundation::PSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation_sW(externalhandle: *mut ldap, oid: super::super::Foundation::PWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, returnedoid: *mut super::super::Foundation::PWSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut *mut berelement) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_entry(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_reference(ld: *mut ldap, res: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controls(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controlsA(controls: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controlsW(controls: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dn(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dnA(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dnW(ld: *mut ldap, entry: *mut LDAPMessage) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_next_page(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, pagesize: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_next_page_s(externalhandle: *mut ldap, searchhandle: *mut ldapsearch, timeout: *mut LDAP_TIMEVAL, pagesize: u32, totalcount: *mut u32, results: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_option(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_optionW(ld: *mut ldap, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_paged_count(externalhandle: *mut ldap, searchblock: *mut ldapsearch, totalcount: *mut u32, results: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_valuesA(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_valuesW(ld: *mut ldap, entry: *mut LDAPMessage, attr: super::super::Foundation::PWSTR) -> *mut super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_len(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_lenA(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_lenW(externalhandle: *mut ldap, message: *mut LDAPMessage, attr: super::super::Foundation::PWSTR) -> *mut *mut LDAP_BERVAL;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_init(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_initA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_initW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfree(block: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfreeA(block: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfreeW(block: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modifyA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modifyW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, mods: *mut *mut ldapmodA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, mods: *mut *mut ldapmodW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2A(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2W(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_s(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_sA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_sW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR, deleteoldrdn: i32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdnA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdnW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_s(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_sA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, newdistinguishedname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_sW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, newdistinguishedname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_msgfree(res: *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attribute(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attributeA(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attributeW(ld: *mut ldap, entry: *mut LDAPMessage, ptr: *mut berelement) -> super::super::Foundation::PWSTR;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_entry(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_reference(ld: *mut ldap, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_open(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_openA(hostname: super::super::Foundation::PSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_openW(hostname: super::super::Foundation::PWSTR, portnumber: u32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_extended_resultA(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut super::super::Foundation::PSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_extended_resultW(connection: *mut ldap, resultmessage: *mut LDAPMessage, resultoid: *mut super::super::Foundation::PWSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_control(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_controlA(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_controlW(externalhandle: *mut ldap, servercontrols: *mut *mut ldapcontrolW, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_reference(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_referenceA(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_referenceW(connection: *mut ldap, resultmessage: *mut LDAPMessage, referrals: *mut *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_result(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PSTR, errormessage: *mut super::super::Foundation::PSTR, referrals: *mut *mut super::super::Foundation::PSTR, servercontrols: *mut *mut *mut ldapcontrolA, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_resultA(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PSTR, errormessage: *mut super::super::Foundation::PSTR, referrals: *mut *mut *mut i8, servercontrols: *mut *mut *mut ldapcontrolA, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_resultW(connection: *mut ldap, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut super::super::Foundation::PWSTR, errormessage: *mut super::super::Foundation::PWSTR, referrals: *mut *mut *mut u16, servercontrols: *mut *mut *mut ldapcontrolW, freeit: super::super::Foundation::BOOLEAN) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_control(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, result: *mut u32, attribute: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, result: *mut u32, attribute: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_vlv_controlA(externalhandle: *mut ldap, control: *mut *mut ldapcontrolA, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_vlv_controlW(externalhandle: *mut ldap, control: *mut *mut ldapcontrolW, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_perror(ld: *mut ldap, msg: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_extA(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_extW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, newrdn: super::super::Foundation::PWSTR, newparent: super::super::Foundation::PWSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, newrdn: super::super::Foundation::PSTR, newparent: super::super::Foundation::PSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, newrdn: super::super::Foundation::PWSTR, newparent: super::super::Foundation::PWSTR, deleteoldrdn: i32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_result(ld: *mut ldap, msgid: u32, all: u32, timeout: *const LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_result2error(ld: *mut ldap, res: *mut LDAPMessage, freeit: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bindA(externalhandle: *mut ldap, distname: super::super::Foundation::PSTR, authmechanism: super::super::Foundation::PSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, messagenumber: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bindW(externalhandle: *mut ldap, distname: super::super::Foundation::PWSTR, authmechanism: super::super::Foundation::PWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, messagenumber: *mut i32) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bind_sA(externalhandle: *mut ldap, distname: super::super::Foundation::PSTR, authmechanism: super::super::Foundation::PSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolA, clientctrls: *mut *mut ldapcontrolA, serverdata: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bind_sW(externalhandle: *mut ldap, distname: super::super::Foundation::PWSTR, authmechanism: super::super::Foundation::PWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut ldapcontrolW, clientctrls: *mut *mut ldapcontrolW, serverdata: *mut *mut LDAP_BERVAL) -> i32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_searchA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_searchW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_abandon_page(externalhandle: *mut ldap, searchblock: *mut ldapsearch) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_extA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_extW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_s(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_sA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const ldapcontrolA, clientcontrols: *const *const ldapcontrolA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_sW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const ldapcontrolW, clientcontrols: *const *const ldapcontrolW, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_page(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PSTR, attributelist: *mut *mut i8, attributesonly: u32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyA) -> *mut ldapsearch;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_pageA(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PSTR, attributelist: *const *const i8, attributesonly: u32, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyA) -> *mut ldapsearch;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_pageW(externalhandle: *mut ldap, distinguishedname: super::super::Foundation::PWSTR, scopeofsearch: u32, searchfilter: super::super::Foundation::PWSTR, attributelist: *const *const u16, attributesonly: u32, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut ldapsortkeyW) -> *mut ldapsearch;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_s(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_sA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_sW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_st(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_stA(ld: *mut ldap, base: super::super::Foundation::PSTR, scope: u32, filter: super::super::Foundation::PSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_stW(ld: *mut ldap, base: super::super::Foundation::PWSTR, scope: u32, filter: super::super::Foundation::PWSTR, attrs: *const *const u16, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`*"]
    pub fn ldap_set_dbg_flags(newflags: u32) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_dbg_routine(debugprintroutine: ::windows::runtime::RawPtr);
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_option(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_optionW(ld: *mut ldap, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bindA(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bindW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_s(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_sA(ld: *mut ldap, dn: super::super::Foundation::PSTR, passwd: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_sW(ld: *mut ldap, dn: super::super::Foundation::PWSTR, passwd: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinit(hostname: super::super::Foundation::PSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinitA(hostname: super::super::Foundation::PSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinitW(hostname: super::super::Foundation::PWSTR, portnumber: u32, secure: i32) -> *mut ldap;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_start_tls_sA(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolA, clientcontrols: *mut *mut ldapcontrolA) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_start_tls_sW(externalhandle: *mut ldap, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut ldapcontrolW, clientcontrols: *mut *mut ldapcontrolW) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_startup(version: *mut ldap_version_info, instance: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_stop_tls_s(externalhandle: *mut ldap) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dn(ufn: super::super::Foundation::PSTR, pdn: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dnA(ufn: super::super::Foundation::PSTR, pdn: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dnW(ufn: super::super::Foundation::PWSTR, pdn: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_unbind(ld: *mut ldap) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_unbind_s(ld: *mut ldap) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_free(vals: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_freeA(vals: *const super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_freeW(vals: *const super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_free_len(vals: *mut *mut LDAP_BERVAL) -> u32;
}
