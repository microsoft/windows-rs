use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl ::core::convert::From<::std::net::Ipv6Addr> for IN6_ADDR {
            fn from(addr: ::std::net::Ipv6Addr) -> Self {
                Self { u: IN6_ADDR_0 { Byte: addr.octets() } }
            }
        }
        impl ::core::convert::From<&::std::net::Ipv6Addr> for IN6_ADDR {
            fn from(addr: &::std::net::Ipv6Addr) -> Self {
                Self { u: IN6_ADDR_0 { Byte: addr.octets() } }
            }
        }
        impl ::core::convert::From<IN6_ADDR> for ::std::net::Ipv6Addr {
            fn from(in6_addr: IN6_ADDR) -> Self {
                Self::from( unsafe { in6_addr.u.Byte } )
            }
        }
        impl ::core::convert::From<&IN6_ADDR> for ::std::net::Ipv6Addr {
            fn from(in6_addr: &IN6_ADDR) -> Self {
                Self::from( unsafe { in6_addr.u.Byte } )
            }
        }
    }
}
