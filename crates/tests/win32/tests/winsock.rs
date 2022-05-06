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

