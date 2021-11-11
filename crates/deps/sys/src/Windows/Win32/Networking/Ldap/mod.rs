#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Networking_Ldap`*"]
    pub fn LdapGetLastError();
    #[doc = "*Required features: `Win32_Networking_Ldap`*"]
    pub fn LdapMapErrorToWin32();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LdapUTF8ToUnicode();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LdapUnicodeToUTF8();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_alloc_t();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvdup();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvecfree();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_bvfree();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_first_element();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_flatten();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_free();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_init();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_next_element();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_peek_tag();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_printf();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_scanf();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ber_skip_tag();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_open();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_openA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn cldap_openW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_abandon();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_addA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_addW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_extA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_extW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_ext_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_add_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bindA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bindW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_bind_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_check_filterA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_check_filterW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_cleanup();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_close_extended_op();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compareA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compareW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_extA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_extW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_ext_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_compare_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_conn_from_msg();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_connect();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_free();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_freeA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_control_freeW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_free();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_freeA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_controls_freeW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_entries();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_references();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_values();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_valuesA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_valuesW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_count_values_len();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_control();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_controlA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_page_controlW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_control();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_controlA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_sort_controlW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_vlv_controlA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_create_vlv_controlW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_deleteA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_deleteW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_extA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_extW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_ext_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_delete_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufn();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufnA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_dn2ufnW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_encode_sort_controlA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_encode_sort_controlW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2string();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2stringA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_err2stringW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_element();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_elementA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_escape_filter_elementW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dn();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dnA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_explode_dnW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operationA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operationW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_extended_operation_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attribute();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attributeA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_attributeW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_entry();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_first_reference();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controls();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controlsA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_free_controlsW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dn();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dnA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_dnW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_next_page();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_next_page_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_option();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_optionW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_paged_count();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_valuesA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_valuesW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_len();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_lenA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_get_values_lenW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_init();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_initA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_initW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfree();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfreeA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_memfreeW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modifyA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modifyW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_extA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_extW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_ext_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modify_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2A();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2W();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn2_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdnA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdnW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_modrdn_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_msgfree();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attribute();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attributeA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_attributeW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_entry();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_next_reference();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_open();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_openA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_openW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_extended_resultA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_extended_resultW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_control();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_controlA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_page_controlW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_reference();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_referenceA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_referenceW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_result();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_resultA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_resultW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_control();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_controlA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_sort_controlW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_vlv_controlA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_parse_vlv_controlW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_perror();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_extA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_extW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_rename_ext_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_result();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_result2error();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bindA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bindW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bind_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sasl_bind_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_searchA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_searchW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_abandon_page();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_extA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_extW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_ext_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_page();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_pageA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_init_pageW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_st();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_stA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_search_stW();
    #[doc = "*Required features: `Win32_Networking_Ldap`*"]
    pub fn ldap_set_dbg_flags();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_dbg_routine();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_option();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_set_optionW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bindA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bindW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_simple_bind_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinit();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinitA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_sslinitW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_start_tls_sA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_start_tls_sW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_startup();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_stop_tls_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dn();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dnA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_ufn2dnW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_unbind();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_unbind_s();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_free();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_freeA();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_freeW();
    #[doc = "*Required features: `Win32_Networking_Ldap`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ldap_value_free_len();
}
