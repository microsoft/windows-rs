use windows::Win32::Networking::WinSock::*;

#[test]
fn from_ipv4addr() {
    let ipv4addr = std::net::Ipv4Addr::new(4, 3, 2, 1);
    let in_addr: IN_ADDR = ipv4addr.into();
    unsafe {
        assert!(in_addr.S_un.S_un_b.s_b1 == 1);
        assert!(in_addr.S_un.S_un_b.s_b2 == 2);
        assert!(in_addr.S_un.S_un_b.s_b3 == 3);
        assert!(in_addr.S_un.S_un_b.s_b4 == 4);
    }
}

#[test]
fn from_ipv6addr() {
    let ipv6addr = std::net::Ipv6Addr::new(0x0011, 0x2233, 0x4455, 0x6677, 0x8899, 0xaabb, 0xccdd, 0xeeff);
    let in6_addr: IN6_ADDR = ipv6addr.into();
    unsafe {
        assert!(in6_addr.u.Byte == ipv6addr.octets());
    }

    let ipv6addr: std::net::Ipv6Addr = in6_addr.into();
    unsafe {
        assert!(in6_addr.u.Byte == ipv6addr.octets());
    }
}
