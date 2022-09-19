#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn LdapGetLastError() -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn LdapGetLastError() -> u32;
    }
    LdapGetLastError()
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn LdapMapErrorToWin32(ldaperror: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn LdapMapErrorToWin32(ldaperror: u32) -> u32;
    }
    LdapMapErrorToWin32(ldaperror)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn LdapUTF8ToUnicode(lpsrcstr: &[u8], lpdeststr: &mut [u16]) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn LdapUTF8ToUnicode(lpsrcstr: ::windows::core::PCSTR, cchsrc: i32, lpdeststr: ::windows::core::PWSTR, cchdest: i32) -> i32;
    }
    LdapUTF8ToUnicode(::core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len() as _, ::core::mem::transmute(lpdeststr.as_ptr()), lpdeststr.len() as _)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn LdapUnicodeToUTF8(lpsrcstr: &[u16], lpdeststr: &mut [u8]) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn LdapUnicodeToUTF8(lpsrcstr: ::windows::core::PCWSTR, cchsrc: i32, lpdeststr: ::windows::core::PSTR, cchdest: i32) -> i32;
    }
    LdapUnicodeToUTF8(::core::mem::transmute(lpsrcstr.as_ptr()), lpsrcstr.len() as _, ::core::mem::transmute(lpdeststr.as_ptr()), lpdeststr.len() as _)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_alloc_t(options: i32) -> *mut BerElement {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_alloc_t(options: i32) -> *mut BerElement;
    }
    ber_alloc_t(options)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_bvdup(pberval: *mut LDAP_BERVAL) -> *mut LDAP_BERVAL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_bvdup(pberval: *mut LDAP_BERVAL) -> *mut LDAP_BERVAL;
    }
    ber_bvdup(::core::mem::transmute(pberval))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_bvecfree(pberval: *mut *mut LDAP_BERVAL) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_bvecfree(pberval: *mut *mut LDAP_BERVAL);
    }
    ber_bvecfree(::core::mem::transmute(pberval))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_bvfree(bv: *mut LDAP_BERVAL) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_bvfree(bv: *mut LDAP_BERVAL);
    }
    ber_bvfree(::core::mem::transmute(bv))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ber_first_element(pberelement: *mut BerElement, plen: *mut u32, ppopaque: *mut *mut super::super::Foundation::CHAR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_first_element(pberelement: *mut BerElement, plen: *mut u32, ppopaque: *mut *mut super::super::Foundation::CHAR) -> u32;
    }
    ber_first_element(::core::mem::transmute(pberelement), ::core::mem::transmute(plen), ::core::mem::transmute(ppopaque))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_flatten(pberelement: *mut BerElement, pberval: *mut *mut LDAP_BERVAL) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_flatten(pberelement: *mut BerElement, pberval: *mut *mut LDAP_BERVAL) -> i32;
    }
    ber_flatten(::core::mem::transmute(pberelement), ::core::mem::transmute(pberval))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_free(pberelement: *mut BerElement, fbuf: i32) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_free(pberelement: *mut BerElement, fbuf: i32);
    }
    ber_free(::core::mem::transmute(pberelement), fbuf)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_init(pberval: *mut LDAP_BERVAL) -> *mut BerElement {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_init(pberval: *mut LDAP_BERVAL) -> *mut BerElement;
    }
    ber_init(::core::mem::transmute(pberval))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_next_element<'a, P0>(pberelement: *mut BerElement, plen: *mut u32, opaque: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_next_element(pberelement: *mut BerElement, plen: *mut u32, opaque: ::windows::core::PCSTR) -> u32;
    }
    ber_next_element(::core::mem::transmute(pberelement), ::core::mem::transmute(plen), opaque.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_peek_tag(pberelement: *mut BerElement, plen: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_peek_tag(pberelement: *mut BerElement, plen: *mut u32) -> u32;
    }
    ber_peek_tag(::core::mem::transmute(pberelement), ::core::mem::transmute(plen))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_printf<'a, P0>(pberelement: *mut BerElement, fmt: P0) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_printf(pberelement: *mut BerElement, fmt: ::windows::core::PCSTR) -> i32;
    }
    ber_printf(::core::mem::transmute(pberelement), fmt.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_scanf<'a, P0>(pberelement: *mut BerElement, fmt: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_scanf(pberelement: *mut BerElement, fmt: ::windows::core::PCSTR) -> u32;
    }
    ber_scanf(::core::mem::transmute(pberelement), fmt.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ber_skip_tag(pberelement: *mut BerElement, plen: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ber_skip_tag(pberelement: *mut BerElement, plen: *mut u32) -> u32;
    }
    ber_skip_tag(::core::mem::transmute(pberelement), ::core::mem::transmute(plen))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn cldap_open<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn cldap_open(hostname: ::windows::core::PCSTR, portnumber: u32) -> *mut LDAP;
    }
    cldap_open(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn cldap_openA<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn cldap_openA(hostname: ::windows::core::PCSTR, portnumber: u32) -> *mut LDAP;
    }
    cldap_openA(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn cldap_openW<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn cldap_openW(hostname: ::windows::core::PCWSTR, portnumber: u32) -> *mut LDAP;
    }
    cldap_openW(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_abandon(ld: *mut LDAP, msgid: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_abandon(ld: *mut LDAP, msgid: u32) -> u32;
    }
    ldap_abandon(::core::mem::transmute(ld), msgid)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_add<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add(ld: *mut LDAP, dn: ::windows::core::PCSTR, attrs: *mut *mut LDAPModA) -> u32;
    }
    ldap_add(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_addA<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_addA(ld: *mut LDAP, dn: ::windows::core::PCSTR, attrs: *mut *mut LDAPModA) -> u32;
    }
    ldap_addA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_addW<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_addW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, attrs: *mut *mut LDAPModW) -> u32;
    }
    ldap_addW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_ext<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_ext(ld: *mut LDAP, dn: ::windows::core::PCSTR, attrs: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_add_ext(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_extA<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_extA(ld: *mut LDAP, dn: ::windows::core::PCSTR, attrs: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_add_extA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_extW<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModW, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_extW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, attrs: *mut *mut LDAPModW, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32;
    }
    ldap_add_extW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_ext_s<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_ext_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, attrs: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_add_ext_s(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_ext_sA<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_ext_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, attrs: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_add_ext_sA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_add_ext_sW<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModW, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_ext_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, attrs: *mut *mut LDAPModW, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32;
    }
    ldap_add_ext_sW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_add_s<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, attrs: *mut *mut LDAPModA) -> u32;
    }
    ldap_add_s(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_add_sA<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, attrs: *mut *mut LDAPModA) -> u32;
    }
    ldap_add_sA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_add_sW<'a, P0>(ld: *mut LDAP, dn: P0, attrs: *mut *mut LDAPModW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_add_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, attrs: *mut *mut LDAPModW) -> u32;
    }
    ldap_add_sW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(attrs))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_bind<'a, P0, P1>(ld: *mut LDAP, dn: P0, cred: P1, method: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_bind(ld: *mut LDAP, dn: ::windows::core::PCSTR, cred: ::windows::core::PCSTR, method: u32) -> u32;
    }
    ldap_bind(::core::mem::transmute(ld), dn.into(), cred.into(), method)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_bindA<'a, P0, P1>(ld: *mut LDAP, dn: P0, cred: P1, method: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_bindA(ld: *mut LDAP, dn: ::windows::core::PCSTR, cred: ::windows::core::PCSTR, method: u32) -> u32;
    }
    ldap_bindA(::core::mem::transmute(ld), dn.into(), cred.into(), method)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_bindW<'a, P0, P1>(ld: *mut LDAP, dn: P0, cred: P1, method: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_bindW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, cred: ::windows::core::PCWSTR, method: u32) -> u32;
    }
    ldap_bindW(::core::mem::transmute(ld), dn.into(), cred.into(), method)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_bind_s<'a, P0, P1>(ld: *mut LDAP, dn: P0, cred: P1, method: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_bind_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, cred: ::windows::core::PCSTR, method: u32) -> u32;
    }
    ldap_bind_s(::core::mem::transmute(ld), dn.into(), cred.into(), method)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_bind_sA<'a, P0, P1>(ld: *mut LDAP, dn: P0, cred: P1, method: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_bind_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, cred: ::windows::core::PCSTR, method: u32) -> u32;
    }
    ldap_bind_sA(::core::mem::transmute(ld), dn.into(), cred.into(), method)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_bind_sW<'a, P0, P1>(ld: *mut LDAP, dn: P0, cred: P1, method: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_bind_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, cred: ::windows::core::PCWSTR, method: u32) -> u32;
    }
    ldap_bind_sW(::core::mem::transmute(ld), dn.into(), cred.into(), method)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_check_filterA<'a, P0>(ld: *mut LDAP, searchfilter: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_check_filterA(ld: *mut LDAP, searchfilter: ::windows::core::PCSTR) -> u32;
    }
    ldap_check_filterA(::core::mem::transmute(ld), searchfilter.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_check_filterW<'a, P0>(ld: *mut LDAP, searchfilter: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_check_filterW(ld: *mut LDAP, searchfilter: ::windows::core::PCWSTR) -> u32;
    }
    ldap_check_filterW(::core::mem::transmute(ld), searchfilter.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_cleanup<'a, P0>(hinstance: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_cleanup(hinstance: super::super::Foundation::HANDLE) -> u32;
    }
    ldap_cleanup(hinstance.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_close_extended_op(ld: *mut LDAP, messagenumber: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_close_extended_op(ld: *mut LDAP, messagenumber: u32) -> u32;
    }
    ldap_close_extended_op(::core::mem::transmute(ld), messagenumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_compare<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare(ld: *mut LDAP, dn: ::windows::core::PCSTR, attr: ::windows::core::PCSTR, value: ::windows::core::PCSTR) -> u32;
    }
    ldap_compare(::core::mem::transmute(ld), dn.into(), attr.into(), value.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_compareA<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compareA(ld: *mut LDAP, dn: ::windows::core::PCSTR, attr: ::windows::core::PCSTR, value: ::windows::core::PCSTR) -> u32;
    }
    ldap_compareA(::core::mem::transmute(ld), dn.into(), attr.into(), value.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_compareW<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compareW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, attr: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> u32;
    }
    ldap_compareW(::core::mem::transmute(ld), dn.into(), attr.into(), value.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_ext<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_ext(ld: *mut LDAP, dn: ::windows::core::PCSTR, attr: ::windows::core::PCSTR, value: ::windows::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_compare_ext(::core::mem::transmute(ld), dn.into(), attr.into(), value.into(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_extA<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2, data: ::core::option::Option<*const LDAP_BERVAL>, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_extA(ld: *mut LDAP, dn: ::windows::core::PCSTR, attr: ::windows::core::PCSTR, value: ::windows::core::PCSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_compare_extA(::core::mem::transmute(ld), dn.into(), attr.into(), value.into(), ::core::mem::transmute(data.unwrap_or(::std::ptr::null())), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_extW<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2, data: ::core::option::Option<*const LDAP_BERVAL>, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_extW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, attr: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32;
    }
    ldap_compare_extW(::core::mem::transmute(ld), dn.into(), attr.into(), value.into(), ::core::mem::transmute(data.unwrap_or(::std::ptr::null())), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_ext_s<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_ext_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, attr: ::windows::core::PCSTR, value: ::windows::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_compare_ext_s(::core::mem::transmute(ld), dn.into(), attr.into(), value.into(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_ext_sA<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2, data: ::core::option::Option<*const LDAP_BERVAL>, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_ext_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, attr: ::windows::core::PCSTR, value: ::windows::core::PCSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_compare_ext_sA(::core::mem::transmute(ld), dn.into(), attr.into(), value.into(), ::core::mem::transmute(data.unwrap_or(::std::ptr::null())), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_compare_ext_sW<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2, data: ::core::option::Option<*const LDAP_BERVAL>, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_ext_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, attr: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR, data: *const LDAP_BERVAL, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32;
    }
    ldap_compare_ext_sW(::core::mem::transmute(ld), dn.into(), attr.into(), value.into(), ::core::mem::transmute(data.unwrap_or(::std::ptr::null())), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_compare_s<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, attr: ::windows::core::PCSTR, value: ::windows::core::PCSTR) -> u32;
    }
    ldap_compare_s(::core::mem::transmute(ld), dn.into(), attr.into(), value.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_compare_sA<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, attr: ::windows::core::PCSTR, value: ::windows::core::PCSTR) -> u32;
    }
    ldap_compare_sA(::core::mem::transmute(ld), dn.into(), attr.into(), value.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_compare_sW<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, attr: P1, value: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_compare_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, attr: ::windows::core::PCWSTR, value: ::windows::core::PCWSTR) -> u32;
    }
    ldap_compare_sW(::core::mem::transmute(ld), dn.into(), attr.into(), value.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_conn_from_msg(primaryconn: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAP {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_conn_from_msg(primaryconn: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAP;
    }
    ldap_conn_from_msg(::core::mem::transmute(primaryconn), ::core::mem::transmute(res))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_connect(ld: *mut LDAP, timeout: *mut LDAP_TIMEVAL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_connect(ld: *mut LDAP, timeout: *mut LDAP_TIMEVAL) -> u32;
    }
    ldap_connect(::core::mem::transmute(ld), ::core::mem::transmute(timeout))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_control_free(control: *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_control_free(control: *mut LDAPControlA) -> u32;
    }
    ldap_control_free(::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_control_freeA(controls: *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_control_freeA(controls: *mut LDAPControlA) -> u32;
    }
    ldap_control_freeA(::core::mem::transmute(controls))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_control_freeW(control: *mut LDAPControlW) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_control_freeW(control: *mut LDAPControlW) -> u32;
    }
    ldap_control_freeW(::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_controls_free(controls: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_controls_free(controls: *mut *mut LDAPControlA) -> u32;
    }
    ldap_controls_free(::core::mem::transmute(controls))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_controls_freeA(controls: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_controls_freeA(controls: *mut *mut LDAPControlA) -> u32;
    }
    ldap_controls_freeA(::core::mem::transmute(controls))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_controls_freeW(control: *mut *mut LDAPControlW) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_controls_freeW(control: *mut *mut LDAPControlW) -> u32;
    }
    ldap_controls_freeW(::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_count_entries(ld: *mut LDAP, res: *mut LDAPMessage) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_count_entries(ld: *mut LDAP, res: *mut LDAPMessage) -> u32;
    }
    ldap_count_entries(::core::mem::transmute(ld), ::core::mem::transmute(res))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_count_references(ld: *mut LDAP, res: *mut LDAPMessage) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_count_references(ld: *mut LDAP, res: *mut LDAPMessage) -> u32;
    }
    ldap_count_references(::core::mem::transmute(ld), ::core::mem::transmute(res))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_count_values(vals: ::core::option::Option<*const ::windows::core::PSTR>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_count_values(vals: *const ::windows::core::PSTR) -> u32;
    }
    ldap_count_values(::core::mem::transmute(vals.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_count_valuesA(vals: ::core::option::Option<*const ::windows::core::PSTR>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_count_valuesA(vals: *const ::windows::core::PSTR) -> u32;
    }
    ldap_count_valuesA(::core::mem::transmute(vals.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_count_valuesW(vals: ::core::option::Option<*const ::windows::core::PWSTR>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_count_valuesW(vals: *const ::windows::core::PWSTR) -> u32;
    }
    ldap_count_valuesW(::core::mem::transmute(vals.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_count_values_len(vals: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_count_values_len(vals: *mut *mut LDAP_BERVAL) -> u32;
    }
    ldap_count_values_len(::core::mem::transmute(vals))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_page_control(externalhandle: *mut LDAP, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_create_page_control(externalhandle: *mut LDAP, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut LDAPControlA) -> u32;
    }
    ldap_create_page_control(::core::mem::transmute(externalhandle), pagesize, ::core::mem::transmute(cookie), iscritical, ::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_page_controlA(externalhandle: *mut LDAP, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_create_page_controlA(externalhandle: *mut LDAP, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut LDAPControlA) -> u32;
    }
    ldap_create_page_controlA(::core::mem::transmute(externalhandle), pagesize, ::core::mem::transmute(cookie), iscritical, ::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_page_controlW(externalhandle: *mut LDAP, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut LDAPControlW) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_create_page_controlW(externalhandle: *mut LDAP, pagesize: u32, cookie: *mut LDAP_BERVAL, iscritical: u8, control: *mut *mut LDAPControlW) -> u32;
    }
    ldap_create_page_controlW(::core::mem::transmute(externalhandle), pagesize, ::core::mem::transmute(cookie), iscritical, ::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_sort_control(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyA, iscritical: u8, control: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_create_sort_control(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyA, iscritical: u8, control: *mut *mut LDAPControlA) -> u32;
    }
    ldap_create_sort_control(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), iscritical, ::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_sort_controlA(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyA, iscritical: u8, control: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_create_sort_controlA(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyA, iscritical: u8, control: *mut *mut LDAPControlA) -> u32;
    }
    ldap_create_sort_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), iscritical, ::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_sort_controlW(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyW, iscritical: u8, control: *mut *mut LDAPControlW) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_create_sort_controlW(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyW, iscritical: u8, control: *mut *mut LDAPControlW) -> u32;
    }
    ldap_create_sort_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), iscritical, ::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_vlv_controlA(externalhandle: *mut LDAP, vlvinfo: *mut LDAPVLVInfo, iscritical: u8, control: *mut *mut LDAPControlA) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_create_vlv_controlA(externalhandle: *mut LDAP, vlvinfo: *mut LDAPVLVInfo, iscritical: u8, control: *mut *mut LDAPControlA) -> i32;
    }
    ldap_create_vlv_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(vlvinfo), iscritical, ::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_create_vlv_controlW(externalhandle: *mut LDAP, vlvinfo: *mut LDAPVLVInfo, iscritical: u8, control: *mut *mut LDAPControlW) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_create_vlv_controlW(externalhandle: *mut LDAP, vlvinfo: *mut LDAPVLVInfo, iscritical: u8, control: *mut *mut LDAPControlW) -> i32;
    }
    ldap_create_vlv_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(vlvinfo), iscritical, ::core::mem::transmute(control))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_delete<'a, P0>(ld: *mut LDAP, dn: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete(ld: *mut LDAP, dn: ::windows::core::PCSTR) -> u32;
    }
    ldap_delete(::core::mem::transmute(ld), dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_deleteA<'a, P0>(ld: *mut LDAP, dn: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_deleteA(ld: *mut LDAP, dn: ::windows::core::PCSTR) -> u32;
    }
    ldap_deleteA(::core::mem::transmute(ld), dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_deleteW<'a, P0>(ld: *mut LDAP, dn: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_deleteW(ld: *mut LDAP, dn: ::windows::core::PCWSTR) -> u32;
    }
    ldap_deleteW(::core::mem::transmute(ld), dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_ext<'a, P0>(ld: *mut LDAP, dn: P0, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_ext(ld: *mut LDAP, dn: ::windows::core::PCSTR, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_delete_ext(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_extA<'a, P0>(ld: *mut LDAP, dn: P0, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_extA(ld: *mut LDAP, dn: ::windows::core::PCSTR, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_delete_extA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_extW<'a, P0>(ld: *mut LDAP, dn: P0, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_extW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32;
    }
    ldap_delete_extW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_ext_s<'a, P0>(ld: *mut LDAP, dn: P0, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_ext_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_delete_ext_s(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_ext_sA<'a, P0>(ld: *mut LDAP, dn: P0, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_ext_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_delete_ext_sA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_delete_ext_sW<'a, P0>(ld: *mut LDAP, dn: P0, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_ext_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32;
    }
    ldap_delete_ext_sW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_delete_s<'a, P0>(ld: *mut LDAP, dn: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_s(ld: *mut LDAP, dn: ::windows::core::PCSTR) -> u32;
    }
    ldap_delete_s(::core::mem::transmute(ld), dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_delete_sA<'a, P0>(ld: *mut LDAP, dn: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR) -> u32;
    }
    ldap_delete_sA(::core::mem::transmute(ld), dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_delete_sW<'a, P0>(ld: *mut LDAP, dn: P0) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_delete_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR) -> u32;
    }
    ldap_delete_sW(::core::mem::transmute(ld), dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_dn2ufn<'a, P0>(dn: P0) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_dn2ufn(dn: ::windows::core::PCSTR) -> ::windows::core::PSTR;
    }
    ldap_dn2ufn(dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_dn2ufnA<'a, P0>(dn: P0) -> ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_dn2ufnA(dn: ::windows::core::PCSTR) -> ::windows::core::PSTR;
    }
    ldap_dn2ufnA(dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_dn2ufnW<'a, P0>(dn: P0) -> ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_dn2ufnW(dn: ::windows::core::PCWSTR) -> ::windows::core::PWSTR;
    }
    ldap_dn2ufnW(dn.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_encode_sort_controlA<'a, P0>(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyA, control: *mut LDAPControlA, criticality: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_encode_sort_controlA(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyA, control: *mut LDAPControlA, criticality: super::super::Foundation::BOOLEAN) -> u32;
    }
    ldap_encode_sort_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), ::core::mem::transmute(control), criticality.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_encode_sort_controlW<'a, P0>(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyW, control: *mut LDAPControlW, criticality: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_encode_sort_controlW(externalhandle: *mut LDAP, sortkeys: *mut *mut LDAPSortKeyW, control: *mut LDAPControlW, criticality: super::super::Foundation::BOOLEAN) -> u32;
    }
    ldap_encode_sort_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(sortkeys), ::core::mem::transmute(control), criticality.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_err2string(err: u32) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_err2string(err: u32) -> ::windows::core::PSTR;
    }
    ldap_err2string(err)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_err2stringA(err: u32) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_err2stringA(err: u32) -> ::windows::core::PSTR;
    }
    ldap_err2stringA(err)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_err2stringW(err: u32) -> ::windows::core::PWSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_err2stringW(err: u32) -> ::windows::core::PWSTR;
    }
    ldap_err2stringW(err)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_escape_filter_element(sourcefilterelement: &[u8], destfilterelement: ::core::option::Option<&mut [u8]>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_escape_filter_element(sourcefilterelement: ::windows::core::PCSTR, sourcelength: u32, destfilterelement: ::windows::core::PSTR, destlength: u32) -> u32;
    }
    ldap_escape_filter_element(::core::mem::transmute(sourcefilterelement.as_ptr()), sourcefilterelement.len() as _, ::core::mem::transmute(destfilterelement.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), destfilterelement.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_escape_filter_elementA(sourcefilterelement: &[u8], destfilterelement: ::core::option::Option<&mut [u8]>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_escape_filter_elementA(sourcefilterelement: ::windows::core::PCSTR, sourcelength: u32, destfilterelement: ::windows::core::PSTR, destlength: u32) -> u32;
    }
    ldap_escape_filter_elementA(::core::mem::transmute(sourcefilterelement.as_ptr()), sourcefilterelement.len() as _, ::core::mem::transmute(destfilterelement.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), destfilterelement.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_escape_filter_elementW(sourcefilterelement: &[u8], destfilterelement: ::core::option::Option<&mut [u8]>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_escape_filter_elementW(sourcefilterelement: ::windows::core::PCSTR, sourcelength: u32, destfilterelement: ::windows::core::PWSTR, destlength: u32) -> u32;
    }
    ldap_escape_filter_elementW(::core::mem::transmute(sourcefilterelement.as_ptr()), sourcefilterelement.len() as _, ::core::mem::transmute(destfilterelement.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), destfilterelement.as_deref().map_or(0, |slice| slice.len() as _))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_explode_dn<'a, P0>(dn: P0, notypes: u32) -> *mut ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_explode_dn(dn: ::windows::core::PCSTR, notypes: u32) -> *mut ::windows::core::PSTR;
    }
    ldap_explode_dn(dn.into(), notypes)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_explode_dnA<'a, P0>(dn: P0, notypes: u32) -> *mut ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_explode_dnA(dn: ::windows::core::PCSTR, notypes: u32) -> *mut ::windows::core::PSTR;
    }
    ldap_explode_dnA(dn.into(), notypes)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_explode_dnW<'a, P0>(dn: P0, notypes: u32) -> *mut ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_explode_dnW(dn: ::windows::core::PCWSTR, notypes: u32) -> *mut ::windows::core::PWSTR;
    }
    ldap_explode_dnW(dn.into(), notypes)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operation<'a, P0>(ld: *mut LDAP, oid: P0, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_extended_operation(ld: *mut LDAP, oid: ::windows::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_extended_operation(::core::mem::transmute(ld), oid.into(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operationA<'a, P0>(ld: *mut LDAP, oid: P0, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_extended_operationA(ld: *mut LDAP, oid: ::windows::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_extended_operationA(::core::mem::transmute(ld), oid.into(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operationW<'a, P0>(ld: *mut LDAP, oid: P0, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_extended_operationW(ld: *mut LDAP, oid: ::windows::core::PCWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32;
    }
    ldap_extended_operationW(::core::mem::transmute(ld), oid.into(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operation_sA<'a, P0>(externalhandle: *mut LDAP, oid: P0, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, returnedoid: *mut ::windows::core::PSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_extended_operation_sA(externalhandle: *mut LDAP, oid: ::windows::core::PCSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, returnedoid: *mut ::windows::core::PSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
    }
    ldap_extended_operation_sA(::core::mem::transmute(externalhandle), oid.into(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(returnedoid), ::core::mem::transmute(returneddata))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_extended_operation_sW<'a, P0>(externalhandle: *mut LDAP, oid: P0, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, returnedoid: *mut ::windows::core::PWSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_extended_operation_sW(externalhandle: *mut LDAP, oid: ::windows::core::PCWSTR, data: *mut LDAP_BERVAL, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, returnedoid: *mut ::windows::core::PWSTR, returneddata: *mut *mut LDAP_BERVAL) -> u32;
    }
    ldap_extended_operation_sW(::core::mem::transmute(externalhandle), oid.into(), ::core::mem::transmute(data), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(returnedoid), ::core::mem::transmute(returneddata))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_attribute(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut *mut BerElement) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_first_attribute(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut *mut BerElement) -> ::windows::core::PSTR;
    }
    ldap_first_attribute(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_attributeA(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut *mut BerElement) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_first_attributeA(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut *mut BerElement) -> ::windows::core::PSTR;
    }
    ldap_first_attributeA(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_attributeW(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut *mut BerElement) -> ::windows::core::PWSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_first_attributeW(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut *mut BerElement) -> ::windows::core::PWSTR;
    }
    ldap_first_attributeW(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_entry(ld: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAPMessage {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_first_entry(ld: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAPMessage;
    }
    ldap_first_entry(::core::mem::transmute(ld), ::core::mem::transmute(res))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_first_reference(ld: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAPMessage {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_first_reference(ld: *mut LDAP, res: *mut LDAPMessage) -> *mut LDAPMessage;
    }
    ldap_first_reference(::core::mem::transmute(ld), ::core::mem::transmute(res))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_free_controls(controls: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_free_controls(controls: *mut *mut LDAPControlA) -> u32;
    }
    ldap_free_controls(::core::mem::transmute(controls))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_free_controlsA(controls: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_free_controlsA(controls: *mut *mut LDAPControlA) -> u32;
    }
    ldap_free_controlsA(::core::mem::transmute(controls))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_free_controlsW(controls: *mut *mut LDAPControlW) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_free_controlsW(controls: *mut *mut LDAPControlW) -> u32;
    }
    ldap_free_controlsW(::core::mem::transmute(controls))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_dn(ld: *mut LDAP, entry: *mut LDAPMessage) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_dn(ld: *mut LDAP, entry: *mut LDAPMessage) -> ::windows::core::PSTR;
    }
    ldap_get_dn(::core::mem::transmute(ld), ::core::mem::transmute(entry))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_dnA(ld: *mut LDAP, entry: *mut LDAPMessage) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_dnA(ld: *mut LDAP, entry: *mut LDAPMessage) -> ::windows::core::PSTR;
    }
    ldap_get_dnA(::core::mem::transmute(ld), ::core::mem::transmute(entry))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_dnW(ld: *mut LDAP, entry: *mut LDAPMessage) -> ::windows::core::PWSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_dnW(ld: *mut LDAP, entry: *mut LDAPMessage) -> ::windows::core::PWSTR;
    }
    ldap_get_dnW(::core::mem::transmute(ld), ::core::mem::transmute(entry))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_get_next_page(externalhandle: *mut LDAP, searchhandle: *mut LDAPSearch, pagesize: u32, messagenumber: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_next_page(externalhandle: *mut LDAP, searchhandle: *mut LDAPSearch, pagesize: u32, messagenumber: *mut u32) -> u32;
    }
    ldap_get_next_page(::core::mem::transmute(externalhandle), ::core::mem::transmute(searchhandle), pagesize, ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_next_page_s(externalhandle: *mut LDAP, searchhandle: *mut LDAPSearch, timeout: *mut LDAP_TIMEVAL, pagesize: u32, totalcount: *mut u32, results: *mut *mut LDAPMessage) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_next_page_s(externalhandle: *mut LDAP, searchhandle: *mut LDAPSearch, timeout: *mut LDAP_TIMEVAL, pagesize: u32, totalcount: *mut u32, results: *mut *mut LDAPMessage) -> u32;
    }
    ldap_get_next_page_s(::core::mem::transmute(externalhandle), ::core::mem::transmute(searchhandle), ::core::mem::transmute(timeout), pagesize, ::core::mem::transmute(totalcount), ::core::mem::transmute(results))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_get_option(ld: *mut LDAP, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_option(ld: *mut LDAP, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
    }
    ldap_get_option(::core::mem::transmute(ld), option, ::core::mem::transmute(outvalue))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_get_optionW(ld: *mut LDAP, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_optionW(ld: *mut LDAP, option: i32, outvalue: *mut ::core::ffi::c_void) -> u32;
    }
    ldap_get_optionW(::core::mem::transmute(ld), option, ::core::mem::transmute(outvalue))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_paged_count(externalhandle: *mut LDAP, searchblock: *mut LDAPSearch, totalcount: *mut u32, results: *mut LDAPMessage) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_paged_count(externalhandle: *mut LDAP, searchblock: *mut LDAPSearch, totalcount: *mut u32, results: *mut LDAPMessage) -> u32;
    }
    ldap_get_paged_count(::core::mem::transmute(externalhandle), ::core::mem::transmute(searchblock), ::core::mem::transmute(totalcount), ::core::mem::transmute(results))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_values<'a, P0>(ld: *mut LDAP, entry: *mut LDAPMessage, attr: P0) -> *mut ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_values(ld: *mut LDAP, entry: *mut LDAPMessage, attr: ::windows::core::PCSTR) -> *mut ::windows::core::PSTR;
    }
    ldap_get_values(::core::mem::transmute(ld), ::core::mem::transmute(entry), attr.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_valuesA<'a, P0>(ld: *mut LDAP, entry: *mut LDAPMessage, attr: P0) -> *mut ::windows::core::PSTR
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_valuesA(ld: *mut LDAP, entry: *mut LDAPMessage, attr: ::windows::core::PCSTR) -> *mut ::windows::core::PSTR;
    }
    ldap_get_valuesA(::core::mem::transmute(ld), ::core::mem::transmute(entry), attr.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_valuesW<'a, P0>(ld: *mut LDAP, entry: *mut LDAPMessage, attr: P0) -> *mut ::windows::core::PWSTR
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_valuesW(ld: *mut LDAP, entry: *mut LDAPMessage, attr: ::windows::core::PCWSTR) -> *mut ::windows::core::PWSTR;
    }
    ldap_get_valuesW(::core::mem::transmute(ld), ::core::mem::transmute(entry), attr.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_values_len<'a, P0>(externalhandle: *mut LDAP, message: *mut LDAPMessage, attr: P0) -> *mut *mut LDAP_BERVAL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_values_len(externalhandle: *mut LDAP, message: *mut LDAPMessage, attr: ::windows::core::PCSTR) -> *mut *mut LDAP_BERVAL;
    }
    ldap_get_values_len(::core::mem::transmute(externalhandle), ::core::mem::transmute(message), attr.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_values_lenA<'a, P0>(externalhandle: *mut LDAP, message: *mut LDAPMessage, attr: P0) -> *mut *mut LDAP_BERVAL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_values_lenA(externalhandle: *mut LDAP, message: *mut LDAPMessage, attr: ::windows::core::PCSTR) -> *mut *mut LDAP_BERVAL;
    }
    ldap_get_values_lenA(::core::mem::transmute(externalhandle), ::core::mem::transmute(message), attr.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_get_values_lenW<'a, P0>(externalhandle: *mut LDAP, message: *mut LDAPMessage, attr: P0) -> *mut *mut LDAP_BERVAL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_get_values_lenW(externalhandle: *mut LDAP, message: *mut LDAPMessage, attr: ::windows::core::PCWSTR) -> *mut *mut LDAP_BERVAL;
    }
    ldap_get_values_lenW(::core::mem::transmute(externalhandle), ::core::mem::transmute(message), attr.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_init<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_init(hostname: ::windows::core::PCSTR, portnumber: u32) -> *mut LDAP;
    }
    ldap_init(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_initA<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_initA(hostname: ::windows::core::PCSTR, portnumber: u32) -> *mut LDAP;
    }
    ldap_initA(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_initW<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_initW(hostname: ::windows::core::PCWSTR, portnumber: u32) -> *mut LDAP;
    }
    ldap_initW(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_memfree<'a, P0>(block: P0)
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_memfree(block: ::windows::core::PCSTR);
    }
    ldap_memfree(block.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_memfreeA<'a, P0>(block: P0)
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_memfreeA(block: ::windows::core::PCSTR);
    }
    ldap_memfreeA(block.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_memfreeW<'a, P0>(block: P0)
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_memfreeW(block: ::windows::core::PCWSTR);
    }
    ldap_memfreeW(block.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modify<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify(ld: *mut LDAP, dn: ::windows::core::PCSTR, mods: *mut *mut LDAPModA) -> u32;
    }
    ldap_modify(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modifyA<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modifyA(ld: *mut LDAP, dn: ::windows::core::PCSTR, mods: *mut *mut LDAPModA) -> u32;
    }
    ldap_modifyA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modifyW<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modifyW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, mods: *mut *mut LDAPModW) -> u32;
    }
    ldap_modifyW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_ext<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_ext(ld: *mut LDAP, dn: ::windows::core::PCSTR, mods: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_modify_ext(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_extA<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_extA(ld: *mut LDAP, dn: ::windows::core::PCSTR, mods: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_modify_extA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_extW<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModW, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_extW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, mods: *mut *mut LDAPModW, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32;
    }
    ldap_modify_extW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_ext_s<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_ext_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, mods: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_modify_ext_s(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_ext_sA<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_ext_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, mods: *mut *mut LDAPModA, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_modify_ext_sA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_modify_ext_sW<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModW, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_ext_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, mods: *mut *mut LDAPModW, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32;
    }
    ldap_modify_ext_sW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modify_s<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, mods: *mut *mut LDAPModA) -> u32;
    }
    ldap_modify_s(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modify_sA<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, mods: *mut *mut LDAPModA) -> u32;
    }
    ldap_modify_sA(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modify_sW<'a, P0>(ld: *mut LDAP, dn: P0, mods: *mut *mut LDAPModW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modify_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, mods: *mut *mut LDAPModW) -> u32;
    }
    ldap_modify_sW(::core::mem::transmute(ld), dn.into(), ::core::mem::transmute(mods))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, newdistinguishedname: ::windows::core::PCSTR) -> u32;
    }
    ldap_modrdn(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn2<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1, deleteoldrdn: i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn2(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, newdistinguishedname: ::windows::core::PCSTR, deleteoldrdn: i32) -> u32;
    }
    ldap_modrdn2(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into(), deleteoldrdn)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn2A<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1, deleteoldrdn: i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn2A(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, newdistinguishedname: ::windows::core::PCSTR, deleteoldrdn: i32) -> u32;
    }
    ldap_modrdn2A(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into(), deleteoldrdn)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn2W<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1, deleteoldrdn: i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn2W(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCWSTR, newdistinguishedname: ::windows::core::PCWSTR, deleteoldrdn: i32) -> u32;
    }
    ldap_modrdn2W(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into(), deleteoldrdn)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn2_s<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1, deleteoldrdn: i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn2_s(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, newdistinguishedname: ::windows::core::PCSTR, deleteoldrdn: i32) -> u32;
    }
    ldap_modrdn2_s(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into(), deleteoldrdn)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn2_sA<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1, deleteoldrdn: i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn2_sA(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, newdistinguishedname: ::windows::core::PCSTR, deleteoldrdn: i32) -> u32;
    }
    ldap_modrdn2_sA(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into(), deleteoldrdn)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn2_sW<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1, deleteoldrdn: i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn2_sW(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCWSTR, newdistinguishedname: ::windows::core::PCWSTR, deleteoldrdn: i32) -> u32;
    }
    ldap_modrdn2_sW(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into(), deleteoldrdn)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdnA<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdnA(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, newdistinguishedname: ::windows::core::PCSTR) -> u32;
    }
    ldap_modrdnA(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdnW<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdnW(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCWSTR, newdistinguishedname: ::windows::core::PCWSTR) -> u32;
    }
    ldap_modrdnW(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn_s<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn_s(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, newdistinguishedname: ::windows::core::PCSTR) -> u32;
    }
    ldap_modrdn_s(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn_sA<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn_sA(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, newdistinguishedname: ::windows::core::PCSTR) -> u32;
    }
    ldap_modrdn_sA(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_modrdn_sW<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, newdistinguishedname: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_modrdn_sW(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCWSTR, newdistinguishedname: ::windows::core::PCWSTR) -> u32;
    }
    ldap_modrdn_sW(::core::mem::transmute(externalhandle), distinguishedname.into(), newdistinguishedname.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_msgfree(res: *mut LDAPMessage) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_msgfree(res: *mut LDAPMessage) -> u32;
    }
    ldap_msgfree(::core::mem::transmute(res))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_attribute(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut BerElement) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_next_attribute(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut BerElement) -> ::windows::core::PSTR;
    }
    ldap_next_attribute(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_attributeA(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut BerElement) -> ::windows::core::PSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_next_attributeA(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut BerElement) -> ::windows::core::PSTR;
    }
    ldap_next_attributeA(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_attributeW(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut BerElement) -> ::windows::core::PWSTR {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_next_attributeW(ld: *mut LDAP, entry: *mut LDAPMessage, ptr: *mut BerElement) -> ::windows::core::PWSTR;
    }
    ldap_next_attributeW(::core::mem::transmute(ld), ::core::mem::transmute(entry), ::core::mem::transmute(ptr))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_entry(ld: *mut LDAP, entry: *mut LDAPMessage) -> *mut LDAPMessage {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_next_entry(ld: *mut LDAP, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    }
    ldap_next_entry(::core::mem::transmute(ld), ::core::mem::transmute(entry))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_next_reference(ld: *mut LDAP, entry: *mut LDAPMessage) -> *mut LDAPMessage {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_next_reference(ld: *mut LDAP, entry: *mut LDAPMessage) -> *mut LDAPMessage;
    }
    ldap_next_reference(::core::mem::transmute(ld), ::core::mem::transmute(entry))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_open<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_open(hostname: ::windows::core::PCSTR, portnumber: u32) -> *mut LDAP;
    }
    ldap_open(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_openA<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_openA(hostname: ::windows::core::PCSTR, portnumber: u32) -> *mut LDAP;
    }
    ldap_openA(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_openW<'a, P0>(hostname: P0, portnumber: u32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_openW(hostname: ::windows::core::PCWSTR, portnumber: u32) -> *mut LDAP;
    }
    ldap_openW(hostname.into(), portnumber)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_extended_resultA<'a, P0>(connection: *mut LDAP, resultmessage: *mut LDAPMessage, resultoid: ::core::option::Option<*mut ::windows::core::PSTR>, resultdata: *mut *mut LDAP_BERVAL, freeit: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_extended_resultA(connection: *mut LDAP, resultmessage: *mut LDAPMessage, resultoid: *mut ::windows::core::PSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
    }
    ldap_parse_extended_resultA(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(resultoid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(resultdata), freeit.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_extended_resultW<'a, P0>(connection: *mut LDAP, resultmessage: *mut LDAPMessage, resultoid: ::core::option::Option<*mut ::windows::core::PWSTR>, resultdata: *mut *mut LDAP_BERVAL, freeit: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_extended_resultW(connection: *mut LDAP, resultmessage: *mut LDAPMessage, resultoid: *mut ::windows::core::PWSTR, resultdata: *mut *mut LDAP_BERVAL, freeit: super::super::Foundation::BOOLEAN) -> u32;
    }
    ldap_parse_extended_resultW(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(resultoid.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(resultdata), freeit.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_page_control(externalhandle: *mut LDAP, servercontrols: *mut *mut LDAPControlA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_page_control(externalhandle: *mut LDAP, servercontrols: *mut *mut LDAPControlA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    }
    ldap_parse_page_control(::core::mem::transmute(externalhandle), ::core::mem::transmute(servercontrols), ::core::mem::transmute(totalcount), ::core::mem::transmute(cookie))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_page_controlA(externalhandle: *mut LDAP, servercontrols: *mut *mut LDAPControlA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_page_controlA(externalhandle: *mut LDAP, servercontrols: *mut *mut LDAPControlA, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    }
    ldap_parse_page_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(servercontrols), ::core::mem::transmute(totalcount), ::core::mem::transmute(cookie))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_page_controlW(externalhandle: *mut LDAP, servercontrols: *mut *mut LDAPControlW, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_page_controlW(externalhandle: *mut LDAP, servercontrols: *mut *mut LDAPControlW, totalcount: *mut u32, cookie: *mut *mut LDAP_BERVAL) -> u32;
    }
    ldap_parse_page_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(servercontrols), ::core::mem::transmute(totalcount), ::core::mem::transmute(cookie))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_reference(connection: *mut LDAP, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows::core::PSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_reference(connection: *mut LDAP, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows::core::PSTR) -> u32;
    }
    ldap_parse_reference(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(referrals))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_referenceA(connection: *mut LDAP, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows::core::PSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_referenceA(connection: *mut LDAP, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows::core::PSTR) -> u32;
    }
    ldap_parse_referenceA(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(referrals))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_referenceW(connection: *mut LDAP, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows::core::PWSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_referenceW(connection: *mut LDAP, resultmessage: *mut LDAPMessage, referrals: *mut *mut ::windows::core::PWSTR) -> u32;
    }
    ldap_parse_referenceW(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(referrals))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_result<'a, P0>(connection: *mut LDAP, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: ::core::option::Option<*mut ::windows::core::PSTR>, errormessage: ::core::option::Option<*mut ::windows::core::PSTR>, referrals: ::core::option::Option<*mut *mut ::windows::core::PSTR>, servercontrols: *mut *mut *mut LDAPControlA, freeit: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_result(connection: *mut LDAP, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut ::windows::core::PSTR, errormessage: *mut ::windows::core::PSTR, referrals: *mut *mut ::windows::core::PSTR, servercontrols: *mut *mut *mut LDAPControlA, freeit: super::super::Foundation::BOOLEAN) -> u32;
    }
    ldap_parse_result(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(returncode), ::core::mem::transmute(matcheddns.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(errormessage.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(referrals.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servercontrols), freeit.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_resultA<'a, P0>(connection: *mut LDAP, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: ::core::option::Option<*mut ::windows::core::PSTR>, errormessage: ::core::option::Option<*mut ::windows::core::PSTR>, referrals: ::core::option::Option<*mut *mut *mut i8>, servercontrols: *mut *mut *mut LDAPControlA, freeit: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_resultA(connection: *mut LDAP, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut ::windows::core::PSTR, errormessage: *mut ::windows::core::PSTR, referrals: *mut *mut *mut i8, servercontrols: *mut *mut *mut LDAPControlA, freeit: super::super::Foundation::BOOLEAN) -> u32;
    }
    ldap_parse_resultA(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(returncode), ::core::mem::transmute(matcheddns.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(errormessage.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(referrals.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servercontrols), freeit.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_resultW<'a, P0>(connection: *mut LDAP, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: ::core::option::Option<*mut ::windows::core::PWSTR>, errormessage: ::core::option::Option<*mut ::windows::core::PWSTR>, referrals: ::core::option::Option<*mut *mut *mut u16>, servercontrols: *mut *mut *mut LDAPControlW, freeit: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOLEAN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_resultW(connection: *mut LDAP, resultmessage: *mut LDAPMessage, returncode: *mut u32, matcheddns: *mut ::windows::core::PWSTR, errormessage: *mut ::windows::core::PWSTR, referrals: *mut *mut *mut u16, servercontrols: *mut *mut *mut LDAPControlW, freeit: super::super::Foundation::BOOLEAN) -> u32;
    }
    ldap_parse_resultW(::core::mem::transmute(connection), ::core::mem::transmute(resultmessage), ::core::mem::transmute(returncode), ::core::mem::transmute(matcheddns.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(errormessage.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(referrals.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(servercontrols), freeit.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_sort_control(externalhandle: *mut LDAP, control: *mut *mut LDAPControlA, result: *mut u32, attribute: *mut ::windows::core::PSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_sort_control(externalhandle: *mut LDAP, control: *mut *mut LDAPControlA, result: *mut u32, attribute: *mut ::windows::core::PSTR) -> u32;
    }
    ldap_parse_sort_control(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(result), ::core::mem::transmute(attribute))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_sort_controlA(externalhandle: *mut LDAP, control: *mut *mut LDAPControlA, result: *mut u32, attribute: ::core::option::Option<*mut ::windows::core::PSTR>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_sort_controlA(externalhandle: *mut LDAP, control: *mut *mut LDAPControlA, result: *mut u32, attribute: *mut ::windows::core::PSTR) -> u32;
    }
    ldap_parse_sort_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(result), ::core::mem::transmute(attribute.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_sort_controlW(externalhandle: *mut LDAP, control: *mut *mut LDAPControlW, result: *mut u32, attribute: ::core::option::Option<*mut ::windows::core::PWSTR>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_sort_controlW(externalhandle: *mut LDAP, control: *mut *mut LDAPControlW, result: *mut u32, attribute: *mut ::windows::core::PWSTR) -> u32;
    }
    ldap_parse_sort_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(result), ::core::mem::transmute(attribute.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_vlv_controlA(externalhandle: *mut LDAP, control: *mut *mut LDAPControlA, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_vlv_controlA(externalhandle: *mut LDAP, control: *mut *mut LDAPControlA, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
    }
    ldap_parse_vlv_controlA(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(targetpos), ::core::mem::transmute(listcount), ::core::mem::transmute(context), ::core::mem::transmute(errcode))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_parse_vlv_controlW(externalhandle: *mut LDAP, control: *mut *mut LDAPControlW, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_parse_vlv_controlW(externalhandle: *mut LDAP, control: *mut *mut LDAPControlW, targetpos: *mut u32, listcount: *mut u32, context: *mut *mut LDAP_BERVAL, errcode: *mut i32) -> i32;
    }
    ldap_parse_vlv_controlW(::core::mem::transmute(externalhandle), ::core::mem::transmute(control), ::core::mem::transmute(targetpos), ::core::mem::transmute(listcount), ::core::mem::transmute(context), ::core::mem::transmute(errcode))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_perror<'a, P0>(ld: *mut LDAP, msg: P0)
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_perror(ld: *mut LDAP, msg: ::windows::core::PCSTR);
    }
    ldap_perror(::core::mem::transmute(ld), msg.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_ext<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, newrdn: P1, newparent: P2, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_rename_ext(ld: *mut LDAP, dn: ::windows::core::PCSTR, newrdn: ::windows::core::PCSTR, newparent: ::windows::core::PCSTR, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_rename_ext(::core::mem::transmute(ld), dn.into(), newrdn.into(), newparent.into(), deleteoldrdn, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_extA<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, newrdn: P1, newparent: P2, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_rename_extA(ld: *mut LDAP, dn: ::windows::core::PCSTR, newrdn: ::windows::core::PCSTR, newparent: ::windows::core::PCSTR, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, messagenumber: *mut u32) -> u32;
    }
    ldap_rename_extA(::core::mem::transmute(ld), dn.into(), newrdn.into(), newparent.into(), deleteoldrdn, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_extW<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, newrdn: P1, newparent: P2, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_rename_extW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, newrdn: ::windows::core::PCWSTR, newparent: ::windows::core::PCWSTR, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, messagenumber: *mut u32) -> u32;
    }
    ldap_rename_extW(::core::mem::transmute(ld), dn.into(), newrdn.into(), newparent.into(), deleteoldrdn, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_ext_s<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, newrdn: P1, newparent: P2, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_rename_ext_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, newrdn: ::windows::core::PCSTR, newparent: ::windows::core::PCSTR, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_rename_ext_s(::core::mem::transmute(ld), dn.into(), newrdn.into(), newparent.into(), deleteoldrdn, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_ext_sA<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, newrdn: P1, newparent: P2, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_rename_ext_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, newrdn: ::windows::core::PCSTR, newparent: ::windows::core::PCSTR, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_rename_ext_sA(::core::mem::transmute(ld), dn.into(), newrdn.into(), newparent.into(), deleteoldrdn, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_rename_ext_sW<'a, P0, P1, P2>(ld: *mut LDAP, dn: P0, newrdn: P1, newparent: P2, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_rename_ext_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, newrdn: ::windows::core::PCWSTR, newparent: ::windows::core::PCWSTR, deleteoldrdn: i32, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32;
    }
    ldap_rename_ext_sW(::core::mem::transmute(ld), dn.into(), newrdn.into(), newparent.into(), deleteoldrdn, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_result(ld: *mut LDAP, msgid: u32, all: u32, timeout: ::core::option::Option<*const LDAP_TIMEVAL>, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_result(ld: *mut LDAP, msgid: u32, all: u32, timeout: *const LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_result(::core::mem::transmute(ld), msgid, all, ::core::mem::transmute(timeout.unwrap_or(::std::ptr::null())), ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_result2error(ld: *mut LDAP, res: *mut LDAPMessage, freeit: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_result2error(ld: *mut LDAP, res: *mut LDAPMessage, freeit: u32) -> u32;
    }
    ldap_result2error(::core::mem::transmute(ld), ::core::mem::transmute(res), freeit)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sasl_bindA<'a, P0, P1>(externalhandle: *mut LDAP, distname: P0, authmechanism: P1, cred: *const LDAP_BERVAL, serverctrls: *mut *mut LDAPControlA, clientctrls: *mut *mut LDAPControlA, messagenumber: *mut i32) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_sasl_bindA(externalhandle: *mut LDAP, distname: ::windows::core::PCSTR, authmechanism: ::windows::core::PCSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut LDAPControlA, clientctrls: *mut *mut LDAPControlA, messagenumber: *mut i32) -> i32;
    }
    ldap_sasl_bindA(::core::mem::transmute(externalhandle), distname.into(), authmechanism.into(), ::core::mem::transmute(cred), ::core::mem::transmute(serverctrls), ::core::mem::transmute(clientctrls), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sasl_bindW<'a, P0, P1>(externalhandle: *mut LDAP, distname: P0, authmechanism: P1, cred: *const LDAP_BERVAL, serverctrls: *mut *mut LDAPControlW, clientctrls: *mut *mut LDAPControlW, messagenumber: *mut i32) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_sasl_bindW(externalhandle: *mut LDAP, distname: ::windows::core::PCWSTR, authmechanism: ::windows::core::PCWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut LDAPControlW, clientctrls: *mut *mut LDAPControlW, messagenumber: *mut i32) -> i32;
    }
    ldap_sasl_bindW(::core::mem::transmute(externalhandle), distname.into(), authmechanism.into(), ::core::mem::transmute(cred), ::core::mem::transmute(serverctrls), ::core::mem::transmute(clientctrls), ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sasl_bind_sA<'a, P0, P1>(externalhandle: *mut LDAP, distname: P0, authmechanism: P1, cred: *const LDAP_BERVAL, serverctrls: *mut *mut LDAPControlA, clientctrls: *mut *mut LDAPControlA, serverdata: *mut *mut LDAP_BERVAL) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_sasl_bind_sA(externalhandle: *mut LDAP, distname: ::windows::core::PCSTR, authmechanism: ::windows::core::PCSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut LDAPControlA, clientctrls: *mut *mut LDAPControlA, serverdata: *mut *mut LDAP_BERVAL) -> i32;
    }
    ldap_sasl_bind_sA(::core::mem::transmute(externalhandle), distname.into(), authmechanism.into(), ::core::mem::transmute(cred), ::core::mem::transmute(serverctrls), ::core::mem::transmute(clientctrls), ::core::mem::transmute(serverdata))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_sasl_bind_sW<'a, P0, P1>(externalhandle: *mut LDAP, distname: P0, authmechanism: P1, cred: *const LDAP_BERVAL, serverctrls: *mut *mut LDAPControlW, clientctrls: *mut *mut LDAPControlW, serverdata: *mut *mut LDAP_BERVAL) -> i32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_sasl_bind_sW(externalhandle: *mut LDAP, distname: ::windows::core::PCWSTR, authmechanism: ::windows::core::PCWSTR, cred: *const LDAP_BERVAL, serverctrls: *mut *mut LDAPControlW, clientctrls: *mut *mut LDAPControlW, serverdata: *mut *mut LDAP_BERVAL) -> i32;
    }
    ldap_sasl_bind_sW(::core::mem::transmute(externalhandle), distname.into(), authmechanism.into(), ::core::mem::transmute(cred), ::core::mem::transmute(serverctrls), ::core::mem::transmute(clientctrls), ::core::mem::transmute(serverdata))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_search<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
    }
    ldap_search(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_searchA<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_searchA(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32) -> u32;
    }
    ldap_searchA(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_searchW<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const u16, attrsonly: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_searchW(ld: *mut LDAP, base: ::windows::core::PCWSTR, scope: u32, filter: ::windows::core::PCWSTR, attrs: *const *const u16, attrsonly: u32) -> u32;
    }
    ldap_searchW(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_search_abandon_page(externalhandle: *mut LDAP, searchblock: *mut LDAPSearch) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_abandon_page(externalhandle: *mut LDAP, searchblock: *mut LDAPSearch) -> u32;
    }
    ldap_search_abandon_page(::core::mem::transmute(externalhandle), ::core::mem::transmute(searchblock))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_ext<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32, servercontrols: ::core::option::Option<*const *const LDAPControlA>, clientcontrols: ::core::option::Option<*const *const LDAPControlA>, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_ext(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const LDAPControlA, clientcontrols: *const *const LDAPControlA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    }
    ldap_search_ext(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(servercontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(clientcontrols.unwrap_or(::std::ptr::null())), timelimit, sizelimit, ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_extA<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32, servercontrols: ::core::option::Option<*const *const LDAPControlA>, clientcontrols: ::core::option::Option<*const *const LDAPControlA>, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_extA(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const LDAPControlA, clientcontrols: *const *const LDAPControlA, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    }
    ldap_search_extA(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(servercontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(clientcontrols.unwrap_or(::std::ptr::null())), timelimit, sizelimit, ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_extW<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const u16, attrsonly: u32, servercontrols: ::core::option::Option<*const *const LDAPControlW>, clientcontrols: ::core::option::Option<*const *const LDAPControlW>, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_extW(ld: *mut LDAP, base: ::windows::core::PCWSTR, scope: u32, filter: ::windows::core::PCWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const LDAPControlW, clientcontrols: *const *const LDAPControlW, timelimit: u32, sizelimit: u32, messagenumber: *mut u32) -> u32;
    }
    ldap_search_extW(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(servercontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(clientcontrols.unwrap_or(::std::ptr::null())), timelimit, sizelimit, ::core::mem::transmute(messagenumber))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_ext_s<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32, servercontrols: ::core::option::Option<*const *const LDAPControlA>, clientcontrols: ::core::option::Option<*const *const LDAPControlA>, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_ext_s(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const LDAPControlA, clientcontrols: *const *const LDAPControlA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_ext_s(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(servercontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(clientcontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(timeout), sizelimit, ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_ext_sA<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32, servercontrols: ::core::option::Option<*const *const LDAPControlA>, clientcontrols: ::core::option::Option<*const *const LDAPControlA>, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_ext_sA(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32, servercontrols: *const *const LDAPControlA, clientcontrols: *const *const LDAPControlA, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_ext_sA(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(servercontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(clientcontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(timeout), sizelimit, ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_ext_sW<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const u16, attrsonly: u32, servercontrols: ::core::option::Option<*const *const LDAPControlW>, clientcontrols: ::core::option::Option<*const *const LDAPControlW>, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_ext_sW(ld: *mut LDAP, base: ::windows::core::PCWSTR, scope: u32, filter: ::windows::core::PCWSTR, attrs: *const *const u16, attrsonly: u32, servercontrols: *const *const LDAPControlW, clientcontrols: *const *const LDAPControlW, timeout: *mut LDAP_TIMEVAL, sizelimit: u32, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_ext_sW(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(servercontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(clientcontrols.unwrap_or(::std::ptr::null())), ::core::mem::transmute(timeout), sizelimit, ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_init_page<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, scopeofsearch: u32, searchfilter: P1, attributelist: *mut *mut i8, attributesonly: u32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut LDAPSortKeyA) -> *mut LDAPSearch
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_init_page(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, scopeofsearch: u32, searchfilter: ::windows::core::PCSTR, attributelist: *mut *mut i8, attributesonly: u32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut LDAPSortKeyA) -> *mut LDAPSearch;
    }
    ldap_search_init_page(::core::mem::transmute(externalhandle), distinguishedname.into(), scopeofsearch, searchfilter.into(), ::core::mem::transmute(attributelist), attributesonly, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), pagetimelimit, totalsizelimit, ::core::mem::transmute(sortkeys))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_init_pageA<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, scopeofsearch: u32, searchfilter: P1, attributelist: *const *const i8, attributesonly: u32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut LDAPSortKeyA) -> *mut LDAPSearch
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_init_pageA(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCSTR, scopeofsearch: u32, searchfilter: ::windows::core::PCSTR, attributelist: *const *const i8, attributesonly: u32, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut LDAPSortKeyA) -> *mut LDAPSearch;
    }
    ldap_search_init_pageA(::core::mem::transmute(externalhandle), distinguishedname.into(), scopeofsearch, searchfilter.into(), ::core::mem::transmute(attributelist), attributesonly, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), pagetimelimit, totalsizelimit, ::core::mem::transmute(sortkeys))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_init_pageW<'a, P0, P1>(externalhandle: *mut LDAP, distinguishedname: P0, scopeofsearch: u32, searchfilter: P1, attributelist: *const *const u16, attributesonly: u32, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut LDAPSortKeyW) -> *mut LDAPSearch
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_init_pageW(externalhandle: *mut LDAP, distinguishedname: ::windows::core::PCWSTR, scopeofsearch: u32, searchfilter: ::windows::core::PCWSTR, attributelist: *const *const u16, attributesonly: u32, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW, pagetimelimit: u32, totalsizelimit: u32, sortkeys: *mut *mut LDAPSortKeyW) -> *mut LDAPSearch;
    }
    ldap_search_init_pageW(::core::mem::transmute(externalhandle), distinguishedname.into(), scopeofsearch, searchfilter.into(), ::core::mem::transmute(attributelist), attributesonly, ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols), pagetimelimit, totalsizelimit, ::core::mem::transmute(sortkeys))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_s<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_s(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_s(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_sA<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_sA(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_sA(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_sW<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const u16, attrsonly: u32, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_sW(ld: *mut LDAP, base: ::windows::core::PCWSTR, scope: u32, filter: ::windows::core::PCWSTR, attrs: *const *const u16, attrsonly: u32, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_sW(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_st<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_st(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_st(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(timeout), ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_stA<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_stA(ld: *mut LDAP, base: ::windows::core::PCSTR, scope: u32, filter: ::windows::core::PCSTR, attrs: *const *const i8, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_stA(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(timeout), ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_search_stW<'a, P0, P1>(ld: *mut LDAP, base: P0, scope: u32, filter: P1, attrs: *const *const u16, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: ::core::option::Option<*mut *mut LDAPMessage>) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_search_stW(ld: *mut LDAP, base: ::windows::core::PCWSTR, scope: u32, filter: ::windows::core::PCWSTR, attrs: *const *const u16, attrsonly: u32, timeout: *mut LDAP_TIMEVAL, res: *mut *mut LDAPMessage) -> u32;
    }
    ldap_search_stW(::core::mem::transmute(ld), base.into(), scope, filter.into(), ::core::mem::transmute(attrs), attrsonly, ::core::mem::transmute(timeout), ::core::mem::transmute(res.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_set_dbg_flags(newflags: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_set_dbg_flags(newflags: u32) -> u32;
    }
    ldap_set_dbg_flags(newflags)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_set_dbg_routine(debugprintroutine: DBGPRINT) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_set_dbg_routine(debugprintroutine: *mut ::core::ffi::c_void);
    }
    ldap_set_dbg_routine(::core::mem::transmute(debugprintroutine))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_set_option(ld: *mut LDAP, option: i32, invalue: *const ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_set_option(ld: *mut LDAP, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
    }
    ldap_set_option(::core::mem::transmute(ld), option, ::core::mem::transmute(invalue))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_set_optionW(ld: *mut LDAP, option: i32, invalue: *const ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_set_optionW(ld: *mut LDAP, option: i32, invalue: *const ::core::ffi::c_void) -> u32;
    }
    ldap_set_optionW(::core::mem::transmute(ld), option, ::core::mem::transmute(invalue))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_simple_bind<'a, P0, P1>(ld: *mut LDAP, dn: P0, passwd: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_simple_bind(ld: *mut LDAP, dn: ::windows::core::PCSTR, passwd: ::windows::core::PCSTR) -> u32;
    }
    ldap_simple_bind(::core::mem::transmute(ld), dn.into(), passwd.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_simple_bindA<'a, P0, P1>(ld: *mut LDAP, dn: P0, passwd: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_simple_bindA(ld: *mut LDAP, dn: ::windows::core::PCSTR, passwd: ::windows::core::PCSTR) -> u32;
    }
    ldap_simple_bindA(::core::mem::transmute(ld), dn.into(), passwd.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_simple_bindW<'a, P0, P1>(ld: *mut LDAP, dn: P0, passwd: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_simple_bindW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, passwd: ::windows::core::PCWSTR) -> u32;
    }
    ldap_simple_bindW(::core::mem::transmute(ld), dn.into(), passwd.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_simple_bind_s<'a, P0, P1>(ld: *mut LDAP, dn: P0, passwd: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_simple_bind_s(ld: *mut LDAP, dn: ::windows::core::PCSTR, passwd: ::windows::core::PCSTR) -> u32;
    }
    ldap_simple_bind_s(::core::mem::transmute(ld), dn.into(), passwd.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_simple_bind_sA<'a, P0, P1>(ld: *mut LDAP, dn: P0, passwd: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_simple_bind_sA(ld: *mut LDAP, dn: ::windows::core::PCSTR, passwd: ::windows::core::PCSTR) -> u32;
    }
    ldap_simple_bind_sA(::core::mem::transmute(ld), dn.into(), passwd.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_simple_bind_sW<'a, P0, P1>(ld: *mut LDAP, dn: P0, passwd: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_simple_bind_sW(ld: *mut LDAP, dn: ::windows::core::PCWSTR, passwd: ::windows::core::PCWSTR) -> u32;
    }
    ldap_simple_bind_sW(::core::mem::transmute(ld), dn.into(), passwd.into())
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_sslinit<'a, P0>(hostname: P0, portnumber: u32, secure: i32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_sslinit(hostname: ::windows::core::PCSTR, portnumber: u32, secure: i32) -> *mut LDAP;
    }
    ldap_sslinit(hostname.into(), portnumber, secure)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_sslinitA<'a, P0>(hostname: P0, portnumber: u32, secure: i32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_sslinitA(hostname: ::windows::core::PCSTR, portnumber: u32, secure: i32) -> *mut LDAP;
    }
    ldap_sslinitA(hostname.into(), portnumber, secure)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_sslinitW<'a, P0>(hostname: P0, portnumber: u32, secure: i32) -> *mut LDAP
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_sslinitW(hostname: ::windows::core::PCWSTR, portnumber: u32, secure: i32) -> *mut LDAP;
    }
    ldap_sslinitW(hostname.into(), portnumber, secure)
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_start_tls_sA(externalhandle: *mut LDAP, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_start_tls_sA(externalhandle: *mut LDAP, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut LDAPControlA, clientcontrols: *mut *mut LDAPControlA) -> u32;
    }
    ldap_start_tls_sA(::core::mem::transmute(externalhandle), ::core::mem::transmute(serverreturnvalue), ::core::mem::transmute(result), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_start_tls_sW(externalhandle: *mut LDAP, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_start_tls_sW(externalhandle: *mut LDAP, serverreturnvalue: *mut u32, result: *mut *mut LDAPMessage, servercontrols: *mut *mut LDAPControlW, clientcontrols: *mut *mut LDAPControlW) -> u32;
    }
    ldap_start_tls_sW(::core::mem::transmute(externalhandle), ::core::mem::transmute(serverreturnvalue), ::core::mem::transmute(result), ::core::mem::transmute(servercontrols), ::core::mem::transmute(clientcontrols))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_startup(version: *mut LDAP_VERSION_INFO, instance: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_startup(version: *mut LDAP_VERSION_INFO, instance: *mut super::super::Foundation::HANDLE) -> u32;
    }
    ldap_startup(::core::mem::transmute(version), ::core::mem::transmute(instance))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ldap_stop_tls_s(externalhandle: *mut LDAP) -> super::super::Foundation::BOOLEAN {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_stop_tls_s(externalhandle: *mut LDAP) -> super::super::Foundation::BOOLEAN;
    }
    ldap_stop_tls_s(::core::mem::transmute(externalhandle))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_ufn2dn<'a, P0>(ufn: P0, pdn: *mut ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_ufn2dn(ufn: ::windows::core::PCSTR, pdn: *mut ::windows::core::PSTR) -> u32;
    }
    ldap_ufn2dn(ufn.into(), ::core::mem::transmute(pdn))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_ufn2dnA<'a, P0>(ufn: P0, pdn: *mut ::windows::core::PSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_ufn2dnA(ufn: ::windows::core::PCSTR, pdn: *mut ::windows::core::PSTR) -> u32;
    }
    ldap_ufn2dnA(ufn.into(), ::core::mem::transmute(pdn))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_ufn2dnW<'a, P0>(ufn: P0, pdn: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_ufn2dnW(ufn: ::windows::core::PCWSTR, pdn: *mut ::windows::core::PWSTR) -> u32;
    }
    ldap_ufn2dnW(ufn.into(), ::core::mem::transmute(pdn))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_unbind(ld: *mut LDAP) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_unbind(ld: *mut LDAP) -> u32;
    }
    ldap_unbind(::core::mem::transmute(ld))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_unbind_s(ld: *mut LDAP) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_unbind_s(ld: *mut LDAP) -> u32;
    }
    ldap_unbind_s(::core::mem::transmute(ld))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_value_free(vals: ::core::option::Option<*const ::windows::core::PSTR>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_value_free(vals: *const ::windows::core::PSTR) -> u32;
    }
    ldap_value_free(::core::mem::transmute(vals.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_value_freeA(vals: ::core::option::Option<*const ::windows::core::PSTR>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_value_freeA(vals: *const ::windows::core::PSTR) -> u32;
    }
    ldap_value_freeA(::core::mem::transmute(vals.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_value_freeW(vals: ::core::option::Option<*const ::windows::core::PWSTR>) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_value_freeW(vals: *const ::windows::core::PWSTR) -> u32;
    }
    ldap_value_freeW(::core::mem::transmute(vals.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[inline]
pub unsafe fn ldap_value_free_len(vals: *mut *mut LDAP_BERVAL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "cdecl" {
        fn ldap_value_free_len(vals: *mut *mut LDAP_BERVAL) -> u32;
    }
    ldap_value_free_len(::core::mem::transmute(vals))
}
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
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const SERVER_SEARCH_FLAG_DOMAIN_SCOPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const SERVER_SEARCH_FLAG_PHANTOM_ROOT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct LDAP_RETCODE(pub i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SUCCESS: LDAP_RETCODE = LDAP_RETCODE(0i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OPERATIONS_ERROR: LDAP_RETCODE = LDAP_RETCODE(1i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PROTOCOL_ERROR: LDAP_RETCODE = LDAP_RETCODE(2i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_TIMELIMIT_EXCEEDED: LDAP_RETCODE = LDAP_RETCODE(3i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SIZELIMIT_EXCEEDED: LDAP_RETCODE = LDAP_RETCODE(4i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_COMPARE_FALSE: LDAP_RETCODE = LDAP_RETCODE(5i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_COMPARE_TRUE: LDAP_RETCODE = LDAP_RETCODE(6i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AUTH_METHOD_NOT_SUPPORTED: LDAP_RETCODE = LDAP_RETCODE(7i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_STRONG_AUTH_REQUIRED: LDAP_RETCODE = LDAP_RETCODE(8i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_REFERRAL_V2: LDAP_RETCODE = LDAP_RETCODE(9i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PARTIAL_RESULTS: LDAP_RETCODE = LDAP_RETCODE(9i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_REFERRAL: LDAP_RETCODE = LDAP_RETCODE(10i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ADMIN_LIMIT_EXCEEDED: LDAP_RETCODE = LDAP_RETCODE(11i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNAVAILABLE_CRIT_EXTENSION: LDAP_RETCODE = LDAP_RETCODE(12i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONFIDENTIALITY_REQUIRED: LDAP_RETCODE = LDAP_RETCODE(13i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SASL_BIND_IN_PROGRESS: LDAP_RETCODE = LDAP_RETCODE(14i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_SUCH_ATTRIBUTE: LDAP_RETCODE = LDAP_RETCODE(16i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNDEFINED_TYPE: LDAP_RETCODE = LDAP_RETCODE(17i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INAPPROPRIATE_MATCHING: LDAP_RETCODE = LDAP_RETCODE(18i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONSTRAINT_VIOLATION: LDAP_RETCODE = LDAP_RETCODE(19i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ATTRIBUTE_OR_VALUE_EXISTS: LDAP_RETCODE = LDAP_RETCODE(20i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INVALID_SYNTAX: LDAP_RETCODE = LDAP_RETCODE(21i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_SUCH_OBJECT: LDAP_RETCODE = LDAP_RETCODE(32i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ALIAS_PROBLEM: LDAP_RETCODE = LDAP_RETCODE(33i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INVALID_DN_SYNTAX: LDAP_RETCODE = LDAP_RETCODE(34i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_IS_LEAF: LDAP_RETCODE = LDAP_RETCODE(35i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ALIAS_DEREF_PROBLEM: LDAP_RETCODE = LDAP_RETCODE(36i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INAPPROPRIATE_AUTH: LDAP_RETCODE = LDAP_RETCODE(48i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INVALID_CREDENTIALS: LDAP_RETCODE = LDAP_RETCODE(49i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_INSUFFICIENT_RIGHTS: LDAP_RETCODE = LDAP_RETCODE(50i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_BUSY: LDAP_RETCODE = LDAP_RETCODE(51i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNAVAILABLE: LDAP_RETCODE = LDAP_RETCODE(52i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_UNWILLING_TO_PERFORM: LDAP_RETCODE = LDAP_RETCODE(53i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_LOOP_DETECT: LDAP_RETCODE = LDAP_RETCODE(54i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SORT_CONTROL_MISSING: LDAP_RETCODE = LDAP_RETCODE(60i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OFFSET_RANGE_ERROR: LDAP_RETCODE = LDAP_RETCODE(61i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NAMING_VIOLATION: LDAP_RETCODE = LDAP_RETCODE(64i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OBJECT_CLASS_VIOLATION: LDAP_RETCODE = LDAP_RETCODE(65i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NOT_ALLOWED_ON_NONLEAF: LDAP_RETCODE = LDAP_RETCODE(66i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NOT_ALLOWED_ON_RDN: LDAP_RETCODE = LDAP_RETCODE(67i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ALREADY_EXISTS: LDAP_RETCODE = LDAP_RETCODE(68i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_OBJECT_CLASS_MODS: LDAP_RETCODE = LDAP_RETCODE(69i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_RESULTS_TOO_LARGE: LDAP_RETCODE = LDAP_RETCODE(70i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AFFECTS_MULTIPLE_DSAS: LDAP_RETCODE = LDAP_RETCODE(71i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_VIRTUAL_LIST_VIEW_ERROR: LDAP_RETCODE = LDAP_RETCODE(76i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_OTHER: LDAP_RETCODE = LDAP_RETCODE(80i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_SERVER_DOWN: LDAP_RETCODE = LDAP_RETCODE(81i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_LOCAL_ERROR: LDAP_RETCODE = LDAP_RETCODE(82i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_ENCODING_ERROR: LDAP_RETCODE = LDAP_RETCODE(83i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_DECODING_ERROR: LDAP_RETCODE = LDAP_RETCODE(84i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_TIMEOUT: LDAP_RETCODE = LDAP_RETCODE(85i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_AUTH_UNKNOWN: LDAP_RETCODE = LDAP_RETCODE(86i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_FILTER_ERROR: LDAP_RETCODE = LDAP_RETCODE(87i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_USER_CANCELLED: LDAP_RETCODE = LDAP_RETCODE(88i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_PARAM_ERROR: LDAP_RETCODE = LDAP_RETCODE(89i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_MEMORY: LDAP_RETCODE = LDAP_RETCODE(90i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONNECT_ERROR: LDAP_RETCODE = LDAP_RETCODE(91i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NOT_SUPPORTED: LDAP_RETCODE = LDAP_RETCODE(92i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_NO_RESULTS_RETURNED: LDAP_RETCODE = LDAP_RETCODE(94i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CONTROL_NOT_FOUND: LDAP_RETCODE = LDAP_RETCODE(93i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_MORE_RESULTS_TO_RETURN: LDAP_RETCODE = LDAP_RETCODE(95i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_CLIENT_LOOP: LDAP_RETCODE = LDAP_RETCODE(96i32);
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub const LDAP_REFERRAL_LIMIT_EXCEEDED: LDAP_RETCODE = LDAP_RETCODE(97i32);
impl ::core::marker::Copy for LDAP_RETCODE {}
impl ::core::clone::Clone for LDAP_RETCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LDAP_RETCODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LDAP_RETCODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for LDAP_RETCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LDAP_RETCODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct BerElement {
    pub opaque: ::windows::core::PSTR,
}
impl ::core::marker::Copy for BerElement {}
impl ::core::clone::Clone for BerElement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BerElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BerElement").field("opaque", &self.opaque).finish()
    }
}
unsafe impl ::windows::core::Abi for BerElement {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BerElement {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BerElement>()) == 0 }
    }
}
impl ::core::cmp::Eq for BerElement {}
impl ::core::default::Default for BerElement {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAP {
    pub ld_sb: LDAP_0,
    pub ld_host: ::windows::core::PSTR,
    pub ld_version: u32,
    pub ld_lberoptions: u8,
    pub ld_deref: u32,
    pub ld_timelimit: u32,
    pub ld_sizelimit: u32,
    pub ld_errno: u32,
    pub ld_matched: ::windows::core::PSTR,
    pub ld_error: ::windows::core::PSTR,
    pub ld_msgid: u32,
    pub Reserved3: [u8; 25],
    pub ld_cldaptries: u32,
    pub ld_cldaptimeout: u32,
    pub ld_refhoplimit: u32,
    pub ld_options: u32,
}
impl ::core::marker::Copy for LDAP {}
impl ::core::clone::Clone for LDAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP")
            .field("ld_sb", &self.ld_sb)
            .field("ld_host", &self.ld_host)
            .field("ld_version", &self.ld_version)
            .field("ld_lberoptions", &self.ld_lberoptions)
            .field("ld_deref", &self.ld_deref)
            .field("ld_timelimit", &self.ld_timelimit)
            .field("ld_sizelimit", &self.ld_sizelimit)
            .field("ld_errno", &self.ld_errno)
            .field("ld_matched", &self.ld_matched)
            .field("ld_error", &self.ld_error)
            .field("ld_msgid", &self.ld_msgid)
            .field("Reserved3", &self.Reserved3)
            .field("ld_cldaptries", &self.ld_cldaptries)
            .field("ld_cldaptimeout", &self.ld_cldaptimeout)
            .field("ld_refhoplimit", &self.ld_refhoplimit)
            .field("ld_options", &self.ld_options)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for LDAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAP {}
impl ::core::default::Default for LDAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAP_0 {
    pub sb_sd: usize,
    pub Reserved1: [u8; 41],
    pub sb_naddr: usize,
    pub Reserved2: [u8; 24],
}
impl ::core::marker::Copy for LDAP_0 {}
impl ::core::clone::Clone for LDAP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAP_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_0").field("sb_sd", &self.sb_sd).field("Reserved1", &self.Reserved1).field("sb_naddr", &self.sb_naddr).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAP_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAP_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAP_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAP_0 {}
impl ::core::default::Default for LDAP_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPAPIFeatureInfoA {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: ::windows::core::PSTR,
    pub ldapaif_version: i32,
}
impl ::core::marker::Copy for LDAPAPIFeatureInfoA {}
impl ::core::clone::Clone for LDAPAPIFeatureInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAPAPIFeatureInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPAPIFeatureInfoA").field("ldapaif_info_version", &self.ldapaif_info_version).field("ldapaif_name", &self.ldapaif_name).field("ldapaif_version", &self.ldapaif_version).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAPAPIFeatureInfoA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPAPIFeatureInfoA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPAPIFeatureInfoA>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPAPIFeatureInfoA {}
impl ::core::default::Default for LDAPAPIFeatureInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPAPIFeatureInfoW {
    pub ldapaif_info_version: i32,
    pub ldapaif_name: ::windows::core::PWSTR,
    pub ldapaif_version: i32,
}
impl ::core::marker::Copy for LDAPAPIFeatureInfoW {}
impl ::core::clone::Clone for LDAPAPIFeatureInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAPAPIFeatureInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPAPIFeatureInfoW").field("ldapaif_info_version", &self.ldapaif_info_version).field("ldapaif_name", &self.ldapaif_name).field("ldapaif_version", &self.ldapaif_version).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAPAPIFeatureInfoW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPAPIFeatureInfoW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPAPIFeatureInfoW>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPAPIFeatureInfoW {}
impl ::core::default::Default for LDAPAPIFeatureInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPAPIInfoA {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut *mut i8,
    pub ldapai_vendor_name: ::windows::core::PSTR,
    pub ldapai_vendor_version: i32,
}
impl ::core::marker::Copy for LDAPAPIInfoA {}
impl ::core::clone::Clone for LDAPAPIInfoA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAPAPIInfoA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPAPIInfoA").field("ldapai_info_version", &self.ldapai_info_version).field("ldapai_api_version", &self.ldapai_api_version).field("ldapai_protocol_version", &self.ldapai_protocol_version).field("ldapai_extensions", &self.ldapai_extensions).field("ldapai_vendor_name", &self.ldapai_vendor_name).field("ldapai_vendor_version", &self.ldapai_vendor_version).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAPAPIInfoA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPAPIInfoA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPAPIInfoA>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPAPIInfoA {}
impl ::core::default::Default for LDAPAPIInfoA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPAPIInfoW {
    pub ldapai_info_version: i32,
    pub ldapai_api_version: i32,
    pub ldapai_protocol_version: i32,
    pub ldapai_extensions: *mut ::windows::core::PWSTR,
    pub ldapai_vendor_name: ::windows::core::PWSTR,
    pub ldapai_vendor_version: i32,
}
impl ::core::marker::Copy for LDAPAPIInfoW {}
impl ::core::clone::Clone for LDAPAPIInfoW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAPAPIInfoW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPAPIInfoW").field("ldapai_info_version", &self.ldapai_info_version).field("ldapai_api_version", &self.ldapai_api_version).field("ldapai_protocol_version", &self.ldapai_protocol_version).field("ldapai_extensions", &self.ldapai_extensions).field("ldapai_vendor_name", &self.ldapai_vendor_name).field("ldapai_vendor_version", &self.ldapai_vendor_version).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAPAPIInfoW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPAPIInfoW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPAPIInfoW>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPAPIInfoW {}
impl ::core::default::Default for LDAPAPIInfoW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAPControlA {
    pub ldctl_oid: ::windows::core::PSTR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAPControlA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAPControlA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPControlA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPControlA").field("ldctl_oid", &self.ldctl_oid).field("ldctl_value", &self.ldctl_value).field("ldctl_iscritical", &self.ldctl_iscritical).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LDAPControlA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPControlA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPControlA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPControlA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPControlA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAPControlW {
    pub ldctl_oid: ::windows::core::PWSTR,
    pub ldctl_value: LDAP_BERVAL,
    pub ldctl_iscritical: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAPControlW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAPControlW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPControlW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPControlW").field("ldctl_oid", &self.ldctl_oid).field("ldctl_value", &self.ldctl_value).field("ldctl_iscritical", &self.ldctl_iscritical).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LDAPControlW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPControlW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPControlW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPControlW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPControlW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
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
    pub Connection: *mut LDAP,
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPMessage {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPMessage")
            .field("lm_msgid", &self.lm_msgid)
            .field("lm_msgtype", &self.lm_msgtype)
            .field("lm_ber", &self.lm_ber)
            .field("lm_chain", &self.lm_chain)
            .field("lm_next", &self.lm_next)
            .field("lm_time", &self.lm_time)
            .field("Connection", &self.Connection)
            .field("Request", &self.Request)
            .field("lm_returncode", &self.lm_returncode)
            .field("lm_referral", &self.lm_referral)
            .field("lm_chased", &self.lm_chased)
            .field("lm_eom", &self.lm_eom)
            .field("ConnectionReferenced", &self.ConnectionReferenced)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LDAPMessage {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPMessage {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPMessage>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPMessage {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPMessage {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPModA {
    pub mod_op: u32,
    pub mod_type: ::windows::core::PSTR,
    pub mod_vals: LDAPModA_0,
}
impl ::core::marker::Copy for LDAPModA {}
impl ::core::clone::Clone for LDAPModA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LDAPModA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPModA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPModA>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPModA {}
impl ::core::default::Default for LDAPModA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub union LDAPModA_0 {
    pub modv_strvals: *mut ::windows::core::PSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
impl ::core::marker::Copy for LDAPModA_0 {}
impl ::core::clone::Clone for LDAPModA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LDAPModA_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPModA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPModA_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPModA_0 {}
impl ::core::default::Default for LDAPModA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPModW {
    pub mod_op: u32,
    pub mod_type: ::windows::core::PWSTR,
    pub mod_vals: LDAPModW_0,
}
impl ::core::marker::Copy for LDAPModW {}
impl ::core::clone::Clone for LDAPModW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LDAPModW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPModW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPModW>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPModW {}
impl ::core::default::Default for LDAPModW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub union LDAPModW_0 {
    pub modv_strvals: *mut ::windows::core::PWSTR,
    pub modv_bvals: *mut *mut LDAP_BERVAL,
}
impl ::core::marker::Copy for LDAPModW_0 {}
impl ::core::clone::Clone for LDAPModW_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for LDAPModW_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPModW_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPModW_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPModW_0 {}
impl ::core::default::Default for LDAPModW_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LDAPSearch(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAPSortKeyA {
    pub sk_attrtype: ::windows::core::PSTR,
    pub sk_matchruleoid: ::windows::core::PSTR,
    pub sk_reverseorder: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAPSortKeyA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAPSortKeyA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPSortKeyA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPSortKeyA").field("sk_attrtype", &self.sk_attrtype).field("sk_matchruleoid", &self.sk_matchruleoid).field("sk_reverseorder", &self.sk_reverseorder).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LDAPSortKeyA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPSortKeyA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPSortKeyA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPSortKeyA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPSortKeyA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct LDAPSortKeyW {
    pub sk_attrtype: ::windows::core::PWSTR,
    pub sk_matchruleoid: ::windows::core::PWSTR,
    pub sk_reverseorder: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for LDAPSortKeyW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for LDAPSortKeyW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAPSortKeyW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPSortKeyW").field("sk_attrtype", &self.sk_attrtype).field("sk_matchruleoid", &self.sk_matchruleoid).field("sk_reverseorder", &self.sk_reverseorder).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LDAPSortKeyW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAPSortKeyW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPSortKeyW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAPSortKeyW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAPSortKeyW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAPVLVInfo {
    pub ldvlv_version: i32,
    pub ldvlv_before_count: u32,
    pub ldvlv_after_count: u32,
    pub ldvlv_offset: u32,
    pub ldvlv_count: u32,
    pub ldvlv_attrvalue: *mut LDAP_BERVAL,
    pub ldvlv_context: *mut LDAP_BERVAL,
    pub ldvlv_extradata: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for LDAPVLVInfo {}
impl ::core::clone::Clone for LDAPVLVInfo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAPVLVInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAPVLVInfo").field("ldvlv_version", &self.ldvlv_version).field("ldvlv_before_count", &self.ldvlv_before_count).field("ldvlv_after_count", &self.ldvlv_after_count).field("ldvlv_offset", &self.ldvlv_offset).field("ldvlv_count", &self.ldvlv_count).field("ldvlv_attrvalue", &self.ldvlv_attrvalue).field("ldvlv_context", &self.ldvlv_context).field("ldvlv_extradata", &self.ldvlv_extradata).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAPVLVInfo {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAPVLVInfo {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAPVLVInfo>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAPVLVInfo {}
impl ::core::default::Default for LDAPVLVInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAP_BERVAL {
    pub bv_len: u32,
    pub bv_val: ::windows::core::PSTR,
}
impl ::core::marker::Copy for LDAP_BERVAL {}
impl ::core::clone::Clone for LDAP_BERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAP_BERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_BERVAL").field("bv_len", &self.bv_len).field("bv_val", &self.bv_val).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAP_BERVAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAP_BERVAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAP_BERVAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAP_BERVAL {}
impl ::core::default::Default for LDAP_BERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LDAP_REFERRAL_CALLBACK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_REFERRAL_CALLBACK").field("SizeOfCallbacks", &self.SizeOfCallbacks).field("QueryForConnection", &self.QueryForConnection.map(|f| f as usize)).field("NotifyRoutine", &self.NotifyRoutine.map(|f| f as usize)).field("DereferenceRoutine", &self.DereferenceRoutine.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LDAP_REFERRAL_CALLBACK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LDAP_REFERRAL_CALLBACK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAP_REFERRAL_CALLBACK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LDAP_REFERRAL_CALLBACK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LDAP_REFERRAL_CALLBACK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
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
impl ::core::fmt::Debug for LDAP_TIMEVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_TIMEVAL").field("tv_sec", &self.tv_sec).field("tv_usec", &self.tv_usec).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAP_TIMEVAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAP_TIMEVAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAP_TIMEVAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAP_TIMEVAL {}
impl ::core::default::Default for LDAP_TIMEVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub struct LDAP_VERSION_INFO {
    pub lv_size: u32,
    pub lv_major: u32,
    pub lv_minor: u32,
}
impl ::core::marker::Copy for LDAP_VERSION_INFO {}
impl ::core::clone::Clone for LDAP_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LDAP_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LDAP_VERSION_INFO").field("lv_size", &self.lv_size).field("lv_major", &self.lv_major).field("lv_minor", &self.lv_minor).finish()
    }
}
unsafe impl ::windows::core::Abi for LDAP_VERSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for LDAP_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<LDAP_VERSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for LDAP_VERSION_INFO {}
impl ::core::default::Default for LDAP_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub type DBGPRINT = ::core::option::Option<unsafe extern "system" fn(format: ::windows::core::PCSTR) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub type DEREFERENCECONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut LDAP, connectiontodereference: *mut LDAP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type NOTIFYOFNEWCONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut LDAP, referralfromconnection: *mut LDAP, newdn: ::windows::core::PCWSTR, hostname: ::windows::core::PCSTR, newconnection: *mut LDAP, portnumber: u32, secauthidentity: *mut ::core::ffi::c_void, currentuser: *mut ::core::ffi::c_void, errorcodefrombind: u32) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Authentication_Identity\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Authentication_Identity", feature = "Win32_Security_Cryptography"))]
pub type QUERYCLIENTCERT = ::core::option::Option<unsafe extern "system" fn(connection: *mut LDAP, trusted_cas: *mut super::super::Security::Authentication::Identity::SecPkgContext_IssuerListInfoEx, ppcertificate: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`*"]
pub type QUERYFORCONNECTION = ::core::option::Option<unsafe extern "system" fn(primaryconnection: *mut LDAP, referralfromconnection: *mut LDAP, newdn: ::windows::core::PCWSTR, hostname: ::windows::core::PCSTR, portnumber: u32, secauthidentity: *mut ::core::ffi::c_void, currentusertoken: *mut ::core::ffi::c_void, connectiontouse: *mut *mut LDAP) -> u32>;
#[doc = "*Required features: `\"Win32_Networking_Ldap\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub type VERIFYSERVERCERT = ::core::option::Option<unsafe extern "system" fn(connection: *mut LDAP, pservercert: *mut *mut super::super::Security::Cryptography::CERT_CONTEXT) -> super::super::Foundation::BOOLEAN>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
