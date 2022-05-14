use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl ::core::convert::From<::std::net::SocketAddrV6> for SOCKADDR_IN6 {
            fn from(addr: ::std::net::SocketAddrV6) -> Self {
                // addr.port() and addr.flowinfo() are in host byte order
                // sin6_port, sin6_flowinfo, and sin6_scope_id must be big-endian, network byte order
                // sin6_scope_id is a bitfield without endianness
                SOCKADDR_IN6 {
                    sin6_family: AF_INET6.0 as u16,
                    sin6_port: addr.port().to_be(),
                    sin6_flowinfo: addr.flowinfo().to_be(),
                    sin6_addr: addr.ip().into(),
                    Anonymous: SOCKADDR_IN6_0 { sin6_scope_id: addr.scope_id() },
                    ..Default::default()
                }
            }
        }
        impl ::core::convert::From<&::std::net::SocketAddrV6> for SOCKADDR_IN6 {
            fn from(addr: &::std::net::SocketAddrV6) -> Self {
                // addr.port() and addr.flowinfo() are in host byte order
                // sin6_port, sin6_flowinfo, and sin6_scope_id must be big-endian, network byte order
                // sin6_scope_id is a bitfield without endianness
                SOCKADDR_IN6 {
                    sin6_family: AF_INET6.0 as u16,
                    sin6_port: addr.port().to_be(),
                    sin6_flowinfo: addr.flowinfo().to_be(),
                    sin6_addr: addr.ip().into(),
                    Anonymous: SOCKADDR_IN6_0 { sin6_scope_id: addr.scope_id() },
                    ..Default::default()
                }
            }
        }
    }
}
