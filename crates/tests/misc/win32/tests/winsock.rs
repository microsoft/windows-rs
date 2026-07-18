#![cfg(windows)]
use windows::Win32::*;
use windows::Win32::*;
use windows::Win32::*;

// The `windows` crate no longer bundles std <-> WinSock conversion extensions.
// These free functions show how a caller can convert between `std::net` types and
// the raw WinSock structs themselves.

fn to_in_addr(addr: std::net::Ipv4Addr) -> IN_ADDR {
    // S_addr is network (big-endian) byte order; u32::from is host byte order.
    IN_ADDR {
        S_un: IN_ADDR_0 {
            S_addr: u32::from(addr).to_be(),
        },
    }
}

fn from_in_addr(in_addr: IN_ADDR) -> std::net::Ipv4Addr {
    std::net::Ipv4Addr::from(u32::from_be(unsafe { in_addr.S_un.S_addr }))
}

fn to_in6_addr(addr: std::net::Ipv6Addr) -> IN6_ADDR {
    IN6_ADDR {
        u: IN6_ADDR_0 {
            Byte: addr.octets(),
        },
    }
}

fn from_in6_addr(in6_addr: IN6_ADDR) -> std::net::Ipv6Addr {
    std::net::Ipv6Addr::from(unsafe { in6_addr.u.Byte })
}

fn to_sockaddr_in(addr: std::net::SocketAddrV4) -> SOCKADDR_IN {
    // sin_port must be network byte order; addr.port() is host byte order.
    SOCKADDR_IN {
        sin_family: ADDRESS_FAMILY(AF_INET as u16),
        sin_port: addr.port().to_be(),
        sin_addr: to_in_addr(*addr.ip()),
        ..Default::default()
    }
}

fn to_sockaddr_in6(addr: std::net::SocketAddrV6) -> SOCKADDR_IN6 {
    // sin6_port and sin6_flowinfo are network byte order; sin6_scope_id has no endianness.
    SOCKADDR_IN6 {
        sin6_family: ADDRESS_FAMILY(AF_INET6 as u16),
        sin6_port: addr.port().to_be(),
        sin6_flowinfo: addr.flowinfo().to_be(),
        sin6_addr: to_in6_addr(*addr.ip()),
        Anonymous: SOCKADDR_IN6_LH_0 {
            sin6_scope_id: addr.scope_id(),
        },
    }
}

fn to_sockaddr_inet_v4(addr: std::net::SocketAddrV4) -> SOCKADDR_INET {
    SOCKADDR_INET {
        Ipv4: to_sockaddr_in(addr),
    }
}

fn to_sockaddr_inet_v6(addr: std::net::SocketAddrV6) -> SOCKADDR_INET {
    SOCKADDR_INET {
        Ipv6: to_sockaddr_in6(addr),
    }
}

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
            let mut chars = [0u8; 24];
            inet_ntop(AF_INET as i32, in_addr as *const IN_ADDR as _, &mut chars);
            std::ffi::CStr::from_ptr(chars.as_ptr() as *const _)
                .to_str()
                .unwrap()
                .to_owned()
        }
    }

    let in_addr: IN_ADDR = to_in_addr(IPV4_ADDR);

    let ipv4_addr_s = format!("{IPV4_ADDR}");
    let in_addr_s = in_addr_to_string(&in_addr);

    // Compare the string values
    assert_eq!(ipv4_addr_s, in_addr_s);

    // Compare the u32 values, network byte order
    assert_eq!(unsafe { in_addr.S_un.S_addr }, u32::from(IPV4_ADDR).to_be());

    // Compare the round-trip
    assert_eq!(from_in_addr(in_addr), IPV4_ADDR);
}

#[test]
fn in6_addr() {
    let in6_addr: IN6_ADDR = to_in6_addr(IPV6_ADDR);

    assert_eq!(unsafe { in6_addr.u.Byte }, IPV6_ADDR.octets());

    // Compare the round-trip
    assert_eq!(from_in6_addr(in6_addr), IPV6_ADDR);
}

#[test]
fn sockaddr_in() {
    let socket_addr_v4 = std::net::SocketAddrV4::new(IPV4_ADDR, PORT_HOST_BYTE_ORDER);
    let sockaddr_in: SOCKADDR_IN = to_sockaddr_in(socket_addr_v4);

    // These fields should be host byte order
    assert_eq!(sockaddr_in.sin_family, ADDRESS_FAMILY(AF_INET as u16));

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
    let sockaddr_in6: SOCKADDR_IN6 = to_sockaddr_in6(socket_addr_v6);

    // These fields should be host byte order
    assert_eq!(sockaddr_in6.sin6_family, ADDRESS_FAMILY(AF_INET6 as u16));
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
    let sockaddr_inet: SOCKADDR_INET = to_sockaddr_inet_v4(socket_addr_v4);

    // These fields should be host byte order
    assert_eq!(
        unsafe { sockaddr_inet.si_family },
        ADDRESS_FAMILY(AF_INET as u16)
    );
    assert_eq!(
        unsafe { sockaddr_inet.Ipv4.sin_family },
        ADDRESS_FAMILY(AF_INET as u16)
    );

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
    let sockaddr_inet: SOCKADDR_INET = to_sockaddr_inet_v6(socket_addr_v6);

    // These fields should be host byte order
    assert_eq!(
        unsafe { sockaddr_inet.si_family },
        ADDRESS_FAMILY(AF_INET6 as u16)
    );
    assert_eq!(
        unsafe { sockaddr_inet.Ipv6.sin6_family },
        ADDRESS_FAMILY(AF_INET6 as u16)
    );

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
