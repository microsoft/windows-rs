use super::*;

pub fn gen() -> TokenStream {
    quote! {
        impl ::core::convert::From<::std::net::Ipv4Addr> for IN_ADDR {
            fn from(addr: ::std::net::Ipv4Addr) -> Self {
                Self { S_un: IN_ADDR_0 { S_addr: u32::from(addr) } }
            }
        }
        impl ::core::convert::From<&::std::net::Ipv4Addr> for IN_ADDR {
            fn from(addr: &::std::net::Ipv4Addr) -> Self {
                Self { S_un: IN_ADDR_0 { S_addr: u32::from(*addr) } }
            }
        }
        impl ::core::convert::Into<::std::net::Ipv4Addr> for &IN_ADDR {
            fn into(self) -> ::std::net::Ipv4Addr {
                ::std::net::Ipv4Addr::from( unsafe { self.S_un.S_addr } )
            }
        }
        impl ::core::convert::Into<::std::net::Ipv4Addr> for IN_ADDR {
            fn into(self) -> ::std::net::Ipv4Addr {
                ::std::net::Ipv4Addr::from( unsafe { self.S_un.S_addr } )
            }
        }
    }
}
