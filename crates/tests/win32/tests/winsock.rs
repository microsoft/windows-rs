use windows::Win32::Networking::WinSock::*;

#[test]
fn in_addr() {
    let ipv4_addr = std::net::Ipv4Addr::new(4, 3, 2, 1);
    let in_addr: IN_ADDR = ipv4_addr.into();

    assert_eq!(unsafe { in_addr.S_un.S_un_b.s_b1 }, 1);
    assert_eq!(unsafe { in_addr.S_un.S_un_b.s_b2 }, 2);
    assert_eq!(unsafe { in_addr.S_un.S_un_b.s_b3 }, 3);
    assert_eq!(unsafe { in_addr.S_un.S_un_b.s_b4 }, 4);

    let ipv4_addr: std::net::Ipv4Addr = in_addr.into();
    assert_eq!(ipv4_addr, std::net::Ipv4Addr::new(4, 3, 2, 1));
}

#[test]
fn in6_addr() {
    let ipv6_addr = std::net::Ipv6Addr::new(0x0011, 0x2233, 0x4455, 0x6677, 0x8899, 0xaabb, 0xccdd, 0xeeff);
    let in6_addr: IN6_ADDR = ipv6_addr.into();

    assert_eq!(unsafe { in6_addr.u.Byte }, ipv6_addr.octets());

    let ipv6_addr: std::net::Ipv6Addr = in6_addr.into();
    assert_eq!(ipv6_addr, std::net::Ipv6Addr::new(0x0011, 0x2233, 0x4455, 0x6677, 0x8899, 0xaabb, 0xccdd, 0xeeff));
}

#[test]
fn sockaddr_in() {
    let socket_addr_v4 = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(5, 6, 7, 8), 1234);
    let sockaddr_in: SOCKADDR_IN = socket_addr_v4.into();

    assert_eq!(sockaddr_in.sin_family, AF_INET.0 as u16);
    assert_eq!(sockaddr_in.sin_port, 1234);
    assert_eq!(unsafe { sockaddr_in.sin_addr.S_un.S_addr }, 0x05060708);
}

#[test]
fn sockaddr_in6() {
    let socket_addr_v6 = std::net::SocketAddrV6::new(std::net::Ipv6Addr::new(0x0011, 0x2233, 0x4455, 0x6677, 0x8899, 0xaabb, 0xccdd, 0xeeff), 1234, 56, 78);
    let sockaddr_in6: SOCKADDR_IN6 = socket_addr_v6.into();

    assert_eq!(sockaddr_in6.sin6_family, AF_INET6.0 as u16);
    assert_eq!(sockaddr_in6.sin6_port, 1234);
    assert_eq!(sockaddr_in6.sin6_flowinfo, 56);
    assert_eq!(unsafe { sockaddr_in6.sin6_addr.u.Byte }, socket_addr_v6.ip().octets());
}

#[test]
fn sockaddr_inet() {
    let socket_addr_v4 = std::net::SocketAddrV4::new(std::net::Ipv4Addr::new(5, 6, 7, 8), 1234);
    let sockaddr_inet: SOCKADDR_INET = socket_addr_v4.into();
    assert_eq!(unsafe { sockaddr_inet.si_family }, AF_INET.0 as u16);
    assert_eq!(unsafe { sockaddr_inet.Ipv4.sin_family }, AF_INET.0 as u16);
    assert_eq!(unsafe { sockaddr_inet.Ipv4.sin_port }, 1234);
    assert_eq!(unsafe { sockaddr_inet.Ipv4.sin_addr.S_un.S_addr }, 0x05060708);

    let socket_addr_v6 = std::net::SocketAddrV6::new(std::net::Ipv6Addr::new(0x0011, 0x2233, 0x4455, 0x6677, 0x8899, 0xaabb, 0xccdd, 0xeeff), 1234, 56, 78);
    let sockaddr_inet: SOCKADDR_INET = socket_addr_v6.into();
    assert_eq!(unsafe { sockaddr_inet.si_family }, AF_INET6.0 as u16);
    assert_eq!(unsafe { sockaddr_inet.Ipv6.sin6_family }, AF_INET6.0 as u16);
    assert_eq!(unsafe { sockaddr_inet.Ipv6.sin6_port }, 1234);
    assert_eq!(unsafe { sockaddr_inet.Ipv6.sin6_flowinfo }, 56);
    assert_eq!(unsafe { sockaddr_inet.Ipv6.sin6_addr.u.Byte }, socket_addr_v6.ip().octets());
}