use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl ::core::convert::From<::std::net::SocketAddrV4> for SOCKADDR_IN {
            fn from(addr: ::std::net::SocketAddrV4) -> Self {
                SOCKADDR_IN {
                    sin_family: AF_INET.0 as u16,
                    sin_port: addr.port(),
                    sin_addr: addr.ip().into(),
                    ..Default::default()
                }
            }
        }
        impl ::core::convert::From<&::std::net::SocketAddrV4> for SOCKADDR_IN {
            fn from(addr: &::std::net::SocketAddrV4) -> Self {
                SOCKADDR_IN {
                    sin_family: AF_INET.0 as u16,
                    sin_port: addr.port(),
                    sin_addr: addr.ip().into(),
                    ..Default::default()
                }
            }
        }
    }
}
