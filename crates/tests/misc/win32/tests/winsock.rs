use windows::Win32::Networking::WinSock::*;

const IPV4_ADDR: std::net::Ipv4Addr = std::net::Ipv4Addr::new(1, 2, 3, 4);
const IPV6_ADDR: std::net::Ipv6Addr = std::net::Ipv6Addr::new(
    0x0011, 0x2233, 0x4455, 0x6677, 0x8899, 0xaabb, 0xccdd, 0xeeff,
);
const PORT_HOST_BYTE_ORDER: u16 = 0x1234_u16;
const PORT_NETWORK_BYTE_ORDER: u16 = PORT_HOST_BYTE_ORDER.to_be();
const FLOWINFO_HOST_BYTE_ORDER: u32 = 0x12345678_u32;
const FLOWINFO_NETWORK_BYTE_ORDER: u32 = FLOWINFO_HOST_BYTE_ORDER.to_be();
const SCOPE_ID: u32 = 256_u32;

#[test]
fn in_addr() {
    fn in_addr_to_string(in_addr: &IN_ADDR) -> String {
        unsafe {
            let mut chars = [0; 24];
            inet_ntop(AF_INET.0 as i32, in_addr as *const IN_ADDR as _, &mut chars);
            std::ffi::CStr::from_ptr(chars.as_ptr() as *const _)
                .to_str()
                .unwrap()
                .to_owned()
        }
    }

    let in_addr: IN_ADDR = IPV4_ADDR.into();

    let ipv4_addr_s = format!("{IPV4_ADDR}");
    let in_addr_s = in_addr_to_string(&in_addr);

    // Compare the string values
    assert_eq!(ipv4_addr_s, in_addr_s);

    // Compare the u32 values, network byte order
    assert_eq!(unsafe { in_addr.S_un.S_addr }, u32::from(IPV4_ADDR).to_be());

    // Compare the round-trip
    assert_eq!(std::net::Ipv4Addr::from(in_addr), IPV4_ADDR);
}

#[test]
fn in6_addr() {
    let in6_addr: IN6_ADDR = IPV6_ADDR.into();

    assert_eq!(unsafe { in6_addr.u.Byte }, IPV6_ADDR.octets());

    // Compare the round-trip
    assert_eq!(std::net::Ipv6Addr::from(in6_addr), IPV6_ADDR)
}

#[test]
fn sockaddr_in() {
    let socket_addr_v4 = std::net::SocketAddrV4::new(IPV4_ADDR, PORT_HOST_BYTE_ORDER);
    let sockaddr_in: SOCKADDR_IN = socket_addr_v4.into();

    // These fields should be host byte order
    assert_eq!(sockaddr_in.sin_family, AF_INET);

    // These fields should be network byte order
    assert_eq!(sockaddr_in.sin_port, PORT_NETWORK_BYTE_ORDER);
    assert_eq!(
        unsafe { sockaddr_in.sin_addr.S_un.S_addr },
        u32::from(IPV4_ADDR).to_be()
    );
}

#[test]
fn sockaddr_in6() {
    let socket_addr_v6 = std::net::SocketAddrV6::new(
        IPV6_ADDR,
        PORT_HOST_BYTE_ORDER,
        FLOWINFO_HOST_BYTE_ORDER,
        SCOPE_ID,
    );
    let sockaddr_in6: SOCKADDR_IN6 = socket_addr_v6.into();

    // These fields should be host byte order
    assert_eq!(sockaddr_in6.sin6_family, AF_INET6);
    assert_eq!(sockaddr_in6.sin6_port, PORT_NETWORK_BYTE_ORDER);

    // These fields should be network byte order
    assert_eq!(sockaddr_in6.sin6_flowinfo, FLOWINFO_NETWORK_BYTE_ORDER);
    assert_eq!(
        unsafe { sockaddr_in6.sin6_addr.u.Byte },
        socket_addr_v6.ip().octets()
    );

    // This bitfield has no endianness
    assert_eq!(unsafe { sockaddr_in6.Anonymous.sin6_scope_id }, SCOPE_ID);
}

#[test]
fn sockaddr_inet4() {
    let socket_addr_v4 = std::net::SocketAddrV4::new(IPV4_ADDR, PORT_HOST_BYTE_ORDER);
    let sockaddr_inet: SOCKADDR_INET = socket_addr_v4.into();

    // These fields should be host byte order
    assert_eq!(unsafe { sockaddr_inet.si_family }, AF_INET);
    assert_eq!(unsafe { sockaddr_inet.Ipv4.sin_family }, AF_INET);

    // These fields should be network byte order
    assert_eq!(
        unsafe { sockaddr_inet.Ipv4.sin_port },
        PORT_NETWORK_BYTE_ORDER
    );
    assert_eq!(
        unsafe { sockaddr_inet.Ipv4.sin_addr.S_un.S_addr },
        u32::from(IPV4_ADDR).to_be()
    );
}

#[test]
fn sockaddr_inet6() {
    let socket_addr_v6 = std::net::SocketAddrV6::new(
        IPV6_ADDR,
        PORT_HOST_BYTE_ORDER,
        FLOWINFO_HOST_BYTE_ORDER,
        SCOPE_ID,
    );
    let sockaddr_inet: SOCKADDR_INET = socket_addr_v6.into();

    // These fields should be host byte order
    assert_eq!(unsafe { sockaddr_inet.si_family }, AF_INET6);
    assert_eq!(unsafe { sockaddr_inet.Ipv6.sin6_family }, AF_INET6);

    // These fields should be network byte order
    assert_eq!(
        unsafe { sockaddr_inet.Ipv6.sin6_port },
        PORT_NETWORK_BYTE_ORDER
    );
    assert_eq!(
        unsafe { sockaddr_inet.Ipv6.sin6_flowinfo },
        FLOWINFO_NETWORK_BYTE_ORDER
    );
    assert_eq!(
        unsafe { sockaddr_inet.Ipv6.sin6_addr.u.Byte },
        socket_addr_v6.ip().octets()
    );

    // This bitfield has no endianness
    assert_eq!(
        unsafe { sockaddr_inet.Ipv6.Anonymous.sin6_scope_id },
        SCOPE_ID
    );
}
