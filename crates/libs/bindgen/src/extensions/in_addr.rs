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
        impl ::core::convert::From<IN_ADDR> for ::std::net::Ipv4Addr {
            fn from(in_addr: IN_ADDR) -> Self {
                // SAFETY: this is safe because the union variants are just views of the same exact data
                Self::from( unsafe { in_addr.S_un.S_addr } )
            }
        }
        impl ::core::convert::From<&IN_ADDR> for  ::std::net::Ipv4Addr{
            fn from(in_addr: &IN_ADDR) -> Self {
                // SAFETY: this is safe because the union variants are just views of the same exact data
                Self::from( unsafe { in_addr.S_un.S_addr } )
            }
        }
    }
}
