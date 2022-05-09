use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl ::core::convert::From<::std::net::SocketAddrV6> for SOCKADDR_IN6 {
            fn from(addr: ::std::net::SocketAddrV6) -> Self {
                SOCKADDR_IN6 {
                    sin6_family: AF_INET6.0 as u16,
                    sin6_port: addr.port(),
                    sin6_flowinfo: addr.flowinfo(),
                    sin6_addr: addr.ip().into(),
                    Anonymous: SOCKADDR_IN6_0 { sin6_scope_id: addr.scope_id() },
                    ..Default::default()
                }
            }
        }
        impl ::core::convert::From<&::std::net::SocketAddrV6> for SOCKADDR_IN6 {
            fn from(addr: &::std::net::SocketAddrV6) -> Self {
                SOCKADDR_IN6 {
                    sin6_family: AF_INET6.0 as u16,
                    sin6_port: addr.port(),
                    sin6_flowinfo: addr.flowinfo(),
                    sin6_addr: addr.ip().into(),
                    Anonymous: SOCKADDR_IN6_0 { sin6_scope_id: addr.scope_id() },
                    ..Default::default()
                }
            }
        }
    }
}
